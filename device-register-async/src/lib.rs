//! Async version of device-register-async. Must be used with rust unstable
#![no_std]
#![deny(unsafe_code, missing_docs)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

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
    fn read_register<'a>(&mut self) -> Self::ReadOutput<'a>;

    /// The return type of the write_register function
    type WriteOutput<'a>: Future<Output = Result<(), E>>
    where
        Self: 'a;

    /// Writes a register to the device
    fn write_register<'a>(&mut self, register: &R) -> Self::WriteOutput<'a>;
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
    fn read<'a>(&mut self) -> Self::Output<'a>;
}

/// Trait to safely write a register. Only a writable register can be written to.
pub trait WriteRegister<R, A, E>
where
    R: WritableRegister<Address = A, Error = E>,
{
    /// The return type of the write
    type Output<'a>: Future<Output = Result<(), E>>
    where
        Self: 'a;

    /// Write a register
    fn write<'a>(&mut self, register: R) -> Self::Output<'a>;
}

/// Trait to safely read-edit-write a register.
/// Usefull when a register has reserved values for internal uses.
/// Avoids writing garbage to the reserved  bits.
pub trait EditRegister<R, A, E>
where
    for<'a> R: EditableRegister<Address = A, Error = E> + 'a,
{
    /// The return type of the write
    type Output<'a>: Future<Output = Result<(), E>>
    where
        Self: 'a;

    /// Edit a register. The closure takes a reference to the register,
    /// the same register must be edited, then returned.
    fn edit<'a: 'w, 'w, F>(&'a mut self, f: F) -> Self::Output<'a>
    where
        F: FnOnce(&'w mut R) -> &'w R + 'a;
}

impl<I, R, A, E> ReadRegister<R, A, E> for I
where
    for<'a> R: ReadableRegister<Address = A, Error = E> + 'a,
    I: RegisterInterface<R, A, E>,
    for<'a> A: 'a,
    for<'a> E: 'a,
{
    type Output<'a> = impl Future<Output = Result<R, E>> +'a where Self: 'a;

    fn read<'a>(&mut self) -> Self::Output<'a> {
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
    type Output<'a> = impl Future<Output = Result<(), E>> +'a where Self: 'a;

    fn write<'a>(&mut self, register: R) -> Self::Output<'a> {
        self.write_register(&register)
    }
}

impl<I, R, A, E> EditRegister<R, A, E> for I
where
    for<'a> R: EditableRegister<Address = A, Error = E> + 'a,
    I: RegisterInterface<R, A, E>,
    for<'a> A: 'a,
    for<'a> E: 'a,
{
    type Output<'a> = impl Future<Output = Result<(), E>> +'a where Self: 'a;

    fn edit<'a: 'w, 'w, F>(&'a mut self, f: F) -> Self::Output<'a>
    where
        F: FnOnce(&'w mut R) -> &'w R + 'a,
    {
        async {
            let mut val = self.read_register().await?;
            let res = f(&mut val);
            self.write_register(&res).await
        }
    }
}
