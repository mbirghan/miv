use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use crate::constants::LOG_LEVEL;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

pub struct Logger {
    log_file: File,
}

impl Logger {
    pub fn new() -> Logger {
        let mut logger = Logger {
            log_file: File::options()
                .append(true)
                .create(true)
                .open("logs/miv-log.txt")
                .unwrap(),
        };

        logger.log(LogLevel::Info, "Logger initialized");
        logger
    }

    fn get_timestamp(&self) -> String {
        let timestamp = chrono::Local::now();
        timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        if level >= LOG_LEVEL {
            let level_str = match level {
                LogLevel::Trace => "TRACE",
                LogLevel::Debug => "DEBUG",
                LogLevel::Info => "INFO",
                LogLevel::Warn => "WARN",
                LogLevel::Error => "ERROR",
                LogLevel::Fatal => "FATAL",
            };
            let log_message = format!("[{}] [{}]: {}\n", self.get_timestamp(), level_str, message);
            self.log_file.write_all(log_message.as_bytes()).unwrap();
        }
    }

    pub fn global() -> Arc<Mutex<Logger>> {
        static INSTANCE: std::sync::OnceLock<Arc<Mutex<Logger>>> = std::sync::OnceLock::new();

        INSTANCE
            .get_or_init(|| Arc::new(Mutex::new(Logger::new())))
            .clone()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.log_file.flush().unwrap();
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Info, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Trace, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Debug, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Info, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Warn, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Error, &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().log($crate::logger::LogLevel::Fatal, &format!($($arg)*));
    }};
}
