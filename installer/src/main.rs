use std::panic;
use ctrlc;

mod pkgs;
mod cli;
mod exec;

use pkgs::Data;
use cli::{Confirm, Lvl, confirm, log};
use exec::rmtmp;

fn main() {
    ctrlc::set_handler(|| {
        log(Lvl::Error, "Installation was interrupted by the user. Cleaning up...");
        end(true);
    }).expect("Error setting Ctrl+C handler");

    let result = panic::catch_unwind(|| {
        let data = Data::load();
        data.start();

        let pkgs = data.version("Choose the version of CUDA you want to install:");
        pkgs.print_support();

        let proceed = confirm("Would you like to proceed with the installation?");
        if proceed != Confirm::Yes {
            end(true);
        }

        let downloaded = pkgs.download();
        pkgs.install(downloaded);
        
        pkgs.goodbye();
        end(false);
    });

    if let Err(_) = result {
        end(true);
    }
}

fn end(panicking: bool) {
    rmtmp();

    if panicking {
        log(Lvl::Error, "Installation exited!");
        std::process::exit(1);
    }
    log(Lvl::Success, "Installation completed successfully!");
}