use crate::{decoder::Decoder as PingDecoder, error::PingError, message::ProtocolMessage};
use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct PingCodec {
    decoder: PingDecoder,
}

impl PingCodec {
    pub fn new() -> Self {
        PingCodec {
            decoder: PingDecoder::new(),
        }
    }
}

impl Decoder for PingCodec {
    type Item = ProtocolMessage;
    type Error = PingError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let decoder = &mut self.decoder;
        let mut consumed = 0;

        loop {
            let byte = match src.get(consumed) {
                Some(b) => *b,
                None => return Ok(None),
            };

            match decoder.parse_byte(byte) {
                crate::decoder::DecoderResult::InProgress => {
                    consumed += 1;
                    if consumed == src.len() {
                        src.advance(consumed)
                    }
                }
                crate::decoder::DecoderResult::Success(msg) => {
                    src.advance(consumed + 1);
                    return Ok(Some(msg));
                }
                crate::decoder::DecoderResult::Error(e) => {
                    src.advance(consumed + 1);
                    return Err(PingError::ParseError(e));
                }
            }
        }
    }
}

impl Encoder<ProtocolMessage> for PingCodec {
    type Error = PingError;

    fn encode(&mut self, item: ProtocolMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item.serialized());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test_ping_codec() {
        let mut codec = PingCodec::new();

        // Define GeneralRequest Buffer
        let buffer: Vec<u8> = vec![
            0x42, 0x52, 0x02, 0x00, // payload length
            0x06, 0x00, // message id
            0x00, 0x00, // src and dst id
            0x05, 0x00, // payload
            0xa1, 0x00, // crc
        ];
        let mut bytes_mut = BytesMut::new();
        bytes_mut.extend_from_slice(&buffer);

        // Define equivalent ProtocolMessage
        let request =
            common::Messages::GeneralRequest(common::GeneralRequestStruct { requested_id: 5 });
        let mut package = crate::message::ProtocolMessage::new();
        package.set_message(&request);

        // Decode the buffer
        let decoded_message = codec.decode(&mut bytes_mut).unwrap().unwrap();

        // Assert that the decoded message matches the expected PingMessage
        assert_eq!(decoded_message, package);

        let mut encoded = BytesMut::new();
        codec.encode(package.clone(), &mut encoded).unwrap();

        // Assert that the encoded bytes match the original buffer
        assert_eq!(encoded.to_vec(), buffer);
    }
}
