#![no_std]
#![allow(dead_code)]

use crate::hal::blocking::i2c;
use embedded_hal as hal;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Address(u8);

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        address.0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum AddressPin {
    A0 = 1,
    A1 = 1 << 1,
    A2 = 1 << 2,
}

#[derive(Debug, Default)]
struct PinConfiguration {
    a0_pin_enabled: bool,
    a1_pin_enabled: bool,
    a2_pin_enabled: bool,
}

#[derive(Debug)]
struct PinConfigurableAddress {
    default_address: Address,
    configuration: Option<PinConfiguration>,
}

impl PinConfigurableAddress {
    fn configured_address(&self) -> Address {
        let mut address_ptr: u8 = self.default_address.into();
        if let Some(config) = &self.configuration {
            if config.a0_pin_enabled {
                address_ptr |= AddressPin::A0 as u8;
            }
            if config.a1_pin_enabled {
                address_ptr |= AddressPin::A1 as u8;
            }
            if config.a2_pin_enabled {
                address_ptr |= AddressPin::A2 as u8;
            }
        }
        Address(address_ptr)
    }
}

trait Register {
    fn len(&self) -> usize;
    fn address(&self) -> u8;
}

trait WriteRegister<T>: Register where T: Into<[u8; 2]>{}

trait ReadRegister<T>: Register where T: From<[u8; 2]>{}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Millicelsius(i16);

impl From<[u8; 2]> for Millicelsius {
    fn from(_: [u8; 2]) -> Self {
        Millicelsius(0)
    }
}

impl Into<[u8; 2]> for Millicelsius {
    fn into(self) -> [u8; 2] {
        [0, 0]
    }
}

#[derive(Debug)]
struct TemperatureRegister;

impl Register for TemperatureRegister {
    fn len(&self) -> usize {
        2
    }

    fn address(&self) -> u8 {
        0b0000
    }
}

impl ReadRegister<Millicelsius> for TemperatureRegister {}
impl WriteRegister<Millicelsius> for TemperatureRegister {}

#[derive(Debug)]
enum Error<E> {
    I2C(E),
}

#[derive(Debug)]
struct MCP9808<I2C> {
    address: Address,
    i2c: I2C,
}

impl<I2C> MCP9808<I2C> {
    fn new(i2c: I2C, address: Address) -> Self {
        MCP9808 { address, i2c, }
    }

    fn write_register<T, E>(&mut self, register: &impl WriteRegister<T>, value: T) -> Result<(), E>
    where
        I2C: i2c::Write<Error = E>,
        T: Into<[u8; 2]>
    {
        let mut buff = [0; 3];
        buff[0] = register.address();
        for (i, item) in value.into().iter().enumerate() {
            buff[i + 1] = *item;
        }
        self.i2c.write(self.address.0, &buff[0..register.len()])?;

        Ok(())
    }

    fn read_register<T, E>(&mut self, register: &impl ReadRegister<T>) -> Result<T, E>
    where
        I2C: i2c::WriteRead<Error = E>,
        T: From<[u8; 2]>
    {
        let mut buff = [0; 2];
        self.i2c
            .write_read(self.address.0, &[register.address()], &mut buff[0..register.len()])?;
        Ok(T::from(buff))
    }

    fn foo(&mut self) where I2C: i2c::WriteRead + i2c::Write {
        let reg = TemperatureRegister;
        self.read_register(&reg).ok();
        self.write_register(&reg, Millicelsius(0)).ok();
    }
}
