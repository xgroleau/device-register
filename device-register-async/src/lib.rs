//! [![crates.io](https://img.shields.io/crates/v/device-register-async)](https://crates.io/crates/device-register-async) [![documentation](https://docs.rs/device-register-async/badge.svg)](https://docs.rs/device-register-async)
//!
//! An async version of the trait from the crate [device-register](device_register)
//! Note that you will need to use nightly and
//! enable and `type_alias_impl_trait` features.
#![no_std]
#![deny(unsafe_code, missing_docs)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

pub use device_register;
use device_register::{EditableRegister, ReadableRegister, Register, WritableRegister};

/// Traits that define how to read and write the registers.
/// Note that those functions should mostly just be implemented and not used since they are not bound by Read/Write/Edit permission.
pub trait RegisterInterface<R, A>
where
    R: Register<Address = A>,
{
    /// The error type returned by the interface
    type Error;

    /// Reads a register and returns it
    async fn read_register(&mut self) -> Result<R, Self::Error>;

    /// Writes a register to the device
    async fn write_register(&mut self, register: &R) -> Result<(), Self::Error>;
}

/// Trait to safely read a register. Only a readable register can be read.
pub trait ReadRegister<R, A>
where
    R: ReadableRegister<Address = A>,
{
    /// The error type returned by reading a register
    type Error;

    /// Read a register
    async fn read(&mut self) -> Result<R, Self::Error>;
}

/// Trait to safely write a register. Only a writable register can be written to.
pub trait WriteRegister<R, A>
where
    R: WritableRegister<Address = A>,
{
    /// The error type returned by writing a register
    type Error;

    /// Write a register
    async fn write(&mut self, register: R) -> Result<(), Self::Error>;
}

/// Trait to safely read-edit-write a register.
/// Usefull when a register has reserved values for internal uses.
/// Avoids writing garbage to the reserved  bits.
pub trait EditRegister<R, A>
where
    for<'a> R: EditableRegister<Address = A> + 'a,
{
    /// The error type returned by editing a register
    type Error;

    /// Edit a register. The closure takes a reference to the register,
    /// the same register must be edited, then returned.
    async fn edit<F>(&mut self, f: F) -> Result<(), Self::Error>
    where
        for<'w> F: FnOnce(&'w mut R);
}

impl<I, R, A> ReadRegister<R, A> for I
where
    for<'a> R: ReadableRegister<Address = A> + 'a,
    I: RegisterInterface<R, A>,
    for<'a> A: 'a,
{
    type Error = I::Error;

    async fn read(&mut self) -> Result<R, Self::Error> {
        self.read_register().await
    }
}

impl<I, R, A> WriteRegister<R, A> for I
where
    for<'a> R: WritableRegister<Address = A> + 'a,
    I: RegisterInterface<R, A>,
    for<'a> A: 'a,
{
    type Error = I::Error;

    async fn write(&mut self, register: R) -> Result<(), Self::Error> {
        self.write_register(&register).await
    }
}

impl<I, R, A> EditRegister<R, A> for I
where
    for<'a> R: EditableRegister<Address = A> + 'a,
    I: RegisterInterface<R, A>,
    for<'a> A: 'a,
{
    type Error = I::Error;

    async fn edit<F>(&mut self, f: F) -> Result<(), Self::Error>
    where
        for<'w> F: FnOnce(&'w mut R),
    {
        let mut val = self.read_register().await?;
        f(&mut val);
        self.write_register(&val).await
    }
}
