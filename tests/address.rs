#[cfg(test)]
mod address {
    use mcp9808::address::Address;

    #[test]
    fn default_address() {
        assert_eq!(0b1_1000u8, Address::Default.into());
    }

    #[cfg(test)]
    mod alternative_address {
        use mcp9808::address::Address;

        #[test]
        fn no_pins_enabled() {
            assert_eq!(Address::Default, Address::Alternative(false, false, false));
        }

        #[test]
        fn configured_with_pins() {
            assert_eq!(0b1_1001u8, Address::Alternative(false, false, true).into());
            assert_eq!(0b1_1010u8, Address::Alternative(false, true, false).into());
            assert_eq!(0b1_1100u8, Address::Alternative(true, false, false).into());
            assert_eq!(0b1_1101u8, Address::Alternative(true, false, true).into());
            assert_eq!(0b1_1110u8, Address::Alternative(true, true, false).into());
            assert_eq!(0b1_1111u8, Address::Alternative(true, true, true).into());
        }
    }
}
