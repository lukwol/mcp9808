use crate::hal::blocking::i2c;
use crate::ResolutionRegister;
use crate::MCP9808;
use core::marker::PhantomData;
use crate::units::{Millicelsius, Celsius};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Resolution<Unit> {
    Deg05c(PhantomData<Unit>),
    Deg025c(PhantomData<Unit>),
    Deg0125c(PhantomData<Unit>),
    Deg00625c(PhantomData<Unit>),
}

impl<Unit> From<[u8; 1]> for Resolution<Unit> {
    fn from(val: [u8; 1]) -> Self {
        match val {
            [0b00] => Resolution::Deg05c(PhantomData),
            [0b01] => Resolution::Deg025c(PhantomData),
            [0b10] => Resolution::Deg0125c(PhantomData),
            [0b11] => Resolution::Deg00625c(PhantomData),
            _ => panic!("invalid resolution"),
        }
    }
}

impl<Unit> From<Resolution<Unit>> for [u8; 1] {
    fn from(res: Resolution<Unit>) -> Self {
        match res {
            Resolution::Deg05c(_) => [0b00],
            Resolution::Deg025c(_) => [0b01],
            Resolution::Deg0125c(_) => [0b10],
            Resolution::Deg00625c(_) => [0b11],
        }
    }
}

impl Resolution<Millicelsius> {
    pub fn precision(self) -> i32 {
        match self {
            Resolution::Deg05c(_) => 500,
            Resolution::Deg025c(_) => 250,
            Resolution::Deg0125c(_) => 125,
            Resolution::Deg00625c(_) => 62,
        }
    }
}

impl Resolution<Celsius> {
    pub fn precision(self) -> f32 {
        match self {
            Resolution::Deg05c(_) => 0.5,
            Resolution::Deg025c(_) => 0.25,
            Resolution::Deg0125c(_) => 0.125,
            Resolution::Deg00625c(_) => 0.625,
        }
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_resolution<Unit, Err>(&mut self) -> Result<Resolution<Unit>, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface.read_register(ResolutionRegister)
    }

    pub fn write_resolution<Unit, Err>(&mut self, resolution: Resolution<Unit>) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(ResolutionRegister, resolution)
    }
}
