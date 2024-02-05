//! Define the USB logging macro.

#[macro_export]
/// Logs the provided information via USB.
macro_rules! log {
    ($string:expr) => {
        log::info!("{}", $string);
    };
}