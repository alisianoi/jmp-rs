use log::{debug, error, info, trace, warn};
use std::env;
use std::env::VarError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    debug!("Hello, world!");

    let var_alex = match env::var("ALEX") {
        Ok(value) => value,
        Err(VarError::NotPresent) => {
            debug!("The $ALEX var is not present");
            ""
        }
        .to_string(),
        Err(VarError::NotUnicode(_)) => {
            warn!("The $ALEX var is not unicode");
            ""
        }
        .to_string(),
    };

    info!("The $ALEX var is {:#?}", var_alex);

    trace!("Here is a trace");
    debug!("Here is a debug");
    info!("Here is an info");
    warn!("Here is a warning");
    error!("Here is an error");

    Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .trace(fern::colors::Color::BrightWhite)
        .debug(fern::colors::Color::White)
        .info(fern::colors::Color::Cyan)
        .warn(fern::colors::Color::Magenta)
        .error(fern::colors::Color::Red);

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
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
