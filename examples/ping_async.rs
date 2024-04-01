#![warn(rust_2018_idioms)]

use bytes::BytesMut;
use futures::stream::StreamExt;
use std::{env, io};
use tokio_util::codec::{Decoder, Encoder};

use tokio_serial::SerialPortBuilderExt;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

struct HexCodec;

impl Decoder for HexCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() > 0 {
            let packet = src.split().to_vec();
            Ok(Some(packet))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Vec<u8>> for HexCodec {
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

    let mut reader = HexCodec.framed(port);

    while let Some(packet_result) = reader.next().await {
        let packet = packet_result.expect("Failed to read packet");
        println!("{:02X?}", packet);
    }
    Ok(())
}
