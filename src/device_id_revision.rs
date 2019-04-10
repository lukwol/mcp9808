use crate::{hal::blocking::i2c, DeviceIdRevisionRegister, MCP9808};
use i2c_reg::Register;

const VALID_DEVICE_ID: u8 = 0x04;

type Raw = <DeviceIdRevisionRegister as Register>::Raw;

/// Bit 15 to bit 8 are used for `DeviceId`.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceId(pub u8);

/// Bit 7 to bit 0 are used for `DeviceRevision`.
/// The revision begins with 0x00 (hex) for the first release,
/// with the number being incremented as revised versions are released.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct DeviceRevision(pub u8);

impl DeviceId {
    /// The Device ID for the MCP9808 is 0x04 (hex)
    pub fn is_valid(self) -> bool {
        self.0 == VALID_DEVICE_ID
    }
}

impl<I2C> MCP9808<I2C> {
    /// Read `DeviceId` and `DeviceRevision` from `DeviceIdRevisionRegister`
    pub fn read_device_information<Err>(&mut self) -> Result<(DeviceId, DeviceRevision), Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(DeviceIdRevisionRegister)
            .map(|raw: Raw| (DeviceId(raw[0]), DeviceRevision(raw[1])))
    }
}
