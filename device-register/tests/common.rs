use std::collections::HashMap;

pub const REGISTER1: u8 = 0x01;
pub const REGISTER2: u8 = 0x02;

// Mock of the device driver
pub struct DeviceDriver {
    // Simulate reading from the device
    pub registers: HashMap<u8, [u8; 2]>,
}
impl DeviceDriver {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(REGISTER1, [0, 0]);
        registers.insert(REGISTER2, [0, 0]);
        Self { registers }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    Get,
}
