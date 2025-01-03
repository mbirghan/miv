use crate::{log, screen::Screen, stdin_raw_mode::StdinRawMode, trace};
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

    num_rows: usize,
    row: Erow,
}

impl Editor {
    pub fn new() -> Result<Editor, Error> {
        log!("Initializing editor");

        let _stdin = StdinRawMode::new().unwrap();
        let screen = Screen::new().unwrap();

        Ok(Editor {
            screen,
            _stdin,
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

        // Refresh screen to show the initial content
        self.screen.editor_refresh_screen();

        loop {
            // TODO: We should not use an error to signal a quit
            let result = self.editor_process_keypress();
            match result {
                Ok(0) => {
                    // 0 means we did not read a key
                    // We should not refresh the screen as we did not read a key
                }
                Ok(_) => {
                    // We read a key and should refresh the screen
                    self.screen.editor_refresh_screen();
                }
                Err(()) => break,
            }

            // Refresh screen to show the updated content
        }
    }

    pub fn editor_process_keypress(&mut self) -> Result<u8, ()> {
        let c = editor_read_key();

        match c {
            c if c == ctrl_key('q') => Err(()),
            c if c == ctrl_key('c') => Err(()),
            b'h' | b'j' | b'k' | b'l' => {
                self.screen.move_cursor(c);
                Ok(c)
            }
            _ => Ok(c),
        }
    }
}

// TODO: Should move this to a separate module
fn editor_read_key() -> u8 {
    let mut buffer = [0; 1];
    let read = io::stdin().read(&mut buffer);
    read.unwrap();
    trace!("Read key: {}", buffer[0]);

    // Check if the key is an escape sequence
    if buffer[0] == b'\x1b' {
        let mut escape_buffer = [0; 1];
        let read = io::stdin().read(&mut escape_buffer);
        read.unwrap();
        trace!("Read escape: {}", escape_buffer[0]);

        // If we do not detect a second byte the key is just esc
        if escape_buffer[0] == 0 {
            return b'\x1b';
        }

        // Check if the key is an arrow key
        if escape_buffer[0] == b'[' {
            let mut move_buffer = [0; 1];
            let read = io::stdin().read(&mut move_buffer);
            read.unwrap();
            trace!("Read move: {}", move_buffer[0]);

            return match move_buffer[0] {
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
