//! The starter code slowly blinks the LED, sets up
//! USB logging, and creates a UART driver using pins
//! 14 and 15. The UART baud rate is [`UART_BAUD`].
//!
//! Despite targeting the Teensy 4.0, this starter code
//! also works on the Teensy 4.1.

#![no_std]
#![no_main]

mod board;

use teensy4_bsp as bsp;
use teensy4_panic as _;

use board::Teensy;

#[bsp::rt::entry]
fn main() -> ! {
    // Create a new Teensy.
    let mut teensy = Teensy::new();
}