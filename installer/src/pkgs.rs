use serde::{Deserialize, Serialize};

use crate::cli::{Lvl, log, get_choice};
use crate::exec::{run, verify_link, mktmp};

const JSONDATA: &[u8] = include_bytes!("data/packages.json");

const TMPDIR: &str = "/tmp/cuda_installer";
const CHOICES: &[&str] = &["Recent", "Compatible"];
const INSTALL: &str = "sudo pacman -U";
const UPDATE: &str = "sudo pacman -Syu";
const DOWNLOAD: &str = "curl -o";
const QUERY: &str = "pacman -Q";

#[derive(Debug, Serialize, Deserialize)]
struct Pkg {
    pkg: String,
    version: f32,
    name: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pkgs {
    support: Vec<String>,
    gcc: Pkg,
    gcc_libs: Pkg,
    cuda: Pkg,
    cudnn: Pkg,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    deps: String,
    recent: Pkgs,
    compatible: Pkgs,
}

impl Data {
    pub fn load() -> Self {
        let json_from_bytes = std::str::from_utf8(JSONDATA).expect("Failed to parse JSON file");
        serde_json::from_str(json_from_bytes).expect("Failed to parse JSON file")
    }

    fn install_deps(&self) {
        let cmd = format!("{} {}", UPDATE, &self.deps);
        match run(&cmd, true, true) {
            Ok(_) => log(Lvl::Success, &format!("Dependencies installed successfully: {}", &self.deps)),
            Err(error) => log(Lvl::Error, &format!("Failed to install dependencies: {}", error)),
        }
    }

    pub fn start(&self) {
        self.install_deps();
        mktmp();
    }

    pub fn version(&self, prompt: &str) -> &Pkgs {
        let v = get_choice(CHOICES[0], CHOICES[1], prompt);

        log(Lvl::Info, &format!("\nSupporting packages for `{}` version:", CHOICES[v]));
        let deps = match v {
            0 => &self.recent,
            1 => &self.compatible,
            _ => panic!("Unknown Error Occurred"),
        };

        deps
    }
}

impl Pkgs {
    fn packages(&self) -> [(&str, &f32, &str, &str); 4] {
        [
            (&self.gcc.pkg, &self.gcc.version, &self.gcc.link, &self.gcc.name),                     //Priority 1
            (&self.gcc_libs.pkg, &self.gcc_libs.version, &self.gcc_libs.link, &self.gcc_libs.name), //Priority 2
            (&self.cuda.pkg, &self.cuda.version, &self.cuda.link, &self.cuda.name),                 //Priority 3
            (&self.cudnn.pkg, &self.cudnn.version, &self.cudnn.link, &self.cudnn.name),             //Priority 4
        ]
    }
    fn packages_to_string(&self) -> String {
        self.packages()
            .iter()
            .map(|&(pkg, _, _, _)| pkg)
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn goodbye(&self) {
        println!("\nTo prevent the system from updating the packages, add the following to /etc/pacman.conf");
        log(Lvl::Info, &format!("\t IgnorePkg = {}", self.packages_to_string()));
    
        println!("\nCommon Error\n# ERROR: libdevice not found at ./libdevice.10.bc");
        println!("To fix this error, run the following command:");
        log(Lvl::Info, "\texport XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda\n");
    }

    pub fn print_support(&self) {
        for supported in self.support.iter() {
            log(Lvl::Info, &format!("\t{}", supported));
        }
        println!();
    }

    fn installed(&self, pkg: Pkg) -> bool {
        let cmd = format!("{} {}", QUERY, pkg.pkg);
        match run(&cmd, false, false) {
            Ok(output) =>  {
                if output.contains(&pkg.pkg) && output.contains(&pkg.version.to_string()) {
                    return true;
                }
                false
            },
            Err(_) => false,
        }
    }

    pub fn download(&self) -> Vec<usize> {
        let mut downloaded = Vec::new();

        for (i, (pkg, version, link, name)) in self.packages().iter().enumerate() {
            if self.installed(Pkg {
                pkg: pkg.to_string(),
                version: **version,
                name: name.to_string(),
                link: link.to_string(),
            }) {
                log(Lvl::Warning, &format!("Package already installed: {}", name));
                continue;
            }

            if !verify_link(link) {
                log(Lvl::Error, "Invalid link");
                panic!();
            }

            let cmd = format!("{} {}/{} {}", DOWNLOAD, TMPDIR, name, link);
            match run(&cmd, true, true) {
                Ok(_) => log(Lvl::Success, &format!("Package downloaded successfully: {}", name)),
                Err(error) => {
                    log(Lvl::Error, &format!("Failed to download package: {}", error));
                    panic!();
                }
            }

            downloaded.push(i);
        }

        downloaded
    }

    pub fn install(&self, downloaded: Vec<usize>) {
        struct Installer {
            gcc: String,
            gcc_libs: String,
            cuda: String,
            cudnn: String,
        }

        let mut installer = Installer {
            gcc: String::new(),
            gcc_libs: String::new(),
            cuda: String::new(),
            cudnn: String::new(),
        };

        for i in downloaded.iter() {
            let (_, _, _, name) = self.packages()[*i];
            match *i {
                0 => installer.gcc = format!("{}/{}", TMPDIR, name.to_string()),
                1 => installer.gcc_libs = format!("{}/{}", TMPDIR, name.to_string()),
                2 => installer.cuda = format!("{}/{}", TMPDIR, name.to_string()),
                3 => installer.cudnn = format!("{}/{}", TMPDIR, name.to_string()),
                _ => panic!("Unknown Error Occurred"),
            }
        }

        if installer.gcc != "" || installer.gcc_libs != "" {
            let cmd = format!("{} {} {}", INSTALL, installer.gcc, installer.gcc_libs);
            match run(&cmd, true, true) {
                Ok(_) => log(Lvl::Success, &format!("GCC installed successfully.")),
                Err(error) => log(Lvl::Error, &format!("GCC not installed successfully: {}", error)),
            }
            
        }

        if installer.cuda != "" || installer.cudnn != "" {
            let cmd = format!("{} {} {}", INSTALL, installer.cuda, installer.cudnn);
            match run(&cmd, true, true) {
                Ok(_) => log(Lvl::Success, &format!("CUDA installed successfully.")),
                Err(error) => log(Lvl::Error, &format!("Package was not installed successfully: {}", error)),
            }
        }

    }
}