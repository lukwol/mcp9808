//! Maximum Accuracy Digital Temperature Sensor
//!
//! # Features
//!
//! * User-Programmable Temperature Alert Output
//! * Operating Voltage Range: 2.7V to 5.5V
//! * OperatingCurrent: 200μA (typical)
//! * ShutdownCurrent: 0.1μA (typical)
//! * 2-wire Interface: I2C™/SMBus Compatible
//! * Available Packages: 2x3DFN-8, MSOP-8
//!
//! ## Accuracy:
//! * ±0.25 (typical) from -40°C to +125°C
//! * ±0.5°C (maximum) from -20°C to 100°C
//! * ±1°C (maximum) from -40°C to +125°C
//!
//! ## User-Selectable Measurement Resolution:
//! * +0.5°C
//! * +0.25°C
//! * +0.125°C
//! * +0.0625°C
//!
//! ## User-Programmable Temperature Limits:
//! * Temperature Window Limit
//! * Critical Temperature Limit
//!
//! # Example
//!
//! ```
//! use embedded_hal::blocking::i2c;
//! use mcp9808::{
//!     temperature::{Celsius, TemperatureMeasurement},
//!     SlaveAddress, MCP9808,
//! };
//!
//! # struct MockI2c;
//! #
//! # impl i2c::WriteRead for MockI2c {
//! #     type Error = ();
//! #     fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
//! #         buffer[0] = 0b0000_11110;
//! #         buffer[1] = 0b0111_1111;
//! #         Ok(())
//! #     }
//! # }
//! #
//! # let i2c = MockI2c;
//! #
//! let mut mcp9808 = MCP9808::new(i2c, SlaveAddress::Default);
//! let measurement: TemperatureMeasurement<Celsius> = mcp9808.read_ambient_temperature().unwrap();
//! assert_eq!(Celsius(-24.063), measurement.temperature);
//! ```

#![no_std]

use crate::registers::*;
pub use address::SlaveAddress;
use embedded_hal as hal;
use i2c_reg::I2cInterface;

mod address;
pub mod configuration;
pub mod device_id_revision;
pub mod manufacturer_id;
mod registers;
pub mod resolution;
pub mod temperature;

pub mod interface;
mod new_registers;

/// Maximum Accuracy Digital Temperature Sensor
pub struct MCP9808<I2C> {
    /// I2C interface
    i2c_interface: I2cInterface<I2C>,
}

impl<I2C> MCP9808<I2C> {
    /// Creates new device with `I2C` and `SlaveAddress`
    pub fn new(i2c: I2C, address: SlaveAddress) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
        }
    }

    /// Device address
    pub fn address(&self) -> u8 {
        self.i2c_interface.address
    }

    /// Release the `I2C`
    pub fn release(self) -> I2C {
        self.i2c_interface.i2c
    }
}
