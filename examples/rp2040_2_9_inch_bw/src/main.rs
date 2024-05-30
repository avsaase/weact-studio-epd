#![no_std]
#![no_main]

use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    spi::{Config, Spi},
};
use embassy_time::Delay;
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

    let spi_bus = Spi::new_blocking_txonly(p.SPI0, scl, sda, Config::default());
    let spi_device = ExclusiveDevice::new(spi_bus, cs, Delay);
    let spi_interface = SPIInterface::new(spi_device, dc);

    let mut driver = Driver::new(spi_interface, busy, res, Delay);
    let mut display = BwDisplay2_9::bw();

    driver.init().unwrap();
    driver.clear_bw_frame().unwrap();

    display.clear_buffer(Color::White);
    let _ = Rectangle::new(Point::new(10, 10), Size::new(40, 40))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(&mut display);
    driver.update_bw_frame(display.buffer()).unwrap();
    driver.display_frame().unwrap();
}
