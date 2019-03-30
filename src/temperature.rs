use crate::hal::blocking::i2c;
use crate::resolution::Resolution;
use crate::AmbientTemperatureRegister;
use crate::MCP9808;
use crate::units::Millicelsius;

pub struct Temperature<Unit> {
    value: Unit,
    resolution: Resolution<Unit>,
}

//impl<Unit> From<[u8; 2]> for Temperature<Unit> {
//    fn from(val: [u8; 2]) -> Self {
//        let (msb, lsb) = (val[0], val[1]);
//
//        let fraction = (lsb & 0b1111) >> (0b11 - self.va)
//
//        let fraction = (i32::from((lsb & 0b1111) >> (0b11 - resolution as u8))
//            * resolution.precision_10k())
//            / 10;
//
//        let celsius = {
//            let mut msb = msb & 0b1_1111;
//            if msb & TEMPERATURE_SIGN_BIT == 0 {
//                i32::from((msb << 4) + (lsb >> 4))
//            } else {
//                msb &= 0b1111;
//                i32::from((msb << 4) + (lsb >> 4)) - 256
//            }
//        };
//
//        Millicelsius(celsius * 1000 + fraction)
//    }
//}

//impl<I2C> MCP9808<I2C> {
//    pub fn read_ambient_temperature<Err>(&mut self) -> Result<Temperature, Err>
//    where
//        I2C: i2c::WriteRead<Error = Err>,
//    {
//        self.i2c_interface.read_register(AmbientTemperatureRegister)
//    }
//}
