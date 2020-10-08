use log::{debug, error, info, trace, warn};
// use std::env;
// use std::env::VarError;
use clap::{App, Arg, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subcommand_install =
        SubCommand::with_name("install").about("Installs jmp-rs");
    let subcommand_uninstall =
        SubCommand::with_name("uninstall").about("Uninstalls jmp-rs");
    let matches = App::new("jmp-rs")
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("q")
                .short("q")
                .long("quiet")
                .multiple(false)
                .help("Disables all output"),
        )
        .subcommand(subcommand_install)
        .subcommand(subcommand_uninstall)
        .get_matches();

    println!("The count for verboisty: {}", matches.occurrences_of("v"));

    setup_logger(matches.is_present("q"), matches.occurrences_of("v"))?;

    debug!("Hello, world!");

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

    trace!("Here is a trace");
    debug!("Here is a debug");
    info!("Here is an info");
    warn!("Here is a warning");
    error!("Here is an error");

    Ok(())
}

fn setup_logger(quiet: bool, verbose: u64) -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .trace(fern::colors::Color::BrightWhite)
        .debug(fern::colors::Color::White)
        .info(fern::colors::Color::Cyan)
        .warn(fern::colors::Color::Magenta)
        .error(fern::colors::Color::Red);

    let level = match verbose {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let level = match quiet {
        true => log::LevelFilter::Off,
        _ => level,
    };

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
