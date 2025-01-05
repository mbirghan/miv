use crate::{stdin_raw_mode::StdinRawMode, trace};
use std::io::{self, Read};

// TODO: This should be part of the input module
pub enum Key {
    None,
    Esc,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Other(u8),
}

pub struct Input {
    // TODO: Maybe we can read from here instead of the stdin module
    _stdin: StdinRawMode,
}

impl Input {
    pub fn new() -> Input {
        Input {
            _stdin: StdinRawMode::new().unwrap(),
        }
    }

    pub fn read_key(&self) -> Key {
        let c = self.read_byte();
        if c != 0 {
            trace!("Read key: {}", c);
        }

        match c {
            b'\x1b' => self.read_escape(),
            0 => Key::None,
            _ => Key::Other(c),
        }
    }

    fn read_byte(&self) -> u8 {
        let mut buffer = [0; 1];
        io::stdin().read(&mut buffer).unwrap();
        buffer[0]
    }

    fn read_escape(&self) -> Key {
        let c = self.read_byte();
        trace!("Read escape sequence: {}", c);

        match c {
            b'[' => self.read_arrow(),
            // If we read 0 bytes, we have pressed esc
            0 => Key::Esc,
            // If the key is not part of an escape sequence, we return None
            // TODO: Not sure if this is the best way to handle this
            _ => Key::None,
        }
    }

    fn read_arrow(&self) -> Key {
        let c = self.read_byte();
        trace!("Read arrow sequence: {}", c);

        match c {
            b'A' => Key::ArrowUp,
            b'B' => Key::ArrowDown,
            b'C' => Key::ArrowRight,
            b'D' => Key::ArrowLeft,
            _ => Key::None,
        }
    }
}
