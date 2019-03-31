use crate::hal::blocking::i2c;
use crate::AmbientTemperatureRegister;
use crate::MCP9808;

const TEMPERATURE_SIGN_BIT: u8 = 0b1_0000;

#[derive(Debug, Clone, Copy)]
pub struct Milicelsius(pub i32);

impl From<[u8; 2]> for Milicelsius {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);

        let fraction = (0..4).fold(0, |acc, x| acc + i32::from(lsb & 1 << x) * 625) / 10;
        let mut celsius = (0..4).fold(0, |acc, x| {
            acc + (i32::from(msb & 1 << x) << 4) + i32::from(lsb >> 4 & 1 << x)
        });
        if msb & TEMPERATURE_SIGN_BIT != 0 {
            celsius -= 1 << 8
        }

        Milicelsius(celsius * 1000 + fraction)
    }
}

impl From<Milicelsius> for [u8; 2] {
    fn from(milicelsius: Milicelsius) -> Self {
        // TODO: Implement
        [0, 0]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Celsius(pub f32);

impl From<Milicelsius> for Celsius {
    fn from(milicelsius: Milicelsius) -> Self {
        Celsius(milicelsius.0 as f32 / 1000.0)
    }
}

impl From<Celsius> for Milicelsius {
    fn from(celsius: Celsius) -> Self {
        Milicelsius((celsius.0 * 1000.0) as i32)
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_ambient_temperature_milicelsius<Err>(&mut self) -> Result<Milicelsius, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(AmbientTemperatureRegister)
    }

    pub fn read_ambient_temperature_celsius<Err>(&mut self) -> Result<Celsius, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.read_ambient_temperature_milicelsius()
            .map(|v| v.into())
    }
}
