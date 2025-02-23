#[cfg(feature = "graphics")]
use embedded_graphics::pixelcolor::{BinaryColor, Rgb555, Rgb565, Rgb888, RgbColor};
use sealed::sealed;

/// Color definition for B/W displays
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Color {
    /// Black color
    Black,
    /// White color
    #[default]
    White,
}

#[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
#[cfg(feature = "graphics")]
impl From<BinaryColor> for Color {
    fn from(value: BinaryColor) -> Self {
        match value {
            BinaryColor::On => Color::White,
            BinaryColor::Off => Color::Black,
        }
    }
}

/// Conversion to RGB888 to use `Color` with `embedded-graphics-simulator`.
#[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
#[cfg(feature = "graphics")]
impl From<Color> for Rgb888 {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => Rgb888::BLACK,
            Color::White => Rgb888::WHITE,
        }
    }
}

/// Conversion from RGB888 to use `Color` with `embedded-graphics-simulator`.
///
/// Panics if the RGB value is not black or white.
#[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
#[cfg(feature = "graphics")]
impl From<Rgb888> for Color {
    fn from(value: Rgb888) -> Self {
        match value {
            Rgb888::BLACK => Color::Black,
            Rgb888::WHITE => Color::White,
            _ => panic!("RGB value must be black or white"),
        }
    }
}

/// Conversion from Rgb565 to use `Color` with tinybmp
#[cfg(feature = "graphics")]
impl From<Rgb565> for Color {
    fn from(rgb: Rgb565) -> Self {
        if rgb == RgbColor::BLACK {
            Color::White
        } else if rgb == RgbColor::WHITE {
            Color::Black
        } else {
            // choose closest color
            if (rgb.r() as u16 + rgb.g() as u16 + rgb.b() as u16) > 255 * 3 / 2 {
                Color::Black
            } else {
                Color::White
            }
        }
    }
}

/// Conversion to Rgb565 to use `Color` with tinybmp
#[cfg(feature = "graphics")]
impl From<Color> for Rgb565 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => Self::WHITE,
            Color::White => Self::BLACK,
        }
    }
}

/// Conversion from Rgb555 to use `Color` with tinybmp
#[cfg(feature = "graphics")]
impl From<Rgb555> for Color {
    fn from(rgb: Rgb555) -> Self {
        if rgb == RgbColor::BLACK {
            Color::White
        } else if rgb == RgbColor::WHITE {
            Color::Black
        } else {
            // choose closest color
            if (rgb.r() as u16 + rgb.g() as u16 + rgb.b() as u16) > 255 * 3 / 2 {
                Color::Black
            } else {
                Color::White
            }
        }
    }
}

/// Conversion to Rgb555 to use `Color` with tinybmp
#[cfg(feature = "graphics")]
impl From<Color> for Rgb555 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => Self::WHITE,
            Color::White => Self::BLACK,
        }
    }
}

/// Color for tri-color displays
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TriColor {
    /// Black color
    Black,
    /// White color
    #[default]
    White,
    /// Red color
    Red,
}

/// Conversion to RGB888 to use `TriColor` with `embedded-graphics-simulator`.
#[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
#[cfg(feature = "graphics")]
impl From<TriColor> for Rgb888 {
    fn from(val: TriColor) -> Self {
        match val {
            TriColor::White => Rgb888::WHITE,
            TriColor::Black => Rgb888::BLACK,
            TriColor::Red => Rgb888::RED,
        }
    }
}

/// Conversion from RGB888 to use `Color` with `embedded-graphics-simulator`.
///
/// Panics if the RGB value is not black or white.
#[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
#[cfg(feature = "graphics")]
impl From<Rgb888> for TriColor {
    fn from(value: Rgb888) -> Self {
        match value {
            Rgb888::BLACK => TriColor::Black,
            Rgb888::WHITE => TriColor::White,
            Rgb888::RED => TriColor::Red,
            _ => panic!("RGB value must be black, white, or red"),
        }
    }
}

/// Color trait for use in `Display`s.
#[sealed]
pub trait ColorType {
    /// Number of buffers used to represent this color type.
    const BUFFER_COUNT: usize;

    /// Byte value of this color in the buffer.
    ///
    /// Useful for setting the full buffer to a single color.
    ///
    /// Return values are:
    /// * `.0`: byte value in the first buffer
    /// * `.1`: byte value in the second buffer (only applicable to TriColor)
    fn byte_value(&self) -> (u8, u8);

    /// Bit value of this color in the buffer.
    ///
    /// Return values are:
    /// * `.0`: bit value in the first buffer
    /// * `.1`: bit value in the second buffer (only applicable to TriColor)
    fn bit_value(&self) -> (u8, u8);
}

#[sealed]
impl ColorType for Color {
    const BUFFER_COUNT: usize = 1;

    fn byte_value(&self) -> (u8, u8) {
        match self {
            Color::Black => (0x00, 0),
            Color::White => (0xFF, 0),
        }
    }

    fn bit_value(&self) -> (u8, u8) {
        match self {
            Color::Black => (0b0, 0),
            Color::White => (0b1, 0),
        }
    }
}

#[sealed]
impl ColorType for TriColor {
    const BUFFER_COUNT: usize = 2;

    fn byte_value(&self) -> (u8, u8) {
        // Red buffer value takes precedence over B/W buffer value.
        match self {
            TriColor::Black => (0x00, 0x00),
            TriColor::White => (0xFF, 0x00),
            TriColor::Red => (0, 0xFF),
        }
    }

    fn bit_value(&self) -> (u8, u8) {
        // Red buffer value takes precedence over B/W buffer value.
        match self {
            TriColor::Black => (0b0, 0b0),
            TriColor::White => (0b1, 0b0),
            TriColor::Red => (0, 0b1),
        }
    }
}
