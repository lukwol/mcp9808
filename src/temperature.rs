use crate::{hal::blocking::i2c, AmbientTemperatureRegister, MCP9808};
use i2c_reg::Register;

const ALERT_CRITICAL_BIT: u8 = 1 << 7;
const ALERT_UPPER_BIT: u8 = 1 << 6;
const ALERT_LOWER_BIT: u8 = 1 << 5;
const TEMPERATURE_SIGN_BIT: u8 = 1 << 4;

type Raw = <AmbientTemperatureRegister as Register>::Raw;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Millicelsius(pub i32);

impl From<Raw> for Millicelsius {
    fn from(raw: Raw) -> Self {
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

impl Into<Raw> for Millicelsius {
    fn into(self) -> Raw {
        let value = self.0;
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

impl From<Raw> for Celsius {
    fn from(raw: Raw) -> Self {
        Millicelsius::from(raw).into()
    }
}

impl Into<Raw> for Celsius {
    fn into(self) -> Raw {
        Millicelsius::from(self).into()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct TemperatureMeasurement<Unit> {
    pub temperature: Unit,
    pub is_critical: bool,
    pub is_upper: bool,
    pub is_lower: bool,
}

impl<Unit> From<Raw> for TemperatureMeasurement<Unit>
where
    Unit: From<Raw>,
{
    fn from(raw: Raw) -> Self {
        let msb = raw[0];
        TemperatureMeasurement {
            temperature: Unit::from(raw),
            is_critical: !msb & ALERT_CRITICAL_BIT == 0,
            is_upper: !msb & ALERT_UPPER_BIT == 0,
            is_lower: !msb & ALERT_LOWER_BIT == 0,
        }
    }
}

macro_rules! impl_read_temperature_register {
    ($register: ident, $function_name: ident) => {
        impl_read_temperature_register!($register, $function_name, Unit);
    };
    ($register: ident, $function_name: ident, $type: ty) => {
        impl<I2C> MCP9808<I2C> {
            pub fn $function_name<Unit, Err>(&mut self) -> Result<$type, Err>
            where
                I2C: i2c::WriteRead<Error = Err>,
                Unit: From<Raw>,
            {
                self.i2c_interface.read_register(&self.$register)
            }
        }
    };
}

macro_rules! impl_write_temperature_register {
    ($register: ident, $function_name: ident) => {
        impl<I2C> MCP9808<I2C> {
            pub fn $function_name<Unit, Err>(&mut self, temperature: Unit) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
                Unit: Into<Raw>,
            {
                self.i2c_interface
                    .write_register(&self.$register, temperature)
            }
        }
    };
}

impl_read_temperature_register!(
    ambient_temperature_register,
    read_ambient_temperature,
    TemperatureMeasurement<Unit>
);

impl_read_temperature_register!(upper_temperature_register, read_upper_temperature);
impl_write_temperature_register!(upper_temperature_register, write_upper_temperature);

impl_read_temperature_register!(lower_temperature_register, read_lower_temperature);
impl_write_temperature_register!(lower_temperature_register, write_lower_temperature);

impl_read_temperature_register!(critical_temperature_register, read_critical_temperature);
impl_write_temperature_register!(critical_temperature_register, write_critical_temperature);
