use bytes::BytesMut;
use hypermqtt::{packet::Packet, serialization::serialize};

static serialize_test_data: Vec<(Packet, &[u8])> = vec![];

#[test]
pub fn test_serialize() {
    serialize_test_data
        .iter()
        .for_each(|c| verify_serialization(&c.0, c.1));
}

fn verify_serialization(packet: &Packet, expected: &[u8]) {
    let mut buffer = BytesMut::with_capacity(1000);

    serialize(&mut buffer, packet).expect("msg");

    assert_eq!(expected, buffer);
}
