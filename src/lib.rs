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

struct Temperature(u16);

impl From<[u8; 2]> for Temperature {
    fn from(_: [u8; 2]) -> Self {
        Temperature(0)
    }
}

struct TemperatureRegister;

impl Register for TemperatureRegister {
    fn address(&self) -> Address {
        Address(0)
    }
}

impl<'a> ReadRegister<'a, [u8; 2]> for TemperatureRegister {
    fn read<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address) -> Result<[u8; 2], Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
    {
        &|i2c, device_address, reg_address| {
            let mut buff = [0; 2];
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
