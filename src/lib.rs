//! Display driver for WeAct Studio e-paper displays.

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
