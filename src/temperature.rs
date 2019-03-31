use crate::hal::blocking::i2c;
use crate::MCP9808;
use crate::{
    AmbientTemperatureRegister, CriticalTemperatureRegister, LowerTemperatureRegister,
    UpperTemperatureRegister,
};

const TEMPERATURE_SIGN_BIT: u8 = 0b1_0000;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Millicelsius(pub i32);

impl From<[u8; 2]> for Millicelsius {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);

        let fraction = (0..4).fold(0, |acc, x| acc + i32::from(lsb & 1 << x) * 625) / 10;
        let mut celsius = (0..4).fold(0, |acc, x| {
            acc + (i32::from(msb & 1 << x) << 4) + i32::from(lsb >> 4 & 1 << x)
        });
        if msb & TEMPERATURE_SIGN_BIT != 0 {
            celsius -= 1 << 8
        }

        Millicelsius(celsius * 1_000 + fraction)
    }
}

impl From<Millicelsius> for [u8; 2] {
    fn from(millicelsius: Millicelsius) -> Self {
        let value = millicelsius.0;
        let write_value = (value + 256_000) % 256_000;

        let integer = write_value / 1_000;
        let fraction = write_value % 1_000;

        let mut msb = (integer >> 4 & 0b1111) as u8;
        if value < 0 {
            msb |= 0b1_0000;
        }
        let lsb = (((integer & 0b1111) << 4) + fraction / 62) as u8;
        [msb, lsb]
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Celsius(pub f32);

impl From<Millicelsius> for Celsius {
    fn from(millicelsius: Millicelsius) -> Self {
        Celsius(millicelsius.0 as f32 / 1_000.0)
    }
}

impl From<Celsius> for Millicelsius {
    fn from(celsius: Celsius) -> Self {
        Millicelsius((celsius.0 * 1_000.0) as i32)
    }
}

impl From<[u8; 2]> for Celsius {
    fn from(raw: [u8; 2]) -> Self {
        Millicelsius::from(raw).into()
    }
}

impl From<Celsius> for [u8; 2] {
    fn from(celsius: Celsius) -> Self {
        Millicelsius::from(celsius).into()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Temperature<Unit>(pub Unit);

impl<Unit> From<[u8; 2]> for Temperature<Unit>
where
    Unit: From<[u8; 2]>,
{
    fn from(raw: [u8; 2]) -> Self {
        Temperature(Unit::from(raw))
    }
}

impl<Unit> From<Temperature<Unit>> for [u8; 2]
where
    Unit: Into<[u8; 2]>,
{
    fn from(temperature: Temperature<Unit>) -> Self {
        temperature.0.into()
    }
}

macro_rules! read_temperature_register {
    ($register: expr, $function_name: ident) => {
        impl<I2C> MCP9808<I2C> {
            pub fn $function_name<Unit, Err>(&mut self) -> Result<Temperature<Unit>, Err>
            where
                I2C: i2c::WriteRead<Error = Err>,
                Unit: From<[u8; 2]>,
            {
                self.i2c_interface.read_register($register)
            }
        }
    };
}

macro_rules! write_temperature_register {
    ($register: expr, $function_name: ident) => {
        impl<I2C> MCP9808<I2C> {
            pub fn $function_name<Unit, Err>(
                &mut self,
                temperature: Temperature<Unit>,
            ) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
                Unit: Into<[u8; 2]>,
            {
                self.i2c_interface.write_register($register, temperature)
            }
        }
    };
}

read_temperature_register!(AmbientTemperatureRegister, read_ambient_temperature);

read_temperature_register!(UpperTemperatureRegister, read_upper_temperature);
write_temperature_register!(UpperTemperatureRegister, write_upper_temperature);

read_temperature_register!(LowerTemperatureRegister, read_lower_temperature);
write_temperature_register!(LowerTemperatureRegister, write_lower_temperature);

read_temperature_register!(CriticalTemperatureRegister, read_critical_temperature);
write_temperature_register!(CriticalTemperatureRegister, write_critical_temperature);
