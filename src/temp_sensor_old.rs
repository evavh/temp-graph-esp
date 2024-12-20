use std::thread;
use std::time::Duration;

use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_hal::gpio;
use max31865::{FilterMode, Max31865, SensorType};

pub struct TempSensor {
    device: Max31865<spi::SpiDeviceDriver<'static, spi::SpiDriver<'static>>, gpio::PinDriver<'static, gpio::Gpio17, gpio::Output>, gpio::PinDriver<'static, gpio::Gpio18, gpio::Input>>
}

pub struct Pins {
    clock: gpio::Gpio14,
    data_out: gpio::Gpio12,
    data_in: gpio::Gpio13,
    chip_select: gpio::Gpio4,
    fake_chip_select: gpio::Gpio17,
    data_ready: gpio::Gpio18,
}

impl Pins {
    pub fn from_pin_peripheral(pins: gpio::Pins) -> Self {
        Pins {
            clock: pins.gpio14,
            data_out: pins.gpio12,
            data_in: pins.gpio13,
            chip_select: pins.gpio4,
            fake_chip_select: pins.gpio17,
            data_ready: pins.gpio18,
        }
    }
}

impl TempSensor {
    pub fn new(spi2: spi::SPI2, pins: Pins) -> Self {
        let config = spi::SpiConfig::default().baudrate(10.kHz().into());
        let spi_driver = spi::SpiDeviceDriver::new_single(
            spi2, // HSPI
            pins.clock,
            pins.data_out,
            Some(pins.data_in),
            spi::Dma::Disabled,
            Option::None::<gpio::Gpio4>,
            &config,
        )
        .unwrap();

        let fake_chip_select = gpio::PinDriver::output(pins.fake_chip_select).unwrap();
        let mut chip_select = gpio::PinDriver::output(pins.chip_select).unwrap();
        chip_select.set_low().unwrap();

        let data_ready = gpio::PinDriver::input(pins.data_ready).unwrap();
        let mut device = Max31865::new(spi_driver, fake_chip_select, data_ready).unwrap();
        device
            .configure(
                true,  // vbias
                true,  // conversion_mode
                false, // one_shot
                SensorType::ThreeWire,
                FilterMode::Filter50Hz,
            )
            .unwrap();
        Self { device }
    }

    pub fn read(&mut self) -> Result<f32, std::io::Error> {
        const TRIES: u16 = 20;
        let mut i = 0;
        while !self.device.is_ready().unwrap() {
            thread::sleep(Duration::from_millis(100));

            i += 1;
            if i > TRIES {
                return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "device was not rdy in time"));
            }
        }

        let temp_centideg = self.device.read_default_conversion().unwrap();
        return Ok(temp_centideg as f32 / 100.0)
    }
}


