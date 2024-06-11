#![no_std]
#![no_main]

use core::fmt::Write;

use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    spi::{Config, Spi},
};
use embassy_time::{Delay, Instant};
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyle,
    text::{Alignment, Text, TextStyle, TextStyleBuilder},
    Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use heapless::String;
use panic_probe as _;
use profont::PROFONT_24_POINT;
use weact_studio_epd::{
    graphics::{buffer_len, Display290BlackWhite, DisplayBlackWhite, DisplayRotation},
    Color, WeActStudio290BlackWhiteDriver,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let busy = Input::new(p.PIN_16, Pull::Up);
    let res = Output::new(p.PIN_20, Level::Low);
    let dc = Output::new(p.PIN_21, Level::Low);
    let cs = Output::new(p.PIN_22, Level::High);
    let scl = p.PIN_18;
    let sda = p.PIN_19;

    let mut spi_config = Config::default();
    spi_config.frequency = 1_000_000;
    let spi_bus = Spi::new_blocking_txonly(p.SPI0, scl, sda, spi_config);
    let spi_device = ExclusiveDevice::new(spi_bus, cs, Delay);
    let spi_interface = SPIInterface::new(spi_device, dc);

    let mut driver = WeActStudio290BlackWhiteDriver::new(spi_interface, busy, res, Delay);

    let mut display = Display290BlackWhite::new();
    display.set_rotation(DisplayRotation::Rotate90);

    let mut partial_display_bw =
        DisplayBlackWhite::<64, 128, { buffer_len::<Color>(64, 128) }>::new();
    partial_display_bw.set_rotation(DisplayRotation::Rotate90);

    let mut now = Instant::now();
    driver.init().unwrap();

    let style = MonoTextStyle::new(&PROFONT_24_POINT, Color::Black);

    let mut string_buf = String::<30>::new();
    write!(string_buf, "Time:\nInterval:").unwrap();
    let _ = Text::with_text_style(&string_buf, Point::new(8, 40), style, TextStyle::default())
        .draw(&mut display)
        .unwrap();
    string_buf.clear();

    driver.update_display(&display).unwrap();

    let text_style = TextStyleBuilder::new().alignment(Alignment::Right).build();
    loop {
        let elapsed = now.elapsed();
        now = Instant::now();
        let _ = write!(
            string_buf,
            "{:8.0}ms\n{}ms",
            now.as_millis(),
            elapsed.as_millis()
        );

        let _ = Text::with_text_style(&string_buf, Point::new(128, 32), style, text_style)
            .draw(&mut partial_display_bw);
        string_buf.clear();

        driver
            .quick_partial_update_display(&partial_display_bw, 56, 156)
            .unwrap();

        partial_display_bw.clear(Color::White);
    }
}
