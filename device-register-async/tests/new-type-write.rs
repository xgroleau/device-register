#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
mod common;

use common::{DeviceDriver, DeviceError};
use device_register::{Register, WORegister};
use device_register_async::*;
use futures::Future;

pub struct Address(pub u8);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, WORegister)]
#[register(
    addr = "Address(common::REGISTER1)",
    ty = "Address",
    err = "DeviceError"
)]
pub struct Register1(pub u16);
impl From<Register1> for u16 {
    fn from(val: Register1) -> Self {
        val.0
    }
}
impl From<u16> for Register1 {
    fn from(val: u16) -> Self {
        Register1(val)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, WORegister)]
#[register(
    addr = "Address(common::REGISTER2)",
    ty = "Address",
    err = "DeviceError"
)]
pub struct Register2(pub u16);
impl From<Register2> for u16 {
    fn from(val: Register2) -> Self {
        val.0
    }
}
impl From<u16> for Register2 {
    fn from(val: u16) -> Self {
        Register2(val)
    }
}

// Implementation of the interface for this type of address
impl<R> RegisterInterface<R, Address, DeviceError> for DeviceDriver
where
    R: Register<Address = Address, Error = DeviceError> + Clone + From<u16>,
    u16: From<R>,
{
    type ReadOutput<'a> = impl Future<Output = Result<R, R::Error>>
    where
        Self: 'a ;

    fn read_register<'a>(&'a mut self) -> Self::ReadOutput<'a> {
        async {
            let bytes = self
                .registers
                .get(&(R::ADDRESS.0))
                .ok_or(DeviceError::Get)?;
            let reg = u16::from_be_bytes(bytes.clone());
            Ok(reg.into())
        }
    }

    type WriteOutput<'a> = impl Future<Output = Result<(), R::Error>>
    where
        Self: 'a,
        R: 'a;

    fn write_register<'a>(&'a mut self, register: &'a R) -> Self::WriteOutput<'a> {
        async {
            let bytes: u16 = register.clone().into();
            self.registers.insert(R::ADDRESS.0, bytes.to_be_bytes());
            Ok(())
        }
    }
}

#[tokio::test]
async fn write_newtype_addr() {
    let mut device = DeviceDriver::new();

    device.write(Register1(0x42)).await.unwrap();
    device.write(Register2(0x45)).await.unwrap();

    assert_eq!(
        device.registers.get(&common::REGISTER1).unwrap(),
        &0x42_u16.to_be_bytes()
    );
    assert_eq!(
        device.registers.get(&common::REGISTER2).unwrap(),
        &0x45_u16.to_be_bytes()
    );
}
