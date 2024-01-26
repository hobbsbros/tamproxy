//! Define a TAMProxy data packet.

const MAX_PACKET_LENGTH: usize = 8;

const START_CODE: u8 = 240;
const END_CODE: u8 = 255;

use crate::Code;

pub struct Packet {
    pub code: Code,
    pub data: [Option<u8>; MAX_PACKET_LENGTH - 3],
}

impl From<Packet> for [u8; MAX_PACKET_LENGTH] {
    fn from(packet: Packet) -> Self {
        let mut output = [0u8; MAX_PACKET_LENGTH];

        output[0] = START_CODE;
        output[1] = Into::<u8>::into(packet.code);
        
        for i in 0..packet.data.len() {
            output[i + 2] = packet.data[i].unwrap_or(0);
        }

        output[MAX_PACKET_LENGTH - 1] = END_CODE;

        output
    }
}