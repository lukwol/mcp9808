use crate::MCP9808;
use embedded_hal::blocking::i2c;
use i2c_reg::*;
use i2c_reg_derive::*;

const VALID_DEVICE_ID: u8 = 0x04;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceId(pub u8);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceRevision(pub u8);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceInformation {
    pub device_id: DeviceId,
    pub device_revision: DeviceRevision,
}

impl DeviceId {
    pub fn is_valid(self) -> bool {
        self.0 == VALID_DEVICE_ID
    }
}

impl From<[u8; 2]> for DeviceInformation {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);
        DeviceInformation {
            device_id: DeviceId(msb),
            device_revision: DeviceRevision(lsb),
        }
    }
}

#[derive(Debug, Register, I2cReadRegister)]
#[addr = 0b0111]
#[len = 2]
struct DeviceInformationRegister;

impl<I2C> MCP9808<I2C> {
    pub fn read_device_information<Err>(&mut self) -> Result<DeviceInformation, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(&DeviceInformationRegister)
    }
}
