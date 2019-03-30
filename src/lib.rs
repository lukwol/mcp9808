#![no_std]

use crate::address::SlaveAddress;
use embedded_hal as hal;
use i2c_reg::I2cInterface;
use registers::*;

mod registers;

mod temperature;

mod address;

pub struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
    ambient_temperature_register: AmbientTemperatureRegister,
}

impl<I2C> MCP9808<I2C> {
    pub fn new(i2c: I2C, address: SlaveAddress) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
            ambient_temperature_register: AmbientTemperatureRegister,
        }
    }
}
