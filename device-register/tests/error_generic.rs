//! Note that using a primitive as an address is not recommended since it would allow other libraries to also implement using the same primitive
//! Then the user could use the registers from one librare with a device of the other.  Using newtypes is recommended
mod common;

use std::marker::PhantomData;

use common::DeviceDriver;
use device_register::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    Bus(E),
}

#[derive(Debug, Clone, Copy, RWRegister)]
#[register(addr = "common::REGISTER1", err = "Error<E>")]
pub struct Register1<E>(pub u16, PhantomData<E>);
impl<E> From<Register1<E>> for u16 {
    fn from(val: Register1<E>) -> Self {
        val.0
    }
}
impl<E> From<u16> for Register1<E> {
    fn from(val: u16) -> Self {
        Register1(val, PhantomData)
    }
}

#[derive(Debug, Clone, Copy, RWRegister)]
#[register(addr = "common::REGISTER2", err = "Error<E>")]
pub struct Register2<E>(pub u16, PhantomData<E>);
impl<E> From<Register2<E>> for u16 {
    fn from(val: Register2<E>) -> Self {
        val.0
    }
}
impl<E> From<u16> for Register2<E> {
    fn from(val: u16) -> Self {
        Register2(val, PhantomData)
    }
}

// Implementation of the interface for a u8
impl<R> RegisterInterface<R, u8, Error<u8>> for DeviceDriver
where
    R: Register<Address = u8, Error = Error<u8>> + Clone + From<u16>,
    u16: From<R>,
{
    fn read_register(&mut self) -> Result<R, Error<u8>> {
        let bytes = self.registers.get(&(R::ADDRESS)).ok_or(Error::Bus(1))?;
        let reg = u16::from_be_bytes(*bytes);
        Ok(reg.into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), Error<u8>> {
        let bytes: u16 = register.clone().into();
        self.registers.insert(R::ADDRESS, bytes.to_be_bytes());
        Ok(())
    }
}

#[test]
fn read_generic_error() {
    let mut device = DeviceDriver::new();
    device
        .registers
        .insert(common::REGISTER1, 0x42_u16.to_be_bytes());
    device
        .registers
        .insert(common::REGISTER2, 0x45_u16.to_be_bytes());

    let reg1: Register1<_> = device.read().unwrap();
    let reg2: Register2<_> = device.read().unwrap();

    // device.edit(|r: &mut Register1<_>| r).unwrap();

    assert_eq!(u16::from(reg1), 0x42);
    assert_eq!(u16::from(reg2), 0x45);
}
