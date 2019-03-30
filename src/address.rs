use i2c_reg::Address;

#[derive(Debug, Clone, Copy)]
pub enum SlaveAddress {
    Default,
    Alternative(bool, bool, bool),
}

impl From<SlaveAddress> for Address {
    fn from(slave_address: SlaveAddress) -> Self {
        let default_addr_ptr = 0b1_1000;
        match slave_address {
            SlaveAddress::Default => Address(default_addr_ptr),
            SlaveAddress::Alternative(a2, a1, a0) => {
                Address(default_addr_ptr | (a2 as u8) << 2 | (a1 as u8) << 1 | (a0 as u8))
            }
        }
    }
}
