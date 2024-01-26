//! Define the delay macro.

#[macro_export]
/// Delays by the provided number of milliseconds.
macro_rules! delay {
    ($teensy:ident, $ms:literal) => {
        $teensy.0.delay.block_ms($ms);
    };

    ($teensy:ident, $ms:ident) => {
        $teensy.0.delay.block_ms($ms);
    };
}