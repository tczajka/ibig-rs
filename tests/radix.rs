use ibig::UBig;

#[test]
fn test_ubig_format() {
    assert_eq!(format!("{:b}", UBig::from(0u32)), "0");
    assert_eq!(format!("{:b}", UBig::from(100u32)), "1100100");
    assert_eq!(format!("{:#b}", UBig::from(100u32)), "0b1100100");
    assert_eq!(format!("{:+b}", UBig::from(100u32)), "+1100100");
    assert_eq!(format!("{:+#b}", UBig::from(100u32)), "+0b1100100");
    assert_eq!(format!("{:10b}", UBig::from(100u32)), "   1100100");
    assert_eq!(format!("{:=<10b}", UBig::from(100u32)), "1100100===");
    assert_eq!(format!("{:=>10b}", UBig::from(100u32)), "===1100100");
    assert_eq!(format!("{:=^10b}", UBig::from(100u32)), "=1100100==");
    assert_eq!(format!("{:=^+10b}", UBig::from(100u32)), "=+1100100=");
    assert_eq!(format!("{:+010b}", UBig::from(100u32)), "+001100100");
    assert_eq!(format!("{:+#010b}", UBig::from(100u32)), "+0b1100100");
    assert_eq!(format!("{:+#01b}", UBig::from(100u32)), "+0b1100100");
    assert_eq!(format!("{:o}", UBig::from(100u32)), "144");
    assert_eq!(format!("{:#o}", UBig::from(100u32)), "0o144");
    assert_eq!(format!("{:x}", UBig::from(3000u32)), "bb8");
    assert_eq!(format!("{:#x}", UBig::from(3000u32)), "0xbb8");
    assert_eq!(format!("{:X}", UBig::from(3000u32)), "BB8");
    assert_eq!(format!("{:#X}", UBig::from(3000u32)), "0xBB8");
    assert_eq!(format!("{:#10X}", UBig::from(3000u32)), "     0xBB8");
}

#[test]
fn test_ubig_in_radix() {
    assert_eq!(format!("{}", UBig::from(0u32).in_radix(2)), "0");
    assert_eq!(format!("{}", UBig::from(100u32).in_radix(4)), "1210");
    assert_eq!(format!("{}", UBig::from(3000u32).in_radix(16)), "bb8");
    assert_eq!(
        format!("{:+010}", UBig::from(3000u32).in_radix(16)),
        "+000000bb8"
    );
    assert_eq!(
        format!("{:+#010}", UBig::from(3000u32).in_radix(16)),
        "+000000BB8"
    );
}

#[test]
fn test_ubig_to_radix_str() {
    assert_eq!(UBig::from(0u32).to_radix_str(16), "0");
    assert_eq!(UBig::from(100u32).to_radix_str(4), "1210");
    assert_eq!(UBig::from(3000u32).to_radix_str(16), "bb8");
    assert_eq!(UBig::from(3000u32).to_radix_str_uppercase(16), "BB8");
    assert_eq!(UBig::from(3000u32).to_radix_str(32), "2to");
    assert_eq!(UBig::from(3000u32).to_radix_str_uppercase(32), "2TO");
}
