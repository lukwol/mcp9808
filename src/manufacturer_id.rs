use crate::MCP9808;
use embedded_hal::blocking::i2c;
use i2c_reg::*;

const VALID_MANUFACTURER_ID: u16 = 0x0054;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ManufacturerId(pub u16);

impl ManufacturerId {
    pub fn is_valid(self) -> bool {
        self.0 == VALID_MANUFACTURER_ID
    }
}

impl From<[u8; 2]> for ManufacturerId {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);
        ManufacturerId(((u16::from(msb)) << 8) + u16::from(lsb))
    }
}

i2c_ro_reg!(ManufacturerIdRegister, addr: 0b0110, len: 2);

impl<I2C> MCP9808<I2C> {
    pub fn read_manufacturer_id<Err>(&mut self) -> Result<ManufacturerId, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(ManufacturerIdRegister)
    }
}
