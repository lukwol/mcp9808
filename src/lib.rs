#![no_std]

use crate::registers::*;
use address::Address;
use embedded_hal as hal;
use i2c_reg::I2cInterface;

pub mod address;
pub mod configuration;
pub mod device_id_revision;
pub mod manufacturer_id;
pub mod resolution;
pub mod temperature;

mod registers;

pub struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
}

impl<I2C> MCP9808<I2C> {
    pub fn new(i2c: I2C, address: Address) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
        }
    }

    pub fn address(&self) -> u8 {
        self.i2c_interface.address
    }

    pub fn release(self) -> I2C {
        self.i2c_interface.i2c
    }
}
