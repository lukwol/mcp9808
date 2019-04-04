#![no_std]

use address::Address;
use embedded_hal as hal;
use i2c_reg::*;

// TODO: Change to private use
pub mod address;
pub mod configuration;
pub mod device_information;
pub mod manufacturer_id;
pub mod resolution;
pub mod temperature;
use crate::manufacturer_id::ManufacturerIdRegister;

pub struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
    manufacturer_id_register: ManufacturerIdRegister,
}

impl<I2C> MCP9808<I2C> {
    pub fn new(i2c: I2C, address: Address) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
            manufacturer_id_register: ManufacturerIdRegister,
        }
    }
}
