#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/1/12/Bluerobotics-logo.svg")]
#![doc = include_str!("../README.md")]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub use bluerobotics_ping_core::{ProtocolMessage, HEADER};
use message::DeserializeGenericMessage;

use std::convert::TryFrom;

pub mod codec;
pub mod device;
pub mod error;
pub mod message;

pub fn calculate_crc(pack_without_payload: &[u8]) -> u16 {
    pack_without_payload
        .iter()
        .fold(0 as u16, |s, &v| s.wrapping_add(v as u16))
}
