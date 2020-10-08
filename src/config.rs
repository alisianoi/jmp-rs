use log;

pub struct Config {
    pub level: log::LevelFilter,
}

impl Config {
    pub fn new(quiet: bool, verbose: u64) -> Config {
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

        Config { level: level }
    }
}
