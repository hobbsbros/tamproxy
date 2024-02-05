//! Define UART macros .

#[macro_export]
/// Construct the LPUART peripheral.
macro_rules! uart {
    ($teensy:ident) => {
        board::lpuart($teensy.0.lpuart2, $teensy.1.p14, $teensy.1.p15, 115200)
    };
}

#[macro_export]
/// Read a single byte from the UART serial bus.
macro_rules! read {
    ($uart:ident) => {
        nb::block!($uart.read()).unwrap()
    };
}

#[macro_export]
/// Write a single byte to the UART serial bus.
macro_rules! write {
    ($uart:ident, $msg:expr) => {
        nb::block!($uart.write($msg)).unwrap()
    };
}