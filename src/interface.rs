use crate::hal::blocking::i2c;
use crate::new_registers::Register;
use core::{mem, ops::Add};
use generic_array::{typenum::bit::B1, typenum::operator_aliases::Add1, ArrayLength, GenericArray};

#[derive(Debug)]
pub struct I2cInterface<I2C> {
    /// Slave device I2C
    pub i2c: I2C,

    /// Slave device address
    pub address: u8,
}

impl<I2C> I2cInterface<I2C> {
    /// Read bytes from register
    pub fn read_register<N, Err>(&mut self, register: Register) -> Result<GenericArray<u8, N>, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        N: ArrayLength<u8>,
    {
        let mut buff: GenericArray<u8, N> = unsafe { mem::zeroed() };
        self.i2c
            .write_read(self.address, &[register as u8], &mut buff)?;
        Ok(buff)
    }

    /// Write bytes to register
    pub fn write_register<N, Err>(
        &mut self,
        register: Register,
        bytes: GenericArray<u8, N>,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
        N: ArrayLength<u8> + Add<B1>,
        Add1<N>: ArrayLength<u8>,
    {
        let mut payload: GenericArray<u8, Add1<N>> = unsafe { mem::zeroed() };
        payload[0] = register as u8;
        for (i, item) in bytes.iter().enumerate() {
            payload[i + 1] = *item;
        }
        self.i2c.write(self.address, &payload)
    }
}
