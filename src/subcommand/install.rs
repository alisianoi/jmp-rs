use crate::config::Config;
use crate::env::var_or_default;
use crate::logger::setup_logger;
use clap::ArgMatches;
use log::{debug, error, info, trace, warn};
use std::env;

use std::path;

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

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    trace!("Enter run()");

    // $SHELL, if present, is the default shell of the current user
    // It is not necessarily the currently running shell
    let (_ver_zero, _ver_zero_set) = var_or_default("0");
    let (_var_path, _var_path_set) = var_or_default("PATH");
    let (var_home, var_home_ok) = var_or_default("HOME");
    let (_var_visual, _var_visual_set) = var_or_default("VISUAL");
    let (_var_editor, _var_editor_set) = var_or_default("EDITOR");
    let (_var_term, _var_term_set) = var_or_default("TERM");
    let (_ver_colorterm, _var_colorterm_set) = var_or_default("COLORTERM");
    let (_var_shell, _var_shell_ok) = var_or_default("SHELL");
    let (var_zdotdir, var_zdotdir_ok) = var_or_default("ZDOTDIR");

    // From man 1 zsh:
    // If ZDOTDIR is unset, HOME is used instead. Files listed above as
    // being in /etc may be in another directory, depending on the system.
    // So, let's replace ZDOTDIR with HOME, if ZDOTDIR not set (and HOME is).
    let (var_zdotdir, var_zdotdir_ok) = if var_zdotdir_ok {
        (var_zdotdir, true)
    } else if var_home_ok {
        (var_home, true)
    } else {
        (var_zdotdir, false)
    };

    let _exists_etc_zsh_zshenv = exists("/etc/zsh");
    let _exists_zdotdir_zshenv =
        exists_if(&format!("{}/.zshenv", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zprofile = exists("/etc/zsh/zprofile");
    let _exists_etc_profile = exists("/etc/profile");
    let _exists_zdotdir_zprofile =
        exists_if(&format!("{}/.zshprofile", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zshrc = exists("/etc/zsh/zshrc");
    let _exists_etc_zsh_zshrc_local = exists("/etc/zsh/zshrc.local");
    let _exists_zdotdir_zshrc_locals =
        exists_if(&format!("{}/.zshrc.local", var_zdotdir), var_zdotdir_ok);
    let _exists_zdotdir_zshrc =
        exists_if(&format!("{}/.zshrc", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zlogin = exists("/etc/zsh/zlogin");
    let _exists_zdotdir_zlogin =
        exists_if(&format!("{}/.zlogin", var_zdotdir), var_zdotdir_ok);
    let _exists_zdotdir_zlogin =
        exists_if(&format!("{}/.zlogout", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zlogout = exists("/etc/zsh/zlogout");

    trace!("Leave run() (Ok)");

    Ok(())
}
