use log::{debug, error, info, trace, warn};
// use std::env;
// use std::env::VarError;
use clap::{App, Arg, SubCommand};

mod config;
use config::Config;

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

    let subcommand_install = SubCommand::with_name("install")
        .about("Installs jmp-rs")
        .arg(&arg_verbose)
        .arg(&arg_quiet);
    let subcommand_uninstall = SubCommand::with_name("uninstall")
        .about("Uninstalls jmp-rs")
        .arg(&arg_verbose)
        .arg(&arg_quiet);

    let args = App::new("jmp-rs")
        .arg(arg_verbose)
        .arg(arg_quiet)
        .subcommand(subcommand_install)
        .subcommand(subcommand_uninstall)
        .get_matches();

    // println!("The count for verboisty: {:#?}", args);

    match args.subcommand() {
        ("install", Some(_)) => {
            println!("subcommand: install");
        }
        ("uninstall", Some(_)) => {
            println!("subcommand: uninstall");
        }
        ("", None) => {
            println!("no subcommands");
        }
        _ => {
            println!("unknown subcommand");
        }
    }

    let config = Config::new(args.is_present("q"), args.occurrences_of("v"));

    setup_logger(&config)?;

    if let Some(_) = args.subcommand_matches("install") {
        debug!("The install subcommand is there");
    }

    if let Some(_) = args.subcommand_matches("uninstall") {
        debug!("The uninstall subcommand is there");
    }

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

fn setup_logger(config: &Config) -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .trace(fern::colors::Color::BrightWhite)
        .debug(fern::colors::Color::White)
        .info(fern::colors::Color::Cyan)
        .warn(fern::colors::Color::Magenta)
        .error(fern::colors::Color::Red);

    let level = config.level;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let level = record.level();
            out.finish(format_args!(
                "{color_lft}{date}{color_rgt} {target} {color_lft}{level}{color_rgt} {message}",
                color_lft = format_args!(
                    "\x1B[{}m",
                    colors.get_color(&level).to_fg_str()
                ),
                color_rgt = "\x1B[0m",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = format!("{:5}", level),
                message = message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
