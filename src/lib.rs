#![no_std]

use self::hal::blocking::i2c;
use address::SlaveAddress;
use embedded_hal as hal;
use i2c_reg::*;

// TODO: Change to public use
pub mod address;
pub mod resolution;
pub mod temperature;

i2c_ro_reg!(AmbientTemperatureRegister, addr: 0b1010, len: 2);
i2c_rw_reg!(ResolutionRegister, addr: 0b1000, len: 1);

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
