// Clippy warns about `FromPrimitive`, which is not useless
#![allow(clippy::useless_attribute)]

use crate::{hal::blocking::i2c, ConfigurationRegister, MCP9808};
use i2c_reg::Register;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

type Raw = <ConfigurationRegister as Register>::Raw;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Hysteresis {
    Deg0C = 0b00,
    Deg1_5C = 0b01,
    Deg3_0C = 0b10,
    Deg6_0C = 0b11,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum ShutdownMode {
    ContinuousConversion = 0,
    Shutdown = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum CriticalTemperatureLock {
    Unlocked,
    Locked,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum TemperatureWindowLock {
    Unlocked = 0,
    Locked = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum InterruptClear {
    NotEffect = 0,
    Cleared = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputStatus {
    NotAsserted = 0,
    Asserted = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputControl {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputSelect {
    UpperLowerCritical = 0,
    CriticalOnly = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputPolarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputMode {
    Comparator = 0,
    Interrupt = 1,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Configuration {
    pub hysteresis: Hysteresis,
    pub shutdown_mode: ShutdownMode,
    pub critical_temperature_lock: CriticalTemperatureLock,
    pub temperature_window_lock: TemperatureWindowLock,
    pub interrupt_clear: InterruptClear,
    pub alert_output_status: AlertOutputStatus,
    pub alert_output_control: AlertOutputControl,
    pub alert_output_select: AlertOutputSelect,
    pub alert_output_polarity: AlertOutputPolarity,
    pub alert_output_mode: AlertOutputMode,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            hysteresis: Hysteresis::Deg0C,
            shutdown_mode: ShutdownMode::ContinuousConversion,
            critical_temperature_lock: CriticalTemperatureLock::Unlocked,
            temperature_window_lock: TemperatureWindowLock::Unlocked,
            interrupt_clear: InterruptClear::NotEffect,
            alert_output_status: AlertOutputStatus::NotAsserted,
            alert_output_control: AlertOutputControl::Disabled,
            alert_output_select: AlertOutputSelect::UpperLowerCritical,
            alert_output_polarity: AlertOutputPolarity::ActiveLow,
            alert_output_mode: AlertOutputMode::Comparator
        }
    }
}

impl From<Raw> for Configuration {
    fn from(raw: Raw) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);
        Configuration {
            hysteresis: Hysteresis::from_u8(msb & 0b11 << 1).unwrap(),
            shutdown_mode: ShutdownMode::from_u8(msb & 1).unwrap(),
            critical_temperature_lock: CriticalTemperatureLock::from_u8(lsb & 1 << 7).unwrap(),
            temperature_window_lock: TemperatureWindowLock::from_u8(lsb & 1 << 6).unwrap(),
            interrupt_clear: InterruptClear::from_u8(lsb & 1 << 5).unwrap(),
            alert_output_status: AlertOutputStatus::from_u8(lsb & 1 << 3).unwrap(),
            alert_output_control: AlertOutputControl::from_u8(lsb & 1 << 3).unwrap(),
            alert_output_select: AlertOutputSelect::from_u8(lsb & 1 << 2).unwrap(),
            alert_output_polarity: AlertOutputPolarity::from_u8(lsb & 1 << 1).unwrap(),
            alert_output_mode: AlertOutputMode::from_u8(lsb & 1).unwrap(),
        }
    }
}

impl Into<Raw> for Configuration {
    fn into(self) -> Raw {
        let (mut msb, mut lsb) = (0, 0);
        msb += self.hysteresis as u8 + self.shutdown_mode as u8;
        lsb += self.critical_temperature_lock as u8
            + self.temperature_window_lock as u8
            + self.interrupt_clear as u8
            + self.alert_output_status as u8
            + self.alert_output_control as u8
            + self.alert_output_select as u8
            + self.alert_output_polarity as u8
            + self.alert_output_mode as u8;
        [msb, lsb]
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_configuration<Err>(&mut self) -> Result<Configuration, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(ConfigurationRegister)
    }

    pub fn write_configuration<Err>(&mut self, configuration: Configuration) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(ConfigurationRegister, configuration)
    }
}
