#[path = "./common.rs"]
mod common;

use common::{DeviceDriver, DeviceError};
use device_register::*;

pub enum Address {
    Register1 = common::REGISTER1 as isize,
    Register2 = common::REGISTER2 as isize,
}

#[derive(Debug, Clone, Copy, EORegister)]
#[register(addr = "Address::Register1", ty = "Address")]
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

#[derive(Debug, Clone, Copy, EORegister)]
#[register(addr = "Address::Register2", ty = "Address")]
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
impl<R> RegisterInterface<R, Address> for DeviceDriver
where
    R: Register<Address = Address> + Clone + From<u16>,
    u16: From<R>,
{
    type Error = DeviceError;

    fn read_register(&mut self) -> Result<R, Self::Error> {
        let bytes = self
            .registers
            .get(&(R::ADDRESS as u8))
            .ok_or(DeviceError::Get)?;
        let reg = u16::from_be_bytes(*bytes);
        Ok(reg.into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), Self::Error> {
        let bytes: u16 = register.clone().into();
        self.registers.insert(R::ADDRESS as u8, bytes.to_be_bytes());
        Ok(())
    }
}

#[test]
fn edit_enum_addr() {
    let mut device = DeviceDriver::new();

    device
        .edit(|r: &mut Register1| {
            assert_eq!(r.0, 0);
            r.0 = 0x42;
            r
        })
        .unwrap();

    device
        .edit(|r: &mut Register2| {
            assert_eq!(r.0, 0);
            r.0 = 0x45;
            r
        })
        .unwrap();

    assert_eq!(
        device.registers.get(&common::REGISTER1).unwrap(),
        &0x42_u16.to_be_bytes()
    );

    assert_eq!(
        device.registers.get(&common::REGISTER2).unwrap(),
        &0x45_u16.to_be_bytes()
    );
}
