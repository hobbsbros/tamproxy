//! Define the USB logging macro for TAMProxy.

#[macro_export]
/// Logs the provided information via USB.
macro_rules! log {
    ($string:literal) => {
        log::info!("{}", $string);
    };

    ($string:ident) => {
        log::info!("{}", $string);
    };
}