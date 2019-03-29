#![no_std]
#![allow(dead_code)]

use crate::hal::blocking::i2c;
use embedded_hal as hal;

#[derive(Debug)]
enum Error<E> {
    I2C(E),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Address(u8);

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        address.0
    }
}

trait Register {
    fn address(&self) -> Address;
}

trait ReadRegister<'a, Size>: Register {
    fn read<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address) -> Result<Size, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

macro_rules! byte_arr {
    ($size: tt) => {
        [u8; $size]
    };
}

macro_rules! new_byte_arr {
    ($size: tt) => {
        [0; $size]
    };
}

struct Temperature(u16);

impl From<byte_arr!(2)> for Temperature {
    fn from(_: byte_arr!(2)) -> Self {
        Temperature(0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct TemperatureRegister;

impl Register for TemperatureRegister {
    fn address(&self) -> Address {
        Address(0)
    }
}

impl<'a> ReadRegister<'a, byte_arr!(2)> for TemperatureRegister {
    fn read<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address) -> Result<byte_arr!(2), Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        &|i2c, device_address, reg_address| {
            let mut buff = new_byte_arr!(2);
            i2c.write_read(device_address.into(), &[reg_address.into()], &mut buff)?;
            Ok(buff)
        }
    }
}

#[derive(Debug)]
struct I2cInterface<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C> I2cInterface<I2C> {
    fn read_register<'a, Size, Err>(
        &mut self,
        register: impl ReadRegister<'a, Size>,
    ) -> Result<Size, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        register.read()(&mut self.i2c, self.address, register.address())
    }
}

struct MCP9808<I2C> {
    i2c_interface: I2cInterface<I2C>,
    temperature_register: TemperatureRegister,
}

impl<I2C> MCP9808<I2C> {
    fn read_temperature<Err>(&mut self) -> Result<Temperature, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        self.i2c_interface
            .read_register(self.temperature_register)
            .map(|a| a.into())
    }
}
