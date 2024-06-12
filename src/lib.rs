#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/// Color definitions
mod color;
mod command;
mod driver;
mod flag;
#[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
#[cfg(feature = "graphics")]
/// `embedded-graphics` support.
pub mod graphics;
mod lut;

pub use color::{Color, TriColor};
pub use driver::*;

/// Alias for `Result<T, DisplayError>`.
pub type Result<T> = core::result::Result<T, display_interface::DisplayError>;
