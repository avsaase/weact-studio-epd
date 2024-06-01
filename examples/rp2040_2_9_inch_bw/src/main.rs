#![no_std]
#![no_main]

use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    spi::{Config, Spi},
};
use embassy_time::{Delay, Timer};
use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_probe as _;
use weact_studio_epd::{
    color::Color,
    graphics::{buffer_len, BwDisplay2_9, DisplayTrait, VarDisplay},
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

    let spi_bus = Spi::new_blocking_txonly(p.SPI0, scl, sda, Config::default());
    let spi_device = ExclusiveDevice::new(spi_bus, cs, Delay);
    let spi_interface = SPIInterface::new(spi_device, dc);

    let mut driver = Driver::new(spi_interface, busy, res, Delay);
    let mut display = BwDisplay2_9::bw();

    driver.init().unwrap();
    // driver.clear_bw_frame().unwrap();
    // driver.display_frame().unwrap();

    display.clear_buffer(Color::White);

    let _ = Rectangle::new(Point::new(16, 16), Size::new(40, 40))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(&mut display);
    driver.update_bw_frame(display.buffer()).unwrap();
    driver.display_frame().unwrap();

    Timer::after_millis(500).await;

    // let mut buffer = [Color::White.get_byte_value(); buffer_len(40, 40)];
    // let mut partial_display = VarDisplay::bw(40, 40, &mut buffer);

    // driver.clear_bw_frame().unwrap();
    // partial_display.clear_buffer(Color::White);
    let _ = Rectangle::new(Point::new(56, 56), Size::new(40, 40))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(&mut display);
    // driver
    //     .update_partial_bw_frame(partial_display.buffer(), 32, 32, 40, 40)
    //     .unwrap();
    driver.update_bw_frame(display.buffer()).unwrap();
    driver.display_partial_frame().unwrap();
}
