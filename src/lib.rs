//! This crate defines types and traits for use with `enum-tryfrom-derive` See
//! the documentation of that crate for usage details.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature="std")]
extern crate core;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }
}

#[derive(PartialEq, Debug)]
pub struct InvalidEnumValue(());

impl InvalidEnumValue {
    pub fn new() -> Self {
        InvalidEnumValue(())
    }
}

