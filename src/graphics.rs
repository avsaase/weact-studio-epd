use display_interface::DisplayError;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use crate::color::Color;

#[derive(Debug, Clone, Copy, Default)]
pub enum DisplayRotation {
    /// No rotation.
    #[default]
    Rotate0,
    /// Rotate by 90 degrees clockwise.
    Rotate90,
    /// Rotate by 180 degrees clockwise.
    Rotate180,
    /// Rotate 270 degrees clockwise.
    Rotate270,
}

/// Computes the needed buffer length. Takes care of rounding up in case `width`
/// is not divisible by 8.
pub const fn buffer_len(width: usize, height: usize) -> usize {
    (width + 7) / 8 * height
}

/// The in-memory display buffer to render on using `embedded-graphics.
///
/// `BUFFER_SIZE` can be calculated using [`buffer_len`].
pub struct Display<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> {
    buffer: [u8; BUFFER_SIZE],
    rotation: DisplayRotation,
    pub is_inverted: bool,
}

/// Display buffer for the WeAct Studio 2.9 inch B/W display.
pub type Bw0290Display = Display<128, 296, { buffer_len(128usize, 296) }>;

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize>
    Display<WIDTH, HEIGHT, BUFFER_SIZE>
{
    pub fn bw() -> Self {
        Self {
            buffer: [Color::White.get_byte_value(); BUFFER_SIZE],
            rotation: DisplayRotation::default(),
            is_inverted: false,
        }
    }
}

pub trait DisplayTrait: DrawTarget {
    /// Clears the buffer of the display with the chosen background color.
    fn clear_buffer(&mut self, background_color: Color) {
        let fill_color = if self.is_inverted() {
            background_color.inverse()
        } else {
            background_color
        };

        for elem in self.buffer_mut().iter_mut() {
            *elem = fill_color.get_byte_value();
        }
    }

    /// Returns the buffer.
    fn buffer(&self) -> &[u8];

    /// Returns a mutable buffer.
    fn buffer_mut(&mut self) -> &mut [u8];

    /// Sets the rotation of the display.
    fn set_rotation(&mut self, rotation: DisplayRotation);

    /// Get the current rotation of the display.
    fn rotation(&self) -> DisplayRotation;

    /// If the color for this display is inverted.
    fn is_inverted(&self) -> bool;

    /// Helper function for the Embedded Graphics draw trait.
    fn draw_helper(
        &mut self,
        width: u32,
        height: u32,
        pixel: Pixel<BinaryColor>,
    ) -> Result<(), Self::Error> {
        let rotation = self.rotation();
        let is_inverted = self.is_inverted();
        let buffer = self.buffer_mut();

        let Pixel(point, color) = pixel;
        if outside_display(point, width, height, rotation) {
            return Ok(());
        }

        let (index, bit) = find_position(point.x as u32, point.y as u32, width, height, rotation);
        let index = index as usize;

        // "Draw" the Pixel on that bit
        match color {
            // White/Red
            BinaryColor::On => {
                if is_inverted {
                    buffer[index] &= !bit;
                } else {
                    buffer[index] |= bit;
                }
            }
            //Black
            BinaryColor::Off => {
                if is_inverted {
                    buffer[index] |= bit;
                } else {
                    buffer[index] &= !bit;
                }
            }
        }
        Ok(())
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> DrawTarget
    for Display<WIDTH, HEIGHT, BUFFER_SIZE>
{
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for p in pixels.into_iter() {
            self.draw_helper(WIDTH, HEIGHT, p)?;
        }
        Ok(())
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> OriginDimensions
    for Display<WIDTH, HEIGHT, BUFFER_SIZE>
{
    fn size(&self) -> Size {
        //if display is rotated 90 deg or 270 then swap height and width
        match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => Size::new(WIDTH, HEIGHT),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => Size::new(HEIGHT, WIDTH),
        }
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> DisplayTrait
    for Display<WIDTH, HEIGHT, BUFFER_SIZE>
{
    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

fn outside_display(p: Point, width: u32, height: u32, rotation: DisplayRotation) -> bool {
    if p.x < 0 || p.y < 0 {
        return true;
    }
    let (x, y) = (p.x as u32, p.y as u32);
    match rotation {
        DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
            if x >= width || y >= height {
                return true;
            }
        }
        DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
            if y >= width || x >= height {
                return true;
            }
        }
    }
    false
}

fn find_position(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u8) {
    let (nx, ny) = find_rotation(x, y, width, height, rotation);
    (nx / 8 + ((width + 7) / 8) * ny, 0x80 >> (nx % 8))
}

fn find_rotation(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u32) {
    let nx;
    let ny;
    match rotation {
        DisplayRotation::Rotate0 => {
            nx = x;
            ny = y;
        }
        DisplayRotation::Rotate90 => {
            nx = width - 1 - y;
            ny = x;
        }
        DisplayRotation::Rotate180 => {
            nx = width - 1 - x;
            ny = height - 1 - y;
        }
        DisplayRotation::Rotate270 => {
            nx = y;
            ny = height - 1 - x;
        }
    }
    (nx, ny)
}

// pub struct VarDisplay<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> {
//     rotation: DisplayRotation,
//     is_inverted: bool,
//     buffer: [u8; BUFFER_SIZE],
// }

// impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize>
//     VarDisplay<WIDTH, HEIGHT, BUFFER_SIZE>
// {
//     pub fn bw() -> Self {
//         Self {
//             rotation: DisplayRotation::default(),
//             is_inverted: false,
//             buffer: [0; BUFFER_SIZE],
//         }
//     }

//     pub fn red() -> Self {
//         Self {
//             rotation: DisplayRotation::default(),
//             is_inverted: true,
//             buffer: [0; BUFFER_SIZE],
//         }
//     }
// }

// impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> DisplayTrait
//     for VarDisplay<WIDTH, HEIGHT, BUFFER_SIZE>
// {
//     fn buffer(&self) -> &[u8] {
//         &self.buffer
//     }

//     fn buffer_mut(&mut self) -> &mut [u8] {
//         &mut self.buffer
//     }

//     fn set_rotation(&mut self, rotation: DisplayRotation) {
//         self.rotation = rotation;
//     }

//     fn rotation(&self) -> DisplayRotation {
//         self.rotation
//     }

//     fn is_inverted(&self) -> bool {
//         self.is_inverted
//     }
// }

// impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> DrawTarget
//     for VarDisplay<WIDTH, HEIGHT, BUFFER_SIZE>
// {
//     type Color = BinaryColor;
//     type Error = DisplayError;

//     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//     where
//         I: IntoIterator<Item = Pixel<Self::Color>>,
//     {
//         for p in pixels.into_iter() {
//             self.draw_helper(WIDTH, HEIGHT, p)?;
//         }
//         Ok(())
//     }
// }

// impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> OriginDimensions
//     for VarDisplay<WIDTH, HEIGHT, BUFFER_SIZE>
// {
//     fn size(&self) -> Size {
//         //if display is rotated 90 deg or 270 then swap height and width
//         match self.rotation() {
//             DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => Size::new(WIDTH, HEIGHT),
//             DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => Size::new(HEIGHT, WIDTH),
//         }
//     }
// }
