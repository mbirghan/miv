use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};
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

        logger.log("Logger initialized");

        logger
    }

    fn get_timestamp(&self) -> String {
        let timestamp = chrono::Local::now();
        timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn log(&mut self, message: &str) {
        let log_message = format!("[{}]: {}\n", self.get_timestamp(), message);
        self.log_file.write_all(log_message.as_bytes()).unwrap();
    }

    pub fn error(&mut self, message: &str) {
        let log_message = format!("[{}]: {}", self.get_timestamp(), message);
        self.log_file.write_all(log_message.as_bytes()).unwrap();
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
        logger.lock().unwrap().log(&format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::Logger::global();
        logger.lock().unwrap().error(&format!($($arg)*));
    }};
}
