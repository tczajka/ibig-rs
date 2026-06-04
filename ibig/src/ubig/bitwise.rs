//! Bitwise operators for [`UBig`].

use crate::UBig;
use crate::macros::forward_commutative_ref_val;
use core::ops::BitAnd;

impl BitAnd<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn bitand(self, rhs: UBig) -> UBig {
        if let Some(a) = bitand_fast_path(&self, &rhs) {
            return a;
        }

        // Reuse storage from shorter operand.
        if self.as_digits().len() <= rhs.as_digits().len() {
            bitand_owned_ref(self, &rhs)
        } else {
            bitand_owned_ref(rhs, &self)
        }
    }
}

impl BitAnd<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn bitand(self, rhs: &UBig) -> UBig {
        if let Some(a) = bitand_fast_path(&self, rhs) {
            return a;
        }
        bitand_owned_ref(self, rhs)
    }
}

forward_commutative_ref_val!(UBig, BitAnd::bitand);

impl BitAnd<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn bitand(self, rhs: &UBig) -> UBig {
        if let Some(a) = bitand_fast_path(self, rhs) {
            return a;
        }

        // Clone shorter operand.
        if self.as_digits().len() <= rhs.as_digits().len() {
            bitand_owned_ref(self.clone(), rhs)
        } else {
            bitand_owned_ref(rhs.clone(), self)
        }
    }
}

fn bitand_fast_path(a: &UBig, b: &UBig) -> Option<UBig> {
    let digit = match (a.try_to_digit(), b.try_to_digit()) {
        (Some(a), Some(b)) => Some(a & b),
        (Some(a), None) => Some(a & b.as_digits()[0]),
        (None, Some(b)) => Some(a.as_digits()[0] & b),
        (None, None) => None,
    };
    digit.map(UBig::from_digit)
}

fn bitand_owned_ref(a: UBig, b: &UBig) -> UBig {
    let mut a = a.into_digits();
    let b = b.as_digits();
    let n = a.len().min(b.len());
    a.truncate(n);
    ibig_core::and_same_len_in_place(&mut a, &b[..n]);
    UBig::from_digits(a)
}
