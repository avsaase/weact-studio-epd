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

pub use driver::DisplayDriver;

pub use driver::WeActStudio213BlackWhiteDriver;
pub use driver::WeActStudio213TriColorDriver;
pub use driver::WeActStudio290BlackWhiteDriver;
pub use driver::WeActStudio290TriColorDriver;

/// Alias for `Result<T, DisplayError>`.
pub type Result<T> = core::result::Result<T, display_interface::DisplayError>;
