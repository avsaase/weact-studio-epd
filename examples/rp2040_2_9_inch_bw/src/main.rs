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
    geometry::{Point, Size},
    mono_font::{iso_8859_13::FONT_10X20, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Text, TextStyle},
    Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use heapless::String;
use panic_probe as _;
use profont::PROFONT_24_POINT;
use weact_studio_epd::{
    color::Color,
    command,
    graphics::{BwDisplay2_9, DisplayTrait},
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

    let mut driver = Driver::new(spi_interface, busy, res, Delay);

    let mut display = BwDisplay2_9::bw();
    display.set_rotation(weact_studio_epd::graphics::DisplayRotation::Rotate90);
    let mut display2 = BwDisplay2_9::bw();
    display2.set_rotation(weact_studio_epd::graphics::DisplayRotation::Rotate90);
    display2.is_inverted = true;

    driver.init().unwrap();
    // driver.set_partial_lut().unwrap();

    // Start with empty frame
    driver.clear_bw_frame().unwrap();
    driver.clear_red_frame().unwrap();
    driver.display_frame().unwrap();
    // display.clear_buffer(Color::White);

    // let mut x = 8;
    // let mut y = 8;

    let style = MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::Off);

    let mut string_buf = String::<15>::new();

    Timer::after_millis(500).await;

    loop {
        // driver.use_partial_frame(x, y, 16, 16).unwrap();
        // driver
        //     .command_with_data(
        //         command::WRITE_BW_DATA,
        //         &[
        //             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //             0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //         ],
        //     )
        //     .unwrap();
        // driver.display_partial_frame().unwrap();
        // driver.clear_bw_frame().unwrap();

        // driver.clear_bw_frame().unwrap();
        // let _ = Rectangle::new(
        //     Point { x, y },
        //     Size {
        //         width: 16,
        //         height: 16,
        //     },
        // )
        // .into_styled(
        //     PrimitiveStyleBuilder::new()
        //         .stroke_width(3)
        //         .fill_color(BinaryColor::Off)
        //         .build(),
        // )
        // .draw(&mut display);

        // driver.update_bw_frame(display.buffer()).unwrap();
        // driver.display_partial_frame().unwrap();

        // x += 32;
        // if x > 120 {
        //     x = 8;
        //     y += 32;
        //     if y > 288 {
        //         y = 8;
        //         driver.clear_bw_frame().unwrap();
        //         driver.display_frame().unwrap();
        //         display.clear_buffer(Color::White);
        //     }
        // }

        let now = Instant::now().as_millis();
        let _ = write!(string_buf, "Time: {now:6.0}ms");

        let _ = Text::with_text_style(&string_buf, Point::new(30, 40), style, TextStyle::default())
            .draw(&mut display);
        let _ = Text::with_text_style(&string_buf, Point::new(30, 40), style, TextStyle::default())
            .draw(&mut display2);

        string_buf.clear();

        driver.update_bw_frame(display.buffer()).unwrap();
        driver.update_red_frame(display2.buffer()).unwrap();
        display.clear_buffer(Color::White);
        display2.clear_buffer(Color::White);

        driver.display_partial_frame().unwrap();
    }
}
