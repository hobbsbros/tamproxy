//! Main executable for TAMProxy.

#![no_std]
#![no_main]

#![allow(unused_imports)]

// Initialize macros
mod delay;
mod io;
mod uart;
mod usb;
mod teensy41;

// Initialize device communication functions
mod code;
mod device;
mod packet;

// Export code & packet
pub use code::Code;
pub use packet::Packet;

// Import board support package
use teensy4_bsp as bsp;

// Import board utilities for macro expansions
use bsp::board;

// Import panic handler
use teensy4_panic as _;

// Import Teensy 4.1 struct.
use teensy41::Teensy41;

/// Milliseconds to delay before toggling the LED.
const DELAY_MS: u32 = 100;

#[bsp::rt::entry]
fn main() -> ! {
    // Create a new Teensy
    let mut teensy = Teensy41::new();

    let led = digital_output!(teensy, p13);

    loop {
        delay!(teensy, DELAY_MS);
        log!("Hello, world!");

        led.toggle();
    }
}