//! Define the `Device` trait to abstract over devices with
//! which TAMProxy can communicate.

use crate::Packet;

pub trait Device {
    /// Initialize this device.
    fn new() -> Packet;
}