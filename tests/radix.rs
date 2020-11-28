use ibig::{IBig, UBig};

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

    let a = UBig::from_be_bytes(&[
        0x05, 0xee, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
        0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    ]);
    assert_eq!(
        format!("{:x}", a),
        "5ee0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
    assert_eq!(
        format!("{:X}", a),
        "5EE0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF"
    );
    assert_eq!(
        format!("{:^100X}", a),
        "        5EE0123456789ABCDEF\
        0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF         "
    );
    assert_eq!(
        format!("{:o}", a),
        "1367001106425474232571573600443212636115274675700221505317046\
        53633674011064254742325715736004432126361152746757"
    );
    assert_eq!(
        format!("{:>120o}", a),
        "         1367001106425474232571573600443212636115274675700221505317046\
        53633674011064254742325715736004432126361152746757"
    );
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

#[test]
fn test_ibig_format() {
    assert_eq!(format!("{:b}", IBig::from(0i32)), "0");
    assert_eq!(format!("{:b}", IBig::from(100i32)), "1100100");
    assert_eq!(format!("{:b}", IBig::from(-100i32)), "-1100100");
    assert_eq!(format!("{:#b}", IBig::from(100i32)), "0b1100100");
    assert_eq!(format!("{:#b}", IBig::from(-100i32)), "-0b1100100");
    assert_eq!(format!("{:+b}", IBig::from(100i32)), "+1100100");
    assert_eq!(format!("{:+b}", IBig::from(-100i32)), "-1100100");
    assert_eq!(format!("{:+#b}", IBig::from(100i32)), "+0b1100100");
    assert_eq!(format!("{:+#b}", IBig::from(-100i32)), "-0b1100100");
    assert_eq!(format!("{:10b}", IBig::from(100i32)), "   1100100");
    assert_eq!(format!("{:10b}", IBig::from(-100i32)), "  -1100100");
    assert_eq!(format!("{:=<10b}", IBig::from(100i32)), "1100100===");
    assert_eq!(format!("{:=<10b}", IBig::from(-100i32)), "-1100100==");
    assert_eq!(format!("{:=>10b}", IBig::from(100i32)), "===1100100");
    assert_eq!(format!("{:=>10b}", IBig::from(-100i32)), "==-1100100");
    assert_eq!(format!("{:=^10b}", IBig::from(100i32)), "=1100100==");
    assert_eq!(format!("{:=^10b}", IBig::from(-100i32)), "=-1100100=");
    assert_eq!(format!("{:=^+10b}", IBig::from(100i32)), "=+1100100=");
    assert_eq!(format!("{:=^+10b}", IBig::from(-100i32)), "=-1100100=");
    assert_eq!(format!("{:+010b}", IBig::from(100i32)), "+001100100");
    assert_eq!(format!("{:+010b}", IBig::from(-100i32)), "-001100100");
    assert_eq!(format!("{:+#010b}", IBig::from(100i32)), "+0b1100100");
    assert_eq!(format!("{:+#010b}", IBig::from(-100i32)), "-0b1100100");
    assert_eq!(format!("{:+#01b}", IBig::from(100i32)), "+0b1100100");
    assert_eq!(format!("{:+#01b}", IBig::from(-100i32)), "-0b1100100");
    assert_eq!(format!("{:o}", IBig::from(100i32)), "144");
    assert_eq!(format!("{:o}", IBig::from(-100i32)), "-144");
    assert_eq!(format!("{:#o}", IBig::from(100i32)), "0o144");
    assert_eq!(format!("{:#o}", IBig::from(-100i32)), "-0o144");
    assert_eq!(format!("{:x}", IBig::from(3000i32)), "bb8");
    assert_eq!(format!("{:x}", IBig::from(-3000i32)), "-bb8");
    assert_eq!(format!("{:#x}", IBig::from(3000i32)), "0xbb8");
    assert_eq!(format!("{:#x}", IBig::from(-3000i32)), "-0xbb8");
    assert_eq!(format!("{:X}", IBig::from(3000i32)), "BB8");
    assert_eq!(format!("{:X}", IBig::from(-3000i32)), "-BB8");
    assert_eq!(format!("{:#X}", IBig::from(3000i32)), "0xBB8");
    assert_eq!(format!("{:#X}", IBig::from(-3000i32)), "-0xBB8");
    assert_eq!(format!("{:#10X}", IBig::from(3000i32)), "     0xBB8");
    assert_eq!(format!("{:#10X}", IBig::from(-3000i32)), "    -0xBB8");
}

#[test]
fn test_ibig_in_radix() {
    assert_eq!(format!("{}", IBig::from(0i32).in_radix(2)), "0");
    assert_eq!(format!("{}", IBig::from(100i32).in_radix(4)), "1210");
    assert_eq!(format!("{}", IBig::from(-100i32).in_radix(4)), "-1210");
    assert_eq!(format!("{}", IBig::from(3000i32).in_radix(16)), "bb8");
    assert_eq!(format!("{}", IBig::from(-3000i32).in_radix(16)), "-bb8");
    assert_eq!(
        format!("{:+010}", IBig::from(3000i32).in_radix(16)),
        "+000000bb8"
    );
    assert_eq!(
        format!("{:+010}", IBig::from(-3000i32).in_radix(16)),
        "-000000bb8"
    );
    assert_eq!(
        format!("{:#010}", IBig::from(3000i32).in_radix(16)),
        "0000000BB8"
    );
    assert_eq!(
        format!("{:#010}", IBig::from(-3000i32).in_radix(16)),
        "-000000BB8"
    );
}

#[test]
fn test_ibig_to_radix_str() {
    assert_eq!(IBig::from(0i32).to_radix_str(16), "0");
    assert_eq!(IBig::from(100i32).to_radix_str(4), "1210");
    assert_eq!(IBig::from(-100i32).to_radix_str(4), "-1210");
    assert_eq!(IBig::from(3000i32).to_radix_str(16), "bb8");
    assert_eq!(IBig::from(-3000i32).to_radix_str(16), "-bb8");
    assert_eq!(IBig::from(3000i32).to_radix_str_uppercase(16), "BB8");
    assert_eq!(IBig::from(-3000i32).to_radix_str_uppercase(16), "-BB8");
    assert_eq!(IBig::from(3000i32).to_radix_str(32), "2to");
    assert_eq!(IBig::from(-3000i32).to_radix_str(32), "-2to");
    assert_eq!(IBig::from(3000i32).to_radix_str_uppercase(32), "2TO");
    assert_eq!(IBig::from(-3000i32).to_radix_str_uppercase(32), "-2TO");
}
