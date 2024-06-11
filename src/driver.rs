use core::iter;

use display_interface::{DataFormat, WriteOnlyDataCommand};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};

use crate::{
    color::{self, ColorType},
    command, flag,
    graphics::Display,
    lut, Color, Result, TriColor,
};

/// Display driver for the WeAct Studio 2.9 inch B/W display.
pub type WeActStudio290BlackWhiteDriver<DI, BSY, RST, DELAY> =
    DisplayDriver<DI, BSY, RST, DELAY, 128, 128, 296, Color>;
/// Display driver for the WeAct Studio 2.9 inch Tri-Color display.
pub type WeActStudio290TriColorDriver<DI, BSY, RST, DELAY> =
    DisplayDriver<DI, BSY, RST, DELAY, 128, 128, 296, TriColor>;
/// Display driver for the WeAct Studio 2.13 inch B/W display.
pub type WeActStudio213BlackWhiteDriver<DI, BSY, RST, DELAY> =
    DisplayDriver<DI, BSY, RST, DELAY, 128, 122, 250, Color>;
/// Display driver for the WeAct Studio 2.13 inch Tri-Color display.
pub type WeActStudio213TriColorDriver<DI, BSY, RST, DELAY> =
    DisplayDriver<DI, BSY, RST, DELAY, 128, 122, 250, TriColor>;

/// The main driver struct that manages the communication with the display.
///
/// You probably want to use one of the display-specific type aliases instead.
pub struct DisplayDriver<
    DI,
    BSY,
    RST,
    DELAY,
    const WIDTH: u32,
    const VISIBLE_WIDTH: u32,
    const HEIGHT: u32,
    C,
> {
    _color: core::marker::PhantomData<C>,
    interface: DI,
    busy: BSY,
    reset: RST,
    delay: DELAY,
    // State
    using_partial_mode: bool,
    initial_full_refresh_done: bool,
}

impl<DI, BSY, RST, DELAY, const WIDTH: u32, const VISIBLE_WIDTH: u32, const HEIGHT: u32, C>
    DisplayDriver<DI, BSY, RST, DELAY, WIDTH, VISIBLE_WIDTH, HEIGHT, C>
