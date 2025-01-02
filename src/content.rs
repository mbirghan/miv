use crate::constants::VERSION;

pub struct Content {
    pub lines: Vec<String>,
    pub num_rows: usize,
}

impl Content {
    pub fn new() -> Content {
        Content {
            lines: vec![format!("Welcome to Miv editor -- version {}", VERSION)],
            num_rows: 1,
        }
    }
}
