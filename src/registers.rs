use crate::hal::blocking::i2c;
use i2c_reg::*;
use i2c_reg_derive::*;

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
