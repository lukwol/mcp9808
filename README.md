# mcp9808
  Rust generic driver for MCP9808 temperature sensor.

## Features

* User-Programmable Temperature Alert Output
* Operating Voltage Range: 2.7V to 5.5V
* OperatingCurrent: 200μA (typical)
* ShutdownCurrent: 0.1μA (typical)
* 2-wire Interface: I2C™/SMBus Compatible
* Available Packages: 2x3DFN-8, MSOP-8

### Accuracy:
* ±0.25 (typical) from -40°C to +125°C
* ±0.5°C (maximum) from -20°C to 100°C
* ±1°C (maximum) from -40°C to +125°C

### User-Selectable Measurement Resolution:
* +0.5°C
* +0.25°C
* +0.125°C
* +0.0625°C

### User-Programmable Temperature Limits:
* Temperature Window Limit
* Critical Temperature Limit

## Example

```rust
use embedded_hal::blocking::i2c;
use mcp9808::{
    temperature::{Celsius, TemperatureMeasurement},
    SlaveAddress, MCP9808,
};

let mut mcp9808 = MCP9808::new(i2c, SlaveAddress::Default);
let measurement: TemperatureMeasurement<Celsius> = mcp9808.read_ambient_temperature().unwrap();
assert_eq!(Celsius(-24.063), measurement.temperature);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
