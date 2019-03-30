use crate::hal::blocking::i2c;
use crate::ResolutionRegister;
use crate::MCP9808;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Resolution {
    Deg05c = 0b00,
    Deg025c = 0b01,
    Deg0125c = 0b10,
    Deg00625c = 0b11,
}

impl From<[u8; 1]> for Resolution {
    fn from(val: [u8; 1]) -> Self {
        match val[0] & 0b11 {
            0b00 => Resolution::Deg05c,
            0b01 => Resolution::Deg025c,
            0b10 => Resolution::Deg0125c,
            0b11 => Resolution::Deg00625c,
            _ => panic!("impossible happened"),
        }
    }
}

impl From<Resolution> for [u8; 1] {
    fn from(res: Resolution) -> Self {
        [res as u8]
    }
}

impl Resolution {
    pub fn precision(self) -> i32 {
        match self {
            Resolution::Deg05c => 5000,
            Resolution::Deg025c => 2500,
            Resolution::Deg0125c => 1250,
            Resolution::Deg00625c => 625,
        }
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_resolution<Err>(&mut self) -> Result<Resolution, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(ResolutionRegister)
    }

    pub fn write_resolution<Err>(&mut self, resolution: Resolution) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(ResolutionRegister, resolution)
    }
}
