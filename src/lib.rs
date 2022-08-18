//! A library to ease the manipulation of writing register mapping for drivers
#![no_std]
#![deny(unsafe_code, missing_docs)]

use core::marker::PhantomData;

pub use device_register_impl::*;

/// Trait of a register containing an address
pub trait Register {
    /// Type of the adress, can be used to constrain the registers accepted
    type Address;

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

/// The possible error returned by the library
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {}

/// Traits that define how to read and write the registers
pub trait RegisterInterface<R, A>
where
    R: Register<Address = A>,
{
    /// Reads a register and returns it
    fn read_register(&mut self) -> Result<R, Error>;

    /// Writes a register to the device
    fn write_register(&mut self, register: &R) -> Result<(), Error>;
}

/// A struct to access the device's register.
pub struct DeviceAccessor<I, R, A>
where
    R: Register<Address = A>,
    I: RegisterInterface<R, A>,
{
    interface: I,
    register: PhantomData<R>,
    address: PhantomData<A>,
}

impl<I, R, A> DeviceAccessor<I, R, A>
where
    R: Register<Address = A>,
    I: RegisterInterface<R, A>,
{
    /// Creates a new instance
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            register: PhantomData,
            address: PhantomData,
        }
    }

    /// Read a register
    pub fn read(&mut self) -> Result<R, Error>
    where
        R: ReadableRegister,
    {
        self.interface.read_register()
    }

    /// Write a register
    pub fn write(&mut self, register: R) -> Result<(), Error>
    where
        R: WritableRegister,
    {
        self.interface.write_register(&register)
    }

    /// Edit a register
    pub fn edit<F>(&mut self, f: F) -> Result<(), Error>
    where
        R: EditableRegister,
        for<'w> F: FnOnce(&'w mut R) -> &'w mut R,
    {
        let mut reg = self.interface.read_register()?;
        f(&mut reg);
        self.interface.write_register(&reg)
    }

    /// Free the communication interface
    pub fn free(self) -> I {
        self.interface
    }
}
