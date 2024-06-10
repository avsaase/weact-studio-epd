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
    /// * `.0`: byte value in the first buffer
    /// * `.1`: byte value in the second buffer (only applicable to TriColor)
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
