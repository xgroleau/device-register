#[path = "./common.rs"]
mod common;

use common::{DeviceDriver, DeviceError};
use device_register::*;

mod test {
    pub struct Address(pub u8);
}
// Verify that using module or the type directly that it works
use test::Address;

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
    addr = "test::Address(common::REGISTER2)",
    ty = "test::Address",
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
    fn read_register(&mut self) -> Result<R, DeviceError> {
        let bytes = self
            .registers
            .get(&(&R::ADDRESS.0))
            .ok_or(DeviceError::Get)?;
        let reg = u16::from_be_bytes(bytes.clone());
        Ok(reg.into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), DeviceError> {
        let bytes: u16 = register.clone().into();
        self.registers.insert(R::ADDRESS.0, bytes.to_be_bytes());
        Ok(())
    }
}

#[test]
fn write_newtype_addr() {
    let mut device = DeviceDriver::new();

    device.write(Register1(0x42)).unwrap();
    device.write(Register2(0x45)).unwrap();

    assert_eq!(
        device.registers.get(&common::REGISTER1).unwrap(),
        &0x42_u16.to_be_bytes()
    );
    assert_eq!(
        device.registers.get(&common::REGISTER2).unwrap(),
        &0x45_u16.to_be_bytes()
    );
}
