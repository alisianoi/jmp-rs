use log::{debug, error, trace};
use std::env;
use std::env::VarError;
use std::path;

use crate::config::Config;
use crate::logger::setup_logger;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    trace!("Enter run()");

    // $SHELL, if present, is the default shell of the current user
    // It is not necessarily the currently running shell
    let var_shell = match env::var("SHELL") {
        Ok(value) => {
            debug!("$SHELL is: {}", value);
            Some(value)
        }
        Err(VarError::NotPresent) => {
            debug!("$SHELL is not present");
            None
        }
        Err(VarError::NotUnicode(_)) => {
            let msg = "$SHELL is not Unicode!";
            error!("{}", msg);
            panic!(msg);
        }
    };

    let exists_etc_zsh = match path::Path::new("/etc/zsh").exists() {
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
