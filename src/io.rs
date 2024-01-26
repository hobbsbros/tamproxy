//! Define digital I/O macros for TAMProxy.

#[macro_export]
/// Construct a digital output pin.
macro_rules! digital_output {
    ($teensy:ident, $pin:ident) => {
        $teensy.0.gpio2.output($teensy.1.$pin)
    };
}

#[macro_export]
/// Construct a digital input pin.
macro_rules! digital_input {
    ($teensy:ident, $pin:ident) => {
        $teensy.0.gpio2.input($teensy.1.$pin)
    };
}