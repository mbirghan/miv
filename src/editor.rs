use crate::{logger::Logger, screen::Screen, stdin_raw_mode::StdinRawMode};
use std::{
    io::{self, Error, Read},
    usize,
};

const fn ctrl_key(k: char) -> u8 {
    (k as u8) & 0x1f
}

struct Erow {
    size: usize,
    content: Vec<u8>,
}

pub struct Editor {
    // Struct fields are dropped in the same order of declaration,
    // so screen will be dropped before _stdin.
    // This ensures raw mode is still enabled while we clean up the screen.
    screen: Screen,
    _stdin: StdinRawMode,

    logger: Logger,

    num_rows: usize,
    row: Erow,
}

impl Editor {
    pub fn new() -> Result<Editor, Error> {
        let _stdin = StdinRawMode::new().unwrap();
        let screen = Screen::new().unwrap();

        Ok(Editor {
            screen,
            _stdin,
            logger: Logger::new(),
            num_rows: 0,
            row: Erow {
                size: 0,
                content: vec![],
            },
        })
    }

    pub fn editor_open(&mut self) {
        let line: Vec<u8> = "Hello, world!".as_bytes().to_vec();
        let line_len: usize = line.len();

        self.row.size = line_len;
        self.row.content = line;

        self.row.content.push(b'\0');
        self.num_rows = 1;

        loop {
            self.screen.editor_refresh_screen();
            // TODO: We should not use an error to signal a quit
            if self.editor_process_keypress().is_err() {
                break;
            }
        }
    }

    pub fn editor_process_keypress(&mut self) -> Result<(), ()> {
        let c = editor_read_key();
        self.logger.log(c.to_string().as_str());

        match c {
            c if c == ctrl_key('q') => Err(()),
            c if c == ctrl_key('c') => Err(()),
            b'h' | b'j' | b'k' | b'l' => {
                self.screen.move_cursor(c);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

// TODO: Add logging to some file
fn editor_read_key() -> u8 {
    let mut buffer = [0; 1];
    let read = io::stdin().read(&mut buffer);
    read.unwrap();

    // Check if the key is an escape sequence
    if buffer[0] == b'\x1b' {
        let mut escape_buffer = [0; 2];
        let read = io::stdin().read(&mut escape_buffer);
        read.unwrap();

        // Check if the key is an arrow key
        if escape_buffer[0] == b'[' {
            return match escape_buffer[1] {
                b'A' => b'k',
                b'B' => b'j',
                b'C' => b'l',
                b'D' => b'h',
                _ => 0,
            };
        }
    }

    buffer[0]
}
