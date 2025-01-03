use crate::constants::BLOCKING;
use std::io::{self, Error};
use std::os::fd::AsRawFd;
use termios::*;

pub struct StdinRawMode {
    orig_termios: Termios,
}

impl StdinRawMode {
    pub fn new() -> Result<StdinRawMode, Error> {
        let orig_termios = Termios::from_fd(io::stdin().as_raw_fd()).unwrap();
        let stdin = StdinRawMode { orig_termios };
        stdin.enable_raw_mode();

        Ok(stdin)
    }

    fn enable_raw_mode(&self) {
        let mut raw = Termios::from_fd(io::stdin().as_raw_fd()).unwrap();

        raw.c_iflag &= !(BRKINT | IXON | INPCK | ISTRIP | ICRNL);
        raw.c_oflag &= !(OPOST);
        raw.c_cflag |= CS8;
        raw.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);

        if BLOCKING {
            // Blocking read
            // downside: pressing esc will not be read until a key is pressed as it could be part of an escape sequence
            raw.c_cc[VMIN] = 1;
            raw.c_cc[VTIME] = 0;
        } else {
            // Non-blocking read
            // downside: we will consistently read 0 bytes until a key is pressed which could lead to consistent rerendering
            raw.c_cc[VMIN] = 0;
            raw.c_cc[VTIME] = 1;
        }

        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &raw).unwrap();
    }
}

impl Drop for StdinRawMode {
    fn drop(&mut self) {
        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &self.orig_termios).unwrap();
    }
}
