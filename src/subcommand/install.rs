use log::{debug, error, trace};
use std::env;
use std::env::VarError;
use std::path;

use crate::config::Config;
use crate::logger::setup_logger;
use clap::ArgMatches;

pub fn var(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(value) => {
            debug!("${} is: {}", name, value);
            Some(value)
        }
        Err(VarError::NotPresent) => {
            debug!("${} is not present", name);
            None
        }
        Err(VarError::NotUnicode(_)) => {
            let msg = format!("${} is not Unicode!", name);
            error!("${}", msg);
            panic!(msg);
        }
    }
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    trace!("Enter run()");

    // $SHELL, if present, is the default shell of the current user
    // It is not necessarily the currently running shell
    let _var_shell = var("SHELL");
    let _var_zdotdir = var("ZDOTDIR");

    let _exists_etc_zsh = match path::Path::new("/etc/zsh").exists() {
        true => {
            debug!("/etc/zsh exists!");
            true
        }
        false => {
            debug!("/etc/zsh does *not* exist!");
            false
        }
    };

    trace!("Leave run (Ok)");

    Ok(())
}
