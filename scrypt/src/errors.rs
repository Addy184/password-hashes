use core::fmt;

/// `scrypt()` error
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidOutputLen;

/// `ScryptParams` error
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidParams;

impl fmt::Display for InvalidOutputLen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid output buffer length")
    }
}

impl core::error::Error for InvalidOutputLen {}

impl fmt::Display for InvalidParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid scrypt parameters")
    }
}

impl core::error::Error for InvalidParams {}
