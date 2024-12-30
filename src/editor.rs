use crate::{screen::Screen, stdin_raw_mode::StdinRawMode};
use core::str;
use std::{
    io::{self, Error, Read, Write},
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
    screen: Screen,

    _stdin: StdinRawMode,

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
            if self.editor_process_keypress().is_err() {
                break;
            }
        }
    }

    pub fn editor_process_keypress(&mut self) -> Result<(), ()> {
        let c = editor_read_key();

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

fn editor_read_key() -> u8 {
    let mut buffer = [0; 1];
    let read = io::stdin().read(&mut buffer);
    read.unwrap();

    buffer[0]
}
