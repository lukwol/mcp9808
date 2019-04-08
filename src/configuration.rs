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

#[derive(Debug, PartialEq, Clone)]
pub struct Configuration {
    hysteresis: Hysteresis,
    shutdown_mode: ShutdownMode,
    critical_temperature_lock: CriticalTemperatureLock,
    temperature_window_lock: TemperatureWindowLock,
    interrupt_clear: InterruptClear,
    alert_output_status: AlertOutputStatus,
    alert_output_control: AlertOutputControl,
    alert_output_select: AlertOutputSelect,
    alert_output_polarity: AlertOutputPolarity,
    alert_output_mode: AlertOutputMode,
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

#[derive(Debug)]
pub enum InvalidConfigurationError {
    TemperatureWindowLocked,
    CriticalTemperatureLocked,
    ShutdownModeEnabled,
}

#[derive(Debug)]
pub struct ConfigurationBuilder {
    initial_configuration: Configuration,
    configuration: Configuration,
}

impl ConfigurationBuilder {
    pub fn new(configuration: Configuration) -> Self {
        ConfigurationBuilder {
            initial_configuration: configuration.clone(),
            configuration,
        }
    }

    pub fn set_hysteresis(mut self, hysteresis: Hysteresis) -> Self {
        self.configuration.hysteresis = hysteresis;
        self
    }

    pub fn set_shutdown_mode(mut self, shutdown_mode: ShutdownMode) -> Self {
        self.configuration.shutdown_mode = shutdown_mode;
        self
    }

    pub fn set_critical_temperature_lock(
        mut self,
        critical_temperature_lock: CriticalTemperatureLock,
    ) -> Self {
        self.configuration.critical_temperature_lock = critical_temperature_lock;
        self
    }

    pub fn set_upper_lower_temperature_window_lock(
        mut self,
        upper_lower_temperature_window_lock: TemperatureWindowLock,
    ) -> Self {
        self.configuration.temperature_window_lock = upper_lower_temperature_window_lock;
        self
    }

    pub fn set_interrupt_clear(mut self, interrupt_clear: InterruptClear) -> Self {
        self.configuration.interrupt_clear = interrupt_clear;
        self
    }

    pub fn set_alert_output_status(mut self, alert_output_status: AlertOutputStatus) -> Self {
        self.configuration.alert_output_status = alert_output_status;
        self
    }

    pub fn set_alert_output_control(mut self, alert_output_control: AlertOutputControl) -> Self {
        self.configuration.alert_output_control = alert_output_control;
        self
    }

    pub fn set_alert_output_select(mut self, alert_output_select: AlertOutputSelect) -> Self {
        self.configuration.alert_output_select = alert_output_select;
        self
    }

    pub fn set_alert_output_polarity(mut self, alert_output_polarity: AlertOutputPolarity) -> Self {
        self.configuration.alert_output_polarity = alert_output_polarity;
        self
    }

    pub fn set_alert_output_mode(mut self, alert_output_mode: AlertOutputMode) -> Self {
        self.configuration.alert_output_mode = alert_output_mode;
        self
    }

    pub fn build(self) -> Result<Configuration, InvalidConfigurationError> {
        let temp_window_locked =
            self.configuration.temperature_window_lock == TemperatureWindowLock::Locked;
        let crit_temp_locked =
            self.configuration.critical_temperature_lock == CriticalTemperatureLock::Locked;

        let shutdown_mode_enabled = self.configuration.shutdown_mode == ShutdownMode::Shutdown;

        let hysteresis_altered =
            self.configuration.hysteresis == self.initial_configuration.hysteresis;
        let alert_status_altered = self.configuration.alert_output_status
            == self.initial_configuration.alert_output_status;
        let alert_control_altered = self.configuration.alert_output_control
            == self.initial_configuration.alert_output_control;
        let alert_select_altered = self.configuration.alert_output_select
            == self.initial_configuration.alert_output_select;
        let alert_polarity_altered = self.configuration.alert_output_polarity
            == self.initial_configuration.alert_output_polarity;
        let alert_mode_altered =
            self.configuration.alert_output_mode == self.initial_configuration.alert_output_mode;

        let temp_window_cleared = self.configuration.temperature_window_lock
            == TemperatureWindowLock::Unlocked
            && self.initial_configuration.temperature_window_lock == TemperatureWindowLock::Locked;
        let crit_temp_cleared = self.configuration.critical_temperature_lock
            == CriticalTemperatureLock::Unlocked
            && self.initial_configuration.critical_temperature_lock
                == CriticalTemperatureLock::Locked;
        let interrupt_cleared = self.configuration.interrupt_clear == InterruptClear::Cleared
            && self.initial_configuration.interrupt_clear == InterruptClear::NotEffect;

        if temp_window_locked
            && (hysteresis_altered
                || alert_select_altered
                || alert_mode_altered
                || alert_polarity_altered
                || alert_control_altered
                || shutdown_mode_enabled
                || temp_window_cleared)
        {
            Err(InvalidConfigurationError::TemperatureWindowLocked)
        } else if crit_temp_locked
            && (hysteresis_altered
                || alert_mode_altered
                || alert_polarity_altered
                || alert_control_altered
                || shutdown_mode_enabled
                || crit_temp_cleared)
        {
            Err(InvalidConfigurationError::CriticalTemperatureLocked)
        } else if shutdown_mode_enabled && (alert_status_altered || interrupt_cleared) {
            Err(InvalidConfigurationError::ShutdownModeEnabled)
        } else {
            Ok(self.configuration)
        }
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
