use std::{future::Future, marker::PhantomData};

use crate::{EditableRegister, Error, ReadableRegister, Register, WritableRegister};

pub trait RegisterInterface<R, A>
where
    R: Register<Address = A>,
{
    type ReadOutput: Future<Output = Result<R, Error>>;
    fn read_register(&mut self) -> Self::ReadOutput;

    type WriteOutput: Future<Output = Result<(), Error>>;
    fn write_register(&mut self, register: &R) -> Self::WriteOutput;
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

    pub async fn read(&mut self) -> Result<R, Error>
    where
        R: ReadableRegister,
    {
        self.interface.read_register().await
    }

    pub async fn write(&mut self, register: R) -> Result<(), Error>
    where
        R: WritableRegister,
    {
        self.interface.write_register(&register).await
    }

    pub async fn edit<F>(&mut self, f: F) -> Result<(), Error>
    where
        R: EditableRegister,
        for<'w> F: FnOnce(&'w mut R) -> &'w mut R,
    {
        let mut reg = self.interface.read_register().await?;
        f(&mut reg);
        self.interface.write_register(&reg).await
    }

    pub fn free(self) -> I {
        self.interface
    }
}
