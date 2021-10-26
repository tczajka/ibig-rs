//! Division operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    div,
    ibig::IBig,
    memory::MemoryAllocation,
    ops::{Abs, DivEuclid, DivRem, DivRemEuclid, RemEuclid},
    primitive::PrimitiveUnsigned,
    shift,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Div, DivAssign, Rem, RemAssign},
};

impl Div<UBig> for UBig {
    type Output = UBig;

    fn div(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(word0, word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0, buffer1)
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl Div<&UBig> for UBig {
    type Output = UBig;

    fn div(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::div_word(word0, *word1),
                Large(_) => UBig::from_word(0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::div_large_word(buffer0, *word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_large(buffer0, buffer1.clone())
                    } else {
                        UBig::from_word(0)
                    }
                }
            },
        }
    }
}

impl Div<UBig> for &UBig {
    type Output = UBig;

    fn div(self, rhs: UBig) -> UBig {
        match self.repr() {
            Small(word0) => match rhs.into_repr() {
                Small(word1) => UBig::div_word(*word0, word1),
                Large(_) => UBig::from_word(0),
            },
            Large(buffer0) => match rhs.into_repr() {
                Small(word1) => UBig::div_large_word(buffer0.clone(), word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_large(buffer0.clone(), buffer1)
                    } else {
                        UBig::from_word(0)
                    }
                }
            },
        }
    }
}

impl Div<&UBig> for &UBig {
    type Output = UBig;

    fn div(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(*word0, *word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0.clone(), buffer1.clone())
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl DivAssign<UBig> for UBig {
    fn div_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&UBig> for UBig {
    fn div_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<UBig> for UBig {
    type Output = UBig;

    fn rem(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(word0, word1),
            (Small(word0), Large(_)) => UBig::from_word(word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(&buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0, buffer1)
                } else {
                    buffer0.into()
                }
            }
        }
    }
}

impl Rem<&UBig> for UBig {
    type Output = UBig;

    fn rem(self, rhs: &UBig) -> UBig {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::rem_word(word0, *word1),
                Large(_) => UBig::from_word(word0),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::rem_large_word(&buffer0, *word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::rem_large(buffer0, buffer1.clone())
                    } else {
                        buffer0.into()
                    }
                }
            },
        }
    }
}

impl Rem<UBig> for &UBig {
    type Output = UBig;

    fn rem(self, rhs: UBig) -> UBig {
        match self.repr() {
            Small(word0) => match rhs.into_repr() {
                Small(word1) => UBig::rem_word(*word0, word1),
                Large(_) => UBig::from_word(*word0),
            },
            Large(buffer0) => match rhs.into_repr() {
                Small(word1) => UBig::rem_large_word(buffer0, word1),
                Large(mut buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::rem_large(buffer0.clone(), buffer1)
                    } else {
                        // Reuse buffer1 for the remainder.
                        buffer1.resizing_clone_from(buffer0);
                        buffer1.into()
                    }
                }
            },
        }
    }
}

impl Rem<&UBig> for &UBig {
    type Output = UBig;

    fn rem(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(*word0, *word1),
            (Small(word0), Large(_)) => UBig::from_word(*word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0.clone(), buffer1.clone())
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl RemAssign<UBig> for UBig {
    fn rem_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&UBig> for UBig {
    fn rem_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: UBig) -> (UBig, UBig) {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(word0, word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0, buffer1)
                } else {
                    (UBig::from_word(0), buffer0.into())
                }
            }
        }
    }
}

impl DivRem<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: &UBig) -> (UBig, UBig) {
        match self.into_repr() {
            Small(word0) => match rhs.repr() {
                Small(word1) => UBig::div_rem_word(word0, *word1),
                Large(_) => (UBig::from_word(0), UBig::from_word(word0)),
            },
            Large(buffer0) => match rhs.repr() {
                Small(word1) => UBig::div_rem_large_word(buffer0, *word1),
                Large(buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_rem_large(buffer0, buffer1.clone())
                    } else {
                        (UBig::from_word(0), buffer0.into())
                    }
                }
            },
        }
    }
}

