#[cfg(test)]
mod temperature {

    #[cfg(test)]
    mod conversion {
        use mcp9808::temperature::{Celsius, Millicelsius};

        #[test]
        fn celsius_to_millicelsius() {
            assert_eq!(Celsius(25.367), Millicelsius(25_367).into());
            assert_eq!(Celsius(-0.789), Millicelsius(-789).into());
        }

        mod temperature_to_raw {
            use super::{Celsius, Millicelsius};
            use i2c_interface::generic_array::arr;

            #[test]
            fn zero_millicelsius() {
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_0000], Millicelsius(0).into());
            }

            #[test]
            fn zero_celsius() {
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_0000], Celsius(0.0).into());
            }

            #[test]
            fn slightly_above_zero_millicelsius() {
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_0001], Millicelsius(62).into());
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_0010], Millicelsius(125).into());
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_1000], Millicelsius(500).into());
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_1010], Millicelsius(625).into());
            }

            #[test]
            fn slightly_above_zero_celsius() {
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_0001], Celsius(0.062).into());
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_0010], Celsius(0.125).into());
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_1000], Celsius(0.500).into());
                assert_eq!(arr![u8; 0b0000_0000, 0b0000_1010], Celsius(0.625).into());
            }

            #[test]
            fn slightly_below_zero_millicelsius() {
                assert_eq!(arr![u8; 0b0001_1111, 0b1111_1111], Millicelsius(-63).into());
                assert_eq!(
                    arr![u8; 0b0001_1111, 0b1111_1110],
                    Millicelsius(-125).into()
                );
                assert_eq!(
                    arr![u8; 0b0001_1111, 0b1111_1100],
                    Millicelsius(-250).into()
                );
                assert_eq!(
                    arr![u8; 0b0001_1111, 0b1111_0111],
                    Millicelsius(-563).into()
                );
                assert_eq!(
                    arr![u8; 0b0001_1111, 0b1111_0101],
                    Millicelsius(-688).into()
                );
            }

            #[test]
            fn slightly_below_zero_celsius() {
                assert_eq!(arr![u8; 0b0001_1111, 0b1111_1111], Celsius(-0.063).into());
                assert_eq!(arr![u8; 0b0001_1111, 0b1111_1110], Celsius(-0.125).into());
                assert_eq!(arr![u8; 0b0001_1111, 0b1111_1100], Celsius(-0.250).into());
                assert_eq!(arr![u8; 0b0001_1111, 0b1111_0111], Celsius(-0.563).into());
                assert_eq!(arr![u8; 0b0001_1111, 0b1111_0101], Celsius(-0.688).into());
            }

            #[test]
            fn above_zero_millicelsius() {
                assert_eq!(
                    arr![u8; 0b0000_00001, 0b1001_0100],
                    Millicelsius(25250).into()
                );
                assert_eq!(
                    arr![u8; 0b0000_00011, 0b1001_0110],
                    Millicelsius(57375).into()
                );
            }

            #[test]
            fn above_zero_celsius() {
                assert_eq!(arr![u8; 0b0000_00001, 0b1001_0100], Celsius(25.250).into());
                assert_eq!(arr![u8; 0b0000_00011, 0b1001_0110], Celsius(57.375).into());
            }

            #[test]
            fn below_zero_millicelsius() {
                assert_eq!(
                    arr![u8; 0b0000_11110, 0b0111_1111],
                    Millicelsius(-24063).into()
                );
                assert_eq!(
                    arr![u8; 0b0000_11100, 0b0111_1010],
                    Millicelsius(-56375).into()
                );
            }

            #[test]
            fn below_zero_celsius() {
                assert_eq!(arr![u8; 0b0000_11110, 0b0111_1111], Celsius(-24.063).into());
                assert_eq!(arr![u8; 0b0000_11100, 0b0111_1010], Celsius(-56.375).into());
            }
        }

        mod raw_to_temperature {
            use super::{Celsius, Millicelsius};
            use i2c_interface::generic_array::arr;

            #[test]
            fn zero_millicelsius() {
                assert_eq!(Millicelsius(0), arr![u8; 0b0000_0000, 0b0000_0000].into());
            }

            #[test]
            fn zero_celsius() {
                assert_eq!(Celsius(0.0), arr![u8; 0b0000_0000, 0b0000_0000].into());
            }

            #[test]
            fn slightly_above_zero_millicelsius() {
                assert_eq!(Millicelsius(62), arr![u8; 0b0000_0000, 0b0000_0001].into());
                assert_eq!(Millicelsius(125), arr![u8; 0b0000_0000, 0b0000_0010].into());
                assert_eq!(Millicelsius(500), arr![u8; 0b0000_0000, 0b0000_1000].into());
                assert_eq!(Millicelsius(625), arr![u8; 0b0000_0000, 0b0000_1010].into());
            }

            #[test]
            fn slightly_above_zero_celsius() {
                assert_eq!(Celsius(0.062), arr![u8; 0b0000_0000, 0b0000_0001].into());
                assert_eq!(Celsius(0.125), arr![u8; 0b0000_0000, 0b0000_0010].into());
                assert_eq!(Celsius(0.500), arr![u8; 0b0000_0000, 0b0000_1000].into());
                assert_eq!(Celsius(0.625), arr![u8; 0b0000_0000, 0b0000_1010].into());
            }

            #[test]
            fn slightly_below_zero_millicelsius() {
                assert_eq!(Millicelsius(-63), arr![u8; 0b0001_1111, 0b1111_1111].into());
                assert_eq!(
                    Millicelsius(-125),
                    arr![u8; 0b0001_1111, 0b1111_1110].into()
                );
                assert_eq!(
                    Millicelsius(-250),
                    arr![u8; 0b0001_1111, 0b1111_1100].into()
                );
                assert_eq!(
                    Millicelsius(-563),
                    arr![u8; 0b0001_1111, 0b1111_0111].into()
                );
                assert_eq!(
                    Millicelsius(-688),
                    arr![u8; 0b0001_1111, 0b1111_0101].into()
                );
            }

            #[test]
            fn slightly_below_zero_celsius() {
                assert_eq!(Celsius(-0.063), arr![u8; 0b0001_1111, 0b1111_1111].into());
                assert_eq!(Celsius(-0.125), arr![u8; 0b0001_1111, 0b1111_1110].into());
                assert_eq!(Celsius(-0.250), arr![u8; 0b0001_1111, 0b1111_1100].into());
                assert_eq!(Celsius(-0.563), arr![u8; 0b0001_1111, 0b1111_0111].into());
                assert_eq!(Celsius(-0.688), arr![u8; 0b0001_1111, 0b1111_0101].into());
            }

            #[test]
            fn above_zero_millicelsius() {
                assert_eq!(
                    Millicelsius(25250),
                    arr![u8; 0b0000_00001, 0b1001_0100].into()
                );
                assert_eq!(
                    Millicelsius(57375),
                    arr![u8; 0b0000_00011, 0b1001_0110].into()
                );
            }

            #[test]
            fn above_zero_celsius() {
                assert_eq!(Celsius(25.250), arr![u8; 0b0000_00001, 0b1001_0100].into());
                assert_eq!(Celsius(57.375), arr![u8; 0b0000_00011, 0b1001_0110].into());
            }

            #[test]
            fn below_zero_millicelsius() {
                assert_eq!(
                    Millicelsius(-24063),
                    arr![u8; 0b0000_11110, 0b0111_1111].into()
                );
                assert_eq!(
                    Millicelsius(-56375),
                    arr![u8; 0b0000_11100, 0b0111_1010].into()
                );
            }

            #[test]
            fn below_zero_celsius() {
                assert_eq!(Celsius(-24.063), arr![u8; 0b0000_11110, 0b0111_1111].into());
                assert_eq!(Celsius(-56.375), arr![u8; 0b0000_11100, 0b0111_1010].into());
            }
        }
    }
}
