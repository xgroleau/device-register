#[path = "./common.rs"]
mod common;

use common::*;
use device_register::*;

pub enum Address {
    Register1 = REGISTER1 as isize,
    Register2 = REGISTER2 as isize,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, RWRegister)]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, RWRegister)]
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
    fn read_register(&mut self) -> Result<R, device_register::Error> {
        let bytes = self.registers.get(&(R::ADDRESS as u8)).unwrap();
        let reg = u16::from_be_bytes(bytes.clone());
        Ok(reg.into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), device_register::Error> {
        let bytes: u16 = register.clone().into();
        self.registers.insert(R::ADDRESS as u8, bytes.to_be_bytes());
        Ok(())
    }
}

#[test]
fn read_edit() {
    let mut device = DeviceDriver::new();

    let some: Register1 = device.read().unwrap();
    let other: Register2 = device.read().unwrap();

    assert_eq!(u16::from(some), 0);
    assert_eq!(u16::from(other), 0);

    device
        .edit(|r: &mut Register1| {
            r.0 = 42;
            r
        })
        .unwrap();

    device
        .edit(|r: &mut Register2| {
            r.0 = 0x42;
            r
        })
        .unwrap();
}
