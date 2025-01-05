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

    cursor_row: usize,
    cursor_column: usize,

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
            cursor_row: 0,
            cursor_column: 0,
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
            self.cursor_row,
            self.cursor_column,
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

                    let (new_column_offset, new_cursor_x) = self.get_horizontal_cursor_position(
                        &self.content.lines[self.cursor_row + self.row_offset],
                        self.cursor_column,
                        self.column_offset,
                    );
                    self.screen.editor_refresh_screen(
                        self.content.clone(),
                        self.cursor_row,
                        new_cursor_x,
                        self.row_offset,
                        new_column_offset,
                    );
                }
            }

            // Refresh screen to show the updated content
        }
    }

    fn move_cursor(&mut self, key: Key) {
        match key {
            Key::ArrowUp | Key::Other(b'k') => self.move_cursor_up(),
            Key::ArrowDown | Key::Other(b'j') => self.move_cursor_down(),
            Key::ArrowRight | Key::Other(b'l') => self.move_cursor_right(),
            Key::ArrowLeft | Key::Other(b'h') => self.move_cursor_left(),
            _ => {}
        }
        trace!("Cursor: {}, {}", self.cursor_row, self.cursor_column);
        trace!("Offset: {}, {}", self.row_offset, self.column_offset);
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;

            if self.cursor_row < self.row_offset {
                self.row_offset -= 1;
            }
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_row < self.content.lines.len() - 1 {
            self.cursor_row += 1;

            if self.cursor_row > self.row_offset + self.screen.get_height() - 1 {
                self.row_offset += 1;
            }
        }
    }

    fn move_cursor_left(&mut self) {
        self.reset_cursor();

        if self.cursor_column > 0 {
            self.cursor_column -= 1;

            if self.cursor_column < self.column_offset {
                self.column_offset -= 1;
            }
        }
    }

    fn move_cursor_right(&mut self) {
        self.reset_cursor();

        if self.cursor_column < self.content.lines[self.cursor_row + self.row_offset].len() - 1 {
            self.cursor_column += 1;

            if self.cursor_column > self.column_offset + self.screen.get_width() - 1 {
                self.column_offset += 1;
            }
        }
    }

    fn reset_cursor(&mut self) {
        // Resetting the cursor position to a valid position
        let (new_column_offset, new_cursor_x) = self.get_horizontal_cursor_position(
            &self.content.lines[self.cursor_row + self.row_offset],
            self.cursor_column,
            self.column_offset,
        );
        self.column_offset = new_column_offset;
        self.cursor_column = new_cursor_x;
    }

    pub fn get_horizontal_cursor_position(
        &self,
        content_line: &str,
        cursor_x: usize,
        column_offset: usize,
    ) -> (usize, usize) {
        if cursor_x >= content_line.len() {
            let new_cursor_x = content_line.len() - 1;

            if new_cursor_x > column_offset {
                (column_offset, new_cursor_x)
            } else {
                (new_cursor_x, new_cursor_x)
            }
        } else {
            (column_offset, cursor_x)
        }
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
