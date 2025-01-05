use crate::content::Content;
use crate::{constants::VERSION, trace};
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

    abuf: Vec<u8>,
}

impl Screen {
    pub fn new() -> Result<Screen, Error> {
        let size = get_window_size();
        let abuf = vec![];

        return Ok(Screen { size, abuf });
    }

    pub fn get_height(&self) -> usize {
        self.size.1
    }

    pub fn get_width(&self) -> usize {
        self.size.0
    }

    fn append_abuf(&mut self, s: &str) {
        self.abuf.extend(&s.as_bytes().to_vec());
    }

    // TODO: I am passing way too many arguments here
    // TODO: This is a mess, the cursor computation makes this very hard to read
    pub fn editor_refresh_screen(
        &mut self,
        content: Content,
        cursor: (usize, usize),
        row_offset: usize,
        column_offset: usize,
    ) {
        trace!("Refreshing screen");

        // Hide the cursor to avoid flickering
        self.append_abuf("\x1b[?25l");

        // Clears the entire screen
        // self.append_abuf("\x1b[2J");

        // Moves the cursor to the top left corner
        // We need to do this to start drawing from the top left corner
        self.append_abuf("\x1b[H");

        let content_line = content.lines[cursor.1 + row_offset].clone();
        let (new_column_offset, new_cursor_x) =
            self.get_horizontal_cursor_position(&content_line, cursor.0, column_offset);

        // Show the window size
        // self.append_abuf(&format!("{}, {}   ", self.get_height(), self.get_width()));
        self.draw_content(content, row_offset, new_column_offset);

        self.append_abuf(&format!("\x1b[{};{}H", cursor.1 + 1, new_cursor_x));

        // Show the cursor again
        self.append_abuf("\x1b[?25h");

        write_flush(str::from_utf8(&self.abuf).unwrap());
        trace!("Screen refreshed");
    }

    // TODO: Move this somewhere else
    pub fn get_horizontal_cursor_position(
        &self,
        content_line: &str,
        cursor_x: usize,
        column_offset: usize,
    ) -> (usize, usize) {
        // We assume that the cursor is between 0 and the screen width
        if cursor_x + column_offset < content_line.len() {
            (column_offset, cursor_x)
        } else {
            if content_line.len() > column_offset {
                (column_offset, content_line.len() - column_offset)
            } else {
                (content_line.len() - 1, 0)
            }
        }
    }

    fn draw_content(&mut self, content: Content, row_offset: usize, column_offset: usize) {
        // Extract content reference before the mutable borrows
        let content_len = content.lines.len();
        match content_len {
            0 => self.draw_welcome_message(),
            _ => {
                self.draw_content_rows(content, row_offset, column_offset)
                    .unwrap();
                self.draw_filler_rows(content_len);
            }
        }
    }

    // TODO: For some reason the lines are flickering when scrolling left and right
    fn draw_content_rows(
        &mut self,
        content: Content,
        row_offset: usize,
        column_offset: usize,
    ) -> Result<(), Error> {
        trace!("Drawing content rows");

        let lines = content.lines.clone();

        // Only iterate up to the minimum of screen height and content length
        let visible_lines = lines.len().min(self.get_height());

        for y in 0..visible_lines {
            let line = &lines[y + row_offset]; // Use reference instead of clone

            // TODO: Clean this up
            // TODO: Support moving up and down with the cursor when the cursor is at the end of the line
            let line_start = column_offset;
            let line_end = line.len().min(self.get_width() + column_offset);
            if line_start < line_end {
                self.append_abuf(&line[line_start..line_end]); // No need for to_vec()
            }

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
