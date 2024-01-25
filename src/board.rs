//! Abstract over the Teensy board.

use core::fmt::Write as _;

use teensy4_bsp as bsp;

use bsp::board::{
    self,
    Resources,
};
use bsp::hal::timer::Blocking;

/// CHANGE ME to vary the baud rate.
const UART_BAUD: u32 = 115200;

/// Milliseconds to delay before toggling the LED
/// and writing text outputs.
const DELAY_MS: u32 = 100;

// We're responsible for configuring our timers.
// This example uses PERCLK_CLK as the GPT1 clock source,
// and it configures a 1 KHz GPT1 frequency by computing a
// GPT1 divider.
use bsp::hal::gpt::ClockSource;

/// The intended GPT1 frequency (Hz).
const GPT1_FREQUENCY: u32 = 1_000;
/// Given this clock source...
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
/// ... the root clock is PERCLK_CLK. To configure a GPT1 frequency,
/// we need a divider of...
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;

/// Create an abstraction over the Teensy board.
pub struct Teensy { }

impl Teensy {
    /// Construct a new Teensy board.
    pub fn new() -> ! {
        // These are peripheral instances. Let the board configure these for us.
        // This function can only be called once!
        let instances = board::instances();

        // Driver resources that are configured by the board. For more information,
        // see the `board` documentation.
        let board::Resources {
            // `pins` has objects that represent the physical pins. The object
            // for pin 13 is `p13`.
            pins,
            // This is a hardware timer. We'll use it for blocking delays.
            mut gpt1,
            // These are low-level USB resources. We'll pass these to a function
            // that sets up USB logging.
            usb,
            // This is the GPIO2 port. We need this to configure the LED as a
            // GPIO output.
            mut gpio2,
            ..
        } = board::t41(instances);

        // When this returns, you can use the `log` crate to write text
        // over USB. Use either `screen` (macOS, Linux) or PuTTY (Windows)
        // to visualize the messages from this example.
        bsp::LoggingFrontend::default_log().register_usb(usb);

        // Configures the GPT1 timer to run at GPT1_FREQUENCY. See the
        // constants below for more information.
        gpt1.disable();
        gpt1.set_divider(GPT1_DIVIDER);
        gpt1.set_clock_source(GPT1_CLOCK_SOURCE);

        // This configures the LED as a GPIO output.
        let led = board::led(&mut gpio2, pins.p13);

        // Convenience for blocking delays.
        let mut delay = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);

        let mut counter: u32 = 0;

        loop {
            led.toggle();
            log::info!("Hello from the USB logger! The count is {counter}");
            delay.block_ms(DELAY_MS);
            counter = counter.wrapping_add(1);
        }
    }
}