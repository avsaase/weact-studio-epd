#[allow(unused)]
mod commands {
    pub const DRIVER_CONTROL: u8 = 0x01;
    pub const SET_SOFTSTART: u8 = 0x0C;
    pub const DEEP_SLEEP: u8 = 0x10;
    pub const DATA_ENTRY_MODE: u8 = 0x11;
    pub const SW_RESET: u8 = 0x12;
    pub const TEMP_CONTROL: u8 = 0x18;
    pub const MASTER_ACTIVATE: u8 = 0x20;
    pub const DISPLAY_UPDATE_CONTROL: u8 = 0x21;
    pub const UPDATE_DISPLAY_CTRL2: u8 = 0x22;
    pub const WRITE_BW_DATA: u8 = 0x24;
    pub const WRITE_RED_DATA: u8 = 0x26;
    pub const WRITE_VCOM: u8 = 0x2C;
    pub const WRITE_LUT: u8 = 0x32;
    pub const BORDER_WAVEFORM_CONTROL: u8 = 0x3C;
    pub const SET_RAMXPOS: u8 = 0x44;
    pub const SET_RAMYPOS: u8 = 0x45;
    pub const SET_RAMX_COUNTER: u8 = 0x4E;
    pub const SET_RAMY_COUNTER: u8 = 0x4F;
    pub const NOP: u8 = 0xFF;
}

pub(crate) use commands::*;
