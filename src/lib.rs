#![no_std]
#![allow(dead_code, unused_macros)]

use crate::hal::blocking::i2c;
use embedded_hal as hal;
use i2c_reg::*;

struct Temperature(u16);

impl From<[u8; 2]> for Temperature {
    fn from(_: [u8; 2]) -> Self {
        Temperature(0)
    }
}

impl Into<[u8; 2]> for Temperature {
    fn into(self) -> [u8; 2] {
        [0, 0]
    }
}

i2c_rw_reg!(TemperatureRegister, addr: 0b1010, len: 2);

struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
    temperature_register: TemperatureRegister,
}

impl<I2C> MCP9808<I2C> {
    fn read_temperature<Err>(&mut self) -> Result<Temperature, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(self.temperature_register)
    }

    fn write_temperature<Err>(&mut self, temperature: Temperature) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(self.temperature_register, temperature)
    }
}
