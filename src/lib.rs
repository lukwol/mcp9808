#![no_std]
#![allow(dead_code)]

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Address(u8);

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        address.0
    }
}

trait AddressPin {
    fn bit(&self) -> u8;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct A0Pin;

impl AddressPin for A0Pin {
    fn bit(&self) -> u8 {
        1
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct A1Pin;

impl AddressPin for A1Pin {
    fn bit(&self) -> u8 {
        1 << 1
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct A2Pin;

impl AddressPin for A2Pin {
    fn bit(&self) -> u8 {
        1 << 2
    }
}

#[derive(Debug)]
struct PinConfiguration {
    a0_pin: Option<A0Pin>,
    a1_pin: Option<A1Pin>,
    a2_pin: Option<A2Pin>,
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
            address_ptr |= config.a0_pin.map_or(0x0, |p| p.bit());
            address_ptr |= config.a1_pin.map_or(0x0, |p| p.bit());
            address_ptr |= config.a2_pin.map_or(0x0, |p| p.bit());
        }
        Address(address_ptr)
    }
}

trait Register {
    fn address(&self) -> Address;
}

trait WriteRegister: Register {}

trait WriteReadRegister: Register {}

#[derive(Debug)]
struct TemperatureRegister;

impl Register for TemperatureRegister {
    fn address(&self) -> Address {
        Address(0x5)
    }
}

impl WriteRegister for TemperatureRegister {}

impl WriteReadRegister for TemperatureRegister {}
