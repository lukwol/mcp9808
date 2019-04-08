/// I2C device address
/// # Usage
///
/// ```
/// # use mcp9808::address::Address;
/// # use mcp9808::MCP9808;
/// # let i2c = ();
/// let mcp9808 = MCP9808::new(i2c, Address::Default);
/// assert_eq!(0b1_1000u8, mcp9808.address());
/// ```
///
/// ```
/// # use mcp9808::address::Address;
/// # use mcp9808::MCP9808;
/// # let i2c = ();
/// let mcp9808 = MCP9808::new(i2c, Address::Alternative { a2: true, a1: false, a0: true });
/// assert_eq!(0b1_1101u8, mcp9808.address());
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Address {
    /// Default slave address - all pins are disconnected
    Default,
    /// Alternative slave address with configurable pins A2, A1, A0
    Alternative { a2: bool, a1: bool, a0: bool },
}

impl From<Address> for u8 {
    fn from(slave_address: Address) -> Self {
        let default_addr = 0b1_1000;
        match slave_address {
            Address::Default => default_addr,
            Address::Alternative { a2, a1, a0 } => {
                default_addr | (a2 as u8) << 2 | (a1 as u8) << 1 | (a0 as u8)
            }
        }
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs): (u8, u8) = ((*self).into(), (*other).into());
        lhs == rhs
    }
}
