//! Define the `Device` trait to abstract over devices with
//! which the Teensy can communicate.

use crate::Packet;

pub trait Device {
    /// Initialize this device.
    fn new() -> Packet;

    /// Perform a request, returning a data packet.
    fn request(packet: Packet) -> Packet;
}