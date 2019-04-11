// Clippy warns about `FromPrimitive`, which is not useless
#![allow(clippy::useless_attribute)]

use crate::{hal::blocking::i2c, ResolutionRegister, MCP9808};
use i2c_reg::Register;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

type Raw = <ResolutionRegister as Register>::Raw;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Resolution {
    Deg0_5C = 0b00,
    Deg0_25C = 0b01,
    Deg0_125C = 0b10,
    Deg0_0625C = 0b11,
}

impl From<Raw> for Resolution {
    fn from(raw: Raw) -> Self {
        Resolution::from_u8(raw[0] & 0b11).unwrap()
    }
}

impl Into<Raw> for Resolution {
    fn into(self) -> Raw {
        [self as u8]
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_resolution<Err>(&mut self) -> Result<Resolution, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(ResolutionRegister)
    }

    pub fn write_resolution<Err>(&mut self, resolution: Resolution) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(ResolutionRegister, resolution)
    }
}
