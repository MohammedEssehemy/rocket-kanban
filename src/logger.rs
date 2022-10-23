use chrono::Local;
use fern::{log_file, Dispatch};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::{env, io::stderr};

pub fn init() -> () {
    // pull log level from env
    let log_level = env::var("LOG_LEVEL")
        .unwrap_or("INFO".into())
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    let mut builder = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        // log to stderr
        .chain(stderr());

    // also log to file if one is provided via env
    if let Ok(log_file_path) = env::var("LOG_FILE") {
        let file_config = Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}][{}] {}",
                    Local::now(),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(LevelFilter::Error)
            .chain(log_file(log_file_path).unwrap());
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
