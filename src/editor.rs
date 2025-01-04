use crate::content::Content;
use crate::{error, log, screen::Screen, stdin_raw_mode::StdinRawMode, trace};
use std::{
    fs::File,
    io::{self, BufRead, Error, Read},
};

pub enum Key {
    CtrlQ,
    CtrlC,
    None,
    Other(u8),
}

const fn ctrl_key(k: char) -> u8 {
    (k as u8) & 0x1f
}

pub struct Editor {
    // Struct fields are dropped in the same order of declaration,
    // so screen will be dropped before _stdin.
    // This ensures raw mode is still enabled while we clean up the screen.
    screen: Screen,
    _stdin: StdinRawMode,

    // TODO: This should live somewhere else
    content: Content,
    filename: String,
}

impl Editor {
    pub fn new() -> Result<Editor, Error> {
        log!("Initializing editor");

        let _stdin = StdinRawMode::new().unwrap();
        let screen = Screen::new().unwrap();

        Ok(Editor {
            screen,
            _stdin,
            content: Content::new(),
            filename: "logs/test-file.txt".to_string(),
        })
    }

    pub fn editor_open_with_file(&mut self, filename: String) {
        log!("Opening editor with file: {}", filename);

        self.filename = filename;
        self.editor_open_file();

        self.editor_open();
    }

    pub fn editor_open(&mut self) {
        // Refresh screen to show the initial content
        self.screen.editor_refresh_screen(self.content.clone());

        loop {
            let result = self.editor_process_keypress();
            match result {
                Ok(Key::None) => {
                    // 0 means we did not read a key
                    // We should not refresh the screen as we did not read a key
                }
                Ok(Key::CtrlQ) | Ok(Key::CtrlC) => break,
                Ok(Key::Other(_)) => {
                    // We read a key and should refresh the screen
                    self.screen.editor_refresh_screen(self.content.clone());
                }
                Err(()) => {
                    error!("Error processing keypress");
                    break;
                }
            }

            // Refresh screen to show the updated content
        }
    }

    pub fn editor_process_keypress(&mut self) -> Result<Key, ()> {
        let c = editor_read_key();

        match c {
            0 => Ok(Key::None),
            c if c == ctrl_key('q') => Ok(Key::CtrlQ),
            c if c == ctrl_key('c') => Ok(Key::CtrlC),
            b'h' | b'j' | b'k' | b'l' => {
                self.screen.move_cursor(c);
                Ok(Key::Other(c))
            }
            c => Ok(Key::Other(c)),
        }
    }

    pub fn editor_open_file(&mut self) {
        self.content.lines.clear();

        let file = File::open(self.filename.clone()).unwrap();
        let reader = io::BufReader::new(file);
        let lines = reader.lines();

        for line in lines {
            let line = line.unwrap();
            self.content.lines.push(line);
        }
    }
}

// TODO: Should move this to a separate module
// TODO: Should return a Key enum
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
