use crate::{
    add::add_same_len_in_place,
    buffer::Buffer,
    mul::sub_mul_word_same_len_in_place,
    primitive::{double_word, extend_word, split_double_word, Word},
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Div, DivAssign, Rem, RemAssign},
};

/// Compute quotient and remainder at the same time.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(ubig!(23).div_rem(ubig!(10)), (ubig!(2), ubig!(3)));
/// ```
pub trait DivRem<Rhs = Self> {
    type OutputDiv;
    type OutputRem;

    fn div_rem(self, rhs: Rhs) -> (Self::OutputDiv, Self::OutputRem);
}

/// Compute Euclidean quotient.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// // TODO: Make this work.
/// // assert_eq!(ibig!(-23).div_euclid(ibig!(10)), ibig!(-3));
/// ```
pub trait DivEuclid<Rhs = Self> {
    type Output;

    fn div_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Compute Euclidean remainder.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// // TODO: Make this work.
/// // assert_eq!(ibig!(-23).rem_euclid(ibig!(10)), ibig!(7));
/// ```
pub trait RemEuclid<Rhs = Self> {
    type Output;

    fn rem_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Compute Euclidean quotient and remainder at the same time.
///
/// # Example
/// ```
/// # use ibig::prelude::*;
/// // TODO: Make this work.
/// // assert_eq!(ibig!(-23).div_rem_euclid(ibig!(10)), (ibig!(-3), ibig!(7)));
/// ```
pub trait DivRemEuclid<Rhs = Self> {
    type OutputDiv;
    type OutputRem;

