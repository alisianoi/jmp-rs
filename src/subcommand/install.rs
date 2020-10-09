use log::trace;

use crate::config::Config;
use crate::logger::setup_logger;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_matches(matches);

    setup_logger(&config)?;

    trace!("Enter run()");
    trace!("Leave run (Ok)");

    Ok(())
}
