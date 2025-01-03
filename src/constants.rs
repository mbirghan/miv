use crate::logger::LogLevel;

// Version info
pub const VERSION: &'static str = "0.0.1";

// Whether to use blocking read for stdin
// Blocking read: pressing esc will not be read until a key is pressed as it could be part of an escape sequence
// Non-blocking read: we will consistently read 0 bytes until a key is pressed which could lead to consistent rerendering
// Using non-blocking read for now to allow for esc to be read
pub const BLOCKING: bool = false;

// Log level
pub const LOG_LEVEL: LogLevel = LogLevel::Info;
