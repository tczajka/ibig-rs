//! Integration tests for the `UBig` addition operator.

use ibig::UBig;
use ibig::proptest::ubig_up_to_bits;
use proptest::prelude::*;

proptest! {
    // `UBig` addition matches `u128` addition, across every operand form.
    #[test]
    fn ubig_vs_u128(a: u128, b: u128) {
        let x = UBig::from(a);
        let y = UBig::from(b);
        let (low, carry) = a.overflowing_add(b);
        let mut sum = UBig::from(low);
        if carry {
            sum |= UBig::from(1u8) << 128;
        }

        prop_assert_eq!(&(x.clone() + y.clone()), &sum);
        prop_assert_eq!(&(x.clone() + &y), &sum);
        prop_assert_eq!(&(&x + y.clone()), &sum);
        prop_assert_eq!(&(&x + &y), &sum);
        let mut t = x.clone();
        t += y.clone();
        prop_assert_eq!(&t, &sum);
        let mut t = x.clone();
        t += &y;
        prop_assert_eq!(&t, &sum);
    }

    // Addition is commutative and associative, and zero is the identity.
    #[test]
    fn ubig_algebra(
        a in ubig_up_to_bits(300),
        b in ubig_up_to_bits(300),
        c in ubig_up_to_bits(300),
    ) {
        prop_assert_eq!(&a + &b, &b + &a);
        prop_assert_eq!((&a + &b) + &c, &a + (&b + &c));
        prop_assert_eq!(&(&a + UBig::ZERO), &a);
    }
}

#[test]
fn add_basic() {
    assert_eq!(UBig::from(2u8) + UBig::from(3u8), UBig::from(5u8));
    assert_eq!(UBig::ZERO + UBig::ZERO, UBig::ZERO);
    // A carry grows the value by a digit.
    assert_eq!(
        UBig::from(u64::MAX) + UBig::from(1u8),
        UBig::from(u128::from(u64::MAX) + 1)
    );
    // A carry propagates through many all-ones digits.
    let almost = UBig::from_le_bytes(&[0xff; 32]);
    let one_more = UBig::from(1u8) << 256;
    assert_eq!(almost + UBig::from(1u8), one_more);
}
