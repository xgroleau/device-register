//! [![crates.io](https://img.shields.io/crates/v/device-register)](https://crates.io/crates/device-register) [![documentation](https://docs.rs/device-register/badge.svg)](https://docs.rs/device-register)
//!
//! A `no_std`, zero cost toolkit to describe the registers of a device to ease driver development.
//! * `no_std` support
//! * Zero cost, no use of dyn
//! * No dsl, just a derive macro and impl a trait.
//! * Error passthrough
//!
//! ## Usage
//! Simply derive using `XXRegister`, where XX is the premission.
//! The following permissions are supported
//! * [`RORegister`](crate::RORegister), read only permission
//! * [`WORegister`](crate::WORegister), write only permission
//! * [`EORegister`](crate::EORegister), edit only permission, when a register need to be read-modify-write
//! * [`RERegister`](crate::RERegister), Read and edit permission
//! * [`RWRegister`](crate::RWRegister), Read, write and edit permission.
//!
//! To define a register, simply derive using the desired permission.
//!
//! Then use the `register` attribute to define it's address, type for the address and the error.
//!
//!
//! ```rust
//! # use device_register::*;
//! # pub type DeviceError = ();
//!
//! #[derive(RWRegister)]
//! #[register( addr = "42", ty = "u8", err = "DeviceError" )]
//! pub struct Register0(pub u16);
//! ```
//! Then, your driver only need to implement the [RegisterInterface](crate::RegisterInterface) to have access to the read/write/edit traits.
//!
//! ### Complete example
//! Here is a complete example.
//! See the `tests` folder for more, or checkout the [tmp117](https://github.com/xgroleau/tmp117-rs) driver for actual usage.
//!
//! ```rust
//! use std::collections::HashMap;
//! use device_register::*;
//!
//! // The type of the address used by the driver
//! struct Address(pub u8);
//!
//! // The type of the error, lets have none for now,
//! type DeviceError = ();
//!
//! // We define the register with Read/Write permission
//! // Then we pass the address type, value and error type of the driveer
//! #[derive(Debug, Copy, PartialEq, Eq, Clone, RWRegister)]
//! #[register( addr = "Address(1)", ty = "Address", err = "DeviceError" )]
//! struct Register0(pub u16);
//! # impl From<Register0> for u16 {
//! #     fn from(val: Register0) -> Self {
//! #        val.0
//! #     }
//! # }
//! # impl From<u16> for Register0 {
//! #     fn from(val: u16) -> Self {
//! #         Register0(val)
//! #     }
//! # }
//!
//! // Mock of the device driver
//! struct DeviceDriver {
//!     // Simulate reading from the device
//!     pub registers: HashMap<u8, u16>,
//! }
//!
//! // Implement a method directly, by passing the trait for specific usecases like async
//! impl DeviceDriver {
//!     pub async fn read_async<R>(&self) -> R
//!     where
//!         R: ReadableRegister<Address = Address> + From<u16>,
//!     {
//!         async {
//!             let bytes = self.registers.get(&R::ADDRESS.0).unwrap();
//!             bytes.clone().into()
//!         }.await
//!     }
//! }
//!
//!
//! // We implement the required interface
//! impl<R> RegisterInterface<R, Address, DeviceError> for DeviceDriver
//! where
//!     R: Register<Address = Address, Error = DeviceError> + Clone + From<u16>,
//!     u16: From<R>,
//! {
//!     fn read_register(&mut self) -> Result<R, DeviceError> {
//!         let bytes = self.registers.get(&R::ADDRESS.0).unwrap();
//!         Ok(bytes.clone().into())
//!     }
//!
//!     fn write_register(&mut self, register: &R) -> Result<(), DeviceError> {
//!         self.registers.insert(R::ADDRESS.0, register.clone().into());
//!         Ok(())
//!     }
//! }
//!
//! let mut device = DeviceDriver{
//!     registers:  HashMap::new(),
//! };
//! // We can the Read/Write/Edit the registers that uses the Address and DeviceError types.
//! let write = Register0(42);
//! device.write(write).unwrap();
//!
//! let read: Register0 = device.read().unwrap();
//!
//! assert_eq!(read, write);
//!
//! device.edit(|r: &mut Register0| {
//!     r.0 = 43;
//!     r
//! }).unwrap();
//!
//!
//! let read: Register0 = device.read().unwrap();
//! assert_eq!(read, Register0(43));
//!
//! // Custom implementation, async is an example of usecase for custom implements
//! tokio_test::block_on( async {
//!     let read_async: Register0 = device.read_async().await;
//!     assert_eq!(read, Register0(43));
//! } );
//!
//! ```
//!
//!
//! ## License
//! Licensed under either of
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <http://www.apache.org/licenses/LICENSE-2.0>)
//!
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Contribution
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
#![no_std]
#![deny(unsafe_code, missing_docs)]

