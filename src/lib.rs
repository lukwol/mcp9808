#![no_std]
#![allow(dead_code, unused_macros)]

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

trait I2cReadRegister<'a, Size>: Register {
    fn i2c_read<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address) -> Result<Size, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

trait I2cWriteRegister<'a, Size>: Register {
    fn i2c_write<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address, Size) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}

struct Temperature(u16);

impl From<[u8; 2]> for Temperature {
    fn from(_: [u8; 2]) -> Self {
        Temperature(0)
    }
}

impl Into<[u8; 2]> for Temperature {
    fn into(self) -> [u8; 2] {
        [0, 0]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
//#[derive(I2cReadRegister)]
//#[len = 2]
//#[address = 0b1010]
struct TemperatureRegister;

macro_rules! i2c_rw_reg {
    ($name: ty, $len: tt, $addr: expr) => {
        impl Register for $name {
            fn address(&self) -> Address {
                Address($addr)
            }
        }

        impl<'a> I2cReadRegister<'a, [u8; $len]> for $name {
            fn i2c_read<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address) -> Result<[u8; 2], Err>
            where
                I2C: i2c::WriteRead<Error = Err>,
            {
                &|i2c, device_address, reg_address| {
                    let mut buff = [0; $len];
                    i2c.write_read(device_address.into(), &[reg_address.into()], &mut buff)?;
                    Ok(buff)
                }
            }
        }

        impl<'a> I2cWriteRegister<'a, [u8; 2]> for $name {
            fn i2c_write<I2C, Err>(
                &self,
            ) -> &Fn(&mut I2C, Address, Address, [u8; $len]) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
            {
                &|i2c, device_address, reg_address, value| {
                    let mut payload = [0; $len + 1];
                    payload[0] = reg_address.into();
                    for (i, item) in value.iter().enumerate() {
                        payload[i + 1] = *item;
                    }
                    i2c.write(device_address.into(), &payload)
                }
            }
        }
    };
}

i2c_rw_reg!(TemperatureRegister, 2, 0b1010);

#[derive(Debug)]
struct I2cInterface<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C> I2cInterface<I2C> {
    fn read_register<'a, Size, Value, Err>(
        &mut self,
        register: impl I2cReadRegister<'a, Size>,
    ) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Value: From<Size>,
    {
        register.i2c_read()(&mut self.i2c, self.address, register.address()).map(|v| v.into())
    }

    fn write_register<'a, Size, Err>(
        &mut self,
        register: impl I2cWriteRegister<'a, Size>,
        value: impl Into<Size>,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        register.i2c_write()(
            &mut self.i2c,
            self.address,
            register.address(),
            value.into(),
        )
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
        self.i2c_interface.read_register(self.temperature_register)
    }

    fn write_temperature<Err>(&mut self, temperature: Temperature) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        self.i2c_interface
            .write_register(self.temperature_register, temperature)
    }
}
