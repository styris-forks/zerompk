#![cfg(all(
    feature = "bytes",
    feature = "uuid",
    feature = "default-vec-u8-as-array",
    feature = "derive"
))]

use zerompk::{FromMessagePack, ToMessagePack};

#[derive(ToMessagePack, FromMessagePack, PartialEq, Debug)]
struct Thing {
    v: Vec<u8>,
    b: bytes::Bytes,
    id: uuid::Uuid,
}

#[test]
fn vec_u8_is_array_bytes_is_bin() {
    let t = Thing {
        v: vec![1, 2, 3],
        b: bytes::Bytes::from_static(&[4, 5, 6]),
        id: uuid::Uuid::from_bytes([7u8; 16]),
    };
    let enc = zerompk::to_msgpack_vec(&t).unwrap();

    // Find the encoded bytes for v (array: fixarray 0x93) and b (bin8: 0xc4).
    // Just assert round-trip and that a bin8 marker (0xc4) appears (from Bytes/uuid)
    // while v stays an array.
    assert!(enc.contains(&0xc4), "expected a bin8 marker from Bytes/uuid");

    let back: Thing = zerompk::from_msgpack(&enc).unwrap();
    assert_eq!(back, t);
}

#[test]
fn standalone_vec_u8_field_array_marker() {
    #[derive(ToMessagePack, FromMessagePack, PartialEq, Debug)]
    struct OnlyVec {
        v: Vec<u8>,
    }
    let t = OnlyVec { v: vec![1, 2, 3] };
    let enc = zerompk::to_msgpack_vec(&t).unwrap();
    // Map(1) { "v": fixarray[1,2,3] } -> the value must be a fixarray 0x93, never bin (0xc4)
    assert!(!enc.contains(&0xc4), "Vec<u8> must not encode as bin: {:?}", enc);
    let back: OnlyVec = zerompk::from_msgpack(&enc).unwrap();
    assert_eq!(back, t);
}
