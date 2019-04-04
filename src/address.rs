#[derive(Debug, Clone, Copy)]
pub enum Address {
    Default,
    Alternative(bool, bool, bool),
}

impl From<Address> for u8 {
    fn from(slave_address: Address) -> Self {
        let default_addr = 0b1_1000;
        match slave_address {
            Address::Default => default_addr,
            Address::Alternative(a2, a1, a0) => {
                default_addr | (a2 as u8) << 2 | (a1 as u8) << 1 | (a0 as u8)
            }
        }
    }
}
