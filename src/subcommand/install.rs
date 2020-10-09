use log::{debug, error, info, trace, warn};
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

pub fn var_or_default(name: &str) -> (String, bool) {
    match var(name) {
        Some(value) => (value, true),
        None => (format!("${}", name), false),
    }
}

pub fn exists(path_name: &str) -> bool {
    match path::Path::new(path_name).exists() {
        true => {
            info!("Exists: {}", path_name);
            true
        }
        false => {
            warn!("Does *not* exist: {}", path_name);
            false
        }
    }
}

pub fn exists_if(path_name: &str, condition: bool) -> bool {
    if condition {
        exists(path_name)
    } else {
        warn!("Does *not* exist: {}", path_name);
        false
    }
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    trace!("Enter run()");

    // $SHELL, if present, is the default shell of the current user
    // It is not necessarily the currently running shell
    let (_var_shell, _var_shell_ok) = var_or_default("SHELL");
    let (var_zdotdir, var_zdotdir_ok) = var_or_default("ZDOTDIR");

    let _exists_etc_zsh = exists("/etc/zsh");
    let _exists_zdotdir_zshenv =
        exists_if(&format!("{}/.zshenv", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zprofile = exists("/etc/zsh/zprofile");
    let _exists_etc_profile = exists("/etc/profile");
    let _exists_zdotdir_zprofile =
        exists_if(&format!("{}/.zshprofile", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zshrc = exists("/etc/zsh/zshrc");
    let _exists_zdotdir_zshrc =
        exists_if(&format!("{}/.zshrc", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zlogin = exists("/etc/zsh/zlogin");
    let _exists_zdotdir_zlogin =
        exists_if(&format!("{}/.zlogin", var_zdotdir), var_zdotdir_ok);
    let _exists_zdotdir_zlogin =
        exists_if(&format!("{}/.zlogout", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zlogout = exists("/etc/zsh/zlogout");

    trace!("Leave run (Ok)");

    Ok(())
}
