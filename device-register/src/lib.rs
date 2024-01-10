#![doc = include_str!("../README.md")]
#![no_std]
#![deny(unsafe_code, missing_docs)]

pub use device_register_macro::*;

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

/// Traits that define how to read and write the registers.
/// Note that those functions should mostly just be implemented and not used since they are not bound by Read/Write/Edit permission.
pub trait RegisterInterface<R, A>
where
    R: Register<Address = A>,
{
    /// Error type returned by the interface
    type Error;

    /// Reads a register and returns it
    fn read_register(&mut self) -> Result<R, Self::Error>;

    /// Writes a register to the device
    fn write_register(&mut self, register: &R) -> Result<(), Self::Error>;
}

/// Trait to safely read a register. Only a readable register can be read.
pub trait ReadRegister<R, A>
where
    R: ReadableRegister<Address = A>,
{
    /// Error type returned by reading the register
    type Error;

    /// Read a register
    fn read(&mut self) -> Result<R, Self::Error>;
}

/// Trait to safely write a register. Only a writable register can be written to.
pub trait WriteRegister<R, A>
where
    R: WritableRegister<Address = A>,
{
    /// Error type returned by writing the register
    type Error;

    /// Write a register
    fn write(&mut self, register: R) -> Result<(), Self::Error>;
}

/// Trait to safely read-edit-write a register.
/// Usefull when a register has reserved values for internal uses.
/// Avoids writing garbage to the reserved  bits.
pub trait EditRegister<R, A>
where
    R: EditableRegister<Address = A>,
{
    /// Error type returned by editing the register
    type Error;

    /// Edit a register. The closure takes a reference to the register,
    /// the same register must be edited, then returned.
    fn edit<F>(&mut self, f: F) -> Result<(), Self::Error>
    where
        for<'w> F: FnOnce(&'w mut R);
}

impl<I, R, A> ReadRegister<R, A> for I
where
    R: ReadableRegister<Address = A>,
    I: RegisterInterface<R, A>,
{
    type Error = I::Error;

    fn read(&mut self) -> Result<R, Self::Error> {
        self.read_register()
    }
}

impl<I, R, A> WriteRegister<R, A> for I
where
    R: WritableRegister<Address = A>,
    I: RegisterInterface<R, A>,
{
    type Error = I::Error;

    fn write(&mut self, register: R) -> Result<(), Self::Error> {
        self.write_register(&register)
    }
}

impl<I, R, A> EditRegister<R, A> for I
where
    R: EditableRegister<Address = A>,
    I: RegisterInterface<R, A>,
{
    type Error = I::Error;

    fn edit<F>(&mut self, f: F) -> Result<(), Self::Error>
    where
        for<'w> F: FnOnce(&'w mut R),
    {
        let mut val = self.read_register()?;
        f(&mut val);
        self.write_register(&val)
    }
}
