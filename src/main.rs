//! MAIN macro definition.
//! 
//! Write your control code here and your code will
//! be copied into `src/bin.rs` for compilation.

#[macro_export]
/// This macro will be executed.
macro_rules! main {
    ($teensy:ident) => {
        // Create Teensy as a local variable
        let mut teensy = $teensy;

        // BEGIN CODE HERE
        
        // SETUP

        // Create an on-board LED
        let led = digital_output!(teensy, p13);

        // LOOP

        loop {
            log!("Hello, world!");
            led.toggle();
            delay!(teensy, DELAY_MS);
        }
    };
}