extern crate alloc;

use alloc::vec::Vec;
use core::fmt::{Debug, Display};

use crate::message::{ProtocolMessage, HEADER};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    InvalidStartByte,
    IncompleteData,
    #[cfg_attr(feature = "serde", serde(with = "protocol_message_serialize"))]
    ChecksumError(ProtocolMessage),
}

// Add a serde serializer helper module for ProtocolMessage inside ParseError
#[cfg(feature = "serde")]
mod protocol_message_serialize {
    use super::ProtocolMessage;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(message: &ProtocolMessage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        message.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ProtocolMessage, D::Error>
    where
        D: Deserializer<'de>,
    {
        ProtocolMessage::deserialize(deserializer)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::InvalidStartByte => write!(f, "Invalid start byte"),
            ParseError::IncompleteData => write!(f, "Incomplete data"),
            ParseError::ChecksumError(_) => write!(f, "Checksum error"),
        }
    }
}

#[derive(Debug)]
pub enum DecoderResult {
    Success(ProtocolMessage),
    InProgress,
    Error(ParseError),
}

#[derive(Debug)]
pub enum DecoderState {
    AwaitingStart1,
    AwaitingStart2,
    ReadingHeader,
    ReadingPayload,
    ReadingChecksum,
}

pub struct Decoder {
    pub state: DecoderState,
    buffer: Vec<u8>,
    message: ProtocolMessage,
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            state: DecoderState::AwaitingStart1,
            buffer: Vec::new(),
            message: ProtocolMessage::new(),
        }
    }

    pub fn parse_byte(&mut self, byte: u8) -> DecoderResult {
        match self.state {
            DecoderState::AwaitingStart1 => {
                if byte == HEADER[0] {
                    self.state = DecoderState::AwaitingStart2;
                    return DecoderResult::InProgress;
                }
                return DecoderResult::Error(ParseError::InvalidStartByte);
            }
            DecoderState::AwaitingStart2 => {
                if byte == HEADER[1] {
                    self.state = DecoderState::ReadingHeader;
                    self.buffer.clear();
                    return DecoderResult::InProgress;
                }
                self.state = DecoderState::AwaitingStart1;
                return DecoderResult::Error(ParseError::InvalidStartByte);
            }
            DecoderState::ReadingHeader => {
                self.buffer.push(byte);
                // Basic information is available, moving to payload state
                if self.buffer.len() == 6 {
                    self.message.payload_length =
                        u16::from_le_bytes([self.buffer[0], self.buffer[1]]);
                    self.message.message_id = u16::from_le_bytes([self.buffer[2], self.buffer[3]]);
                    self.message.src_device_id = self.buffer[4];
                    self.message.dst_device_id = self.buffer[5];

                    if self.message.payload_length == 0 {
                        self.state = DecoderState::ReadingChecksum
                    } else {
                        self.state = DecoderState::ReadingPayload;
                    }
                    self.buffer.clear();
                }
                return DecoderResult::InProgress;
            }
            DecoderState::ReadingPayload => {
                self.buffer.push(byte);
                if self.buffer.len() == self.message.payload_length as usize {
                    self.message.payload = self.buffer.clone();
                    self.state = DecoderState::ReadingChecksum;
                    self.buffer.clear();
                }
                return DecoderResult::InProgress;
            }
            DecoderState::ReadingChecksum => {
                self.buffer.push(byte);
                if self.buffer.len() == 2 {
                    self.message.checksum = u16::from_le_bytes([self.buffer[0], self.buffer[1]]);
                    self.reset();
                    let message = self.message.clone();
                    self.message = ProtocolMessage::new();
                    if !message.has_valid_crc() {
                        return DecoderResult::Error(ParseError::ChecksumError(message));
                    }
                    return DecoderResult::Success(message);
                }
                return DecoderResult::InProgress;
            }
        }
    }

    fn reset(&mut self) {
        self.state = DecoderState::AwaitingStart1;
        self.buffer.clear();
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn test_simple_decoding() {
        let mut decoder = Decoder::new();

        // Test buffer from ping protocol documentation
        let buffer: Vec<u8> = vec![
            0x42, 0x52, 0x02, 0x00, // payload length
            0x06, 0x00, // message id
            0x00, 0x00, // src and dst id
            0x05, 0x00, // payload
            0xa1, 0x00, // crc
        ];

        for byte in &buffer[0..buffer.len() - 2] {
            assert!(matches!(
                decoder.parse_byte(*byte),
                DecoderResult::InProgress
            ));
        }
        assert!(matches!(
            decoder.parse_byte(buffer[buffer.len() - 2]),
            DecoderResult::InProgress
        ));
        let DecoderResult::Success(_message) = decoder.parse_byte(buffer[buffer.len() - 1]) else {
            panic!("Failed to use decoder with valid message");
        };

        // Test with wrong CRC
        for byte in &buffer[0..buffer.len() - 2] {
            assert!(matches!(
                decoder.parse_byte(*byte),
                DecoderResult::InProgress
            ));
        }
        assert!(matches!(
            decoder.parse_byte(buffer[buffer.len() - 2]),
            DecoderResult::InProgress
        ));
        assert!(matches!(
            decoder.parse_byte(0x01), // Force CRC error
            DecoderResult::Error(ParseError::ChecksumError(_))
        ));
    }
}