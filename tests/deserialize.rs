use std::convert::TryFrom;

use ping_rs::common::Messages as common_messages;
use ping_rs::decoder::*;
use ping_rs::{common, Messages};

#[test]
fn test_simple_deserialization() {
    let mut decoder = Decoder::new();
    let general_request =
        common_messages::GeneralRequest(common::GeneralRequestStruct { requested_id: 5 });

    // From official ping protocol documentation
    let buffer: Vec<u8> = vec![
        0x42, 0x52, 0x02, 0x00, // payload length
        0x06, 0x00, // message id
        0x00, 0x00, // src and dst id
        0x05, 0x00, // payload
        0xa1, 0x00, // crc
    ];
    let Messages::Common(parsed) = Messages::try_from(&buffer).unwrap() else {
        panic!("Failed to parse common message.");
    };
    assert_eq!(general_request, parsed);

    for byte in &buffer[0..buffer.len() - 2] {
        dbg!(byte, &decoder.state);
        assert!(matches!(
            decoder.parse_byte(*byte),
            DecoderResult::InProgress
        ));
    }
    assert!(matches!(
        decoder.parse_byte(buffer[buffer.len() - 2]),
        DecoderResult::InProgress
    ));
    let DecoderResult::Success(_message) = decoder.parse_byte(buffer[buffer.len() - 1]) else {
        dbg!(decoder.state);
        panic!("Failed to use decoder with valid message");
    };

    // Wrong CRC test
    let buffer: Vec<u8> = vec![
        0x42, 0x52, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x05, 0x00, 0xa1, 0x01,
    ];
    assert!(Messages::try_from(&buffer).is_err());
}
