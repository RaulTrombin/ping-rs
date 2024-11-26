use alloc::{string::String, vec::Vec};

pub const HEADER: [u8; 2] = ['B' as u8, 'R' as u8];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProtocolMessage {
    pub payload_length: u16,
    pub message_id: u16,
    pub src_device_id: u8,
    pub dst_device_id: u8,
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    pub payload: Vec<u8>,
    pub checksum: u16,
}

impl ProtocolMessage {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn has_valid_crc(&self) -> bool {
        self.checksum == self.calculate_crc()
    }

    #[inline]
    pub fn checksum(&self) -> u16 {
        self.checksum
    }

    #[inline]
    pub fn update_checksum(&mut self) {
        self.checksum = self.calculate_crc();
    }

    pub fn set_message(&mut self, message: &impl PingMessage) {
        self.message_id = message.message_id();
        self.payload = message.serialize(); // Assuming serialize returns Vec<u8>
        self.payload_length = self.payload.len() as u16;
        self.update_checksum();
    }

    #[inline]
    pub fn set_src_device_id(&mut self, src_device_id: u8) {
        self.src_device_id = src_device_id;
        self.update_checksum();
    }

    #[inline]
    pub fn set_dst_device_id(&mut self, dst_device_id: u8) {
        self.dst_device_id = dst_device_id;
        self.update_checksum();
    }

    #[inline]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn calculate_crc(&self) -> u16 {
        let mut checksum: u16 = 0;
        checksum = checksum.wrapping_add(HEADER[0] as u16);
        checksum = checksum.wrapping_add(HEADER[1] as u16);
        self.payload_length
            .to_le_bytes()
            .iter()
            .for_each(|byte| checksum = checksum.wrapping_add(*byte as u16));
        self.message_id
            .to_le_bytes()
            .iter()
            .for_each(|byte| checksum = checksum.wrapping_add(*byte as u16));
        checksum = checksum.wrapping_add(self.src_device_id as u16);
        checksum = checksum.wrapping_add(self.dst_device_id as u16);
        for &byte in &self.payload {
            checksum = checksum.wrapping_add(byte as u16);
        }
        checksum
    }

    pub fn length(&self) -> usize {
        HEADER.len() + 2 + 2 + 1 + 1 + self.payload_length as usize + 2
    }

    pub fn serialized(&self) -> Vec<u8> {
        let mut serialized_data = Vec::with_capacity(self.length());
        serialized_data.extend_from_slice(&HEADER);
        serialized_data.extend_from_slice(&self.payload_length.to_le_bytes());
        serialized_data.extend_from_slice(&self.message_id.to_le_bytes());
        serialized_data.push(self.src_device_id);
        serialized_data.push(self.dst_device_id);
        serialized_data.extend_from_slice(&self.payload);
        serialized_data.extend_from_slice(&self.checksum.to_le_bytes());
        serialized_data
    }
}

// This information is only related to the message itself,
// not the entire package with header, dst, src and etc.
pub trait PingMessage
where
    Self: Sized + SerializePayload + SerializePayload,
{
    fn message_id(&self) -> u16;
    fn message_name(&self) -> &'static str;

    fn message_id_from_name(name: &str) -> Result<u16, String>;
}

pub trait SerializePayload {
    fn serialize(&self) -> Vec<u8>;
}

pub trait DeserializePayload {
    fn deserialize(payload: &[u8]) -> Self;
}

pub trait MessageInfo {
    fn id() -> u16;
}

pub trait DeserializeGenericMessage
where
    Self: Sized,
{
    fn deserialize(message_id: u16, payload: &[u8]) -> Result<Self, &'static str>;
}