impl DivRem<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: UBig) -> (UBig, UBig) {
        match self.repr() {
            Small(word0) => match rhs.into_repr() {
                Small(word1) => UBig::div_rem_word(*word0, word1),
                Large(_) => (UBig::from_word(0), UBig::from_word(*word0)),
            },
            Large(buffer0) => match rhs.into_repr() {
                Small(word1) => UBig::div_rem_large_word(buffer0.clone(), word1),
                Large(mut buffer1) => {
                    if buffer0.len() >= buffer1.len() {
                        UBig::div_rem_large(buffer0.clone(), buffer1)
                    } else {
                        // Reuse buffer1 for the remainder.
                        buffer1.resizing_clone_from(buffer0);
                        (UBig::from_word(0), buffer1.into())
                    }
                }
            },
        }
    }
}

impl DivRem<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem(self, rhs: &UBig) -> (UBig, UBig) {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(*word0, *word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(*word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0.clone(), buffer1.clone())
                } else {
                    (UBig::from_word(0), self.clone())
                }
            }
        }
    }
}

impl DivEuclid<UBig> for UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<UBig> for &UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for &UBig {
    type Output = UBig;

    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl RemEuclid<UBig> for UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<UBig> for &UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for &UBig {
    type Output = UBig;

    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl DivRemEuclid<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl Div<IBig> for IBig {
    type Output = IBig;

    fn div(self, rhs: IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<&IBig> for IBig {
    type Output = IBig;

    fn div(self, rhs: &IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<IBig> for &IBig {
    type Output = IBig;

    fn div(self, rhs: IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<&IBig> for &IBig {
    type Output = IBig;

    fn div(self, rhs: &IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl DivAssign<IBig> for IBig {
    fn div_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&IBig> for IBig {
    fn div_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<IBig> for IBig {
    type Output = IBig;

    fn rem(self, rhs: IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (_, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<&IBig> for IBig {
    type Output = IBig;

    fn rem(self, rhs: &IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = self.into_sign_magnitude();
        let mag1 = rhs.magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<IBig> for &IBig {
    type Output = IBig;

    fn rem(self, rhs: IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (_, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<&IBig> for &IBig {
    type Output = IBig;

    fn rem(self, rhs: &IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let mag1 = rhs.magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl RemAssign<IBig> for IBig {
    fn rem_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&IBig> for IBig {
    fn rem_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<&IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: &IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = rhs.into_sign_magnitude();
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<&IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem(self, rhs: &IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivEuclid<IBig> for IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: IBig) -> IBig {
        let s = rhs.signum();
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - s,
        }
    }
}

impl DivEuclid<&IBig> for IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: &IBig) -> IBig {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - rhs.signum(),
        }
    }
}

impl DivEuclid<IBig> for &IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: IBig) -> IBig {
        let s = rhs.signum();
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - s,
        }
    }
}

impl DivEuclid<&IBig> for &IBig {
    type Output = IBig;

    fn div_euclid(self, rhs: &IBig) -> IBig {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - rhs.signum(),
        }
    }
}

impl RemEuclid<IBig> for IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: IBig) -> IBig {
        let r = self % &rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl RemEuclid<&IBig> for IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: &IBig) -> IBig {
        let r = self % rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl RemEuclid<IBig> for &IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: IBig) -> IBig {
        let r = self % &rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl RemEuclid<&IBig> for &IBig {
    type Output = IBig;

    fn rem_euclid(self, rhs: &IBig) -> IBig {
        let r = self % rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl DivRemEuclid<IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(&rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

impl DivRemEuclid<&IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: &IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

impl DivRemEuclid<IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(&rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

impl DivRemEuclid<&IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    fn div_rem_euclid(self, rhs: &IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

macro_rules! impl_div_ubig_unsigned {
    ($t:ty) => {
        impl Div<$t> for UBig {
            type Output = UBig;

            fn div(self, rhs: $t) -> UBig {
                self.div_unsigned(rhs)
            }
        }

        impl Div<&$t> for UBig {
            type Output = UBig;

            fn div(self, rhs: &$t) -> UBig {
                self.div_unsigned(*rhs)
            }
        }

        impl Div<$t> for &UBig {
            type Output = UBig;

            fn div(self, rhs: $t) -> UBig {
                self.ref_div_unsigned(rhs)
            }
        }

        impl Div<&$t> for &UBig {
            type Output = UBig;

            fn div(self, rhs: &$t) -> UBig {
                self.ref_div_unsigned(*rhs)
            }
        }

        impl DivAssign<$t> for UBig {
            fn div_assign(&mut self, rhs: $t) {
                self.div_assign_unsigned(rhs)
            }
        }

        impl DivAssign<&$t> for UBig {
            fn div_assign(&mut self, rhs: &$t) {
                self.div_assign_unsigned(*rhs)
            }
        }

        impl Rem<$t> for UBig {
            type Output = $t;

            fn rem(self, rhs: $t) -> $t {
                self.rem_unsigned(rhs)
            }
        }

        impl Rem<&$t> for UBig {
            type Output = $t;

            fn rem(self, rhs: &$t) -> $t {
                self.rem_unsigned(*rhs)
            }
        }

        impl Rem<$t> for &UBig {
            type Output = $t;

            fn rem(self, rhs: $t) -> $t {
                self.ref_rem_unsigned(rhs)
            }
        }

        impl Rem<&$t> for &UBig {
            type Output = $t;

            fn rem(self, rhs: &$t) -> $t {
                self.ref_rem_unsigned(*rhs)
            }
        }

        impl RemAssign<$t> for UBig {
            fn rem_assign(&mut self, rhs: $t) {
                self.rem_assign_unsigned(rhs)
            }
        }

        impl RemAssign<&$t> for UBig {
            fn rem_assign(&mut self, rhs: &$t) {
                self.rem_assign_unsigned(*rhs)
            }
        }

        impl DivRem<$t> for UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            fn div_rem(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_unsigned(rhs)
            }
        }

        impl DivRem<&$t> for UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            fn div_rem(self, rhs: &$t) -> (UBig, $t) {
                self.div_rem_unsigned(*rhs)
            }
        }

        impl DivRem<$t> for &UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            fn div_rem(self, rhs: $t) -> (UBig, $t) {
                self.ref_div_rem_unsigned(rhs)
            }
        }

        impl DivRem<&$t> for &UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            fn div_rem(self, rhs: &$t) -> (UBig, $t) {
                self.ref_div_rem_unsigned(*rhs)
            }
        }
    };
}

impl_div_ubig_unsigned!(u8);
impl_div_ubig_unsigned!(u16);
impl_div_ubig_unsigned!(u32);
impl_div_ubig_unsigned!(u64);
impl_div_ubig_unsigned!(u128);
impl_div_ubig_unsigned!(usize);

impl UBig {
    /// `lhs / rhs`
    fn div_word(lhs: Word, rhs: Word) -> UBig {
        match lhs.checked_div(rhs) {
            Some(res) => UBig::from_word(res),
            None => panic_divide_by_0(),
        }
    }

    /// `lhs % rhs`
    fn rem_word(lhs: Word, rhs: Word) -> UBig {
        match lhs.checked_rem(rhs) {
            Some(res) => UBig::from_word(res),
            None => panic_divide_by_0(),
        }
    }

    /// (lhs / rhs, lhs % rhs)
    fn div_rem_word(lhs: Word, rhs: Word) -> (UBig, UBig) {
        // If division works, remainder also works.
        match lhs.checked_div(rhs) {
            Some(res) => (UBig::from_word(res), UBig::from_word(lhs % rhs)),
            None => panic_divide_by_0(),
        }
    }

    /// `lhs / rhs`
    fn div_large_word(lhs: Buffer, rhs: Word) -> UBig {
        let (q, _) = UBig::div_rem_large_word(lhs, rhs);
        q
    }

    /// `lhs % rhs`
    fn rem_large_word(lhs: &[Word], rhs: Word) -> UBig {
        if rhs == 0 {
            panic_divide_by_0();
        }
        UBig::from_word(div::rem_by_word(lhs, rhs))
    }

    /// (buffer / rhs, buffer % rhs)
    fn div_rem_large_word(mut buffer: Buffer, rhs: Word) -> (UBig, UBig) {
        if rhs == 0 {
            panic_divide_by_0();
        }
        let rem = div::div_by_word_in_place(&mut buffer, rhs);
        (buffer.into(), UBig::from_word(rem))
    }

    /// `lhs / rhs`
    fn div_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let _shift = UBig::div_rem_in_lhs(&mut lhs, &mut rhs);
        lhs.erase_front(rhs.len());
        lhs.into()
    }

    /// `lhs % rhs`
    fn rem_large(mut lhs: Buffer, mut rhs: Buffer) -> UBig {
        let shift = UBig::div_rem_in_lhs(&mut lhs, &mut rhs);
        let n = rhs.len();
        rhs.copy_from_slice(&lhs[..n]);
        let low_bits = shift::shr_in_place(&mut rhs, shift);
        debug_assert!(low_bits == 0);
        rhs.into()
    }

    /// `(lhs / rhs, lhs % rhs)`
    fn div_rem_large(mut lhs: Buffer, mut rhs: Buffer) -> (UBig, UBig) {
        let shift = UBig::div_rem_in_lhs(&mut lhs, &mut rhs);
        let n = rhs.len();
        rhs.copy_from_slice(&lhs[..n]);
        let low_bits = shift::shr_in_place(&mut rhs, shift);
        debug_assert!(low_bits == 0);
        lhs.erase_front(n);
        (lhs.into(), rhs.into())
    }

    /// lhs = (lhs / rhs, lhs % rhs)
    ///
    /// Returns shift.
    fn div_rem_in_lhs(lhs: &mut Buffer, rhs: &mut Buffer) -> u32 {
        let (shift, fast_div_rhs_top) = div::normalize_large(rhs);
        let lhs_carry = shift::shl_in_place(lhs, shift);
        if lhs_carry != 0 {
            lhs.push_may_reallocate(lhs_carry);
        }
        let mut allocation =
            MemoryAllocation::new(div::memory_requirement_exact(lhs.len(), rhs.len()));
        let mut memory = allocation.memory();
        let overflow = div::div_rem_in_place(lhs, rhs, fast_div_rhs_top, &mut memory);
        if overflow {
            lhs.push_may_reallocate(1);
        }
        shift
    }

    fn div_unsigned<T: PrimitiveUnsigned>(self, rhs: T) -> UBig {
        self / UBig::from_unsigned(rhs)
    }

    fn ref_div_unsigned<T: PrimitiveUnsigned>(&self, rhs: T) -> UBig {
        self / UBig::from_unsigned(rhs)
    }

    fn div_assign_unsigned<T: PrimitiveUnsigned>(&mut self, rhs: T) {
        self.div_assign(UBig::from_unsigned(rhs))
    }

    fn rem_unsigned<T: PrimitiveUnsigned>(self, rhs: T) -> T {
        (self % UBig::from_unsigned(rhs)).try_to_unsigned().unwrap()
    }

    fn ref_rem_unsigned<T: PrimitiveUnsigned>(&self, rhs: T) -> T {
        (self % UBig::from_unsigned(rhs)).try_to_unsigned().unwrap()
    }

    fn rem_assign_unsigned<T: PrimitiveUnsigned>(&mut self, rhs: T) {
        self.rem_assign(UBig::from_unsigned(rhs))
    }

    fn div_rem_unsigned<T: PrimitiveUnsigned>(self, rhs: T) -> (UBig, T) {
        let (q, r) = self.div_rem(UBig::from_unsigned(rhs));
        (q, r.try_to_unsigned().unwrap())
    }

    fn ref_div_rem_unsigned<T: PrimitiveUnsigned>(&self, rhs: T) -> (UBig, T) {
        let (q, r) = self.div_rem(UBig::from_unsigned(rhs));
        (q, r.try_to_unsigned().unwrap())
    }
}

fn panic_divide_by_0() -> ! {
    panic!("divide by 0")
}
