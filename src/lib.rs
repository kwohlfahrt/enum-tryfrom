//! This crate defines types and traits for use with `enum-tryfrom-derive` See
//! the documentation of that crate for usage details.

use std::error::Error;
use std::fmt::Display;

#[derive(Debug,Default)]
pub struct InvalidEnumValue(());

const DESCRIPTION : &str = "Attempted to convert invalid value to enum";

impl InvalidEnumValue {
    pub fn new() -> Self {
        InvalidEnumValue(())
    }
}

impl Display for InvalidEnumValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        DESCRIPTION.fmt(fmt)
    }
}

impl Error for InvalidEnumValue {
    fn description(&self) -> &str {
        DESCRIPTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description() {
        assert_eq!(InvalidEnumValue::new().description(), DESCRIPTION);
    }
}
