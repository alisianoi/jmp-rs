use crate::config::Config;

pub fn setup_logger(config: &Config) -> Result<(), fern::InitError> {
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
                // 28 is the maximum length for the target in this project,
                // for example: jmp_rs::subcommand::uninstall
                target = format!("{:28}", record.target()), // 28
                // 5 is the maximum length for the log leve, e.g. DEBUG
                level = format!("{:5}", level),
                message = message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
