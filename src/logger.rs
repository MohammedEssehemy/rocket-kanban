use std::env;
use log::{debug, error, info, trace, warn};

pub fn init() -> () {
    // pull log level from env
    let log_level = env::var("LOG_LEVEL")
        .unwrap_or("INFO".into())
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        // log to stderr
        .chain(std::io::stderr());

    // also log to file if one is provided via env
    if let Ok(log_file) = env::var("LOG_FILE") {
        let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Error)
        .chain(fern::log_file(log_file).unwrap());
        builder = builder.chain(file_config);
    }

    // globally apply logger
    builder.apply().unwrap();

    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");
}