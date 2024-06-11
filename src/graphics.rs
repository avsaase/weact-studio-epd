use core::convert::Infallible;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::PixelColor,
    Pixel,
};

use crate::color::{Color, ColorType, TriColor};

/// Rotation of the display.
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
pub const fn buffer_len<C>(width: usize, height: usize) -> usize
where
    C: ColorType,
{
    (width + 7) / 8 * height * C::BUFFER_COUNT
}

/// In-memory display buffer to render to using `embedded-graphics`.
///
/// `BUFFER_SIZE` can be calculated using [`buffer_len`].
pub struct Display<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize, C> {
    buffer: [u8; BUFFER_SIZE],
    rotation: DisplayRotation,
    background_color: C,
    _color: core::marker::PhantomData<C>,
}

/// Display buffer for the WeAct Studio 2.9 inch B/W display.
pub type Display290BlackWhite = Display<128, 296, { buffer_len::<Color>(128, 296) }, Color>;
/// Display buffer for the WeAct Studio 2.9 inch tri-color display.
pub type Display290TriColor = Display<128, 296, { buffer_len::<TriColor>(128, 296) }, TriColor>;
/// Display buffer for the WeAct Studio 2.13 inch B/W display.
pub type Display213BlackWhite = Display<128, 250, { buffer_len::<Color>(128, 250) }, Color>;
/// Display buffer for the WeAct Studio 2.13 inch tri-color display.
pub type Display213TriColor = Display<128, 250, { buffer_len::<TriColor>(128, 250) }, TriColor>;

/// Generically-sized B/W display buffer.
///
/// `BUFFER_SIZE` can be calculated using [`buffer_len`].
pub type DisplayBlackWhite<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> =
    Display<WIDTH, HEIGHT, BUFFER_SIZE, Color>;

/// Generically-sized tri-color display buffer.
///
/// `BUFFER_SIZE` can be calculated using [`buffer_len`].
pub type DisplayTriColor<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize> =
    Display<WIDTH, HEIGHT, BUFFER_SIZE, TriColor>;

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize>
    Display<WIDTH, HEIGHT, BUFFER_SIZE, Color>
{
    /// Creates a new display buffer filled with the default color.
    pub fn new() -> Self {
        let background_color = Color::default();
        Self {
            buffer: [background_color.byte_value().0; BUFFER_SIZE],
            rotation: Default::default(),
            background_color,
            _color: core::marker::PhantomData,
        }
    }

    /// Get the internal buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Clear the display buffer with the default color.
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(self.background_color.byte_value().0);
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize>
    Display<WIDTH, HEIGHT, BUFFER_SIZE, TriColor>
{
    /// Creates a new display buffer filled with the default color.
    pub fn new() -> Self {
        let background_color = TriColor::default();

        let mut buffer = [0; BUFFER_SIZE];
        buffer[..(BUFFER_SIZE / 2)].fill(background_color.byte_value().0);
        buffer[(BUFFER_SIZE / 2)..].fill(background_color.byte_value().1);

        Self {
            buffer,
            rotation: Default::default(),
            background_color,
            _color: core::marker::PhantomData,
        }
    }

    /// Get the internal B/W buffer.
    pub fn bw_buffer(&self) -> &[u8] {
        &self.buffer[..(BUFFER_SIZE / 2)]
    }

    /// Get the internal red buffer.
    pub fn red_buffer(&self) -> &[u8] {
        &self.buffer[(BUFFER_SIZE / 2)..]
    }

    /// Clear the display buffer with the default color.
    pub fn clear_buffer(&mut self) {
        self.buffer[..(BUFFER_SIZE / 2)].fill(self.background_color.byte_value().0);
        self.buffer[(BUFFER_SIZE / 2)..].fill(self.background_color.byte_value().1);
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize, C>
    Display<WIDTH, HEIGHT, BUFFER_SIZE, C>
where
    C: ColorType + PixelColor,
{
    // /// Get the internal buffer.
    // pub fn buffer(&self) -> &[u8] {
    //     &self.buffer
    // }

    /// Get the current rotation of the display.
    pub fn rotation(&self) -> DisplayRotation {
        self.rotation
    }

    /// Sets the rotation of the display.
    pub fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn set_pixel(&mut self, pixel: Pixel<C>) {
        // let rotation = self.rotation;
        let Pixel(point, color) = pixel;
        let Point { x, y } = point;

        if outside_display(point, WIDTH, HEIGHT, self.rotation) {
            return;
        }

        let (index, bit) =
            pixel_position_in_buffer(x as u32, y as u32, WIDTH, HEIGHT, self.rotation);
        let index = index as usize;
        let (bw_bit, red_bit) = color.bit_value();

        #[allow(clippy::collapsible_else_if)]
        if C::BUFFER_COUNT == 2 {
            if red_bit == 1 {
                // Red buffer takes precendence over B/W buffer so no need to update B/W buffer.
                self.buffer[index + BUFFER_SIZE / 2] |= bit;
            } else {
                if bw_bit == 1 {
                    self.buffer[index] |= bit;
                } else {
                    self.buffer[index] &= !bit;
                }
                self.buffer[index + BUFFER_SIZE / 2] &= !bit;
            }
        } else {
            if bw_bit == 1 {
                self.buffer[index] |= bit;
            } else {
                self.buffer[index] &= !bit;
            }
        }
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize, C> DrawTarget
    for Display<WIDTH, HEIGHT, BUFFER_SIZE, C>
where
    C: PixelColor + ColorType,
{
    type Color = C;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for p in pixels.into_iter() {
            self.set_pixel(p);
        }
        Ok(())
    }
}

impl PixelColor for Color {
    type Raw = ();
}

impl PixelColor for TriColor {
    type Raw = ();
}

impl<const WIDTH: u32, const HEIGHT: u32, const BUFFER_SIZE: usize, C> OriginDimensions
    for Display<WIDTH, HEIGHT, BUFFER_SIZE, C>
where
    C: PixelColor + ColorType,
{
    fn size(&self) -> Size {
        //if display is rotated 90 deg or 270 then swap height and width
        match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => Size::new(WIDTH, HEIGHT),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => Size::new(HEIGHT, WIDTH),
        }
    }
}

fn outside_display(p: Point, width: u32, height: u32, rotation: DisplayRotation) -> bool {
    if p.x < 0 || p.y < 0 {
        return true;
    }
    let (x, y) = (p.x as u32, p.y as u32);
    match rotation {
        DisplayRotation::Rotate0 | DisplayRotation::Rotate180 if x >= width || y >= height => true,
        DisplayRotation::Rotate90 | DisplayRotation::Rotate270 if y >= width || x >= height => true,
        _ => false,
    }
}

/// Returns the position of the pixel in the (single color) buffer.
///
/// Return type is (byte index, bit)
fn pixel_position_in_buffer(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rotation: DisplayRotation,
) -> (u32, u8) {
    let (nx, ny) = find_rotation(x, y, width, height, rotation);
    (nx / 8 + bytes_per_line(width) * ny, 0x80 >> (nx % 8))
}

fn find_rotation(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u32) {
    match rotation {
        DisplayRotation::Rotate0 => (x, y),
        DisplayRotation::Rotate90 => (width - 1 - y, x),
        DisplayRotation::Rotate180 => (width - 1 - x, height - 1 - y),
        DisplayRotation::Rotate270 => (y, height - 1 - x),
    }
}

/// Count the number of bytes per line knowing that it may contains padding bits
const fn bytes_per_line(width: u32) -> u32 {
    (width + 7) / 8
}
