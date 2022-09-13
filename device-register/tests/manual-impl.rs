//! Manual implementation of the read/write without using the traits
mod common;

use common::DeviceDriver;
use device_register::*;

#[derive(Debug, Clone, Copy, RORegister)]
#[register(addr = "common::REGISTER1")] // Using  default for address type and error type
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

#[derive(Debug, Clone, Copy, RORegister)]
#[register(addr = "common::REGISTER2")]
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

// Implement a method directly, by passing the trait for specific usecases like async
impl DeviceDriver {
    pub async fn read_async<R>(&self) -> R
    where
        R: ReadableRegister<Address = u8> + From<u16>,
    {
        async {
            let bytes = self.registers.get(&(R::ADDRESS)).unwrap();
            u16::from_be_bytes(*bytes).into()
        }
        .await
    }
}

#[test]
fn read_async() {
    let mut device = DeviceDriver::new();
    device
        .registers
        .insert(Register1::ADDRESS, 0x42_u16.to_be_bytes());
    device
        .registers
        .insert(Register2::ADDRESS, 0x45_u16.to_be_bytes());

    tokio_test::block_on(async {
        let reg1: Register1 = device.read_async().await;
        let reg2: Register2 = device.read_async().await;
        assert_eq!(u16::from(reg1), 0x42);
        assert_eq!(u16::from(reg2), 0x45);
    });
}
