//! Integration tests for miscellaneous `IBig` trait implementations.

use ibig::IBig;

#[test]
fn default() {
    assert_eq!(IBig::default(), IBig::ZERO);
}
