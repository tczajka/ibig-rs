use ibig::UBig;
use std::convert::TryFrom;

#[test]
fn test_from_to_le_bytes() {
    assert_eq!(UBig::from_le_bytes(&[]).to_le_bytes(), []);
    assert_eq!(UBig::from_le_bytes(&[0; 100]).to_le_bytes(), []);
    assert_eq!(UBig::from_le_bytes(&[1, 2, 3, 0]).to_le_bytes(), [1, 2, 3]);
    let bytes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    assert_eq!(UBig::from_le_bytes(&bytes).to_le_bytes(), bytes);
}

#[test]
fn test_from_to_be_bytes() {
    assert_eq!(UBig::from_be_bytes(&[]).to_be_bytes(), []);
    assert_eq!(UBig::from_be_bytes(&[0; 100]).to_be_bytes(), []);
    assert_eq!(UBig::from_be_bytes(&[0, 1, 2, 3]).to_be_bytes(), [1, 2, 3]);
    let bytes = [
        100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
    ];
    assert_eq!(UBig::from_be_bytes(&bytes).to_be_bytes(), bytes);
}

#[test]
fn test_from_unsigned() {
    assert_eq!(UBig::from(0xf1u8), UBig::from_be_bytes(&[0xf1]));
    assert_eq!(UBig::from(0xf123u16), UBig::from_be_bytes(&[0xf1, 0x23]));
    assert_eq!(
        UBig::from(0xf1234567u32),
        UBig::from_be_bytes(&[0xf1, 0x23, 0x45, 0x67])
    );
    assert_eq!(
        UBig::from(0xf123456701234567u64),
        UBig::from_be_bytes(&[0xf1, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67])
    );
    assert_eq!(
        UBig::from(0xf1234567012345670123456701234567u128),
        UBig::from_be_bytes(&[
            0xf1, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67, 0x01, 0x23,
            0x45, 0x67
        ])
    );

    assert_eq!(UBig::from(5u128), UBig::from_be_bytes(&[5]));
    assert_eq!(UBig::from(5usize), UBig::from_be_bytes(&[5]));
}

#[test]
fn test_from_bool() {
    assert_eq!(UBig::from(false), UBig::from(0u8));
    assert_eq!(UBig::from(true), UBig::from(1u8));
}

#[test]
fn test_from_char() {
    assert_eq!(UBig::from('a'), UBig::from(0x61u8));
    assert_eq!(UBig::from('Ł'), UBig::from(0x141u16));
}

#[test]
fn test_from_signed() {
    assert!(UBig::try_from(-5i32).is_err());
    assert_eq!(UBig::try_from(5i32), Ok(UBig::from(5u32)));
    assert_eq!(UBig::try_from(5i128 << 120), Ok(UBig::from(5u128 << 120)));
}
