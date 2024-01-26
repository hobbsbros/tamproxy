//! MAIN macro definition.
//! 
//! Write your control code here and your code will
//! be copied into `src/bin.rs` for compilation.

#[macro_export]
/// This macro will be executed.
macro_rules! main {
    () => {
        // Create a new Teensy
        let mut teensy = Teensy41::new();

        // Create an LED
        let led = digital_output!(teensy, p13);

        loop {
            log!("Hello, world!");
            led.toggle();
            delay!(teensy, DELAY_MS);
        }
    };
}