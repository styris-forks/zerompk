#![cfg(feature = "preserve-int-width")]

fn enc<T: zerompk::ToMessagePack>(v: T) -> Vec<u8> {
    zerompk::to_msgpack_vec(&v).unwrap()
}

#[test]
fn small_values_keep_declared_width() {
    assert_eq!(enc(5u8), vec![0xcc, 5]);
    assert_eq!(enc(5u16), vec![0xcd, 0, 5]);
    assert_eq!(enc(5u32), vec![0xce, 0, 0, 0, 5]);
    assert_eq!(enc(5u64), vec![0xcf, 0, 0, 0, 0, 0, 0, 0, 5]);
    assert_eq!(enc(5i8), vec![0xd0, 5]);
    assert_eq!(enc(5i16), vec![0xd1, 0, 5]);
    assert_eq!(enc(5i32), vec![0xd2, 0, 0, 0, 5]);
    assert_eq!(enc(5i64), vec![0xd3, 0, 0, 0, 0, 0, 0, 0, 5]);
}

#[test]
fn roundtrips() {
    macro_rules! rt {
        ($v:expr, $ty:ty) => {{
            let bytes = enc($v);
            let back: $ty = zerompk::from_msgpack(&bytes).unwrap();
            assert_eq!(back, $v);
        }};
    }
    rt!(5u8, u8);
    rt!(300u16, u16);
    rt!(70_000u32, u32);
    rt!(5_000_000_000u64, u64);
    rt!(-5i8, i8);
    rt!(-300i16, i16);
    rt!(-70_000i32, i32);
    rt!(-5_000_000_000i64, i64);
}
