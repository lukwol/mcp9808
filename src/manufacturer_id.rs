use crate::{hal::blocking::i2c, ManufacturerIdRegister, MCP9808};
use i2c_reg::Register;

const VALID_MANUFACTURER_ID: u16 = 0x0054;

type Raw = <ManufacturerIdRegister as Register>::Raw;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ManufacturerId(pub u16);

impl ManufacturerId {
    pub fn is_valid(self) -> bool {
        self.0 == VALID_MANUFACTURER_ID
    }
}

impl From<Raw> for ManufacturerId {
    fn from(raw: Raw) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);
        ManufacturerId(((u16::from(msb)) << 8) + u16::from(lsb))
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_manufacturer_id<Err>(&mut self) -> Result<ManufacturerId, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(&self.manufacturer_id_register)
    }
}
