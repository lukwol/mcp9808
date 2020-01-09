/// MCP9808 Registers
pub enum Register {
    /// Read/write Configuration register (CONFIG)
    /// The MCP9808 has a 16-bit Configuration register (CONFIG) that allows the user
    /// to set various functions for a robust temperature monitoring system.
    ConfigurationRegister = 0b0001,

    /// Read/write Alert Temperature Upper Boundary Trip register (T_UPPER)
    /// Power-Up Default for T_UPPER is 0°C
    UpperTemperatureRegister = 0b0010,

    /// Read/write Alert Temperature Lower Boundary Trip register (T_LOWER)
    /// /// Power-Up Default for T_LOWER is 0°C
    LowerTemperatureRegister = 0b0011,

    /// Read/write Critical Temperature Trip register (T_CRIT)
    /// /// /// Power-Up Default for T_CRIT is 0°C
    CriticalTemperatureRegister = 0b0100,

    /// Read only Ambient temperature register (T_A)
    AmbientTemperatureRegister = 0b0101,

    /// Read only Manufacturer ID register.
    /// This register is used to identify the manufacturer of the
    /// device in order to perform manufacturer-specific
    /// operation.
    ManufacturerIdRegister = 0b0110,

    /// Read only Device ID/Revision register.
    /// The upper byte of this register is used to specify the
    /// device identification and the lower byte is used to
    /// specify the device revision.
    DeviceIdRevisionRegister = 0b0111,

    /// Read/Write Temperature resolution register
    /// This register allows the user to change the sensor resolution.
    /// The POR default resolution is +0.0625°C.
    /// The selected resolution is also reflected in the Capability register.
    ResolutionRegister = 0b1000,
}

impl Into<u8> for Register {
    fn into(self) -> u8 {
        self as u8
    }
}
