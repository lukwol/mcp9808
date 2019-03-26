#![no_std]
// #![allow(dead_code)]

#[derive(Debug)]
struct Address(u8);

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        address.0
    }
}

trait AddressPin {
    fn bit() -> bool;
}

#[derive(Debug)]
struct A0Pin;

#[derive(Debug)]
struct A1Pin;

#[derive(Debug)]
struct A2Pin;

#[derive(Debug)]
struct PinConfiguration {
    a0_pin: Option<A0Pin>,
    a1_pin: Option<A1Pin>,
    a2_pin: Option<A2Pin>,
}

#[derive(Debug)]
struct PinConfigurableAddress {
    address: Address,
    configuration: PinConfiguration,
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
