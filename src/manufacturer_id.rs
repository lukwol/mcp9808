//! Manufacturer ID

use crate::{hal::blocking::i2c, registers::Register, MCP9808};
use i2c_interface::generic_array::{typenum::consts::U2, GenericArray};

const VALID_MANUFACTURER_ID: u16 = 0x0054;

type Raw = GenericArray<u8, U2>;

/// Manufacturer ID is used to identify the manufacturer of the
/// device in order to perform manufacturer-specific
/// operation.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ManufacturerId(pub u16);

impl ManufacturerId {
    /// The Manufacturer ID for the MCP9808 is 0x0054 (hexadecimal)
    pub fn is_valid(self) -> bool {
        self.0 == VALID_MANUFACTURER_ID
    }
}

impl From<Raw> for ManufacturerId {
    fn from(raw: Raw) -> Self {
        ManufacturerId(u16::from_be_bytes(raw.into()))
    }
}

impl<I2C> MCP9808<I2C> {
    /// Read `ManufacturerId` from `ManufacturerIdRegister`
    pub fn read_manufacturer_id<Err>(&mut self) -> Result<ManufacturerId, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(Register::ManufacturerIdRegister)
            .map(ManufacturerId::from)
    }
}
