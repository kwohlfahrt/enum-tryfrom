//! This crate defines types and traits for use with `enum-tryfrom-derive` See
//! the documentation of that crate for usage details.
#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="std")]
extern crate core;

use core::fmt::Display;

#[derive(Debug,Default)]
pub struct InvalidEnumValue(());

const DESCRIPTION : &str = "Attempted to convert invalid value to enum";

impl InvalidEnumValue {
    pub fn new() -> Self {
        InvalidEnumValue(())
    }
}

impl Display for InvalidEnumValue {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        DESCRIPTION.fmt(fmt)
    }
}

#[cfg(feature="std")]
impl std::error::Error for InvalidEnumValue {
    fn description(&self) -> &str {
        DESCRIPTION
    }
}

#[cfg(all(test, feature="std"))]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_description() {
        assert_eq!(InvalidEnumValue::new().description(), DESCRIPTION);
    }
}
