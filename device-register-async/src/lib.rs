//! [![crates.io](https://img.shields.io/crates/v/device-register-async)](https://crates.io/crates/device-register-async) [![documentation](https://docs.rs/device-register-async/badge.svg)](https://docs.rs/device-register-async)
//!
//! An async version of the trait from the crate [device-register](device_register)
//! Note that you will need to use nightly and
//! enable `generic_associated_types` and `type_alias_impl_trait` features.
#![no_std]
#![deny(unsafe_code, missing_docs)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub use device_register;
use device_register::{EditableRegister, ReadableRegister, Register, WritableRegister};
use futures::Future;

/// Traits that define how to read and write the registers.
/// Note that those functions should mostly just be implemented and not used since they are not bound by Read/Write/Edit permission.
pub trait RegisterInterface<R, A, E>
where
    R: Register<Address = A, Error = E>,
{
    /// The return type of the read_register function
    type ReadOutput<'a>: Future<Output = Result<R, E>>
    where
        Self: 'a;

    /// Reads a register and returns it
    fn read_register(&mut self) -> Self::ReadOutput<'_>;

    /// The return type of the write_register function
    type WriteOutput<'a>: Future<Output = Result<(), E>>
    where
        Self: 'a,
        R: 'a;

    /// Writes a register to the device
    fn write_register<'a>(&'a mut self, register: &'a R) -> Self::WriteOutput<'a>;
}

/// Trait to safely read a register. Only a readable register can be read.
pub trait ReadRegister<R, A, E>
where
    R: ReadableRegister<Address = A, Error = E>,
{
    /// The return type of the read
    type Output<'a>: Future<Output = Result<R, E>>
    where
        Self: 'a;

    /// Read a register
    fn read(&mut self) -> Self::Output<'_>;
}

/// Trait to safely write a register. Only a writable register can be written to.
pub trait WriteRegister<R, A, E>
where
    R: WritableRegister<Address = A, Error = E>,
{
    /// The return type of the write
    type Output<'a>: Future<Output = Result<(), E>>
    where
        Self: 'a,
        R: 'a;

    /// Write a register
    fn write(&mut self, register: R) -> Self::Output<'_>;
}

/// Trait to safely read-edit-write a register.
/// Usefull when a register has reserved values for internal uses.
/// Avoids writing garbage to the reserved  bits.
pub trait EditRegister<R, A, E>
where
    for<'a> R: EditableRegister<Address = A, Error = E> + 'a,
{
    /// The return type of the write
    type Output<'a, F>: Future<Output = Result<(), E>>
    where
        Self: 'a,
        F: FnOnce(R) -> R + 'a;

    /// Edit a register. The closure takes a reference to the register,
    /// the same register must be edited, then returned.
    fn edit<'a, F>(&'a mut self, f: F) -> Self::Output<'a, F>
    where
        F: FnOnce(R) -> R + 'a;
}

impl<I, R, A, E> ReadRegister<R, A, E> for I
where
    for<'a> R: ReadableRegister<Address = A, Error = E> + 'a,
    I: RegisterInterface<R, A, E>,
    for<'a> A: 'a,
    for<'a> E: 'a,
{
    type Output<'a> = impl Future<Output = Result<R, E>> +'a where Self: 'a;

    fn read(&mut self) -> Self::Output<'_> {
        self.read_register()
    }
}

impl<I, R, A, E> WriteRegister<R, A, E> for I
where
    for<'a> R: WritableRegister<Address = A, Error = E> + 'a,
    I: RegisterInterface<R, A, E>,
    for<'a> A: 'a,
    for<'a> E: 'a,
{
    type Output<'a> = impl Future<Output = Result<(), E>> +'a where Self: 'a, R: 'a;

    fn write(&mut self, register: R) -> Self::Output<'_> {
        async move { self.write_register(&register).await }
    }
}

impl<I, R, A, E> EditRegister<R, A, E> for I
where
    for<'a> R: EditableRegister<Address = A, Error = E> + 'a,
    I: RegisterInterface<R, A, E>,
    for<'a> A: 'a,
    for<'a> E: 'a,
{
    type Output<'a, F> = impl Future<Output = Result<(), E>> +'a where
        Self: 'a,
        F: FnOnce(R) -> R + 'a,
    ;

    fn edit<'a, F>(&'a mut self, f: F) -> Self::Output<'a, F>
    where
        F: FnOnce(R) -> R + 'a,
    {
        async {
            let val = self.read_register().await?;
            let res = f(val);
            self.write_register(&res).await
        }
    }
}
