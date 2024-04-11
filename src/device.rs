use std::convert::TryFrom;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::broadcast;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};

use crate::{
    codec::PingCodec,
    common::{self, ProtocolVersionStruct},
    error::PingError,
    message::{self, ProtocolMessage},
    Messages,
};

pub struct Common {
    pub id: u8,
    tx: SplitSink<Framed<SerialStream, PingCodec>, ProtocolMessage>,
    rx: broadcast::Receiver<ProtocolMessage>,
}

impl Common {
    pub fn new() -> Result<Self, PingError> {
        let mut port = tokio_serial::new("/dev/ttyUSB0", 115200).open_native_async()?;
        #[cfg(unix)]
        port.set_exclusive(false)?;

        let stream: Framed<tokio_serial::SerialStream, PingCodec> = PingCodec::new().framed(port);
        let (serial_tx, serial_rx) = stream.split();

        let (tx, mut rx) = broadcast::channel::<ProtocolMessage>(16);

        tokio::spawn(async move {
            let mut stream = serial_rx;

            loop {
                while let Some(msg) = stream.next().await {
                    if let Ok(msg) = msg {
                        let _ = tx.send(msg);
                    }
                }
            }
        });

        Ok(Common {
            id: 0,
            tx: serial_tx,
            rx: rx,
        })
    }

    pub async fn send_message(&mut self, message: ProtocolMessage) -> Result<(), PingError> {
        self.tx.send(message).await.map_err(|e| e.into())
    }

    pub async fn receive_message(&mut self) -> Result<ProtocolMessage, PingError> {
        self.rx.recv().await.map_err(|_| {
            PingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Broadcast channel closed",
            ))
        })
    }
}

pub trait PingDevice {
    fn get_common(&self) -> &Common;
    fn get_mut_common(&mut self) -> &mut Common;

    fn set_message_callback(&mut self, _callback: fn(String));

    fn get_id(&self) -> u8 {
        self.get_common().id
    }

    fn set_id(&mut self, id: u8) {
        self.get_mut_common().id = id;
    }

    async fn get_firmware(&mut self) -> Result<ProtocolVersionStruct, PingError> {
        let request =
            common::Messages::GeneralRequest(common::GeneralRequestStruct { requested_id: 5 });
        let mut package = message::ProtocolMessage::new();
        package.set_message(&request);
        self.get_mut_common().send_message(package).await;

        loop {
            let answer = match self.get_mut_common().receive_message().await {
                Ok(answer) => answer,
                Err(_) => continue,
            };

            match Messages::try_from(&answer) {
                Ok(Messages::Common(common::Messages::ProtocolVersion(answer))) => {
                    return Ok(answer)
                }
                e => {}
            }
        }
    }
}

pub struct Ping360 {
    common: Common,
    head: u8,
}

impl Default for Ping360 {
    fn default() -> Self {
        Self {
            common: Common::new().unwrap(),
            head: 128,
        }
    }
}

impl Ping360 {
    pub fn get_head(&self) -> u8 {
        self.head
    }
}

impl PingDevice for Ping360 {
    fn get_common(&self) -> &Common {
        &self.common
    }

    fn get_mut_common(&mut self) -> &mut Common {
        &mut self.common
    }

    fn set_message_callback(&mut self, _callback: fn(String)) {
        // Implement if needed
    }
}
