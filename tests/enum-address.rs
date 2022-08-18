use std::collections::HashMap;

use device_register::*;

pub enum SomeAddress {
    SomeRegister = 0x42,
    SomeOtherRegister = 0x45,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, RWRegister)]
#[register(addr = "SomeAddress::SomeRegister", ty = "SomeAddress")]
pub struct SomeRegister(pub u16);
impl From<SomeRegister> for u16 {
    fn from(val: SomeRegister) -> Self {
        val.0
    }
}
impl From<u16> for SomeRegister {
    fn from(val: u16) -> Self {
        SomeRegister(val)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, RWRegister)]
#[register(addr = "SomeAddress::SomeOtherRegister", ty = "SomeAddress")]
pub struct SomeOtherRegister(pub u16);
impl From<SomeOtherRegister> for u16 {
    fn from(val: SomeOtherRegister) -> Self {
        val.0
    }
}
impl From<u16> for SomeOtherRegister {
    fn from(val: u16) -> Self {
        SomeOtherRegister(val)
    }
}

// Mock of the device driver
pub struct DeviceDriver {
    // Simulate reading from the device
    pub registers: HashMap<u8, [u8; 2]>,
}
impl DeviceDriver {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(SomeRegister::ADDRESS as u8, [0, 0]);
        registers.insert(SomeOtherRegister::ADDRESS as u8, [0, 0]);
        Self { registers }
    }
}

// Implementation of the interface
impl<R> RegisterInterface<R, SomeAddress> for DeviceDriver
where
    R: Register<Address = SomeAddress> + Clone + From<u16>,
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

    let some: SomeRegister = device.read().unwrap();
    let other: SomeOtherRegister = device.read().unwrap();

    assert_eq!(u16::from(some), 0);
    assert_eq!(u16::from(other), 0);

    device
        .edit(|r: &mut SomeRegister| {
            r.0 = 42;
            r
        })
        .unwrap();

    device
        .edit(|r: &mut SomeOtherRegister| {
            r.0 = 0x42;
            r
        })
        .unwrap();

    // let val = device
    //     .registers
    //     .get(&(SomeRegister::ADDRESS as u8))
    //     .unwrap();
    // let reg: SomeRegister = device.read_register().unwrap();
    // let reg: SomeOtherRegister = device.read_register().unwrap();
}
