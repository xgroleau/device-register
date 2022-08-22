# device-register

[![crates.io](https://img.shields.io/crates/v/device-register)](https://crates.io/crates/device-register) [![documentation](https://docs.rs/device-register/badge.svg)](https://docs.rs/device-register)

A zero cost toolkit to describe the register of your devies to ease driver development with `no_std` support.
* `no_std` support
* Zero cost, no use of dyn
* No dsl, just a derive macro and impl a trait.

### Usage
Simply derive using `XXRegister`, where XX is the premission.
The following permissions are supported
* [`RORegister`](crate::RORegister), read only permission
* [`WORegister`](crate::WORegister), write only permission
* [`EORegister`](crate::EORegister), edit only permission, when a register need to be read-modify-write
* [`RERegister`](crate::RERegister), Read and edit permission
* [`RWRegister`](crate::RWRegister), Read, write and edit permission.

To define a register, simply derive using the desired permission.
Then use the `register` attribute to define it's address, type for the address and the error type.
```rust

#[derive(RWRegister)]
#[register( addr = "42", ty = "u8", err = "DeviceError" )]
pub struct Register0(pub u16);
```
The your driver only need to implement the [RegisterInterface](crate::RegisterInterface)

#### Completed example
Here is a complete example. See the `tests` folder for more.
```rust
use std::collections::HashMap;
use device_register::*;

// The type of the address used by the driver
struct Address(pub u8);

// The type of the error
type DeviceError = ();

// We define the register with Read/Write permission
// Then we pass the address type, value and error type of the driveer
#[derive(Debug, Copy, PartialEq, Eq, Clone, RWRegister)]
#[register( addr = "Address(1)", ty = "Address", err = "DeviceError" )]
struct Register0(pub u16);

// Mock of the device driver
struct DeviceDriver {
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
// We can the Read/Write/Edit the registers that uses the Address and DeviceError types.
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
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


License: MIT OR Apache-2.0