    fn div_rem_euclid(self, rhs: Rhs) -> (Self::OutputDiv, Self::OutputRem);
}

impl Div<UBig> for UBig {
    type Output = UBig;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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
    #[inline]
    fn div_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&UBig> for UBig {
    #[inline]
    fn div_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<UBig> for UBig {
    type Output = UBig;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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
    #[inline]
    fn rem_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&UBig> for UBig {
    #[inline]
    fn rem_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl RemEuclid<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl DivRemEuclid<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

fn panic_divide_by_0() -> ! {
    panic!("divide by 0")
}

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
        let mut rem: Word = 0;
        for word in lhs.iter().rev() {
            let (v0, v1) = split_double_word(double_word(*word, rem) % extend_word(rhs));
            debug_assert!(v1 == 0);
            rem = v0;
        }
        UBig::from_word(rem)
    }

    /// (buffer / rhs, buffer % rhs)
    fn div_rem_large_word(mut buffer: Buffer, rhs: Word) -> (UBig, UBig) {
        if rhs == 0 {
            panic_divide_by_0();
        }
        let mut rem: Word = 0;
        for word in buffer.iter_mut().rev() {
            let a = double_word(*word, rem);
            let (q0, q1) = split_double_word(a / extend_word(rhs));
            debug_assert!(q1 == 0);
            let (r0, r1) = split_double_word(a % extend_word(rhs));
            debug_assert!(r1 == 0);
            *word = q0;
            rem = r0;
        }
        (buffer.into(), UBig::from_word(rem))
    }

    /// `lhs / rhs`
    fn div_large(lhs: Buffer, rhs: Buffer) -> UBig {
        debug_assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
        let (q, _) = UBig::div_rem_large(lhs, rhs);
        q
    }

    /// `lhs % rhs`
    fn rem_large(lhs: Buffer, rhs: Buffer) -> UBig {
        debug_assert!(lhs.len() >= rhs.len() && rhs.len() >= 2);
        let (_, r) = UBig::div_rem_large(lhs, rhs);
        r
    }

    /// `(lhs / rhs, lhs % rhs)`
    fn div_rem_large(lhs: Buffer, rhs: Buffer) -> (UBig, UBig) {
        debug_assert!(lhs.len() >= rhs.len() && rhs.len() >= 2 && *rhs.last().unwrap() != 0);
        // Normalize the divisor: leading bit must be 1.
        let shift = rhs.last().unwrap().leading_zeros();
        if let Large(buffer0) = (UBig::from(lhs) << shift).into_repr() {
            if let Large(buffer1) = (UBig::from(rhs) << shift).into_repr() {
                // Always use the simple algorithm for now.
                let (q, r) = UBig::div_rem_simple(buffer0, &buffer1);
                return (q, r >> shift);
            }
        }
        unreachable!()
    }

    /// Simple division algorithm.
    ///
    /// `lhs` must have at least the same length as `rhs`.
    /// `rhs` must have at least 2 words with the leading bit 1.
    ///
    /// Returns (quotient, remainder).
    fn div_rem_simple(mut lhs: Buffer, rhs: &[Word]) -> (UBig, UBig) {
        // The Art of Computer Programming, algorithm 4.3.1D.

        let n = rhs.len();
        debug_assert!(lhs.len() >= n && n >= 2);
        let rhs0 = rhs[n - 1];
        debug_assert!(rhs0.leading_zeros() == 0);
        let rhs1 = rhs[n - 2];

        // lhs0 is an extra word of `lhs` not stored in the buffer.
        let mut lhs0: Word = 0;
        let mut quotient = Buffer::allocate(lhs.len() - n + 1);
        quotient.push_zeros(lhs.len() - n + 1);

        while lhs.len() >= n {
            let m = lhs.len();
            let lhs1 = lhs[m - 1];
            let lhs2 = lhs[m - 2];

            // Approximate the next word of quotient by
            // q = floor([lhs0, lhs1] / rhs0)
            let lhs10 = double_word(lhs1, lhs0);
            let rhs0_extended = extend_word(rhs0);
            let mut q = lhs10 / rhs0_extended;
            let mut r = lhs10 % rhs0_extended;

            // exact_q = floor([lhs0, lhs1, ...] / [rhs0, ...])
            // [lhs0, lhs1, ...] / [rhs0, ...] < ([lhs0, lhs1] + [0..1)) / rhs0
            // exact_q <= floor(([lhs0, lhs1] + [0..1)) / rhs0) = q
            //
            // B = WORD_BITS, rhs0 >= 2^(B-1)
            //
            // [lhs0, lhs1, ...] / [rhs0, ...] > [lhs0, lhs1] / (rhs0 + 1)
            //   = [lhs0, lhs1] / rhs0 * (1 - 1 / (rhs0+1))
            //   >= q * (1 - 1/(rhs0+1)) = q - (q / (rhs0+1))
            //   >= q - (2^B+1) / 2^(B-1) > q-2
            // exact_q >= q-2
            //
            // Therefore q is never too small and at most 2 too large.

            // Now improve the approximation:
            // q' = min(floor([lhs0, lhs1, lhs2] / [rhs0, rhs1]), Word::MAX)
            // q'-1 <= exact_q <= q' <= q
            // Most of the time exact_q = q'.
            // (by the same reasoning as above, except with 2B instead of B).

            // We must decrease q at most twice.
            // [lhs0, lhs1] = q * rhs0 + r
            //
            // q must be decreased if q > Word::MAX or:
            // q-1 >= floor([lhs0, lhs1, lhs2] / [rhs0, rhs1])
            // q > [lhs0, lhs1, lhs2] / [rhs0, rhs1]
            // q * [rhs0, rhs1] > [lhs0, lhs1, lhs2]
            // q * rhs1 > [r, lhs2]
            loop {
                let (q_lo, q_hi) = split_double_word(q);
                let (r_lo, r_hi) = split_double_word(r);
                if q_hi != 0
                    || (r_hi == 0
                        && extend_word(q_lo) * extend_word(rhs1) > double_word(lhs2, r_lo))
                {
                    q -= 1;
                    r += rhs0_extended;
                } else {
                    break;
                }
            }
            let (mut q_lo, q_hi) = split_double_word(q);
            debug_assert!(q_hi == 0);

            // Subtract a multiple of rhs.
            let offset = lhs.len() - n;
            let borrow = sub_mul_word_same_len_in_place(&mut lhs[offset..], q_lo, &rhs);

            if borrow > lhs0 {
                // Rare case: q is too large (by 1).
                // Add a correction.
                q_lo -= 1;
                let carry = add_same_len_in_place(&mut lhs[offset..], &rhs);
                debug_assert!(carry && borrow - 1 == lhs0);
            }
            // lhs0 is now logically zeroed out
            quotient[m - n] = q_lo;
            lhs0 = lhs.pop().unwrap();
        }
        lhs.push(lhs0);
        (quotient.into(), lhs.into())
    }
}
