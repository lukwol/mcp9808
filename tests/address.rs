#[cfg(test)]
mod address {
    use mcp9808::SlaveAddress;

    #[test]
    fn default_address() {
        assert_eq!(0b1_1000u8, SlaveAddress::Default.into());
    }

    #[cfg(test)]
    mod alternative_address {
        use mcp9808::SlaveAddress;

        #[test]
        fn no_pins_enabled() {
            assert_eq!(
                SlaveAddress::Default,
                SlaveAddress::Alternative {
                    a2: false,
                    a1: false,
                    a0: false
                }
            );
        }

        #[test]
        fn configured_with_pins() {
            assert_eq!(
                0b1_1001u8,
                SlaveAddress::Alternative {
                    a2: false,
                    a1: false,
                    a0: true
                }
                .into()
            );
            assert_eq!(
                0b1_1010u8,
                SlaveAddress::Alternative {
                    a2: false,
                    a1: true,
                    a0: false
                }
                .into()
            );
            assert_eq!(
                0b1_1100u8,
                SlaveAddress::Alternative {
                    a2: true,
                    a1: false,
                    a0: false
                }
                .into()
            );
            assert_eq!(
                0b1_1101u8,
                SlaveAddress::Alternative {
                    a2: true,
                    a1: false,
                    a0: true
                }
                .into()
            );
            assert_eq!(
                0b1_1110u8,
                SlaveAddress::Alternative {
                    a2: true,
                    a1: true,
                    a0: false
                }
                .into()
            );
            assert_eq!(
                0b1_1111u8,
                SlaveAddress::Alternative {
                    a2: true,
                    a1: true,
                    a0: true
                }
                .into()
            );
        }
    }
}
