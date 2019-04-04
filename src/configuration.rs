use crate::hal::blocking::i2c;
use crate::MCP9808;

macro_rules! config_bits {
    ($name: ident, $off: ident, $on: ident, $bit: expr) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        #[repr(u8)]
        enum $name {
            $off = 0,
            $on = 1,
        }

        impl From<$name> for u8 {
            fn from(config: $name) -> u8 {
                (config as u8) << $bit
            }
        }

        impl From<u8> for $name {
            fn from(raw: u8) -> Self {
                match raw >> $bit {
                    0 => $name::$off,
                    1 => $name::$on,
                    _ => panic!("impossible happened"),
                }
            }
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Hysteresis {
    Deg0C = 0b00,
    Deg1_5C = 0b01,
    Deg3_0C = 0b10,
    Deg6_0C = 0b11,
}

impl From<Hysteresis> for u8 {
    fn from(config: Hysteresis) -> Self {
        (config as u8) << 2
    }
}

impl From<u8> for Hysteresis {
    fn from(raw: u8) -> Self {
        match raw >> 2 & 0b11 {
            0b00 => Hysteresis::Deg0C,
            0b01 => Hysteresis::Deg1_5C,
            0b10 => Hysteresis::Deg3_0C,
            0b11 => Hysteresis::Deg6_0C,
            _ => panic!("impossible happened"),
        }
    }
}

config_bits!(Mode, ContinuousConversion, Shutdown, 1);

config_bits!(CriticalTemperature, Unlocked, Locked, 7);
config_bits!(UpperLowerTemperature, Unlocked, Locked, 6);
config_bits!(InterruptOutput, NotCleared, Cleared, 5);
config_bits!(AlertOutputStatus, NotAsserted, Asserted, 4);
config_bits!(AlertOutputControl, Disabled, Enabled, 3);
config_bits!(AlertOutputSelect, UpperLowerCritical, CriticalOnly, 2);
config_bits!(AlertOutputPolarity, ActiveLow, ActiveHigh, 1);
config_bits!(AlertOutputMode, Comparator, Interrupt, 0);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Configuration {
    // TODO: Add configuration fields
}

impl From<[u8; 2]> for Configuration {
    fn from(_raw: [u8; 2]) -> Self {
        // TODO: Implement conversion from raw data

        Configuration {}
    }
}

impl From<Configuration> for [u8; 2] {
    fn from(_res: Configuration) -> Self {
        // TODO: Implement conversion to raw data
        [0; 2]
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_configuration<Err>(&mut self) -> Result<Configuration, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(&self.configuration_register)
    }

    pub fn write_configuration<Err>(&mut self, resolution: Configuration) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(&self.configuration_register, resolution)
    }
}
