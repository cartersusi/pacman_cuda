use std::process::{Command, Stdio};
use std::panic;

use serde::{Deserialize, Serialize};
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
}

fn log(msg_type: Lvl, msg: &str) -> String {
    let color_code = match msg_type {
        Lvl::Warning => "0;33",
        Lvl::Success => "0;32",
        Lvl::Error => "0;31",
    };
    format!("\x1b[{}m{}\x1b[0m", color_code, msg)
}

fn main() {
    ctrlc::set_handler(|| {
        println!("{}", log(Lvl::Error, "Installation was interrupted by the user"));
        end(true);
    }).expect("Error setting Ctrl+C handler");

    let result = panic::catch_unwind(|| {
        let json_from_bytes = std::str::from_utf8(JSONDATA).expect(&log(Lvl::Error, "Failed to convert JSON data to string"));
        let data: Data = serde_json::from_str(json_from_bytes).expect(&log(Lvl::Error, "Failed to parse JSON file"));

        data.start();

        println!("Would you like to install the recent or compatible version?\n1. Recent\n2. Compatible");

        let c = get_choice(3, "1", "2").expect(&log(Lvl::Error, "Failed to get choice"));
        let c = c.parse::<i32>().expect(&log(Lvl::Error, "Failed to parse choice"));

        println!("Supporting packages for `{}` version:", CHOICES[c as usize - 1]);
        let deps = match c {
            1 => &data.recent,
            2 => &data.compatible,
            _ => panic!("{}", log(Lvl::Error, "Invalid choice")),
        };

        deps.print_support();
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
            Ok(output) => println!("Command output: {}", output),
            Err(error) => panic!("{}", log(Lvl::Error, &format!("Failed to create tmp directory: {}", error))),
        }
    }

    fn install_deps(&self) {
        let cmd = format!("sudo pacman -Syu {}", &self.deps);
        match run(&cmd, true) {
            Ok(output) => println!("Command output: {}", output),
            Err(error) => eprintln!("{}", log(Lvl::Error, &format!("Failed to install dependencies: {}", error))),
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
            println!("{}", supported);
        }
    }

    fn download(&self) {
        for (link, name) in self.packages() {
            if !verify_link(link) {
                panic!("{}", log(Lvl::Error, "Invalid link"));
            }
            let cmd = format!("curl -o {}/{} {}", TMPDIR, name, link);
            match run(&cmd, true) {
                Ok(output) => println!("{}", log(Lvl::Success, &format!("Package downloaded successfully: {}", output))),
                Err(error) => panic!("{}", log(Lvl::Error, &format!("Failed to download package: {}", error))),
            }
        }
    }

    fn install(&self) {
        for (_, name) in self.packages() {
            let cmd = format!("sudo pacman -U {}/{}", TMPDIR, name);
            match run(&cmd, true) {
                Ok(output) => println!("{}", log(Lvl::Success, &format!("Package installed successfully: {}", output))),
                Err(error) => panic!("{}", log(Lvl::Error, &format!("Failed to install package: {}", error))),
            }
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
                println!("{}", log(Lvl::Success, "Command executed successfully"));
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

fn get_choice(tries: i32, c1: &str, c2: &str) -> Result<String, String> {
    let mut choice = String::new();
    for _ in 0..tries {
        match std::io::stdin().read_line(&mut choice) {
            Ok(_) => {
                choice = choice.trim().to_string();
                if choice == c1 || choice == c2 {
                    return Ok(choice);
                }
                choice.clear();
                eprintln!("{}", log(Lvl::Warning, "Invalid choice"));
            }
            Err(error) => {
                choice.clear();
                eprintln!("Failed to read input: {}", error);
            }
        }
    }

    Err("Failed to read input".to_string())
}

fn verify_link(link: &str) -> bool {
    let cmd = format!("curl -I {}", link);
    match run(&cmd, false) {
        Ok(output) => if output.contains("HTTP/2 200") {
            println!("{}" , log(Lvl::Success, "Link is valid"));
            return true;
        },
        Err(error) => eprintln!("Failed to check link: {}", error),
    }

    false
}

fn end(panicking: bool) {
    let cmd = format!("rm -rf {}", TMPDIR);
    match run(&cmd, false) {
        Ok(output) => println!("Command output: {}", output),
        Err(error) => panic!("Failed to remove tmp directory: {}", error),
    }

    if panicking {
        println!("{}", log(Lvl::Error, "Installation failed!"));
        std::process::exit(1);
    }
    println!("{}", log(Lvl::Success, "Installation completed successfully!"));
    println!("To prevent the system from updating the packages, add the following to /etc/pacman.conf");
    println!("\tIgnorePkg = cuda cudnn gcc12 gcc12-libs");
    println!("Common Error: # ERROR: libdevice not found at ./libdevice.10.bc");
    println!("To fix this error, run the following command:\n\texport XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda");
}