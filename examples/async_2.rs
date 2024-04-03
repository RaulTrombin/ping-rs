use futures::{stream::StreamExt, SinkExt};
use futures::TryStreamExt;
use ping_rs::ping1d::DeviceIdStruct;
use std::{convert::TryFrom, env, io, str};
use tokio::time::{sleep, Duration};
use tokio_util::codec::{Decoder, Encoder, Framed};
use bytes::{BufMut, BytesMut};
use tokio_serial::SerialPortBuilderExt;
use ping_rs::{
    common, decoder::Decoder as PingDecoder, message::{self, ProtocolMessage}, ping1d, Messages,
};
use bytes::Buf;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

struct PingCodec {
    decoder: PingDecoder,
}

impl PingCodec {
    fn new() -> Self {
        PingCodec {
            decoder: PingDecoder::new(),
        }
    }
}

impl Decoder for PingCodec {
    type Item = ProtocolMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut decoder = &mut self.decoder;
        let mut consumed = 0;

        loop {
            let byte = match src.get(consumed) {
                Some(b) => *b,
                None => return Ok(None), // No more bytes, return None
            };

            match decoder.parse_byte(byte) {
                ping_rs::decoder::DecoderResult::InProgress => {
                    consumed += 1;
                    if consumed == src.len() {
                        src.advance(consumed);
                        return Ok(None); // Not enough bytes to decode a message
                    }
                }
                ping_rs::decoder::DecoderResult::Success(msg) => {
                    src.advance(consumed + 1); // Consume the bytes used for decoding
                    return Ok(Some(msg)); // Return the decoded message
                }
                ping_rs::decoder::DecoderResult::Error(e) => {
                    println!("{:?}", e);
                    src.advance(consumed + 1); // Consume the bytes up to the error
                    return Ok(None); // Return None on error
                }
            }
        }
    }
}

impl Encoder<Vec<u8>> for PingCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());
    let mut port = tokio_serial::new(tty_path, 115_200).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let stream: Framed<tokio_serial::SerialStream, PingCodec> = PingCodec::new().framed(port);
    let (mut tx, mut rx) = stream.split();

    tokio::spawn(async move {
        loop {
            let item = rx
                .next()
                .await
                .expect("Error awaiting future in RX stream.")
                .expect("Reading stream resulted in an error");
            println!("{:?}", item);

            // let structure = match Messages::try_from(&item) {
            //     Ok(Messages::Common(common::Messages::ProtocolVersion(answer))) => Ok(answer),
            //     _ => Err({}),
            // };

            // println!("{:?}", structure);

            // let ProfileStruct = match Messages::try_from(&item) {
            //     Ok(Messages::Ping1D(ping1d::Messages::Profile(answer))) => Ok(answer),
            //     _ => Err({}),
            // };

            // println!("{:?}", ProfileStruct);


        }
    });

    // tokio::spawn(async move {
        loop {
            // let request = common::Messages::GeneralRequest(common::GeneralRequestStruct {
            //     requested_id: 5,
            // });
            // let mut package = ProtocolMessage::new();
            // package.set_message(&request);

            // let request = ping1d::Messages::DeviceId(ping1d::DeviceIdStruct {device_id : 0});
            // let mut package = ProtocolMessage::new();
            // package.set_message(&request);

            let request = common::Messages::GeneralRequest(common::GeneralRequestStruct {
                requested_id: 1201,
            });
            let mut package = ProtocolMessage::new();
            package.set_message(&request);

            let write_result = tx.send(package.serialized()).await;
            sleep(Duration::from_secs(2)).await;
            match write_result {
                Ok(_) => (),
                Err(err) => println!("{:?}", err),
            }
        }
    // });

    // loop {}
}