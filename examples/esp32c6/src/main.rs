#![no_std]
#![no_main]

use core::fmt::Write;
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyle,
    text::{Text, TextStyle},
    Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    esp_riscv_rt::entry,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    peripherals::Peripherals,
    spi::{self, master::Spi},
    time::Rate,
};
use log::{error, info};

use heapless::String;
use profont::PROFONT_24_POINT;
use weact_studio_epd::{graphics::Display290BlackWhite, Color};
use weact_studio_epd::{graphics::DisplayRotation, WeActStudio290BlackWhiteDriver};

esp_bootloader_esp_idf::esp_app_desc!();

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    // esp32 gpio pins
    let sclk_pin = peripherals.GPIO19;
    let mosi_pin = peripherals.GPIO18;
    let cs_pin = peripherals.GPIO20;
    let dc_pin = peripherals.GPIO21;
    let rst_pin = peripherals.GPIO22;
    let busy_pin = peripherals.GPIO23;

    // Create pin drivers for bare pins
    // /*
    //     CS: OutputPin,
    //     BUSY: InputPin,
    //     DC: OutputPin,
    //     RST: OutputPin,
    // */
    let cs = Output::new(cs_pin, Level::High, OutputConfig::default());
    let busy = Input::new(busy_pin, InputConfig::default().with_pull(Pull::Up));
    let dc = Output::new(dc_pin, Level::Low, OutputConfig::default());
    let rst = Output::new(rst_pin, Level::High, OutputConfig::default());
    let delay = Delay::new();

    let spi_bus = {
        info!("Intializing SPI Bus...");

        let config = spi::master::Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(spi::Mode::_0);

        Spi::new(peripherals.SPI2, config)
            .inspect_err(|e| {
                error!("Error while creating SPI: {e}");
            })
            .unwrap()
            .with_sck(sclk_pin)
            .with_mosi(mosi_pin)
    };

    info!("Intializing SPI Device...");
    let spi_device = ExclusiveDevice::new(spi_bus, cs, delay.clone())
        .inspect_err(|e| error!("Error creating exclusive spi device: {e}"))
        .unwrap();

    let spi_interface = SPIInterface::new(spi_device, dc);

    // Setup EPD
    log::info!("Intializing EPD...");
    let mut driver = WeActStudio290BlackWhiteDriver::new(spi_interface, busy, rst, delay.clone());
    let mut display = Display290BlackWhite::new();
    display.set_rotation(DisplayRotation::Rotate90);
    driver.init().unwrap();

    let style = MonoTextStyle::new(&PROFONT_24_POINT, Color::Black);
    let _ = Text::with_text_style(
        "Hello World!",
        Point::new(8, 68),
        style,
        TextStyle::default(),
    )
    .draw(&mut display);

    driver.full_update(&display).unwrap();

    log::info!("Sleeping for 5s...");
    driver.sleep().unwrap();

    delay.delay_millis(5_000);

    let mut n: u8 = 0;
    loop {
        log::info!("Wake up!");
        driver.wake_up().unwrap();

        display.clear(Color::White);

        let mut string_buf = String::<30>::new();
        write!(string_buf, "Hello World {}!", n).unwrap();
        let _ = Text::with_text_style(&string_buf, Point::new(8, 68), style, TextStyle::default())
            .draw(&mut display)
            .unwrap();
        string_buf.clear();

        // TODO: try fast update?
        driver.full_update(&display).unwrap();

        n = n.wrapping_add(1); // Wrap from 0..255

        log::info!("Sleeping for 5s...");
        driver.sleep().unwrap();
        delay.delay_millis(5_000);
    }
}
