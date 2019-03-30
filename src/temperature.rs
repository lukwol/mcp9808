use crate::hal::blocking::i2c;
use crate::AmbientTemperatureRegister;
use crate::MCP9808;

pub struct Temperature(u16);

impl From<[u8; 2]> for Temperature {
    fn from(_: [u8; 2]) -> Self {
        Temperature(0)
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_ambient_temperature<Err>(&mut self) -> Result<Temperature, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(AmbientTemperatureRegister)
    }
}
