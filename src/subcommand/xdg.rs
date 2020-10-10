use crate::config::Config;
use crate::env::var_or_default;
use crate::logger::setup_logger;
use log::trace;

use clap::ArgMatches;

/// See: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    trace!("Enter run()");
    // Default is: ${HOME}/.local/share
    let (_var_xdg_data_home, _var_xdg_data_home_set) =
        var_or_default("XDG_DATA_HOME");
    // Default is: ${HOME}/.config
    let (_var_xdg_config_home, _var_xdg_config_home_set) =
        var_or_default("XDG_CONFIG_HOME");
    // Default is: /usr/local/share:/usr/share
    let (_var_xdg_data_dirs, _var_xdg_data_dirs_set) =
        var_or_default("XDG_DATA_DIRS");
    // Default is: /etc/xdg
    let (_var_xdg_config_dirs, _var_xdg_config_dirs_set) =
        var_or_default("XDG_CONFIG_DIRS");
    // Default is: ${HOME}/.cache
    let (_var_xdg_cache_home, _var_xdg_cache_home_set) =
        var_or_default("XDG_CACHE_HOME");
    // Default is: no default, the app chooses a directory on its own
    let (_var_xdg_runtime_dir, _var_xdg_runtime_dir_set) =
        var_or_default("XDG_RUNTIME_DIR");
    trace!("Leave run() (Ok)");

    Ok(())
}
