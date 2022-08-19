//! Note that using a primitive as an address is not recommended since it would allow other libraries to also implement using the same primitive
//! Then the user could use the registers from one librare with a device of the other.  Using newtypes is recommended
mod common;

use common::{DeviceDriver, DeviceError};
use device_register_async::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, RORegister)]
#[register(addr = "common::REGISTER1", err = "DeviceError")] // No need to specify  the type since it's u8  by default
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
#[derive(Debug, Clone, Copy, RORegister)]
#[register(addr = "common::REGISTER2", err = "DeviceError")]
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

// Implementation of the interface for a u8
impl<R> RegisterInterface<R, u8, DeviceError> for DeviceDriver
where
    R: Register<Address = u8, Error = DeviceError> + Clone + From<u16>,
    u16: From<R>,
{
    fn read_register(&mut self) -> Result<R, DeviceError> {
        let bytes = self.registers.get(&(R::ADDRESS)).ok_or(DeviceError::Get)?;
        let reg = u16::from_be_bytes(bytes.clone());
        Ok(reg.into())
    }

    fn write_register(&mut self, register: &R) -> Result<(), DeviceError> {
        let bytes: u16 = register.clone().into();
        self.registers.insert(R::ADDRESS, bytes.to_be_bytes());
        Ok(())
    }
}

#[test]
fn read_edit() {
    let mut device = DeviceDriver::new();
    device
        .registers
        .insert(Register1::ADDRESS, 0x42_u16.to_be_bytes());
    device
        .registers
        .insert(Register2::ADDRESS, 0x45_u16.to_be_bytes());

    let reg1: Register1 = device.read().unwrap();
    let reg2: Register2 = device.read().unwrap();

    assert(false);
    assert_eq!(u16::from(reg2), 0x45);
}
