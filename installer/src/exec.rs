use std::process::{Command, Stdio};

use crate::cli::{Lvl, log};

const TMPDIR: &str = "/tmp/cuda_installer";
const MKDIR: &str = "mkdir -p";
const RMDIR: &str = "rm -rf";
const PING: &str = "curl -I";

pub fn run(cmd: &str, interactive: bool, verbose: bool) -> Result<String, String> {
    if verbose {
        log(Lvl::Info, &format!("Running command: {}", cmd));
    }
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
                if verbose {
                    log(Lvl::Success, "Command executed successfully");
                }
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                let exit_code = output.status.code().unwrap_or(-1);
                if exit_code == 1 {
                    if verbose {
                        log(Lvl::Warning, "User declined to proceed with the installation.");
                    }
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

pub fn verify_link(link: &str) -> bool {
    let cmd = format!("{} {}", PING, link);
    match run(&cmd, false, true) {
        Ok(output) => if output.contains("HTTP/2 200") {
            log(Lvl::Success, "Valid link");
            return true;
        },
        Err(error) => eprintln!("Failed to check link: {}", error),
    }

    false
}

pub fn mktmp() {
    let cmd = format!("{} {}", MKDIR, TMPDIR);
    match run(&cmd, false, true) {
        Ok(_) => log(Lvl::Success, &format!("Tmp directory created: {}", TMPDIR)),
        Err(error) => {
            log(Lvl::Error, &format!("Failed to create tmp directory: {}", error));
            panic!();
        }
    }
}

pub fn rmtmp() {
    let cmd = format!("{} {}", RMDIR, TMPDIR);
    match run(&cmd, false, true) {
        Ok(_) => log(Lvl::Success, &format!("Tmp directory removed: {}", TMPDIR)),
        Err(error) => {
            log(Lvl::Error, &format!("Failed to remove tmp directory: {}", error));
            log(Lvl::Warning, &format!("Please remove the tmp directory manually at {}", TMPDIR));
        }
    }
}