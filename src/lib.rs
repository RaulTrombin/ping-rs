include!(concat!(env!("OUT_DIR"), "/mod.rs"));

use decoder::Decoder;
use embedded_io::{Read, Write};
use message::ProtocolMessage;

use crate::{
    decoder::DecoderResult,
    message::{DeserializeGenericMessage, HEADER},
};

use std::convert::TryFrom;

pub mod decoder;
pub mod message;

/// Error type for reading data from the sensor
#[derive(Debug)]
pub enum PingError<E> {
    SerialRead(E),
    SerialWrite(E),
    ParsingError,
    ParsingErrorMissingBytes,
}

/// A wrapper around [`Read`](embedded-io::Read) for reading messages from the sensor
pub struct PingDevice<RW> {
    port: RW,
    decoder: Decoder,
}

impl<D: Read + Write> PingDevice<D> {
    pub fn new(port: D) -> Self {
        Self {
            port,
            decoder: Decoder::new(),
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PingError<D::Error>> {
        match self.port.read(buf) {
            Ok(value) => Ok(value),
            Err(_e) => Err(PingError::SerialRead(_e)),
        }
    }

    pub fn write(&mut self, request: Vec<u8>) -> Result<(), PingError<D::Error>> {
        match self.port.write_all(&request) {
            Ok(_e) => {
                self.port.flush().unwrap();
                Ok(_e)
            }
            Err(_e) => Err(PingError::SerialWrite(_e)),
        }
    }

    pub fn request(
        &mut self,
        request: ProtocolMessage,
    ) -> Result<ProtocolMessage, PingError<D::Error>> {
        self.write(request.serialized()).unwrap();

        let mut serial_buf: Vec<u8> = vec![0; 20];

        self.port.read(serial_buf.as_mut_slice()).unwrap();

        for byte in &serial_buf {
            match self.decoder.parse_byte(byte.clone()) {
                DecoderResult::InProgress => continue,
                DecoderResult::Error(_e) => {
                    return Err(PingError::ParsingError);
                }
                DecoderResult::Success(_e) => {
                    return Ok(_e);
                }
            }
        }
        self.decoder.reset();
        Err(PingError::ParsingErrorMissingBytes) //missing bytes, cant't complete.
    }

    pub fn get_protocol_version(&mut self) -> Result<common::Messages, PingError<D::Error>> {
        let request =
            common::Messages::GeneralRequest(common::GeneralRequestStruct { requested_id: 5 });

        let mut package = ProtocolMessage::new();
        package.set_message(&request);

        let answer = self.request(package)?;

        match Messages::try_from(&answer) {
            Ok(Messages::Common(answer)) => Ok(answer),
            _ => Err(PingError::ParsingError),
        }
    }

    pub fn get_device_information(&mut self) -> Result<common::Messages, PingError<D::Error>> {
        let request =
            common::Messages::GeneralRequest(common::GeneralRequestStruct { requested_id: 4 });

        let mut package = ProtocolMessage::new();
        package.set_message(&request);

        let answer = self.request(package)?;

        match Messages::try_from(&answer) {
            Ok(Messages::Common(answer)) => Ok(answer),
            _ => Err(PingError::ParsingError),
        }
    }

    pub fn set_device_id(
        &mut self,
        device_id: u8,
    ) -> Result<common::Messages, PingError<D::Error>> {
        let request = common::Messages::SetDeviceId(common::SetDeviceIdStruct { device_id });

        let mut package = ProtocolMessage::new();
        package.set_message(&request);

        let answer = self.request(package)?;

        match Messages::try_from(&answer) {
            Ok(Messages::Common(answer)) => Ok(answer),
            _ => Err(PingError::ParsingError),
        }
    }
}

pub fn calculate_crc(pack_without_payload: &[u8]) -> u16 {
    return pack_without_payload
        .iter()
        .fold(0 as u16, |s, &v| s.wrapping_add(v as u16));
}
