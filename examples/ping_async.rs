#![warn(rust_2018_idioms)]

use bytes::BytesMut;
use futures::stream::StreamExt;
use futures::AsyncWrite;
use std::{env, io::{self, IoSlice}, time::Duration};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, task};
use tokio_util::codec::{Decoder, Encoder};

use ping_rs::{
    common,
    decoder::Decoder as PingDecoder,
    message::{self, ProtocolMessage},
    ping1d, Messages,
};
use std::convert::TryFrom;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec, LinesCodecError};

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
                ping_rs::decoder::DecoderResult::InProgress => {}
                ping_rs::decoder::DecoderResult::Success((msg)) => {
                    println!("sucess {:?}", msg);
                    match Messages::try_from(&msg) {
                        Ok(Messages::Common(msg)) => return Ok(Some(Messages::Common((msg)))),
                        Ok(Messages::Ping1D(msg)) => {
                            println!("{:?}", msg);
                            return Ok(Some(Messages::Ping1D((msg))));
                        }
                        Ok(Messages::Bluebps(msg)) => {}
                        Ok(Messages::Ping360(msg)) => {}
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }
                ping_rs::decoder::DecoderResult::Error(e) => {
                    println!("{:?}", e)
                }
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

    let mut port = tokio_serial::new(tty_path, 115200).open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let (read, write) = tokio::io::split(port);


    let sender_task = task::spawn(async move {
        let mut write = write; // Clone the port for the sender task
        loop {
            let request =
            common::Messages::GeneralRequest(common::GeneralRequestStruct { requested_id: 5 });
        let mut package = ProtocolMessage::new();
        package.set_message(&request);

        if let Err(e) = write.write(&mut &package.clone().serialized()).await {
            eprintln!("Error writing to port: {:?}", e);
            break; // Exit the loop if write operation fails
        }

            println!("sleep");
            // Sleep for 1 minute
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    let framed_stdin = FramedRead::new(read, PingCodec{});


    let receiver_task = framed_stdin
    .for_each(|msg| async {
        match msg {
            Ok(message) => println!("Received: "),
            Err(err) => eprintln!("Error decoding message: {:?}", err),
        }
    });

    // Wait for the sender task to finish
    // sender_task.await;
    tokio::join!(sender_task, receiver_task);
    Ok(())
}
