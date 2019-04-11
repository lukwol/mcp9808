# mcp9808
  Rust generic driver for MCP9808 temperature sensor.

## Example

```rust
use embedded_hal::blocking::i2c;
use mcp9808::{
    temperature::{Celsius, TemperatureMeasurement},
    SlaveAddress, MCP9808,
};

let mut mcp9808 = MCP9808::new(i2c, SlaveAddress::Default);
let temperature: TemperatureMeasurement<Celsius> = mcp9808.read_ambient_temperature().unwrap();
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
