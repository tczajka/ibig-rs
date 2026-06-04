//! Integration tests for `IBig` sign operations.

use ibig::IBig;

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
