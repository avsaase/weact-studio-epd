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
use embassy_time::{Delay, Instant, Timer};
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    text::{Alignment, Text, TextStyle, TextStyleBuilder},
    Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use heapless::String;
use panic_probe as _;
use profont::PROFONT_24_POINT;
use weact_studio_epd::{
    color::Color,
    graphics::{buffer_len, Display, Display290Bw, DisplayRotation, DisplayTrait},
    Driver,
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

    let style = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::Off);

    let mut driver = Driver::new(spi_interface, busy, res, Delay);

    let mut display = Display290Bw::bw();
    display.set_rotation(DisplayRotation::Rotate90);

    let mut partial_display_bw: Display<64, 128, { buffer_len(64, 128) }> = Display::bw();
    partial_display_bw.set_rotation(DisplayRotation::Rotate90);

    let mut now = Instant::now();
    driver.init().unwrap();

    driver.clear_red_buffer().unwrap();
    driver.clear_bw_buffer().unwrap();

    let mut string_buf = String::<30>::new();
    let _ = write!(string_buf, "Time:\nElapsed:").unwrap();
    let _ = Text::with_text_style(&string_buf, Point::new(8, 40), style, TextStyle::default())
        .draw(&mut display)
        .unwrap();
    driver.write_bw_buffer(display.buffer()).unwrap();
    driver.full_refresh().unwrap();
    driver.write_red_buffer(display.buffer()).unwrap();

    Timer::after_millis(500).await;

    let text_style = TextStyleBuilder::new().alignment(Alignment::Right).build();
    loop {
        let elapsed = now.elapsed();
        now = Instant::now();
        let _ = write!(
            string_buf,
            "{:6.0}ms\n{}ms",
            now.as_millis(),
            elapsed.as_millis()
        );

        let _ = Text::with_text_style(&string_buf, Point::new(128, 32), style, text_style)
            .draw(&mut partial_display_bw);
        string_buf.clear();

        driver
            .write_partial_bw_buffer(partial_display_bw.buffer(), 56, 136, 64, 128)
            .unwrap();
        driver.quick_refresh().unwrap();
        driver
            .write_partial_red_buffer(partial_display_bw.buffer(), 56, 136, 64, 128)
            .unwrap();

        partial_display_bw.clear_buffer(Color::White);
    }
}
