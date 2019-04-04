use crate::hal::blocking::i2c;
use crate::MCP9808;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
pub enum UpperLowerTemperatureWindowLock {
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Configuration {
    hysteresis: Hysteresis,
    shutdown_mode: ShutdownMode,
    critical_temperature_lock: CriticalTemperatureLock,
    upper_lower_temperature_window_lock: UpperLowerTemperatureWindowLock,
    interrupt_clear: InterruptClear,
    alert_output_status: AlertOutputStatus,
    alert_output_control: AlertOutputControl,
    alert_output_select: AlertOutputSelect,
    alert_output_polarity: AlertOutputPolarity,
    alert_output_mode: AlertOutputMode,
}

impl From<[u8; 2]> for Configuration {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);
        Configuration {
            hysteresis: Hysteresis::from_u8(msb & 0b11 << 1).unwrap(),
            shutdown_mode: ShutdownMode::from_u8(msb & 1).unwrap(),
            critical_temperature_lock: CriticalTemperatureLock::from_u8(lsb & 1 << 7).unwrap(),
            upper_lower_temperature_window_lock: UpperLowerTemperatureWindowLock::from_u8(
                lsb & 1 << 6,
            )
            .unwrap(),
            interrupt_clear: InterruptClear::from_u8(lsb & 1 << 5).unwrap(),
            alert_output_status: AlertOutputStatus::from_u8(lsb & 1 << 3).unwrap(),
            alert_output_control: AlertOutputControl::from_u8(lsb & 1 << 3).unwrap(),
            alert_output_select: AlertOutputSelect::from_u8(lsb & 1 << 2).unwrap(),
            alert_output_polarity: AlertOutputPolarity::from_u8(lsb & 1 << 1).unwrap(),
            alert_output_mode: AlertOutputMode::from_u8(lsb & 1).unwrap(),
        }
    }
}

impl From<Configuration> for [u8; 2] {
    fn from(config: Configuration) -> Self {
        let (mut msb, mut lsb) = (0, 0);
        msb += config.hysteresis as u8 + config.shutdown_mode as u8;
        lsb += config.critical_temperature_lock as u8
            + config.upper_lower_temperature_window_lock as u8
            + config.interrupt_clear as u8
            + config.alert_output_status as u8
            + config.alert_output_control as u8
            + config.alert_output_select as u8
            + config.alert_output_polarity as u8
            + config.alert_output_mode as u8;
        [msb, lsb]
    }
}

#[derive(Debug)]
pub enum InvalidConfigurationError {
    Hysteresis,
    ShutdownMode,
    CriticalTemperatureLock,
    UpperLowerTemperatureWindowLock,
    InterruptClear,
    AlertOutputStatus,
    AlertOutputControl,
    AlertOutputSelect,
    AlertOutputPolarity,
    AlertOutputMode,
}

#[derive(Debug)]
pub struct ConfigurationBuilder {
    configuration: Configuration,
}

impl ConfigurationBuilder {
    pub fn new(configuration: Configuration) -> Self {
        ConfigurationBuilder { configuration }
    }

    pub fn set_hysteresis(&mut self, hysteresis: Hysteresis) {
        self.configuration.hysteresis = hysteresis;
    }

    pub fn set_shutdown_mode(&mut self, shutdown_mode: ShutdownMode) {
        self.configuration.shutdown_mode = shutdown_mode;
    }

    pub fn set_critical_temperature_lock(&mut self, critical_temperature_lock: CriticalTemperatureLock) {
        self.configuration.critical_temperature_lock = critical_temperature_lock;
    }

    pub fn set_upper_lower_temperature_window_lock(&mut self, upper_lower_temperature_window_lock: UpperLowerTemperatureWindowLock) {
        self.configuration.upper_lower_temperature_window_lock = upper_lower_temperature_window_lock;
    }

    pub fn set_interrupt_clear(&mut self, interrupt_clear: InterruptClear) {
        self.configuration.interrupt_clear = interrupt_clear;
    }

    pub fn set_alert_output_status(&mut self, alert_output_status: AlertOutputStatus) {
        self.configuration.alert_output_status = alert_output_status;
    }

    pub fn set_alert_output_control(&mut self, alert_output_control: AlertOutputControl) {
        self.configuration.alert_output_control = alert_output_control;
    }

    pub fn set_alert_output_select(&mut self, alert_output_select: AlertOutputSelect) {
        self.configuration.alert_output_select = alert_output_select;
    }

    pub fn set_alert_output_polarity(&mut self, alert_output_polarity: AlertOutputPolarity) {
        self.configuration.alert_output_polarity = alert_output_polarity;
    }

    pub fn set_alert_output_mode(&mut self, alert_output_mode: AlertOutputMode) {
        self.configuration.alert_output_mode = alert_output_mode;
    }

    pub fn build(self) -> Result<Configuration, InvalidConfigurationError> {
        Ok(self.configuration)
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_configuration<Err>(&mut self) -> Result<Configuration, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(&self.configuration_register)
    }

    pub fn write_configuration<Err>(&mut self, resolution: Configuration) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(&self.configuration_register, resolution)
    }
}
