//! Defines read/write codes.

#[repr(u8)]
pub enum Code {
    /// Write code processed successfully
    Ok,
}

impl From<Code> for u8 {
    fn from(code: Code) -> Self {
        use Code::*;
        
        match code {
            Ok => 0,
        }
    }
}