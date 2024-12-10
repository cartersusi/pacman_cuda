use std::process::{Command, Stdio};
use std::panic;

use serde::{Deserialize, Serialize};
use dialoguer::{Select, theme::ColorfulTheme};
use dialoguer::console::{Term, Style, Color};
use ctrlc;

const JSONDATA: &[u8] = include_bytes!("data/packages.json");
const TMPDIR: &str = "/tmp/cuda_installer";
const CHOICES: &[&str] = &["Recent", "Compatible"];

#[derive(Debug, Serialize, Deserialize)]
struct Pkg {
    version: f32,
    name: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Deps {
    support: Vec<String>,
    gcc: Pkg,
    gcc_libs: Pkg,
    cuda: Pkg,
    cudnn: Pkg,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    deps: String,
    recent: Deps,
    compatible: Deps,
}
enum Lvl {
    Warning,
    Success,
    Error,
    Standard,
}

fn log(msg_type: Lvl, msg: &str) {
    let color_code = match msg_type {
        Lvl::Warning => Style::new().fg(Color::Yellow).bold(),
        Lvl::Success => Style::new().fg(Color::Green).bold(),
        Lvl::Error => Style::new().fg(Color::Red).bold(),
        Lvl::Standard => Style::new().fg(Color::White).bold(),
    };
    Term::stdout().write_line(&color_code.apply_to(msg).to_string()).unwrap();
}

fn main() {
    ctrlc::set_handler(|| {
        log(Lvl::Error, "Installation was interrupted by the user. Cleaning up...");
        end(true);
    }).expect("Error setting Ctrl+C handler");

    let result = panic::catch_unwind(|| {
        let json_from_bytes = std::str::from_utf8(JSONDATA).expect("Failed to parse JSON file");
        let data: Data = serde_json::from_str(json_from_bytes).expect("Failed to parse JSON file");

        data.start();

        let versions = &["Recent", "Compatible"];
        let versions = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to install the recent or compatible version?")
            .items(&versions[..])
            .default(0)
            .interact()
            .expect("Failed to get user choice");

        let c = versions + 1;
        log(Lvl::Standard, &format!("\nSupporting packages for `{}` version:", CHOICES[c as usize - 1]));
        let deps = match c {
            1 => &data.recent,
            2 => &data.compatible,
            _ => {
                log(Lvl::Error, "Invalid Choice");
                panic!();
            }
        };

        deps.print_support();

        let cont = &["Yes", "No"];
        let cont = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to proceed with the installation?")
            .items(&cont[..])
            .default(0)
            .interact()
            .expect("Failed to get user choice");

        if cont == 1 {
            end(true);
        }

        deps.download();
        deps.install();

        end(false);
    });

    if let Err(_) = result {
        end(true);
    }
}

impl Data {
    fn start(&self) {
        self.install_deps();
    
        let cmd = format!("mkdir -p {}", TMPDIR);
        match run(&cmd, false) {
            Ok(_) => log(Lvl::Success, &format!("Tmp directory created: {}", TMPDIR)),
            Err(error) => {
                log(Lvl::Error, &format!("Failed to create tmp directory: {}", error));
                panic!();
            }
        }
    }

    fn install_deps(&self) {
        let cmd = format!("sudo pacman -Syu {}", &self.deps);
        match run(&cmd, true) {
            Ok(_) => log(Lvl::Success, &format!("Dependencies installed successfully: {}", &self.deps)),
            Err(error) => log(Lvl::Error, &format!("Failed to install dependencies: {}", error)),
        }
    }
}

impl Deps {
    fn packages(&self) -> [(&str, &str); 4] {
        [
            (&self.gcc.link, &self.gcc.name),
            (&self.gcc_libs.link, &self.gcc_libs.name),
            (&self.cuda.link, &self.cuda.name),
            (&self.cudnn.link, &self.cudnn.name),
        ]
    }

    fn print_support(&self) {
        for supported in self.support.iter() {
            log(Lvl::Standard, &format!("\t{}", supported));
        }
        println!();
    }

    fn download(&self) {
        for (link, name) in self.packages() {
            if !verify_link(link) {
                log(Lvl::Error, "Invalid link");
                panic!();
            }
            let cmd = format!("curl -o {}/{} {}", TMPDIR, name, link);
            match run(&cmd, true) {
                Ok(_) => log(Lvl::Success, &format!("Package downloaded successfully: {}", name)),
                Err(error) => {
                    log(Lvl::Error, &format!("Failed to download package: {}", error));
                    panic!();
                }
            }
        }
    }

    fn install(&self) {
        let gcc = format!("{}/{} {}/{}", TMPDIR, self.gcc.name, TMPDIR, self.gcc_libs.name);
        let gcccmd = format!("sudo pacman -U {}", gcc);
        match run(&gcccmd, true) {
            Ok(_) => log(Lvl::Success, &format!("Package installed successfully: {}, {}", self.gcc.name, self.gcc_libs.name)),
            Err(error) => log(Lvl::Error, &format!(" Packages were not installed successfully: {}", error)),
        }

        let cuda = format!("{}/{} {}/{}", TMPDIR, self.cuda.name, TMPDIR, self.cudnn.name);
        let cudacmd = format!("sudo pacman -U {}", cuda);
        match run(&cudacmd, true) {
            Ok(_) => log(Lvl::Success, &format!("Package installed successfully: {}, {}", self.cuda.name, self.cudnn.name)),
            Err(error) => log(Lvl::Error, &format!(" Packages were not installed successfully: {}", error)),
        }
    }
}

fn run(cmd: &str, interactive: bool) -> Result<String, String> {
    println!("Running command: {}", cmd);
    let output = if interactive {
        Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    } else {
        Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                log(Lvl::Success, "Command executed successfully");
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                let exit_code = output.status.code().unwrap_or(-1);
                if exit_code == 1 {
                    println!("User declined to proceed with the installation.");
                    Err("Operation was canceled by the user.".to_string())
                } else {
                    let error_msg = format!(
                        "Command executed with errors: {}\nError: {}",
                        cmd,
                        String::from_utf8_lossy(&output.stderr)
                    );
                    Err(error_msg)
                }
            }
        }
        Err(error) => {
            let error_msg = format!("Failed to execute command: {}", error);
            Err(error_msg)
        }
    }
}

fn verify_link(link: &str) -> bool {
    let cmd = format!("curl -I {}", link);
    match run(&cmd, false) {
        Ok(output) => if output.contains("HTTP/2 200") {
            log(Lvl::Success, "Valid link");
            return true;
        },
        Err(error) => eprintln!("Failed to check link: {}", error),
    }

    false
}

fn end(panicking: bool) {
    let cmd = format!("rm -rf {}", TMPDIR);
    match run(&cmd, false) {
        Ok(_) => log(Lvl::Success, &format!("Tmp directory removed: {}", TMPDIR)),
        Err(error) => {
            log(Lvl::Error, &format!("Failed to remove tmp directory: {}", error));
            log(Lvl::Warning, &format!("Please remove the tmp directory manually at {}", TMPDIR));

        }
    }

    if panicking {
        log(Lvl::Error, "Installation exited!");
        std::process::exit(1);
    }
    log(Lvl::Success, "Installation completed successfully!");

    println!("\nTo prevent the system from updating the packages, add the following to /etc/pacman.conf");
    log(Lvl::Standard, "\tIgnorePkg = cuda cudnn gcc12 gcc12-libs");

    println!("\nCommon Error\n# ERROR: libdevice not found at ./libdevice.10.bc");
    println!("To fix this error, run the following command:");
    log(Lvl::Standard, "\texport XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda");
}