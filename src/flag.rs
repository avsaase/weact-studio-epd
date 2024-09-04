#[allow(unused)]
mod flags {
    pub const DEEP_SLEEP_NORMAL_MODE: u8 = 0b00; // Also called Power on Reset [POR]
    pub const DEEP_SLEEP_MODE_1: u8 = 0b01; // RAM is retained (but not accessible)
    pub const DEEP_SLEEP_MODE_2: u8 = 0b11; // RAM is not retained
    pub const DATA_ENTRY_INCRY_INCRX: u8 = 0b11;
    pub const INTERNAL_TEMP_SENSOR: u8 = 0x80;
    pub const BORDER_WAVEFORM_FOLLOW_LUT: u8 = 0b0100;
    pub const BORDER_WAVEFORM_LUT0: u8 = 0b00;
    pub const BORDER_WAVEFORM_LUT1: u8 = 0b01;
    pub const BORDER_WAVEFORM_LUT2: u8 = 0b10;
    pub const BORDER_WAVEFORM_LUT3: u8 = 0b11;
    pub const DISPLAY_MODE_1: u8 = 0xF7;
    pub const DISPLAY_MODE_2: u8 = 0xFF;
    /// Undocumented value for the "Display Update Control 2 (`0x22`) command.
    /// Together with the [`crate::lut::LUT_PARTIAL_UPDATE`] lut this yields much better looking
    /// quick refreshes.
    pub const UNDOCUMENTED: u8 = 0xCC;
}

pub(crate) use flags::*;
