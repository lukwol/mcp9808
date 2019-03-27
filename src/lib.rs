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
    fn len(&self) -> u8;
    fn address(&self) -> u8;
}

trait WriteRegister: Register {
    fn write_value(&self) -> [u8; 2];
}

trait ReadRegister: Register {
    fn read_value(&self) -> [u8; 2];
}

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

    fn write_register<E>(&mut self, register: impl WriteRegister) -> Result<(), E>
    where
        I2C: i2c::Write<Error = E>,
    {
        let mut buff = [0; 3];
        buff[0] = register.address();
        for (i, item) in register.write_value().iter().enumerate() {
            buff[i + 1] = *item;
        }
        self.i2c.write(self.address.0, &buff)?;

        Ok(())
    }

    fn read_register<T, E>(&mut self, register: impl ReadRegister) -> Result<T, E>
    where
        I2C: i2c::WriteRead<Error = E>,
        T: From<u16>
    {
        let mut buff = [0; 2];
        for (i, item) in register.read_value().iter().enumerate() {
            buff[i + 1] = *item;
        }
        self.i2c
            .write_read(self.address.0, &[register.address()], &mut buff)?;

        Ok(T::from(0))
    }
}
