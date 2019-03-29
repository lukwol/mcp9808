#![no_std]
#![allow(dead_code)]

use crate::hal::blocking::i2c;
use embedded_hal as hal;

use core::marker::PhantomData;

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

struct Register<Size> {
    address: Address,
    _pd: PhantomData<Size>,
}

trait ReadRegister<'a, Size> {
    fn read_register<I2C, Value, Err>(
        &self,
    ) -> &Fn(&mut I2C, Address, Address) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Value: From<Size>;
}

impl<'a> ReadRegister<'a, [u8; 2]> for Register<[u8; 2]> {
    fn read_register<I2C, Value, Err>(
        &self,
    ) -> &Fn(&mut I2C, Address, Address) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Value: From<[u8; 2]>,
    {
        &|i2c, device_address, reg_address| {
            let mut buff = [0; 2];
            i2c.write_read(device_address.into(), &[reg_address.into()], &mut buff)?;
            Ok(Value::from(buff))
        }
    }
}

//fn read_1_byte_register<Value, Err>(
//    &mut self,
//    register: &impl Read1BReg<Value>,
//) -> Result<Value, Err>
//    where
//        I2C: i2c::WriteRead<Error = Err>,
//        Value: From<[u8; 1]>,
//{
//    let mut buff = [0; 1];
//    let address_ptr = self.address().into();
//    self.i2c()
//        .write_read(address_ptr, &[register.address().into()], &mut buff)?;
//    Ok(Value::from(buff))
//}

//
//trait WriteRegister<Payload> {
//}

//impl ReadRegister<[] for Register<[u8; 1]> {
//
//}
