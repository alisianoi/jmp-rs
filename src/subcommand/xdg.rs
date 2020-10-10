use crate::config::Config;
use crate::logger::setup_logger;
use log::{debug, error, info, trace, warn};

use clap::ArgMatches;

/// See: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    trace!("Enter run()");
    trace!("Leave run() (Ok)");

    Ok(())
}
