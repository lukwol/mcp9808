use crate::hal::blocking::i2c;
use crate::MCP9808;

use i2c_reg::*;

const ALERT_CRITICAL_BIT: u8 = 1 << 7;
const ALERT_UPPER_BIT: u8 = 1 << 6;
const ALERT_LOWER_BIT: u8 = 1 << 5;
const TEMPERATURE_SIGN_BIT: u8 = 1 << 4;

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
            msb |= TEMPERATURE_SIGN_BIT;
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
pub struct Temperature<Unit> {
    pub degrees: Unit,
    pub is_critical: bool,
    pub is_upper: bool,
    pub is_lower: bool,
}

impl<Unit> From<[u8; 2]> for Temperature<Unit>
where
    Unit: From<[u8; 2]>,
{
    fn from(raw: [u8; 2]) -> Self {
        let msb = raw[0];
        Temperature {
            degrees: Unit::from(raw),
            is_critical: msb & ALERT_CRITICAL_BIT == 1,
            is_upper: msb & ALERT_UPPER_BIT == 1,
            is_lower: msb & ALERT_LOWER_BIT == 1,
        }
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
            pub fn $function_name<Unit, Err>(&mut self, temperature: Unit) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
                Unit: Into<[u8; 2]>,
            {
                self.i2c_interface.write_register($register, temperature)
            }
        }
    };
}

i2c_ro_reg!(AmbientTemperatureRegister, addr: 0b0101, len: 2);
read_temperature_register!(AmbientTemperatureRegister, read_ambient_temperature);

i2c_rw_reg!(UpperTemperatureRegister, addr: 0b0010, len: 2);
read_temperature_register!(UpperTemperatureRegister, read_upper_temperature);
write_temperature_register!(UpperTemperatureRegister, write_upper_temperature);

i2c_rw_reg!(LowerTemperatureRegister, addr: 0b0011, len: 2);
read_temperature_register!(LowerTemperatureRegister, read_lower_temperature);
write_temperature_register!(LowerTemperatureRegister, write_lower_temperature);

i2c_rw_reg!(CriticalTemperatureRegister, addr: 0b0100, len: 2);
read_temperature_register!(CriticalTemperatureRegister, read_critical_temperature);
write_temperature_register!(CriticalTemperatureRegister, write_critical_temperature);
