#![no_std]

use crate::hal::blocking::i2c;
use address::Address;
use embedded_hal as hal;
use i2c_reg::*;
use i2c_reg_derive::*;

// TODO: Change to private use
pub mod address;
pub mod configuration;
pub mod device_id_revision;
pub mod manufacturer_id;
pub mod resolution;
pub mod temperature;

#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0001]
#[len = 2]
pub struct ConfigurationRegister;

#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0010]
#[len = 2]
pub struct UpperTemperatureRegister;

#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0011]
#[len = 2]
pub struct LowerTemperatureRegister;

#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0100]
#[len = 2]
pub struct CriticalTemperatureRegister;

#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0101]
#[len = 2]
pub struct AmbientTemperatureRegister;

#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0110]
#[len = 2]
pub struct ManufacturerIdRegister;

#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0111]
#[len = 2]
pub struct DeviceIdRevisionRegister;

#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b1000]
#[len = 1]
pub struct ResolutionRegister;

pub struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
    configuration_register: ConfigurationRegister,
    upper_temperature_register: UpperTemperatureRegister,
    lower_temperature_register: LowerTemperatureRegister,
    critical_temperature_register: CriticalTemperatureRegister,
    ambient_temperature_register: AmbientTemperatureRegister,
    manufacturer_id_register: ManufacturerIdRegister,
    device_information_register: DeviceIdRevisionRegister,
    resolution_register: ResolutionRegister,
}

impl<I2C> MCP9808<I2C> {
    pub fn new(i2c: I2C, address: Address) -> Self {
        MCP9808 {
            i2c_interface: I2cInterface {
                i2c,
                address: address.into(),
            },
            manufacturer_id_register: ManufacturerIdRegister,
            configuration_register: ConfigurationRegister,
            upper_temperature_register: UpperTemperatureRegister,
            lower_temperature_register: LowerTemperatureRegister,
            critical_temperature_register: CriticalTemperatureRegister,
            ambient_temperature_register: AmbientTemperatureRegister,
            device_information_register: DeviceIdRevisionRegister,
            resolution_register: ResolutionRegister,
        }
    }
}
