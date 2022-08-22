# device-register

## device-register
A toolkit to describe the register of your devies to ease driver development

### Example
```rust
use std::collections::HashMap;
use device_register::*;

// The type of the address used by the driver
pub struct Address(pub u8);

// The type of the error
pub type DeviceError = ();

// We define the register with Read/Write permission
// Then we pass the address type, value and error type of the driveer
#[derive(Debug, Copy, PartialEq, Eq, Clone, RWRegister)]
#[register( addr = "Address(1)", ty = "Address", err = "DeviceError" )]
pub struct Register0(pub u16);

// Mock of the device driver
pub struct DeviceDriver {
    // Simulate reading from the device
    pub registers: HashMap<u8, u16>,
}

// We implement the required interface
impl<R> RegisterInterface<R, Address, DeviceError> for DeviceDriver
where
    R: Register<Address = Address, Error = DeviceError> + Clone + From<u16>,
    u16: From<R>,
{
    fn read_register(&mut self) -> Result<R, DeviceError> {
        let bytes = self.registers.get(&R::ADDRESS.0).unwrap();
        Ok(bytes.clone().into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), DeviceError> {
        self.registers.insert(R::ADDRESS.0, register.clone().into());
        Ok(())
    }
}

let mut device = DeviceDriver{
    registers:  HashMap::new(),
};
let write = Register0(42);
device.write(write).unwrap();

let read: Register0 = device.read().unwrap();

assert_eq!(read, write);

device.edit(|r: &mut Register0| {
    r.0 = 43;
    r
}).unwrap();

let read: Register0 = device.read().unwrap();
assert_eq!(read, Register0(43));

```

### License
Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

License: MIT OR Apache-2.0
