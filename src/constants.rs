use crate::logger::LogLevel;
use once_cell::sync::Lazy;
use std::env;

// Version info
pub const VERSION: &'static str = "0.0.1";

// Whether to use blocking read for stdin
// Blocking read: pressing esc will not be read until a key is pressed as it could be part of an escape sequence
// Non-blocking read: we will consistently read 0 bytes until a key is pressed which could lead to consistent rerendering
// Using non-blocking read for now to allow for esc to be read
pub const BLOCKING_READ: Lazy<bool> = Lazy::new(|| {
    env::var("MIV_BLOCKING_READ")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap()
});

// Log level
pub static LOG_LEVEL: Lazy<LogLevel> = Lazy::new(|| {
    LogLevel::from_string_or_default(&env::var("MIV_LOG_LEVEL").unwrap_or("INFO".to_string()))
});

// Tab width
pub const TAB_WIDTH: Lazy<usize> = Lazy::new(|| {
    env::var("MIV_TAB_WIDTH")
        .unwrap_or("4".to_string())
        .parse()
        .unwrap()
});