pub use device_register_macro::*;

/// Trait of a register containing an address
pub trait Register {
    /// Type of the adress, can be used to constrain the registers accepted
    type Address;

    /// The error type for the read/write of the register
    type Error;

    /// The address of the register
    const ADDRESS: Self::Address;
}

/// Trait of a read only  register
pub trait ReadableRegister: Register {}

/// Trait of a register that can only be edited.
/// Some registers require a read-edit-write operation since some bits a reserved internally
/// Editing a register allows to "safely" modify only a subset of values
pub trait EditableRegister: Register {}

/// Trait a writable register, like a register but can be written to
pub trait WritableRegister: Register {}

/// Traits that define how to read and write the registers.
/// Note that those functions should mostly just be implemented and not used since they are not bound by Read/Write/Edit permission.
pub trait RegisterInterface<R, A, E>
where
    R: Register<Address = A, Error = E>,
{
    /// Reads a register and returns it
    fn read_register(&mut self) -> Result<R, R::Error>;

    /// Writes a register to the device
    fn write_register(&mut self, register: &R) -> Result<(), R::Error>;
}

/// Trait to safely read a register. Only a readable register can be read.
pub trait ReadRegister<R, A, E>
where
    R: ReadableRegister<Address = A, Error = E>,
{
    /// Read a register
    fn read(&mut self) -> Result<R, R::Error>;
}

/// Trait to safely write a register. Only a writable register can be written to.
pub trait WriteRegister<R, A, E>
where
    R: WritableRegister<Address = A, Error = E>,
{
    /// Write a register
    fn write(&mut self, register: R) -> Result<(), R::Error>;
}

/// Trait to safely read-edit-write a register.
/// Usefull when a register has reserved values for internal uses.
/// Avoids writing garbage to the reserved  bits.
pub trait EditRegister<R, A, E>
where
    R: EditableRegister<Address = A, Error = E>,
{
    /// Edit a register. The closure takes a reference to the register,
    /// the same register must be edited, then returned.
    fn edit<F>(&mut self, f: F) -> Result<(), R::Error>
    where
        for<'w> F: FnOnce(&'w mut R) -> &'w mut R;
}

impl<I, R, A, E> ReadRegister<R, A, E> for I
where
    R: ReadableRegister<Address = A, Error = E>,
    I: RegisterInterface<R, A, E>,
{
    fn read(&mut self) -> Result<R, R::Error> {
        self.read_register()
    }
}

impl<I, R, A, E> WriteRegister<R, A, E> for I
where
    R: WritableRegister<Address = A, Error = E>,
    I: RegisterInterface<R, A, E>,
{
    fn write(&mut self, register: R) -> Result<(), R::Error> {
        self.write_register(&register)
    }
}

impl<I, R, A, E> EditRegister<R, A, E> for I
where
    R: EditableRegister<Address = A, Error = E>,
    I: RegisterInterface<R, A, E>,
{
    fn edit<F>(&mut self, f: F) -> Result<(), R::Error>
    where
        for<'w> F: FnOnce(&'w mut R) -> &'w mut R,
    {
        let mut val = self.read_register()?;
        let val = f(&mut val);
        self.write_register(val)
    }
}
