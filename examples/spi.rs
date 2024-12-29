use esp_idf_svc::{
    hal::{
        gpio,
        prelude::*,
        spi::{
            self,
            config::{BitOrder, DriverConfig, MODE_1},
            SpiDeviceDriver, SpiDriver, SpiSharedDeviceDriver,
            SpiSoftCsDeviceDriver,
        },
    },
    log::EspLogger,
    sys,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    sys::link_patches();
    EspLogger::initialize_default();
    unsafe {
        sys::nvs_flash_init();
    }

    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;
    let spi = peripherals.spi2;

    let sclk = pins.gpio6;
    let sdo = pins.gpio2;
    let sdi = pins.gpio7;
    let cs = pins.gpio10;
    let config = DriverConfig::new();
    let driver = SpiDriver::new(spi, sclk, sdo, Some(sdi), &config).unwrap();

    const SPI_CONFIG: spi::config::Config = spi::config::Config {
        baudrate: Hertz(5_000_000),
        bit_order: BitOrder::MsbFirst,
        data_mode: MODE_1,
        write_only: false,
        duplex: esp_idf_svc::hal::spi::config::Duplex::Full,
        cs_active_high: false,
        cs_pre_delay_us: None,
        cs_post_delay_us: None,
        input_delay_ns: 0,
        polling: true,
        allow_pre_post_delays: false,
        queue_size: 1,
    };

    const CONFIG_REGISTER_WRITE: u8 = 0x80;
    const CONFIG_REGISTER_READ: u8 = 0x00;
    const FAULT_REGISTER_READ: u8 = 0x07;

    const RUN_AUTO_FAULT_DETECTION: u8 = 0b10010101;

    let shared = SpiSharedDeviceDriver::new(driver, &SPI_CONFIG).unwrap();
    let mut spi_driver =
        SpiSoftCsDeviceDriver::new(shared, cs, gpio::Level::Low).unwrap();

    spi_driver
        .write(&[CONFIG_REGISTER_WRITE, RUN_AUTO_FAULT_DETECTION])
        .unwrap();
    println!("Wrote auto fault detection instruction to config register");
    println!("Config should be {RUN_AUTO_FAULT_DETECTION} after this");

    std::thread::sleep(std::time::Duration::from_micros(200));

    let mut config_read = [CONFIG_REGISTER_READ, 0];
    spi_driver.read(&mut config_read).unwrap();
    println!("Config register is now {}", config_read[1]);

    let mut fault_read = [FAULT_REGISTER_READ, 0];
    spi_driver.read(&mut fault_read).unwrap();
    println!("Fault register is now {}", fault_read[1]);

    Ok(())
}
