use std::collections::HashMap;
use std::u8;

use device_register::{RORegister, Register, RegisterInterface};
use modular_bitfield::bitfield;
pub enum AddressType {
    SomeRegister = 0x42,
    SomeOtherRegister = 0x45,
}

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, RORegister)]
#[register(addr = "AddressType::SomeRegister", ty = "AddressType")]

pub struct SomeRegister(u16);

#[bitfield]
#[repr(u16)]
#[derive(Debug, Clone, Copy, RORegister)]
#[register(addr = "AddressType::SomeOtherRegister", ty = "AddressType")]

pub struct SomeOtherRegister(u16);

pub struct DeviceDriver {
    // Simulate reading from the device
    pub registers: HashMap<u8, [u8; 2]>,
}

impl DeviceDriver {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(SomeRegister::ADDRESS as u8, [0, 0]);
        Self { registers }
    }
}
impl<R> RegisterInterface<R, AddressType> for DeviceDriver
where
    R: Register<Address = AddressType> + From<u16> + Clone,
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

fn main() {
    let mut device = DeviceDriver::new();
    let val = device
        .registers
        .get(&(SomeRegister::ADDRESS as u8))
        .unwrap();
    let reg: SomeRegister = device.read_register().unwrap();
    let reg: SomeOtherRegister = device.read_register().unwrap();
}
