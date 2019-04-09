use crate::hal::blocking::i2c;
use i2c_reg::*;
use i2c_reg_derive::*;

/// Configuration register (CONFIG)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0001]
#[size = 2]
pub struct ConfigurationRegister;

/// Alert Temperature Upper Boundary Trip register (T_UPPER)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0010]
#[size = 2]
pub struct UpperTemperatureRegister;

/// Alert Temperature Lower Boundary Trip register (T_LOWER)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0011]
#[size = 2]
pub struct LowerTemperatureRegister;

/// Critical Temperature Trip register (T_CRTI)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0100]
#[size = 2]
pub struct CriticalTemperatureRegister;

/// Ambient temperature register (T_A)
#[derive(Debug, Register, I2cReadRegister)]
#[address = 0b0101]
#[size = 2]
pub struct AmbientTemperatureRegister;

/// Manufacturer ID register
#[derive(Debug, Register, I2cReadRegister)]
#[address = 0b0110]
#[size = 2]
pub struct ManufacturerIdRegister;

/// Device ID/Revision register
#[derive(Debug, Register, I2cReadRegister)]
#[address = 0b0111]
#[size = 2]
pub struct DeviceIdRevisionRegister;

/// Temperature resolution register
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b1000]
#[size = 1]
pub struct ResolutionRegister;
