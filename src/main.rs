use log::{debug, info, warn};
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

    Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .info(fern::colors::Color::Green);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
