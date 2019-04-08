use crate::hal::blocking::i2c;
use i2c_reg::*;
use i2c_reg_derive::*;

/// Configuration register (CONFIG)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0001]
#[len = 2]
pub struct ConfigurationRegister;

/// Alert Temperature Upper Boundary Trip register (T_UPPER)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0010]
#[len = 2]
pub struct UpperTemperatureRegister;

/// Alert Temperature Lower Boundary Trip register (T_LOWER)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0011]
#[len = 2]
pub struct LowerTemperatureRegister;

/// Critical Temperature Trip register (T_CRTI)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b0100]
#[len = 2]
pub struct CriticalTemperatureRegister;

/// Ambient temperature register (T_A)
#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0101]
#[len = 2]
pub struct AmbientTemperatureRegister;

/// Manufacturer ID register
#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0110]
#[len = 2]
pub struct ManufacturerIdRegister;

/// Device ID/Revision register
#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0111]
#[len = 2]
pub struct DeviceIdRevisionRegister;

/// Temperature resolution register
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b1000]
#[len = 1]
pub struct ResolutionRegister;
