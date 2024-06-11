#![no_std]
#![no_main]

use core::fmt::Write;

use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::time::{mhz, Hertz};
use embassy_stm32::{spi, Config};

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
use weact_studio_epd::graphics::{Display290BlackWhite, DisplayBlackWhite};
use weact_studio_epd::{
    graphics::{buffer_len, DisplayRotation},
    Color, WeActStudio290BlackWhiteDriver,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL85,
            divp: None,
            divq: Some(PllQDiv::DIV8), // 42.5 Mhz for fdcan.
            divr: Some(PllRDiv::DIV2), // Main system clock at 170 MHz
        });
        config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
        config.rcc.sys = Sysclk::PLL1_R;
    }
    let p = embassy_stm32::init(config);

    let mut spi_config = spi::Config::default();
    spi_config.frequency = mhz(1);

    let busy = Input::new(p.PA1, Pull::Up);
    let cs = Output::new(p.PA2, Level::High, Speed::Low);
    let dc = Output::new(p.PA3, Level::High, Speed::Low);
    let res = Output::new(p.PA4, Level::High, Speed::Low);
    let scl = p.PA5;
    let sda = p.PA7;

    let spi_bus = spi::Spi::new_txonly(p.SPI1, scl, sda, p.DMA1_CH1, spi_config);
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

    driver.full_update(&display).unwrap();

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
            .fast_partial_update(&partial_display_bw, 56, 156)
            .unwrap();

        partial_display_bw.clear(Color::White);
    }
}
