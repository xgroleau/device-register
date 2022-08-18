use std::collections::HashMap;
use std::u8;

use modular_bitfield::bitfield;
use device_register::{RORegister, Register, RegisterInterface};

#[bitfield]
#[repr(u16)]
#[derive(Debug, RORegister)]
#[register(addr = 42)]
pub struct SomeRegister(u16);
pub struct DeviceDriver {
    // Simulate reading from the device
    pub registers: HashMap<u8, [u8; 2]>,
}

impl DeviceDriver {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(SomeRegister::ADDRESS, [0, 0]);
        Self { registers }
    }
}
impl<R> RegisterInterface<R, u8> for DeviceDriver
where
    R: Register<Address = u8> + From<u16>,
    u16: From<R>,
{
    fn read_register(&mut self) -> Result<R, device_register::Error> {
        let bytes = self.registers.get(&R::ADDRESS).unwrap();
        let reg = u16::from_be_bytes(bytes.clone());
        Ok(reg.into())
    }

    fn write_register(&mut self, register: R) -> Result<(), device_register::Error> {
        let bytes: u16 = register.into();
        self.registers.insert(R::ADDRESS, bytes.to_be_bytes());
        Ok(())
    }
}

fn main() {
    let mut device = DeviceDriver::new();
    let val = device.registers.get(&SomeRegister::ADDRESS).unwrap();
    let reg: SomeRegister = device.read_register().unwrap();
}
