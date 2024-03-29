# device-register

[![crates.io](https://img.shields.io/crates/v/device-register)](https://crates.io/crates/device-register) [![documentation](https://docs.rs/device-register/badge.svg)](https://docs.rs/device-register)

A `no_std` library to describe the registers permissions of a device to ease driver development.
* `no_std` support
* Zero cost, no use of dyn
* No dsl, just a derive macro and impl a trait.
* Error passthrough

### Usage
Simply derive using `XXRegister`, where XX is the premission.
The following permissions are supported
* [`RORegister`](crate::RORegister), read only permission
* [`WORegister`](crate::WORegister), write only permission
* [`EORegister`](crate::EORegister), edit only permission, when a register need to be read-modify-write
* [`RERegister`](crate::RERegister), Read and edit permission
* [`RWRegister`](crate::RWRegister), Read, write and edit permission.

To define a register, simply derive using the desired permission.

Then use the `register` attribute to define it's address, type for the address and the error.


```rust
use device_register::*;

#[derive(RWRegister)]
#[register( addr = "42", ty = "u8")]
pub struct Register0(pub u16);
```
Then, your driver only need to implement the [RegisterInterface](crate::RegisterInterface) to have access to the read/write/edit traits.

#### Complete example
Here is a complete example.
See the `tests` folder for more, or checkout the [tmp117](https://github.com/xgroleau/tmp117-rs) driver for actual usage.

```no_run
use std::collections::HashMap;
use device_register::*;

// The type of the address used by the driver
struct Address(pub u8);

// We define the register with Read/Write permission
// Then we pass the address type, value and error type of the driveer
#[derive(Debug, Copy, PartialEq, Eq, Clone, RWRegister)]
#[register( addr = "Address(1)", ty = "Address")]
struct Register0(pub u16);

impl From<u16> for Register0 {
    fn from(value: u16) -> Self {
        Register0(value)
    }
}

impl From<Register0> for u16 {
    fn from(value: Register0) -> Self {
        value.0
    }
}

// Mock of the device driver
struct DeviceDriver {
    // Simulate reading from the device
    pub registers: HashMap<u8, u16>,
}

// Implement a method directly, by passing the trait for specific usecases like async
impl DeviceDriver {
    pub async fn read_async<R>(&self) -> R
    where
        R: ReadableRegister<Address = Address> + From<u16>,
    {
        async {
            let bytes = self.registers.get(&R::ADDRESS.0).unwrap();
            bytes.clone().into()
        }.await
    }
}


// We implement the required interface
impl<R> RegisterInterface<R, Address> for DeviceDriver
where
    R: Register<Address = Address> + Clone + From<u16>,
    u16: From<R>,
{
    // The type of the error, lets have none for now,
    type Error = ();

    fn read_register(&mut self) -> Result<R, Self::Error> {
        let bytes = self.registers.get(&R::ADDRESS.0).unwrap();
        Ok(bytes.clone().into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), Self::Error> {
        self.registers.insert(R::ADDRESS.0, register.clone().into());
        Ok(())
    }
}

let mut device = DeviceDriver{
    registers:  HashMap::new(),
};
// We can the Read/Write/Edit the registers that uses the Address type.
let write = Register0(42);
device.write(write).unwrap();

let read: Register0 = device.read().unwrap();

assert_eq!(read, write);

device.edit(|r: &mut Register0| {
    r.0 = 43;
}).unwrap();


let read: Register0 = device.read().unwrap();
assert_eq!(read, Register0(43));

// Custom implementation, async is an example of usecase for custom implements
tokio_test::block_on( async {
    let read_async: Register0 = device.read_async().await;
    assert_eq!(read, Register0(43));
} );

```


### MSRV
The minimum supported rust version is `1.75.0`, but previous versions might work with the library

### License
Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


License: MIT OR Apache-2.0
