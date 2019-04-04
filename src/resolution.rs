use crate::hal::blocking::i2c;
use crate::MCP9808;
use i2c_reg::*;
use i2c_reg_derive::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum Resolution {
    Deg0_5C = 0b00,
    Deg0_25C = 0b01,
    Deg0_125C = 0b10,
    Deg0_0625C = 0b11,
}

impl From<Resolution> for [u8; 1] {
    fn from(res: Resolution) -> Self {
        [res as u8]
    }
}

#[derive(Debug, Register, I2cReadRegister, I2cWriteRegister)]
#[addr = 0b1000]
#[len = 1]
struct ResolutionRegister;

impl<I2C> MCP9808<I2C> {
    pub fn read_resolution<Err>(&mut self) -> Result<Resolution, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(&ResolutionRegister)
            .map(|raw: [u8; 1]| Resolution::from_u8(raw[0] & 0b11).unwrap())
    }

    pub fn write_resolution<Err>(&mut self, resolution: Resolution) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(&ResolutionRegister, resolution)
    }
}
