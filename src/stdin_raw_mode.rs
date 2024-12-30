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

        raw.c_cc[VMIN] = 1;
        raw.c_cc[VTIME] = 0;

        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &raw).unwrap();
    }
}

impl Drop for StdinRawMode {
    fn drop(&mut self) {
        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &self.orig_termios).unwrap();
    }
}
