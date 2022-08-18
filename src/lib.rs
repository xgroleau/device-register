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

    fn write_register(&mut self, register: R) -> Result<(), Error>;
}

/// Trait of a read only  register
pub trait ReadableRegister: Register {}

/// Trait of a register that can only be edited.
/// Some registers require a read-edit-write operation since some bits a reserved internally
/// Editing a register allows to `safely` modify only a subset of values
pub trait EditableRegister: Register {}

/// Trait a writable register, like a register but can be written to
pub trait WritableRegister: Register {}
