use crate::{hal::blocking::i2c, DeviceIdRevisionRegister, MCP9808};
use i2c_reg::Register;

const VALID_DEVICE_ID: u8 = 0x04;

type Raw = <DeviceIdRevisionRegister as Register>::Raw;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceId(pub u8);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceRevision(pub u8);

impl DeviceId {
    pub fn is_valid(self) -> bool {
        self.0 == VALID_DEVICE_ID
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_device_information<Err>(&mut self) -> Result<(DeviceId, DeviceRevision), Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(&self.device_information_register)
            .map(|raw: Raw| (DeviceId(raw[0]), DeviceRevision(raw[1])))
    }
}
