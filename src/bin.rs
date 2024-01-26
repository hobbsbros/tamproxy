//! Main executable.

#![no_std]
#![no_main]

#![allow(unused_imports)]
#![allow(special_module_name)]

// Initialize macros
mod delay;
mod io;
mod uart;
mod usb;
mod teensy41;
mod main;

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
    main!();
}

