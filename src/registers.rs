use crate::hal::blocking::i2c;
use i2c_reg::*;

i2c_ro_reg!(AmbientTemperatureRegister, addr: 0b1010, len: 2);
