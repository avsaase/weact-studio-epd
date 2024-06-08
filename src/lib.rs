#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

/// Color definitions
pub mod color;
mod command;
mod driver;
mod flag;
#[cfg(feature = "graphics")]
/// Graphics utilities
pub mod graphics;
mod lut;

/// Display driver
pub use driver::Driver;

/// Alias for `Result<T, DisplayError>`.
pub type Result<T> = core::result::Result<T, display_interface::DisplayError>;
