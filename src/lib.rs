use std::marker::PhantomData;

pub use device_register_impl::*;

/// Trait of a register containing an address
pub trait Register {
    /// Type of the adress
    type Address;

    /// The address of the register
    const ADDRESS: Self::Address;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {}

pub trait RegisterInterface<R, A>
where
    R: Register<Address = A>,
{
    fn read_register(&mut self) -> Result<R, Error>;

    fn write_register(&mut self, register: &R) -> Result<(), Error>;
}

pub struct RegisterReader<I, R, A>
where
    R: Register<Address = A>,
    I: RegisterInterface<R, A>,
{
    interface: I,
    register: PhantomData<R>,
    address: PhantomData<A>,
}

impl<I, R, A> RegisterReader<I, R, A>
where
    R: Register<Address = A>,
    I: RegisterInterface<R, A>,
{
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            register: PhantomData,
            address: PhantomData,
        }
    }

    pub fn read(&mut self) -> Result<R, Error>
    where
        R: ReadableRegister,
    {
        self.interface.read_register()
    }

    pub fn write(&mut self, register: R) -> Result<(), Error>
    where
        R: WritableRegister,
    {
        self.interface.write_register(&register)
    }

    pub fn edit<F>(&mut self, f: F) -> Result<(), Error>
    where
        R: EditableRegister,
        for<'w> F: FnOnce(&'w mut R) -> &'w mut R,
    {
        let mut reg = self.interface.read_register()?;
        f(&mut reg);
        self.interface.write_register(&reg)
    }
}

/// Trait of a read only  register
pub trait ReadableRegister: Register {}

/// Trait of a register that can only be edited.
/// Some registers require a read-edit-write operation since some bits a reserved internally
/// Editing a register allows to `safely` modify only a subset of values
pub trait EditableRegister: Register {}

/// Trait a writable register, like a register but can be written to
pub trait WritableRegister: Register {}
