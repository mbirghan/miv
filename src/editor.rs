use crate::content::Content;
use crate::input::{Input, Key};
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
            // TODO: This should probably be moved to a function
            let key = self.input.read_key();
            match key {
                Key::None => {
                    // None means we did not read a key
                    // We should not refresh the screen as we did not read a key
                }
                Key::Other(c) if c == ctrl_key('q') => break,
                Key::Other(c) if c == ctrl_key('c') => break,
                // TODO: This feels weird, it should be moved to the controller module
                // Currently this is redundant as we already have a similar function in the screen module
                Key::ArrowUp | Key::Other(b'k') // Up
                | Key::ArrowDown | Key::Other(b'j') // Down
                | Key::ArrowLeft | Key::Other(b'h') // Left
                | Key::ArrowRight | Key::Other(b'l') // Right
                => {
                    // We read a movement key and should refresh the screen
                    self.screen.move_cursor(key);
                    self.screen.editor_refresh_screen(self.content.clone());
                }
                Key::Esc | Key::Other(_) => {
                    // We read a key and should refresh the screen
                    // self.screen.editor_refresh_screen(self.content.clone());
                    // TODO: Right now we do not need to refresh the screen as we do not change the content
                }
            }

            // Refresh screen to show the updated content
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
