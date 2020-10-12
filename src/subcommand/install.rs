use crate::config::Config;
use crate::env::var_or_default;
use crate::logger::setup_logger;
use crate::shell::Shell;
use crate::subcommand::install;
use clap::ArgMatches;
use log::trace;

mod zsh;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    // read_lines("/etc/shells")?;

    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    trace!("Enter run()");

    // $SHELL, if present, is the default shell of the current user
    // It is not necessarily the currently running shell
    let (_var_shell, _var_shell_ok) = var_or_default("SHELL");
    let (_var_path, _var_path_set) = var_or_default("PATH");
    let (_var_home, _var_home_ok) = var_or_default("HOME");
    let (_var_visual, _var_visual_set) = var_or_default("VISUAL");
    let (_var_editor, _var_editor_set) = var_or_default("EDITOR");
    let (_var_term, _var_term_set) = var_or_default("TERM");
    let (_ver_colorterm, _var_colorterm_set) = var_or_default("COLORTERM");

    // At this point the install decision is known, so let's go:
    let target = Shell::ZSH;

    match target {
        Shell::ZSH => install::zsh::run()?,
    }

    trace!("Leave run() (Ok)");

    Ok(())
}
