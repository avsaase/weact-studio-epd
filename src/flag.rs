#[allow(unused)]
mod flags {
    pub const DATA_ENTRY_INCRY_INCRX: u8 = 0b11;
    pub const INTERNAL_TEMP_SENSOR: u8 = 0x80;
    pub const BORDER_WAVEFORM_FOLLOW_LUT: u8 = 0b0100;
    pub const BORDER_WAVEFORM_LUT0: u8 = 0b00;
    pub const BORDER_WAVEFORM_LUT1: u8 = 0b01;
    pub const BORDER_WAVEFORM_LUT2: u8 = 0b10;
    pub const BORDER_WAVEFORM_LUT3: u8 = 0b11;
    pub const DISPLAY_MODE_1: u8 = 0xF7;
    pub const DISPLAY_MODE_2: u8 = 0xFF;
}

pub(crate) use flags::*;
