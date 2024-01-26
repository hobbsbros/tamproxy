//! Abstract over the Teensy board.

#![allow(dead_code)]

use teensy4_bsp as bsp;

use bsp::{
    board,
    hal::{
        timer::Blocking,
        gpt::{
            ClockSource,
            Gpt,
        },
        gpio::Port,
    },
    ral::{
        Instance,
        lpuart::RegisterBlock,
    },
    pins::t41::Pins,
};

/// The intended GPT1 frequency (Hz).
const GPT1_FREQUENCY: u32 = 1_000;
/// Given this clock source...
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
/// ... the root clock is PERCLK_CLK. To configure a GPT1 frequency,
/// we need a divider of...
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;

/// Create an abstraction over the Teensy 4.1 board.
pub struct Teensy41 {
    pub delay: Blocking<Gpt<1>, 1000>,
    pub gpio2: Port<2>,
    pub lpuart2: Instance<RegisterBlock, 2>,
}

impl Teensy41 {
    /// Construct a new Teensy board.
    pub fn new() -> (Self, Pins) {
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
            gpio2,
            // UART peripheral
            lpuart2,
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

        // Convenience for blocking delays.
        let delay = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);

        (Self {
            delay,
            gpio2,
            lpuart2,
        }, pins)
    }

    /// Delay for a provided number of milliseconds.
    pub fn delay(&mut self, ms: u32) {
        self.delay.block_ms(ms);
    }

    /// Log the provided information via USB.
    pub fn log(&mut self, info: &str) {
        log::info!("{}", info);
    }
}