use core::panic;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    /// Black color
    Black,
    /// White color
    White,
}

impl Color {
    pub fn get_byte_value(self) -> u8 {
        match self {
            Color::Black => 0x00,
            Color::White => 0xff,
        }
    }

    pub fn inverse(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color value: {}", value),
        }
    }
}
