// Clippy warns about `FromPrimitive`, which is not useless
#![allow(clippy::useless_attribute)]

use crate::{hal::blocking::i2c, ConfigurationRegister, MCP9808};
use i2c_reg::Register;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

type Raw = <ConfigurationRegister as Register>::Raw;

/// T_HYST: T_UPPER and T_LOWER Limit Hysteresis bits
/// This bit can not be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Hysteresis {
    /// 0°C(power-up default)
    Deg0C = 0b00,
    /// +1.5°C
    Deg1_5C = 0b01,
    /// +3.0°C
    Deg3_0C = 0b10,
    /// +6.0°C
    Deg6_0C = 0b11,
}

/// Shutdown Mode bit
/// In shutdown, all power-consuming activities are disabled,
/// though all registers can be written to or read.
/// This bit cannot be set to ‘1’ when either of the Lock bits is set (bit 6 and bit 7).
/// However, it can be cleared to ‘0’ for continuous conversion while locked.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum ShutdownMode {
    /// Continuous conversion (power-up default)
    ContinuousConversion = 0,
    /// Shutdown (Low-Power mode)
    Shutdown = 1,
}

/// T_CRIT Lock bit
/// When enabled, this bit remains set to ‘1’ or locked until cleared by an internal Reset.
/// This bit can be programmed in Shutdown mode.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum CriticalTemperatureLock {
    /// Unlocked. T_CRIT register can be written (power-up default)
    Unlocked,
    /// Locked. TCRIT register can not be written
    Locked,
}

/// T_UPPER and T_LOWER Window Lock bit
/// When enabled, this bit remains set to ‘1’ or locked until cleared by an internal Reset.
/// This bit can be programmed in Shutdown mode.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum TemperatureWindowLock {
    /// Unlocked; T_UPPER and T_LOWER registers can be written (power-up default)
    Unlocked = 0,
    /// Locked; T_UPPER and T_LOWER registers can not be written
    Locked = 1,
}

/// Interrupt Clear bit
/// This bit can not be set to ‘1’ in Shutdown mode,
/// but it can be cleared after the device enters Shutdown mode.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum InterruptClear {
    /// No effect (power-up default)
    NotEffect = 0,
    /// Clear interrupt output; when read, this bit returns to ‘0’
    Cleared = 1,
}

/// Alert Output Status bit
/// This bit can not be set to ‘1’ or cleared to ‘0’ in Shutdown mode.
/// However, if the Alert output is configured as Interrupt mode,
/// and if the host controller clears to ‘0’,
/// the interrupt, using bit 5 while the device is in Shutdown mode,
/// then this bit will also be cleared ‘0’.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputStatus {
    /// Alert output is not asserted by the device (power-up default)
    NotAsserted = 0,
    /// Alert output is asserted as a comparator/Interrupt or critical temperature output
    Asserted = 1,
}

/// Alert Output Control bit
/// This bit can not be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputControl {
    /// Disabled (power-up default)
    Disabled = 0,
    /// Enabled
    Enabled = 1,
}

/// Alert Output Select bit
/// When the Alarm Window Lock bit is set, this bit cannot be altered until unlocked (bit 6).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputSelect {
    /// Alert output for T_UPPER, T_LOWER and T_CRIT (power-up default)
    UpperLowerCritical = 0,
    /// T_A > T_CRIT only (T_UPPER and T_LOWER temperature boundaries are disabled)
    CriticalOnly = 1,
}

/// Alert Output Polarity bit
/// This bit cannot be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputPolarity {
    /// Active-low (power-up default; pull-up resistor required)
    ActiveLow = 0,
    /// Active-high
    ActiveHigh = 1,
}

/// Alert Output Mode bit
/// This bit cannot be altered when either of the Lock bits are set (bit 6 and bit 7).
/// This bit can be programmed in Shutdown mode, but the Alert output will not assert or deassert.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum AlertOutputMode {
    /// Comparator output (power-up default)
    Comparator = 0,
    /// Interrupt output
    Interrupt = 1,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Configuration {
    /// T_HYST: T_UPPER and T_LOWER Limit Hysteresis bits
    pub hysteresis: Hysteresis,

    /// Shutdown Mode bit
    pub shutdown_mode: ShutdownMode,

    /// T_CRIT Lock bit
    pub critical_temperature_lock: CriticalTemperatureLock,

    /// T_CRIT Lock bit
    pub temperature_window_lock: TemperatureWindowLock,

    /// Interrupt Clear bit
    pub interrupt_clear: InterruptClear,

    /// Alert Output Status bit
    pub alert_output_status: AlertOutputStatus,

    /// Alert Output Control bit
    pub alert_output_control: AlertOutputControl,

    /// Alert Output Select bit
    pub alert_output_select: AlertOutputSelect,

    /// Alert Output Polarity bit
    pub alert_output_polarity: AlertOutputPolarity,

    /// Alert Output Mode bit
    pub alert_output_mode: AlertOutputMode,
}

impl Default for Configuration {
    /// Default `Configuration` with power-on default settings
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
            alert_output_mode: AlertOutputMode::Comparator,
        }
    }
}

impl From<Raw> for Configuration {
    /// Convert `Configuration` to bytes
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
    /// Convert raw bytes to `Configuration`
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
    /// Read `Configuration` from `ConfigurationRegister`
    pub fn read_configuration<Err>(&mut self) -> Result<Configuration, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(ConfigurationRegister)
    }

    /// Write `Configuration` from `ConfigurationRegister`
    pub fn write_configuration<Err>(&mut self, configuration: Configuration) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(ConfigurationRegister, configuration)
    }
}
