use crate::env::var_or_default;
use crate::file::copy_once;
use crate::path::{exists, exists_if};
use crate::shell::Shell;
use log::debug;

fn zdotdir() -> (String, bool) {
    let (var_home, var_home_ok) = var_or_default("HOME");
    let (var_zdotdir, var_zdotdir_ok) = var_or_default("ZDOTDIR");

    // From man 1 zsh:
    // If ZDOTDIR is unset, HOME is used instead. Files listed above as
    // being in /etc may be in another directory, depending on the system.
    // So, let's replace ZDOTDIR with HOME, if ZDOTDIR not set (and HOME is).
    if var_zdotdir_ok {
        (var_zdotdir, true)
    } else if var_home_ok {
        (var_home, true)
    } else {
        (var_zdotdir, false)
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (var_zdotdir, var_zdotdir_ok) = zdotdir();

    let _exists_etc_zsh_zshenv = exists("/etc/zsh");
    let _exists_zdotdir_zshenv =
        exists_if(&format!("{}/.zshenv", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zprofile = exists("/etc/zsh/zprofile");
    let _exists_etc_profile = exists("/etc/profile");
    let _exists_zdotdir_zprofile =
        exists_if(&format!("{}/.zshprofile", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zshrc = exists("/etc/zsh/zshrc");
    let _exists_etc_zsh_zshrc_local = exists("/etc/zsh/zshrc.local");
    let zdotdir_zshrc_local_path = format!("{}/.zshrc.local", var_zdotdir);
    let exists_zdotdir_zshrc_local =
        exists_if(&zdotdir_zshrc_local_path, var_zdotdir_ok);
    let zdotdir_zshrc_path = format!("{}/.zshrc", var_zdotdir);
    let exists_zdotdir_zshrc =
        exists_if(&zdotdir_zshrc_path, var_zdotdir_ok);
    let _exists_etc_zsh_zlogin = exists("/etc/zsh/zlogin");
    let _exists_zdotdir_zlogin =
        exists_if(&format!("{}/.zlogin", var_zdotdir), var_zdotdir_ok);
    let _exists_zdotdir_zlogin =
        exists_if(&format!("{}/.zlogout", var_zdotdir), var_zdotdir_ok);
    let _exists_etc_zsh_zlogout = exists("/etc/zsh/zlogout");

    if exists_zdotdir_zshrc_local {
        debug!("Install for grml config to {}", zdotdir_zshrc_local_path);
        copy_once(&Shell::ZSH, &zdotdir_zshrc_local_path)?;
    } else if exists_zdotdir_zshrc {
        debug!("Install for default config to {}", zdotdir_zshrc_path);
        copy_once(&Shell::ZSH, &zdotdir_zshrc_path)?;
    } else {
        panic!("Don't know where to isntall ZSH");
    }

    Ok(())
}
