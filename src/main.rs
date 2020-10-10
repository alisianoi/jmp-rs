// use std::env;
// use std::env::VarError;
use clap::{App, Arg, SubCommand};

mod config;
mod logger;
mod subcommand;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_verbose = Arg::with_name("v")
        .short("v")
        .long("verbose")
        .multiple(true)
        .help("Sets the level of verbosity");
    let arg_quiet = Arg::with_name("q")
        .short("q")
        .long("quiet")
        .multiple(false)
        .help("Disables all output");

    let subcommand_xdg = SubCommand::with_name("xdg")
        .about("Inspects XDG directories")
        .arg(&arg_verbose)
        .arg(&arg_quiet);
    let subcommand_jump = SubCommand::with_name("jump")
        .about("Jumps to a directory")
        .arg(&arg_verbose)
        .arg(&arg_quiet);
    let subcommand_install = SubCommand::with_name("install")
        .about("Installs jmp-rs")
        .arg(&arg_verbose)
        .arg(&arg_quiet);
    let subcommand_reinstall = SubCommand::with_name("reinstall")
        .about("Uninstalls then installs jmp-rs")
        .arg(&arg_verbose)
        .arg(&arg_quiet);
    let subcommand_uninstall = SubCommand::with_name("uninstall")
        .about("Uninstalls jmp-rs")
        .arg(&arg_verbose)
        .arg(&arg_quiet);

    let args = App::new("jmp-rs")
        .arg(arg_verbose)
        .arg(arg_quiet)
        .subcommand(subcommand_xdg)
        .subcommand(subcommand_jump)
        .subcommand(subcommand_install)
        .subcommand(subcommand_reinstall)
        .subcommand(subcommand_uninstall)
        .get_matches();

    match args.subcommand() {
        ("xdg", Some(matches)) => {
            return subcommand::xdg::run(&matches);
        }
        ("jump", Some(_)) => {
            println!("subcommand: jump");
        }
        ("install", Some(matches)) => {
            return subcommand::install::run(&matches);
        }
        ("uninstall", Some(_)) => {
            println!("subcommand: uninstall");
        }
        ("reinstall", Some(_)) => {
            println!("subcommand; reinstall");
        }
        ("", None) => {
            println!("no subcommands");
        }
        _ => {
            panic!("Failed to understand a command, this is a bug :(");
        }
    }

    // let config = Config::new(args.is_present("q"),
    // args.occurrences_of("v"));

    // setup_logger(&config)?;

    // if let Some(_) = args.subcommand_matches("install") {
    //     debug!("The install subcommand is there");
    // }

    // if let Some(_) = args.subcommand_matches("uninstall") {
    //     debug!("The uninstall subcommand is there");
    // }

    // debug!("Hello, world!");

    // let var_alex = match env::var("ALEX") {
    //     Ok(value) => value,
    //     Err(VarError::NotPresent) => {
    //         debug!("The $ALEX var is not present");
    //         ""
    //     }
    //     .to_string(),
    //     Err(VarError::NotUnicode(_)) => {
    //         warn!("The $ALEX var is not unicode");
    //         ""
    //     }
    //     .to_string(),
    // };

    // info!("The $ALEX var is {:#?}", var_alex);

    // trace!("Here is a trace");
    // debug!("Here is a debug");
    // info!("Here is an info");
    // warn!("Here is a warning");
    // error!("Here is an error");

    Ok(())
}
