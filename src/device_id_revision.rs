use crate::MCP9808;
use embedded_hal::blocking::i2c;

const VALID_DEVICE_ID: u8 = 0x04;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceId(pub u8);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceRevision(pub u8);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceIdRevision {
    pub device_id: DeviceId,
    pub device_revision: DeviceRevision,
}

impl DeviceId {
    pub fn is_valid(self) -> bool {
        self.0 == VALID_DEVICE_ID
    }
}

impl From<[u8; 2]> for DeviceIdRevision {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);
        DeviceIdRevision {
            device_id: DeviceId(msb),
            device_revision: DeviceRevision(lsb),
        }
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_device_information<Err>(&mut self) -> Result<DeviceIdRevision, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(&self.device_information_register)
    }
}
