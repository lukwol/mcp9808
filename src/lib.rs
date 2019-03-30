#![no_std]
#![allow(dead_code)]

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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum SlaveAddress {
    Default,
    Alternative(bool, bool, bool),
}

impl From<SlaveAddress> for Address {
    fn from(slave_address: SlaveAddress) -> Self {
        let default_addr_ptr = 0b1_1000;
        match slave_address {
            SlaveAddress::Default => Address(default_addr_ptr),
            SlaveAddress::Alternative(a2, a1, a0) => {
                Address(default_addr_ptr | (a2 as u8) << 2 | (a1 as u8) << 1 | (a0 as u8))
            }
        }
    }
}

struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
    temperature_register: TemperatureRegister,
}

impl<I2C> MCP9808<I2C> {
    fn new(i2c: I2C, address: SlaveAddress) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
            temperature_register: TemperatureRegister,
        }
    }
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
