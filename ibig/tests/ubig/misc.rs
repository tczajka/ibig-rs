//! Integration tests for miscellaneous `UBig` trait implementations.

use ibig::UBig;

#[test]
fn default() {
    assert_eq!(UBig::default(), UBig::ZERO);
}
