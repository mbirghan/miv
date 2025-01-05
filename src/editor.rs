use crate::content::Content;
use crate::input::{Input, Key};
use crate::trace;
use crate::{log, screen::Screen};
use std::{
    fs::File,
    io::{self, BufRead, Error},
};

// TODO: This should be part of the input module
const fn ctrl_key(k: char) -> u8 {
    (k as u8) & 0x1f
}

pub struct Editor {
    // Struct fields are dropped in the same order of declaration,
    // so screen will be dropped before input.
    // This ensures raw mode is still enabled while we clean up the screen.
    screen: Screen,
    input: Input,

    // TODO: This should live somewhere else
    content: Content,
    filename: String,

    cursor: (usize, usize),
    row_offset: usize,
    column_offset: usize,
}

impl Editor {
    pub fn new() -> Result<Editor, Error> {
        log!("Initializing editor");

        let input = Input::new();
        let screen = Screen::new().unwrap();

        Ok(Editor {
            screen,
            input,
            content: Content::new(),
            filename: "logs/test-file.txt".to_string(),
            cursor: (0, 0),
            row_offset: 0,
            column_offset: 0,
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
        self.screen.editor_refresh_screen(
            self.content.clone(),
            self.cursor,
            self.row_offset,
            self.column_offset,
        );

        loop {
            // TODO: This should probably be moved to a function
            let key = self.input.read_key();
            match key {
                Key::None => {
                    // None means we did not read a key
                    // We should not refresh the screen as we did not read a key
                }
                Key::Other(c) if c == ctrl_key('q') => {
                    log!("Ctrl Q, Exiting");
                    break;
                }
                Key::Other(c) if c == ctrl_key('c') => {
                    log!("Ctrl C, Exiting");
                    break;
                }
                _ => {
                    // We read a movement key and should refresh the screen
                    self.move_cursor(key);
                    self.screen.editor_refresh_screen(
                        self.content.clone(),
                        self.cursor,
                        self.row_offset,
                        self.column_offset,
                    );
                }
            }

            // Refresh screen to show the updated content
        }
    }

    fn move_cursor(&mut self, key: Key) {
        match key {
            Key::ArrowUp | Key::Other(b'k') => {
                if self.cursor.1 + self.row_offset > 0 {
                    if self.cursor.1 > 0 {
                        self.cursor.1 -= 1;
                    } else {
                        self.row_offset -= 1;
                    }
                }
            }
            Key::ArrowRight | Key::Other(b'l') => {
                self.reset_cursor();

                if self.cursor.0 + self.column_offset
                    < (self
                        .screen
                        .get_width()
                        .max(self.content.lines[self.cursor.1 + self.row_offset].len())
                        - 1)
                {
                    if self.cursor.0 < self.screen.get_width() - 1 {
                        self.cursor.0 += 1;
                    } else {
                        self.column_offset += 1;
                    }
                }
            }
            Key::ArrowLeft | Key::Other(b'h') => {
                self.reset_cursor();

                if self.cursor.0 + self.column_offset > 0 {
                    if self.cursor.0 > 0 {
                        self.cursor.0 -= 1;
                    } else {
                        self.column_offset -= 1;
                    }
                }
            }
            Key::ArrowDown | Key::Other(b'j') => {
                if self.cursor.1 + self.row_offset
                    < (self.screen.get_height().max(self.content.lines.len()) - 1)
                {
                    if self.cursor.1 < self.screen.get_height() - 1 {
                        self.cursor.1 += 1;
                    } else {
                        self.row_offset += 1;
                    }
                }
            }
            _ => {}
        }
        trace!("Cursor: {:?}", self.cursor);
        trace!("Row offset: {}", self.row_offset);
    }

    fn reset_cursor(&mut self) {
        // Resetting the cursor position to a valid position
        let (new_column_offset, new_cursor_x) = self.screen.get_horizontal_cursor_position(
            &self.content.lines[self.cursor.1 + self.row_offset],
            self.cursor.0,
            self.column_offset,
        );
        self.column_offset = new_column_offset;
        self.cursor.0 = new_cursor_x;
    }

    pub fn editor_open_file(&mut self) {
        self.content.lines.clear();

        let file = File::open(self.filename.clone());
        if let Err(e) = file {
            log!("Failed to open file: {}", e);
            return;
        }
        let reader = io::BufReader::new(file.unwrap());
        let lines = reader.lines();

        for line in lines {
            let line = line.unwrap();
            self.content.lines.push(line);
        }
    }
}
