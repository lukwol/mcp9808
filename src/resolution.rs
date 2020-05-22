//! Device Temperature Resolution

// Clippy warns about `FromPrimitive`, which is not useless
#![allow(clippy::useless_attribute)]

use crate::{hal::blocking::i2c, MCP9808};
use i2c_interface::generic_array::{typenum::consts::U1, GenericArray};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::registers::Register;

type Raw = GenericArray<u8, U1>;

/// Device temperature resolution
/// Used to adjust Temperature Sensor Accuracy and Temperature Conversion Time
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Resolution {
    /// +0.5°C (t_CONV = 30 ms typical)
    Deg0_5C = 0b00,

    /// +0.25°C (t_CONV = 65 ms typical)
    Deg0_25C = 0b01,

    /// +0.125°C (t_CONV = 130 ms typical)
    Deg0_125C = 0b10,

    /// +0.0625°C (power-up default, t_CONV = 250 ms typical)
    Deg0_0625C = 0b11,
}

impl From<Raw> for Resolution {
    fn from(raw: Raw) -> Self {
        Resolution::from_u8(raw[0] & 0b11).unwrap()
    }
}

impl Into<Raw> for Resolution {
    fn into(self) -> Raw {
        [self as u8].into()
    }
}

impl<I2C> MCP9808<I2C> {
    /// Read `Resolution` from `ResolutionRegister`
    pub fn read_resolution<Err>(&mut self) -> Result<Resolution, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(Register::ResolutionRegister)
            .map(Resolution::from)
    }

    /// Write `Resolution` to `ResolutionRegister`
    pub fn write_resolution<Err>(&mut self, resolution: Resolution) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(Register::ResolutionRegister, resolution.into())
    }
}
