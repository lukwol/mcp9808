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

#[derive(Debug)]
struct MCP9808<I2C> {
    address: Address,
    i2c: I2C,
}

trait Register {
    fn address(&self) -> Address;
}

// I2cRead1BReg

trait Read1BReg<Value>: Register
where
    Value: From<[u8; 1]>,
{
}

trait I2cRead1BReg<I2C> {
    fn read_1_byte_register<Value, Err>(
        &mut self,
        register: &impl Read1BReg<Value>,
    ) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Value: From<[u8; 1]>;
}

impl<I2C> I2cRead1BReg<I2C> for MCP9808<I2C> {
    fn read_1_byte_register<Value, Err>(
        &mut self,
        register: &impl Read1BReg<Value>,
    ) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Value: From<[u8; 1]>,
    {
        let mut buff = [0; 1];
        self.i2c
            .write_read(self.address.into(), &[register.address().into()], &mut buff)?;
        Ok(Value::from(buff))
    }
}

// I2cWrite1BReg

trait Write1BReg<Value>: Register
where
    Value: Into<[u8; 1]>,
{
}

trait I2cWrite1BReg<I2C> {
    fn write_1_byte_register<Value, Err>(
        &mut self,
        register: &impl Write1BReg<Value>,
        value: Value,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        Value: Into<[u8; 1]>;
}

impl<I2C> I2cWrite1BReg<I2C> for MCP9808<I2C> {
    fn write_1_byte_register<Value, Err>(
        &mut self,
        register: &impl Write1BReg<Value>,
        value: Value,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        Value: Into<[u8; 1]>,
    {
        let mut payload = [0; 2];
        payload[0] = register.address().into();
        for (i, item) in value.into().iter().enumerate() {
            payload[i + 1] = *item;
        }
        self.i2c.write(self.address.into(), &payload)?;
        Ok(())
    }
}

//ReadWrite1BReg

trait ReadWrite1BReg<Value>: Read1BReg<Value> + Write1BReg<Value>
where
    Value: From<[u8; 1]>,
    Value: Into<[u8; 1]>,
{
}

trait I2cReadWrite1BReg<I2C>: I2cRead1BReg<I2C> + I2cWrite1BReg<I2C> {}
