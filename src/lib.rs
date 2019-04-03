#![no_std]

use address::SlaveAddress;
use embedded_hal as hal;
use i2c_reg::I2cInterface;

// TODO: Change to private use
pub mod address;
pub mod device_information;
pub mod manufacturer_id;
pub mod resolution;
pub mod temperature;
pub mod configuration;

pub struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
}

impl<I2C> MCP9808<I2C> {
    pub fn new(i2c: I2C, address: SlaveAddress) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
        }
    }
}
