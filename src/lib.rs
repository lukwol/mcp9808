#![no_std]
#![allow(dead_code)]

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Address(u8);

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        address.0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum AddressPin {
    A0 = 1,
    A1 = 1 << 1,
    A2 = 1 << 2,
}

#[derive(Debug, Default)]
struct PinConfiguration {
    a0_pin_enabled: bool,
    a1_pin_enabled: bool,
    a2_pin_enabled: bool,
}

#[derive(Debug)]
struct PinConfigurableAddress {
    default_address: Address,
    configuration: Option<PinConfiguration>,
}

impl PinConfigurableAddress {
    fn configured_address(&self) -> Address {
        let mut address_ptr: u8 = self.default_address.into();
        if let Some(config) = &self.configuration {
            if config.a0_pin_enabled {
                address_ptr |= AddressPin::A0 as u8;
            }
            if config.a1_pin_enabled {
                address_ptr |= AddressPin::A1 as u8;
            }
            if config.a2_pin_enabled {
                address_ptr |= AddressPin::A2 as u8;
            }
        }
        Address(address_ptr)
    }
}

trait Register {
    fn address(&self) -> Address;
}

trait WriteRegister: Register {
    type WriteValue;
}

trait ReadRegister: Register {
    type ReadValue;
}

#[derive(Debug)]
struct TemperatureRegister;

impl Register for TemperatureRegister {
    fn address(&self) -> Address {
        Address(0x5)
    }
}

// impl WriteRegister for TemperatureRegister {}

// impl ReadRegister for TemperatureRegister {}
