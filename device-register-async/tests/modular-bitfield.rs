#![allow(clippy::identity_op)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait, impl_trait_projections)]

#[path = "./common.rs"]
mod common;

use common::{DeviceDriver, DeviceError};
use device_register::{RWRegister, Register};
use device_register_async::*;
use modular_bitfield::bitfield;

pub struct Address(pub u8);

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, RWRegister)]
#[register(addr = "Address(common::REGISTER1)", ty = "Address")]
pub struct Register1 {
    pub field1: u8,
    pub field2: u8,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, RWRegister)]
#[register(addr = "Address(common::REGISTER2)", ty = "Address")]
pub struct Register2 {
    pub field1: u8,
    pub field2: u8,
}

// Implementation of the interface for this type of address
impl<R> RegisterInterface<R, Address> for DeviceDriver
where
    R: Register<Address = Address> + Clone + From<u16>,
    u16: From<R>,
{
    type Error = DeviceError;

    async fn read_register(&mut self) -> Result<R, Self::Error> {
        let bytes = self
            .registers
            .get(&(R::ADDRESS.0))
            .ok_or(DeviceError::Get)?;
        let reg = u16::from_be_bytes(*bytes);
        Ok(reg.into())
    }

    async fn write_register(&mut self, register: &R) -> Result<(), Self::Error> {
        let bytes: u16 = register.clone().into();
        self.registers.insert(R::ADDRESS.0, bytes.to_be_bytes());
        Ok(())
    }
}

#[tokio::test]
async fn modular_bitfield() {
    let mut device = DeviceDriver::new();

    let reg1 = Register1::new().with_field1(0x42).with_field2(0x43);
    let reg2 = Register2::new().with_field1(0x45).with_field2(0x46);

    device.write(reg1).await.unwrap();
    device.write(reg2).await.unwrap();

    let reg: Register1 = device.read().await.unwrap();
    assert_eq!(reg, reg1);
    let reg: Register2 = device.read().await.unwrap();
    assert_eq!(reg, reg2);

    device
        .edit(|r: &mut Register1| {
            r.set_field1(0);
            r.set_field2(0);
        })
        .await
        .unwrap();
    device
        .edit(|r: &mut Register2| {
            r.set_field1(0);
            r.set_field2(0);
        })
        .await
        .unwrap();

    let reg: Register1 = device.read().await.unwrap();
    assert_eq!(u16::from(reg), 0);
    let reg: Register2 = device.read().await.unwrap();
    assert_eq!(u16::from(reg), 0);
}
