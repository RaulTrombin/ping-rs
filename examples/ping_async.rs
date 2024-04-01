#![warn(rust_2018_idioms)]

use bytes::BytesMut;
use futures::stream::StreamExt;
use std::{env, io};
use tokio_util::codec::{Decoder, Encoder};

use ping_rs::{common, decoder::Decoder as PingDecoder, message::ProtocolMessage, Messages};
use std::convert::TryFrom;
use tokio_serial::SerialPortBuilderExt;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

struct PingCodec;

impl Decoder for PingCodec {
    type Item = Messages;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut decoder = PingDecoder::new();

        for byte in src.iter() {
            match decoder.parse_byte(*byte) {
                ping_rs::decoder::DecoderResult::InProgress => { return Ok(None) }
                ping_rs::decoder::DecoderResult::Success((msg)) => {
                    println!("sucess");
                    match Messages::try_from(&msg) {
                        Ok(Messages::Common(msg)) => return Ok(Some(Messages::Common((msg)))),
                        Ok(Messages::Ping1D(msg)) => return Ok(Some(Messages::Ping1D((msg)))),
                        _ => {},
                    }
                }
                ping_rs::decoder::DecoderResult::Error(_) => {},
            }
        }


        Ok(None)
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

    let mut port = tokio_serial::new(tty_path, 9600).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut reader = PingCodec.framed(port);

    // // Spawn a separate task for sending messages
    // let sender_task = task::spawn(async move {
    //     loop {
    //         // Send your serial message here
    //         // For demonstration, let's send a simple message
    //         let message = vec![0x01, 0x02, 0x03];
    //         if let Err(err) = port.write_all(&message).await {
    //             eprintln!("Failed to send message: {:?}", err);
    //         }
    //         tokio::time::sleep(Duration::from_secs(30)).await;
    //     }
    // });


    while let Some(message_result) = reader.next().await {
        let message = message_result.expect("Failed to read message");

        // Handle your message here
        match message {
            ping_rs::Messages::Common(common::Messages::GeneralRequest(general_request)) => {
                // Handle general request message
                println!("General request message: {:?}", general_request);
            }
            ping_rs::Messages::Ping1D(something) => {
                // Handle general request message
                println!("General request message: {:?}", something);
            }
            ping_rs::Messages::Common(common::Messages::Nack(nack)) => {
                // Handle nack message
                println!("Nack message: {:?}", nack);
            }
            _ => {
                // Handle other message types
            }
        }
    }

    Ok(())
}
