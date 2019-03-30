use crate::hal::blocking::i2c;
use crate::resolution::Resolution;
use crate::AmbientTemperatureRegister;
use crate::MCP9808;

const TEMPERATURE_SIGN_BIT: u8 = 0b1_0000;

#[derive(Debug, Clone, Copy)]
pub struct Milicelsius(pub i32);

impl Milicelsius {
    fn from_reading(reading: [u8; 2], resolution: Resolution) -> Milicelsius {
        let (msb, lsb) = (reading[0], reading[1]);

        let fraction =
            (i32::from((lsb & 0b1111) >> (0b11 - resolution as u8)) * resolution.precision()) / 10;

        let celsius = {
            let mut msb = msb & 0b1_1111;
            if msb & TEMPERATURE_SIGN_BIT == 0 {
                i32::from((msb << 4) + (lsb >> 4))
            } else {
                msb &= 0b1111;
                i32::from((msb << 4) + (lsb >> 4)) - 256
            }
        };

        Milicelsius(celsius * 1000 + fraction)
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
    pub fn read_ambient_temperature_milicelsius<Err>(
        &mut self,
        resolution: Resolution,
    ) -> Result<Milicelsius, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(AmbientTemperatureRegister)
            .map(|v| Milicelsius::from_reading(v, resolution))
    }

    pub fn read_ambient_temperature_celsius<Err>(
        &mut self,
        resolution: Resolution,
    ) -> Result<Celsius, Err>
        where
            I2C: i2c::WriteRead<Error = Err>,
    {
        self.read_ambient_temperature_milicelsius(resolution).map(|v| v.into())
    }
}