where
    DI: WriteOnlyDataCommand,
    BSY: InputPin,
    RST: OutputPin,
    DELAY: DelayNs,
    C: ColorType,
{
    const RESET_DELAY_MS: u32 = 50;

    /// Create a new display driver.
    ///
    /// Use [`Self::init`] to initialize the display.
    pub fn new(interface: DI, busy: BSY, reset: RST, delay: DELAY) -> Self {
        Self {
            _color: core::marker::PhantomData,
            interface,
            busy,
            reset,
            delay,
            using_partial_mode: false,
            initial_full_refresh_done: false,
        }
    }

    /// Initialize the display
    pub fn init(&mut self) -> Result<()> {
        self.hw_reset();
        self.command(command::SW_RESET)?;
        self.delay.delay_ms(10);
        self.wait_until_idle();
        self.command_with_data(
            command::DRIVER_CONTROL,
            &[(HEIGHT - 1) as u8, ((HEIGHT - 1) >> 8) as u8, 0x00],
        )?;
        self.command_with_data(command::DATA_ENTRY_MODE, &[flag::DATA_ENTRY_INCRY_INCRX])?;
        self.command_with_data(
            command::BORDER_WAVEFORM_CONTROL,
            &[flag::BORDER_WAVEFORM_FOLLOW_LUT | flag::BORDER_WAVEFORM_LUT1],
        )?;
        self.command_with_data(command::DISPLAY_UPDATE_CONTROL, &[0x00, 0x80])?;
        self.command_with_data(command::TEMP_CONTROL, &[flag::INTERNAL_TEMP_SENSOR])?;
        self.use_full_frame()?;
        self.wait_until_idle();
        Ok(())
    }

    /// Perform a hardware reset of the display.
    pub fn hw_reset(&mut self) {
        self.reset.set_low().unwrap();
        self.delay.delay_ms(Self::RESET_DELAY_MS);
        self.reset.set_high().unwrap();
        self.delay.delay_ms(Self::RESET_DELAY_MS);
    }

    /// Write to the B/W buffer.
    pub fn write_bw_buffer(&mut self, buffer: &[u8]) -> Result<()> {
        self.use_full_frame()?;
        self.command_with_data(command::WRITE_BW_DATA, buffer)?;
        Ok(())
    }

    /// Write to the red buffer.
    ///
    /// On B/W displays this buffer is used for quick refreshes.
    pub fn write_red_buffer(&mut self, buffer: &[u8]) -> Result<()> {
        self.use_full_frame()?;
        self.command_with_data(command::WRITE_RED_DATA, buffer)?;
        Ok(())
    }

    /// Write to the B/W buffer at the given position.
    ///
    /// `x`, and `width` must be multiples of 8.
    pub fn write_partial_bw_buffer(
        &mut self,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.use_partial_frame(x, y, width, height)?;
        self.command_with_data(command::WRITE_BW_DATA, buffer)?;
        Ok(())
    }

    /// Write to the red buffer at the given position.
    ///
    /// `x`, and `width` must be multiples of 8.
    ///
    /// On B/W displays this buffer is used for quick refreshes.
    pub fn write_partial_red_buffer(
        &mut self,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.use_partial_frame(x, y, width, height)?;
        self.command_with_data(command::WRITE_RED_DATA, buffer)?;
        Ok(())
    }

    /// Make the whole black and white frame on the display driver white.
    pub fn clear_bw_buffer(&mut self) -> Result<()> {
        self.use_full_frame()?;

        // TODO: allow non-white background color
        let color = color::Color::White.byte_value().0;

        self.command(command::WRITE_BW_DATA)?;
        self.data_x_times(color, u32::from(WIDTH) / 8 * u32::from(HEIGHT))?;
        Ok(())
    }

    /// Make the whole red frame on the display driver white.
    ///
    /// On B/W displays this buffer is used for quick refreshes.
    pub fn clear_red_buffer(&mut self) -> Result<()> {
        self.use_full_frame()?;

        // TODO: allow non-white background color
        let color = color::Color::White.byte_value().1;

        self.command(command::WRITE_RED_DATA)?;
        self.data_x_times(color, u32::from(WIDTH) / 8 * u32::from(HEIGHT))?;
        Ok(())
    }

    /// Start a full refresh of the display.
    pub fn refresh(&mut self) -> Result<()> {
        self.initial_full_refresh_done = true;
        self.using_partial_mode = false;

        self.command_with_data(command::UPDATE_DISPLAY_CTRL2, &[flag::DISPLAY_MODE_1])?;
        self.command(command::MASTER_ACTIVATE)?;
        self.wait_until_idle();
        Ok(())
    }

    fn use_full_frame(&mut self) -> Result<()> {
        self.use_partial_frame(0, 0, u32::from(WIDTH), u32::from(HEIGHT))?;
        Ok(())
    }

    fn use_partial_frame(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<()> {
        // TODO: make sure positions are byte-aligned
        self.set_ram_area(x, y, x + width - 1, y + height - 1)?;
        self.set_ram_counter(x, y)?;
        Ok(())
    }

    fn set_ram_area(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Result<()> {
        assert!(start_x < end_x);
        assert!(start_y < end_y);

        self.command_with_data(
            command::SET_RAMXPOS,
            &[(start_x >> 3) as u8, (end_x >> 3) as u8],
        )?;

        self.command_with_data(
            command::SET_RAMYPOS,
            &[
                start_y as u8,
                (start_y >> 8) as u8,
                end_y as u8,
                (end_y >> 8) as u8,
            ],
        )?;
        Ok(())
    }

    fn set_ram_counter(&mut self, x: u32, y: u32) -> Result<()> {
        // x is positioned in bytes, so the last 3 bits which show the position inside a byte in the ram
        // aren't relevant
        self.command_with_data(command::SET_RAMX_COUNTER, &[(x >> 3) as u8])?;

        // 2 Databytes: A[7:0] & 0..A[8]
        self.command_with_data(command::SET_RAMY_COUNTER, &[y as u8, (y >> 8) as u8])?;
        Ok(())
    }

    /// Send a command to the display.
    fn command(&mut self, command: u8) -> Result<()> {
        self.interface.send_commands(DataFormat::U8(&[command]))?;
        Ok(())
    }

    /// Send an array of bytes to the display.
    fn data(&mut self, data: &[u8]) -> Result<()> {
        self.interface.send_data(DataFormat::U8(data))?;
        self.wait_until_idle();
        Ok(())
    }

    /// Waits until device isn't busy anymore (busy == HIGH).
    fn wait_until_idle(&mut self) {
        while self.busy.is_high().unwrap_or(true) {
            self.delay.delay_ms(1)
        }
    }

    /// Sending a command and the data belonging to it.
    fn command_with_data(&mut self, command: u8, data: &[u8]) -> Result<()> {
        self.command(command)?;
        self.data(data)?;
        Ok(())
    }

    /// Send a byte to the display mutiple times.
    fn data_x_times(&mut self, data: u8, repetitions: u32) -> Result<()> {
        let mut iter = iter::repeat(data).take(repetitions as usize);
        self.interface.send_data(DataFormat::U8Iter(&mut iter))?;
        Ok(())
    }
}

// Functions avialable only for B/W displays
impl<DI, BSY, RST, DELAY, const WIDTH: u32, const VISIBLE_WIDTH: u32, const HEIGHT: u32>
    DisplayDriver<DI, BSY, RST, DELAY, WIDTH, VISIBLE_WIDTH, HEIGHT, Color>
where
    DI: WriteOnlyDataCommand,
    BSY: InputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// Start a quick refresh of the display.
    ///
    /// If the display hasn't done a full refresh yet, it will do that first.
    pub fn quick_refresh(&mut self) -> Result<()> {
        if !self.initial_full_refresh_done {
            // There a bug here which causes the new image to overwrite the existing image which then
            // fades out over several updates.
            self.refresh()?;
        }

        if !self.using_partial_mode {
            self.command_with_data(command::WRITE_LUT, &lut::LUT_PARTIAL_UPDATE)?;
            self.using_partial_mode = true;
        }
        self.command_with_data(command::UPDATE_DISPLAY_CTRL2, &[flag::UNDOCUMENTED])?;
        self.command(command::MASTER_ACTIVATE)?;
        self.wait_until_idle();
        Ok(())
    }

    /// Update the screen with the provided buffer using a full refresh.
    pub fn update(&mut self, buffer: &[u8]) -> Result<()> {
        self.write_red_buffer(buffer)?;
        self.write_bw_buffer(buffer)?;
        self.refresh()?;
        self.write_red_buffer(buffer)?;
        self.write_bw_buffer(buffer)?;
        Ok(())
    }

    /// Update the screen with the provided buffer using a quick refresh.
    pub fn quick_update(&mut self, buffer: &[u8]) -> Result<()> {
        self.write_red_buffer(buffer)?;
        self.quick_refresh()?;
        self.write_red_buffer(buffer)?;
        self.write_bw_buffer(buffer)?;
        Ok(())
    }

    /// Update the screen with the provided buffer at the given position using a partial refresh.
    ///
    /// `x`, and `width` must be multiples of 8.
    pub fn quick_partial_update(
        &mut self,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.write_partial_bw_buffer(buffer, x, y, width, height)?;
        self.quick_refresh()?;
        self.write_partial_red_buffer(buffer, x, y, width, height)?;
        self.write_partial_bw_buffer(buffer, x, y, width, height)?;
        Ok(())
    }

    /// Update the screen with the provided [`Display`] using a full refresh.
    #[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
    #[cfg(feature = "graphics")]
    pub fn update_display<const W: u32, const H: u32, const BUFFER_SIZE: usize>(
        &mut self,
        display: &Display<W, H, BUFFER_SIZE, crate::color::Color>,
    ) -> Result<()> {
        self.update(display.buffer())
    }

    /// Update the screen with the provided [`Display`] at the given position using a partial refresh.
    ///
    /// `x` must be multiples of 8.
    #[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
    #[cfg(feature = "graphics")]
    pub fn quick_partial_update_display<const W: u32, const H: u32, const BUFFER_SIZE: usize>(
        &mut self,
        display: &Display<W, H, BUFFER_SIZE, crate::color::Color>,
        x: u32,
        y: u32,
    ) -> Result<()> {
        self.quick_partial_update(display.buffer(), x, y, WIDTH, HEIGHT)
    }
}

// Functions avialable only for Tricolor displays
impl<DI, BSY, RST, DELAY, const WIDTH: u32, const VISIBLE_WIDTH: u32, const HEIGHT: u32>
    DisplayDriver<DI, BSY, RST, DELAY, WIDTH, VISIBLE_WIDTH, HEIGHT, TriColor>
where
    DI: WriteOnlyDataCommand,
    BSY: InputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// Update the screen with the provided buffers using a full refresh.
    pub fn update(&mut self, bw_buffer: &[u8], red_buffer: &[u8]) -> Result<()> {
        self.write_red_buffer(red_buffer)?;
        self.write_bw_buffer(bw_buffer)?;
        self.refresh()?;
        Ok(())
    }

    /// Update the screen with the provided [`Display`] using a full refresh.
    #[cfg_attr(docsrs, doc(cfg(feature = "graphics")))]
    #[cfg(feature = "graphics")]
    pub fn update_display<const W: u32, const H: u32, const BUFFER_SIZE: usize>(
        &mut self,
        display: Display<W, H, BUFFER_SIZE, crate::color::TriColor>,
    ) -> Result<()> {
        self.update(display.bw_buffer(), display.red_buffer())
    }
}
