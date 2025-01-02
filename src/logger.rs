use std::{fs::File, io::Write};
pub struct Logger {
    log_file: File,
}

// TODO: Make this a singleton
impl Logger {
    pub fn new() -> Logger {
        Logger {
            log_file: File::create(format!(
                "logs/miv-log-{}.txt",
                chrono::Utc::now().timestamp()
            ))
            .unwrap(),
        }
    }

    pub fn log(&mut self, message: &[u8]) {
        self.log_file.write_all(message).unwrap();
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.log_file.flush().unwrap();
    }
}
