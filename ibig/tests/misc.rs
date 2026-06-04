//! Integration tests for miscellaneous `UBig` and `IBig` trait implementations.

use ibig::{IBig, UBig};

#[test]
fn ubig_default() {
    assert_eq!(UBig::default(), UBig::ZERO);
}

#[test]
fn ibig_default() {
    assert_eq!(IBig::default(), IBig::ZERO);
}
