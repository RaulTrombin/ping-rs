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
    type Error = io::Error;

    fn encode(&mut self, item: ProtocolMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item.serialized());
        Ok(())
    }
}
