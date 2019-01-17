use log::{Record, Level, Metadata, SetLoggerError};
use chrono::Local;


struct Logger {
    level: Level
}


impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] {:<5} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(level: Level) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Logger{ level }))
        .map(|()| log::set_max_level(level.to_level_filter()))
}

