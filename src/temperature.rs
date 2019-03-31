use crate::hal::blocking::i2c;
use crate::AmbientTemperatureRegister;
use crate::MCP9808;

const TEMPERATURE_SIGN_BIT: u8 = 0b1_0000;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Millicelsius(pub i32);

impl From<[u8; 2]> for Millicelsius {
    fn from(raw: [u8; 2]) -> Self {
        let (msb, lsb) = (raw[0], raw[1]);

        let fraction = (0..4).fold(0, |acc, x| acc + i32::from(lsb & 1 << x) * 625) / 10;
        let mut celsius = (0..4).fold(0, |acc, x| {
            acc + (i32::from(msb & 1 << x) << 4) + i32::from(lsb >> 4 & 1 << x)
        });
        if msb & TEMPERATURE_SIGN_BIT != 0 {
            celsius -= 1 << 8
        }

        Millicelsius(celsius * 1_000 + fraction)
    }
}

impl From<Millicelsius> for [u8; 2] {
    fn from(_millicelsius: Millicelsius) -> Self {
        // TODO: Implement
        [0, 0]
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Celsius(pub f32);

impl From<Millicelsius> for Celsius {
    fn from(millicelsius: Millicelsius) -> Self {
        Celsius(millicelsius.0 as f32 / 1_000.0)
    }
}

impl From<Celsius> for Millicelsius {
    fn from(celsius: Celsius) -> Self {
        Millicelsius((celsius.0 * 1_000.0) as i32)
    }
}

impl From<[u8; 2]> for Celsius {
    fn from(raw: [u8; 2]) -> Self {
        Millicelsius::from(raw).into()
    }
}

impl From<Celsius> for [u8; 2] {
    fn from(celsius: Celsius) -> Self {
        Millicelsius::from(celsius).into()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Temperature<Unit>(pub Unit);

impl<Unit> From<[u8; 2]> for Temperature<Unit>
where
    Unit: From<[u8; 2]>,
{
    fn from(raw: [u8; 2]) -> Self {
        Temperature(Unit::from(raw))
    }
}

impl<Unit> From<Temperature<Unit>> for [u8; 2]
where
    Unit: Into<[u8; 2]>,
{
    fn from(temperature: Temperature<Unit>) -> Self {
        temperature.0.into()
    }
}

impl<I2C> MCP9808<I2C> {
    pub fn read_ambient_temperature<Unit, Err>(&mut self) -> Result<Temperature<Unit>, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Unit: From<[u8; 2]>,
    {
        self.i2c_interface.read_register(AmbientTemperatureRegister)
    }
}

#[cfg(test)]
mod conversion {
    use crate::temperature::{Celsius, Millicelsius};

    #[test]
    fn millicelsius_to_celsius() {
        assert_eq!(Celsius(25.367), Millicelsius(25_367).into());
        assert_eq!(Celsius(-0.789), Millicelsius(-789).into());
    }

    mod temperature_to_raw {
        use crate::temperature::{Celsius, Millicelsius};

        #[test]
        fn zero_millicelsius() {
            assert_eq!([0b0000_0000, 0b0000_0000], {
                let raw: [u8; 2] = Millicelsius(0).into();
                raw
            });
        }

        #[test]
        fn zero_celsius() {
            assert_eq!([0b0000_0000, 0b0000_0000], {
                let raw: [u8; 2] = Celsius(0.0).into();
                raw
            });
        }

        #[test]
        fn slightly_above_zero_millicelsius() {
            assert_eq!([0b0000_0000, 0b0000_0001], {
                let raw: [u8; 2] = Millicelsius(62).into();
                raw
            });
            assert_eq!([0b0000_0000, 0b0000_0010], {
                let raw: [u8; 2] = Millicelsius(125).into();
                raw
            });
            assert_eq!([0b0000_0000, 0b0000_1000], {
                let raw: [u8; 2] = Millicelsius(500).into();
                raw
            });
            assert_eq!([0b0000_0000, 0b0000_1010], {
                let raw: [u8; 2] = Millicelsius(625).into();
                raw
            });
        }

        #[test]
        fn slightly_above_zero_celsius() {
            assert_eq!([0b0000_0000, 0b0000_0001], {
                let raw: [u8; 2] = Celsius(0.062).into();
                raw
            });
            assert_eq!([0b0000_0000, 0b0000_0010], {
                let raw: [u8; 2] = Celsius(0.125).into();
                raw
            });
            assert_eq!([0b0000_0000, 0b0000_1000], {
                let raw: [u8; 2] = Celsius(0.500).into();
                raw
            });
            assert_eq!([0b0000_0000, 0b0000_1010], {
                let raw: [u8; 2] = Celsius(0.625).into();
                raw
            });
        }

        #[test]
        fn slightly_below_zero_millicelsius() {
            assert_eq!([0b0001_1111, 0b1111_1111], {
                let raw: [u8; 2] = Millicelsius(-63).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_1110], {
                let raw: [u8; 2] = Millicelsius(-125).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_1100], {
                let raw: [u8; 2] = Millicelsius(-250).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_0111], {
                let raw: [u8; 2] = Millicelsius(-563).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_0101], {
                let raw: [u8; 2] = Millicelsius(-688).into();
                raw
            });
        }

        #[test]
        fn slightly_below_zero_celsius() {
            assert_eq!([0b0001_1111, 0b1111_1111], {
                let raw: [u8; 2] = Celsius(-0.063).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_1110], {
                let raw: [u8; 2] = Celsius(-0.125).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_1100], {
                let raw: [u8; 2] = Celsius(-0.250).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_0111], {
                let raw: [u8; 2] = Celsius(-0.563).into();
                raw
            });
            assert_eq!([0b0001_1111, 0b1111_0101], {
                let raw: [u8; 2] = Celsius(-0.688).into();
                raw
            });
        }

        #[test]
        fn above_zero_millicelsius() {
            assert_eq!([0b0000_00001, 0b1001_0100], {
                let raw: [u8; 2] = Millicelsius(25250).into();
                raw
            });
            assert_eq!([0b0000_00011, 0b1001_0110], {
                let raw: [u8; 2] = Millicelsius(57375).into();
                raw
            });
        }

        #[test]
        fn above_zero_celsius() {
            assert_eq!([0b0000_00001, 0b1001_0100], {
                let raw: [u8; 2] = Celsius(25.250).into();
                raw
            });
            assert_eq!([0b0000_00011, 0b1001_0110], {
                let raw: [u8; 2] = Celsius(57.375).into();
                raw
            });
        }

        #[test]
        fn below_zero_millicelsius() {
            assert_eq!([0b0000_11110, 0b0111_1111], {
                let raw: [u8; 2] = Millicelsius(-24063).into();
                raw
            });
            assert_eq!([0b0000_11100, 0b0111_1010], {
                let raw: [u8; 2] = Millicelsius(-56375).into();
                raw
            });
        }

        #[test]
        fn below_zero_celsius() {
            assert_eq!([0b0000_11110, 0b0111_1111], {
                let raw: [u8; 2] = Celsius(-24.063).into();
                raw
            });
            assert_eq!([0b0000_11100, 0b0111_1010], {
                let raw: [u8; 2] = Celsius(-56.375).into();
                raw
            });
        }
    }

    mod raw_to_temperature {
        use crate::temperature::{Celsius, Millicelsius};

        #[test]
        fn zero_millicelsius() {
            assert_eq!(Millicelsius(0), [0b0000_0000, 0b0000_0000].into());
        }

        #[test]
        fn zero_celsius() {
            assert_eq!(Celsius(0.0), [0b0000_0000, 0b0000_0000].into());
        }

        #[test]
        fn slightly_above_zero_millicelsius() {
            assert_eq!(Millicelsius(62), [0b0000_0000, 0b0000_0001].into());
            assert_eq!(Millicelsius(125), [0b0000_0000, 0b0000_0010].into());
            assert_eq!(Millicelsius(500), [0b0000_0000, 0b0000_1000].into());
            assert_eq!(Millicelsius(625), [0b0000_0000, 0b0000_1010].into());
        }

        #[test]
        fn slightly_above_zero_celsius() {
            assert_eq!(Celsius(0.062), [0b0000_0000, 0b0000_0001].into());
            assert_eq!(Celsius(0.125), [0b0000_0000, 0b0000_0010].into());
            assert_eq!(Celsius(0.500), [0b0000_0000, 0b0000_1000].into());
            assert_eq!(Celsius(0.625), [0b0000_0000, 0b0000_1010].into());
        }

        #[test]
        fn slightly_below_zero_millicelsius() {
            assert_eq!(Millicelsius(-63), [0b0001_1111, 0b1111_1111].into());
            assert_eq!(Millicelsius(-125), [0b0001_1111, 0b1111_1110].into());
            assert_eq!(Millicelsius(-250), [0b0001_1111, 0b1111_1100].into());
            assert_eq!(Millicelsius(-563), [0b0001_1111, 0b1111_0111].into());
            assert_eq!(Millicelsius(-688), [0b0001_1111, 0b1111_0101].into());
        }

        #[test]
        fn slightly_below_zero_celsius() {
            assert_eq!(Celsius(-0.063), [0b0001_1111, 0b1111_1111].into());
            assert_eq!(Celsius(-0.125), [0b0001_1111, 0b1111_1110].into());
            assert_eq!(Celsius(-0.250), [0b0001_1111, 0b1111_1100].into());
            assert_eq!(Celsius(-0.563), [0b0001_1111, 0b1111_0111].into());
            assert_eq!(Celsius(-0.688), [0b0001_1111, 0b1111_0101].into());
        }

        #[test]
        fn above_zero_millicelsius() {
            assert_eq!(Millicelsius(25250), [0b0000_00001, 0b1001_0100].into());
            assert_eq!(Millicelsius(57375), [0b0000_00011, 0b1001_0110].into());
        }

        #[test]
        fn above_zero_celsius() {
            assert_eq!(Celsius(25.250), [0b0000_00001, 0b1001_0100].into());
            assert_eq!(Celsius(57.375), [0b0000_00011, 0b1001_0110].into());
        }

        #[test]
        fn below_zero_millicelsius() {
            assert_eq!(Millicelsius(-24063), [0b0000_11110, 0b0111_1111].into());
            assert_eq!(Millicelsius(-56375), [0b0000_11100, 0b0111_1010].into());
        }

        #[test]
        fn below_zero_celsius() {
            assert_eq!(Celsius(-24.063), [0b0000_11110, 0b0111_1111].into());
            assert_eq!(Celsius(-56.375), [0b0000_11100, 0b0111_1010].into());
        }
    }
}
