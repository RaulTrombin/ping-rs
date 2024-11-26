use std::io::Write;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Re-export core protocol types
pub use bluerobotics_ping_core::{
    DeserializeGenericMessage, DeserializePayload, MessageInfo, PingMessage, ProtocolMessage,
    SerializePayload, HEADER,
};

// Add std-specific extension methods for ProtocolMessage
pub trait ProtocolMessageStd {
    fn write(&self, writer: &mut dyn Write) -> std::io::Result<usize>;
}

impl ProtocolMessageStd for ProtocolMessage {
    fn write(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        let data = self.serialized();
        writer.write_all(&data)?;
        Ok(data.len())
    }
}
