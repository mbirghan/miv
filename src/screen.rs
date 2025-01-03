use crate::constants::VERSION;
use crate::content::Content;
use core::str;
use std::{
    io::{self, Error, Write},
    usize,
};

// TODO: How can we react to window size changes?
fn get_window_size() -> (usize, usize) {
    term_size::dimensions_stdout().unwrap()
    // TODO: Add a fallback for when term_size is not available
}

fn write_flush(s: &str) {
    io::stdout().write_all(s.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub struct Screen {
    size: (usize, usize),
    cursor: (usize, usize),

    abuf: Vec<u8>,
    // TODO: Move this to the editor struct
    content: Option<Content>,
}

impl Screen {
    pub fn new() -> Result<Screen, Error> {
        let size = get_window_size();
        let cursor = (0, 0);
        let abuf = vec![];

        return Ok(Screen {
            size,
            cursor,
            abuf,
            content: None,
        });
    }

    fn get_height(&self) -> usize {
        self.size.1
    }

    fn get_width(&self) -> usize {
        self.size.0
    }

    fn append_abuf(&mut self, s: &str) {
        self.abuf.extend(&s.as_bytes().to_vec());
    }

    pub fn editor_refresh_screen(&mut self) {
        // Hide the cursor to avoid flickering
        self.append_abuf("\x1b[?25l");

        // Clears the entire screen
        // self.append_abuf("\x1b[2J");

        // Moves the cursor to the top left corner
        // We need to do this to start drawing from the top left corner
        self.append_abuf("\x1b[H");

        // Show the window size
        // self.append_abuf(&format!("{}, {}   ", self.get_height(), self.get_width()));
        self.draw_content();

        self.append_abuf(&format!(
            "\x1b[{};{}H",
            self.cursor.1 + 1,
            self.cursor.0 + 1
        ));

        // Show the cursor again
        self.append_abuf("\x1b[?25h");

        write_flush(str::from_utf8(&self.abuf).unwrap());
    }

    pub fn move_cursor(&mut self, key: u8) {
        match key {
            b'k' => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1
                }
            }
            b'l' => {
                if self.cursor.0 < (self.get_width() - 1) {
                    self.cursor.0 += 1
                }
            }
            b'h' => {
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1
                }
            }
            b'j' => {
                if self.cursor.1 < (self.get_height() - 1) {
                    self.cursor.1 += 1
                }
            }
            _ => {}
        }
    }

    fn draw_content(&mut self) {
        // Extract content reference before the mutable borrows
        let content_len = self.content.as_ref().map(|c| c.lines.len()).unwrap_or(0);
        match &self.content {
            Some(_) => {
                self.draw_content_rows().unwrap();
                self.draw_filler_rows(content_len);
            }
            None => self.draw_welcome_message(),
        }
    }
    fn draw_content_rows(&mut self) -> Result<(), Error> {
        let lines = self
            .content
            .as_ref()
            .unwrap_or(&Content::new())
            .lines
            .clone();

        // Only iterate up to the minimum of screen height and content length
        let visible_lines = lines.len().min(self.get_height());

        for y in 0..visible_lines {
            let line = &lines[y]; // Use reference instead of clone
            self.append_abuf(line); // No need for to_vec()

            // Clears the line we are rerendering
            self.append_abuf("\x1b[K");
            if y < self.get_height() - 1 {
                self.append_abuf("\r\n");
            }
        }

        Ok(())
    }

    fn draw_filler_rows(&mut self, start_row: usize) {
        for y in start_row..self.get_height() {
            self.append_abuf("~");

            // Clears the line we are rerendering
            self.append_abuf("\x1b[K");

            if y < self.get_height() - 1 {
                self.append_abuf("\r\n");
            }
        }
    }

    fn draw_welcome_message(&mut self) {
        let welcome_message = format!("Kilo editor -- version {}", VERSION);
        self.append_abuf(&welcome_message);
        // Clears the line we are rerendering
        self.append_abuf("\x1b[K");

        self.append_abuf("\r\n");
        self.draw_filler_rows(1);
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.abuf.clear();
        self.append_abuf("\x1b[2J");
        self.append_abuf("\x1b[H");
        write_flush(str::from_utf8(&self.abuf).unwrap());
    }
}
