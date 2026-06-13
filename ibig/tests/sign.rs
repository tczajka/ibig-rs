//! Integration tests for `IBig` sign operations.

use ibig::IBig;
use ibig::proptest::ibig_up_to_bits;
use proptest::prelude::*;

#[test]
fn is_negative() {
    assert!(!IBig::ZERO.is_negative());
    assert!(!IBig::from(5i8).is_negative());
    assert!(IBig::from(-5i8).is_negative());
    assert!(!IBig::from(u64::MAX).is_negative());
    assert!(IBig::from(-1i128 << 100).is_negative());
}

#[test]
fn is_positive() {
    assert!(!IBig::ZERO.is_positive());
    assert!(IBig::from(5i8).is_positive());
    assert!(!IBig::from(-5i8).is_positive());
    assert!(IBig::from(u64::MAX).is_positive());
    assert!(!IBig::from(-1i128 << 100).is_positive());
}

#[test]
fn signum() {
    assert_eq!(IBig::ZERO.signum(), IBig::ZERO);
    assert_eq!(IBig::from(5i8).signum(), IBig::from(1i8));
    assert_eq!(IBig::from(-5i8).signum(), IBig::from(-1i8));
    assert_eq!(IBig::from(u64::MAX).signum(), IBig::from(1i8));
    assert_eq!(IBig::from(1i128 << 100).signum(), IBig::from(1i8));
    assert_eq!(IBig::from(-1i128 << 100).signum(), IBig::from(-1i8));
}

#[test]
fn neg() {
    assert_eq!(-IBig::from(5), IBig::from(-5));
    assert_eq!(-IBig::from(-5), IBig::from(5));
    assert_eq!(-IBig::ZERO, IBig::ZERO);
    // Negation of a borrowed value.
    assert_eq!(-&IBig::from(7), IBig::from(-7));

    // Negating the most-negative single-digit value needs an extra digit.
    assert_eq!(-IBig::from(i64::MIN), IBig::from(1) << 63);
    assert_eq!(-IBig::from(i16::MIN), IBig::from(1) << 15);

    // Multi-digit values.
    assert_eq!(-(IBig::from(1) << 200), IBig::from(-1) << 200);
    assert_eq!(-(IBig::from(-1) << 200), IBig::from(1) << 200);
}

proptest! {
    // Double negation is the identity, and negation equals subtracting from zero.
    #[test]
    fn neg_props(a in ibig_up_to_bits(300)) {
        prop_assert_eq!(&-(-&a), &a);
        prop_assert_eq!(&-(-a.clone()), &a);
        prop_assert_eq!(-&a, IBig::ZERO - &a);
    }
}
