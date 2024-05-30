use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};

use crate::{color, command, flag};

const RESET_DELAY_MS: u32 = 10;

pub struct Driver<DI, BSY, RST, DELAY> {
    interface: DI,
    busy: BSY,
    reset: RST,
    delay: DELAY,
}

impl<DI, BSY, RST, DELAY> Driver<DI, BSY, RST, DELAY>
where
    DI: WriteOnlyDataCommand,
    BSY: InputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// Display height
    const HEIGHT: u16 = 296;

    /// Display width
    const WIDTH: u16 = 128;

    /// Create a new display driver
    pub fn new(interface: DI, busy: BSY, reset: RST, delay: DELAY) -> Self {
        Self {
            interface,
            busy,
            reset,
            delay,
        }
    }

    /// Initialize the display
    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.reset();
        self.command(command::SW_RESET)?;
        self.wait_until_idle();

        self.command_with_data(
            command::DRIVER_CONTROL,
            &[
                (Self::HEIGHT - 1) as u8,
                ((Self::HEIGHT - 1) >> 8) as u8,
                0x00,
            ],
        )?;

        self.command_with_data(command::DATA_ENTRY_MODE, &[flag::DATA_ENTRY_INCRY_INCRX])?;

        self.command_with_data(
            command::BORDER_WAVEFORM_CONTROL,
            &[flag::BORDER_WAVEFORM_FOLLOW_LUT | flag::BORDER_WAVEFORM_LUT1],
        )?;

        self.command_with_data(command::TEMP_CONTROL, &[flag::INTERNAL_TEMP_SENSOR])?;

        self.command_with_data(command::DISPLAY_UPDATE_CONTROL, &[0x00, 0x80])?;

        self.use_full_frame()?;

        self.wait_until_idle();
        Ok(())
    }

    /// Update the whole BW buffer on the display driver
    pub fn update_bw_frame(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.use_full_frame()?;
        self.command_with_data(command::WRITE_BW_DATA, &buffer)
    }

    /// Update the whole Red buffer on the display driver
    pub fn update_red_frame(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.use_full_frame()?;
        self.command_with_data(command::WRITE_RED_DATA, &buffer)
    }

    /// Start an update of the whole display
    pub fn display_frame(&mut self) -> Result<(), DisplayError> {
        self.command_with_data(command::UPDATE_DISPLAY_CTRL2, &[flag::DISPLAY_MODE_1])?;
        self.command(command::MASTER_ACTIVATE)?;
        self.wait_until_idle();
        Ok(())
    }

    /// Make the whole black and white frame on the display driver white
    pub fn clear_bw_frame(&mut self) -> Result<(), DisplayError> {
        self.use_full_frame()?;

        // TODO: allow non-white background color
        let color = color::Color::White.get_byte_value();

        self.command(command::WRITE_BW_DATA)?;
        self.data_x_times(color, u32::from(Self::WIDTH) / 8 * u32::from(Self::HEIGHT))?;
        Ok(())
    }

    fn reset(&mut self) {
        self.reset.set_low().unwrap();
        self.delay.delay_ms(RESET_DELAY_MS);
        self.reset.set_high().unwrap();
        self.delay.delay_ms(RESET_DELAY_MS);
    }

    /// Wrapper function around display-interface send_commands function
    fn command(&mut self, command: u8) -> Result<(), DisplayError> {
        self.interface.send_commands(DataFormat::U8(&[command]))
    }

    /// Basic function for sending an array of u8-values of data over spi
    /// Wrapper function around display-interface send_data function
    fn data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(DataFormat::U8(data))
    }

    /// Basic function for sending a command and the data belonging to it.
    fn command_with_data(&mut self, command: u8, data: &[u8]) -> Result<(), DisplayError> {
        self.command(command)?;
        self.data(data)
    }

    /// Waits until device isn't busy anymore (busy == HIGH)
    fn wait_until_idle(&mut self) {
        while self.busy.is_high().unwrap_or(true) {
            self.delay.delay_ms(1)
        }
    }

    fn use_full_frame(&mut self) -> Result<(), DisplayError> {
        // choose full frame/ram
        self.set_ram_area(
            0,
            0,
            u32::from(Self::WIDTH) - 1,
            u32::from(Self::HEIGHT) - 1,
        )?;

        // start from the beginning
        self.set_ram_counter(0, 0)
    }

    fn set_ram_area(
        &mut self,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    ) -> Result<(), DisplayError> {
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

    fn set_ram_counter(&mut self, x: u32, y: u32) -> Result<(), DisplayError> {
        // x is positioned in bytes, so the last 3 bits which show the position inside a byte in the ram
        // aren't relevant
        self.command_with_data(command::SET_RAMX_COUNTER, &[(x >> 3) as u8])?;

        // 2 Databytes: A[7:0] & 0..A[8]
        self.command_with_data(command::SET_RAMY_COUNTER, &[y as u8, (y >> 8) as u8])?;
        Ok(())
    }

    fn data_x_times(&mut self, data: u8, repetitions: u32) -> Result<(), DisplayError> {
        for _ in 0..repetitions {
            self.data(&[data])?;
        }
        Ok(())
    }
}
