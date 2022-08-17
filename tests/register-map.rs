use std::collections::HashMap;
use std::u8;

use device_register::{RORegister, RegisterInterface, Register};
use device_register::modular_bitfield::{bitfield, BitfieldSpecifier};


#[bitfield]
#[derive(Debug, BitfieldSpecifier, RORegister)]
#[address(0x01)]
pub struct SomeRegister {
    pub val1: u8,
    pub val2: u8,
}

pub struct DeviceDriver {
    // Simulate reading from the device
    registers: HashMap<u8, [u8; 2]>,
}

impl DeviceDriver {
    pub fn new() -> Self{
        let registers = HashMap::new();
        registers.insert(SomeRegister::ADDRESS, [0, 0]);
        Self{ registers }
    }
    
}
impl<R> RegisterInterface<R, u8> for DeviceDriver where R: Register<Address = u8> {
    fn read_register(&mut self, value: &mut [u8]) -> Result<R, device_register::Error> {

        let bytes = self.registers.get(&R::ADDRESS).unwrap();
        let reg = R::from_bytes(bytes).unwrap();
        Ok(reg)
    }

    fn write_register(&mut self, register: R) -> Result<(), device_register::Error> {
        let bytes = R::into_bytes(register).unwrap();
        self.registers.insert(R::ADDRESS, bytes.into());
        Ok(())
    }
}
