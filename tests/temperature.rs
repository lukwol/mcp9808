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
            use super::{Celsius, Millicelsius};

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
}
