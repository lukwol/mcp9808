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
    fn address(&self) -> u8;
}

trait Write1BitRegister<T>: Register
where
    T: Into<[u8; 1]>,
{
}

trait Write2BitRegister<T>: Register
where
    T: Into<[u8; 2]>,
{
}

trait Read1BitRegister<T>: Register
where
    T: From<[u8; 1]>,
{
}

trait Read2BitRegister<T>: Register
where
    T: From<[u8; 2]>,
{
}

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

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Celsius(i16);

impl From<[u8; 1]> for Celsius {
    fn from(_: [u8; 1]) -> Self {
        Celsius(0)
    }
}

impl Into<[u8; 1]> for Celsius {
    fn into(self) -> [u8; 1] {
        [0]
    }
}

#[derive(Debug)]
struct TemperatureRegister;

impl Register for TemperatureRegister {
    fn address(&self) -> u8 {
        0b0000
    }
}

impl Read1BitRegister<Celsius> for TemperatureRegister {}
impl Read2BitRegister<Millicelsius> for TemperatureRegister {}
impl Write1BitRegister<Celsius> for TemperatureRegister {}
impl Write2BitRegister<Millicelsius> for TemperatureRegister {}

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
        MCP9808 { address, i2c }
    }

    fn write_1_bit_register<T, E>(
        &mut self,
        register: &impl Write1BitRegister<T>,
        value: T,
    ) -> Result<(), E>
    where
        I2C: i2c::Write<Error = E>,
        T: Into<[u8; 1]>,
    {
        let mut buff = [0; 2];
        buff[0] = register.address();
        for (i, item) in value.into().iter().enumerate() {
            buff[i + 1] = *item;
        }
        self.i2c.write(self.address.0, &buff)?;

        Ok(())
    }

    fn write_2_bit_register<T, E>(
        &mut self,
        register: &impl Write2BitRegister<T>,
        value: T,
    ) -> Result<(), E>
    where
        I2C: i2c::Write<Error = E>,
        T: Into<[u8; 2]>,
    {
        let mut buff = [0; 3];
        buff[0] = register.address();
        for (i, item) in value.into().iter().enumerate() {
            buff[i + 1] = *item;
        }
        self.i2c.write(self.address.0, &buff)?;

        Ok(())
    }

    fn read_1_bit_register<T, E>(&mut self, register: &impl Read1BitRegister<T>) -> Result<T, E>
    where
        I2C: i2c::WriteRead<Error = E>,
        T: From<[u8; 1]>,
    {
        let mut buff = [0; 1];
        self.i2c
            .write_read(self.address.0, &[register.address()], &mut buff)?;
        Ok(T::from(buff))
    }

    fn read_2_bit_register<T, E>(&mut self, register: &impl Read2BitRegister<T>) -> Result<T, E>
    where
        I2C: i2c::WriteRead<Error = E>,
        T: From<[u8; 2]>,
    {
        let mut buff = [0; 2];
        self.i2c
            .write_read(self.address.0, &[register.address()], &mut buff)?;
        Ok(T::from(buff))
    }
}
