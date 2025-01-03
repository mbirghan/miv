use std::{fs::File, io::Write};
pub struct Logger {
    log_file: File,
}

// TODO: Make this a singleton
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
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.log_file.flush().unwrap();
    }
}
