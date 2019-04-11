use crate::hal::blocking::i2c;
use i2c_reg::*;
use i2c_reg_derive::*;

/// Read/write Configuration register (CONFIG)
/// The MCP9808 has a 16-bit Configuration register (CONFIG) that allows the user
/// to set various functions for a robust temperature monitoring system.
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0001]
#[size = 2]
pub struct ConfigurationRegister;

/// Read/write Alert Temperature Upper Boundary Trip register (T_UPPER)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0010]
#[size = 2]
pub struct UpperTemperatureRegister;

/// Read/write Alert Temperature Lower Boundary Trip register (T_LOWER)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0011]
#[size = 2]
pub struct LowerTemperatureRegister;

/// Read/write Critical Temperature Trip register (T_CRTI)
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b0100]
#[size = 2]
pub struct CriticalTemperatureRegister;

/// Read only Ambient temperature register (T_A)
#[derive(Debug, Register, I2cReadRegister)]
#[address = 0b0101]
#[size = 2]
pub struct AmbientTemperatureRegister;

/// Read only Manufacturer ID register.
/// This register is used to identify the manufacturer of the
/// device in order to perform manufacturer-specific
/// operation.
#[derive(Debug, Register, I2cReadRegister)]
#[address = 0b0110]
#[size = 2]
pub struct ManufacturerIdRegister;

/// Read only Device ID/Revision register.
/// The upper byte of this register is used to specify the
/// device identification and the lower byte is used to
/// specify the device revision.
#[derive(Debug, Register, I2cReadRegister)]
#[address = 0b0111]
#[size = 2]
pub struct DeviceIdRevisionRegister;

/// Read/Write Temperature resolution register
#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b1000]
#[size = 1]
pub struct ResolutionRegister;
