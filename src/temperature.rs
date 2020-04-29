//! Ambient, Critical, Upper, Lower Temperature

use crate::{hal::blocking::i2c, registers::Register, MCP9808};
use generic_array::{typenum::consts::U2, GenericArray};

const ALERT_CRITICAL_BIT: u8 = 1 << 7;
const ALERT_UPPER_BIT: u8 = 1 << 6;
const ALERT_LOWER_BIT: u8 = 1 << 5;
const TEMPERATURE_SIGN_BIT: u8 = 1 << 4;

type Raw = GenericArray<u8, U2>;

/// Temperature value in Millicelsius
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
        [msb, lsb].into()
    }
}

/// Temperature value in Celsius
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

/// Describes Temperature Unit value
pub trait TemperatureUnit {}

impl TemperatureUnit for Millicelsius {}
impl TemperatureUnit for Celsius {}

/// Ambient temperature measurement with additional information
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct TemperatureMeasurement<Unit>
where
    Unit: TemperatureUnit,
{
    /// Temperature in Celsius or Millicelsius
    pub temperature: Unit,

    /// T_A is greater or equal than T_CRIT
    pub is_critical: bool,

    /// T_A is greater than T_UPPER
    pub is_upper: bool,

    /// T_A is lower than T_LOWER
    pub is_lower: bool,
}

impl<Unit> From<Raw> for TemperatureMeasurement<Unit>
where
    Unit: From<Raw> + TemperatureUnit,
{
    fn from(raw: Raw) -> Self {
        let msb = raw[0];
        TemperatureMeasurement {
            temperature: raw.into(),
            is_critical: !msb & ALERT_CRITICAL_BIT == 0,
            is_upper: !msb & ALERT_UPPER_BIT == 0,
            is_lower: !msb & ALERT_LOWER_BIT == 0,
        }
    }
}

impl<I2C> MCP9808<I2C> {
    /// Read `TemperatureMeasurement` with Temperature `Unit` from `AmbientTemperatureRegister`
    pub fn read_ambient_temperature<Unit, Err>(
        &mut self,
    ) -> Result<TemperatureMeasurement<Unit>, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Unit: From<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .read_register(Register::AmbientTemperatureRegister)
            .map(TemperatureMeasurement::from)
    }

    /// Read Temperature `Unit` from `UpperTemperatureRegister`
    pub fn read_upper_temperature<Unit, Err>(&mut self) -> Result<Unit, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Unit: From<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .read_register(Register::UpperTemperatureRegister)
            .map(Unit::from)
    }

    /// Write Temperature `Unit` to `UpperTemperatureRegister`
    pub fn write_upper_temperature<Unit, Err>(&mut self, temperature: Unit) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        Unit: Into<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .write_register(Register::UpperTemperatureRegister, temperature.into())
    }

    /// Read Temperature `Unit` from `LowerTemperatureRegister`
    pub fn read_lower_temperature<Unit, Err>(&mut self) -> Result<Unit, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Unit: From<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .read_register(Register::LowerTemperatureRegister)
            .map(Unit::from)
    }

    /// Write Temperature `Unit` to `LowerTemperatureRegister`
    pub fn write_lower_temperature<Unit, Err>(&mut self, temperature: Unit) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        Unit: Into<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .write_register(Register::LowerTemperatureRegister, temperature.into())
    }

    /// Read Temperature `Unit` from `CriticalTemperatureRegister`
    pub fn read_critical_temperature<Unit, Err>(&mut self) -> Result<Unit, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Unit: From<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .read_register(Register::CriticalTemperatureRegister)
            .map(Unit::from)
    }

    /// Write Temperature `Unit` to `CriticalTemperatureRegister`
    pub fn write_critical_temperature<Unit, Err>(&mut self, temperature: Unit) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        Unit: Into<Raw> + TemperatureUnit,
    {
        self.i2c_interface
            .write_register(Register::CriticalTemperatureRegister, temperature.into())
    }
}
