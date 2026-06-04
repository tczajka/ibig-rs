//! Internal traits describing unary and binary operations.

use crate::repr::{AsDigits, Digits};
use ibig_core::Digit;

/// A unary operation on a value of type `T`.
///
/// The operand can be borrowed or owned.
pub(crate) trait UnaryOp<T> {
    /// The operand is borrowed.
    fn apply_ref(value: &T) -> T;

    /// The operand is owned.
    fn apply_val(value: T) -> T;
}

/// A unary operation implemented on the digit representation of a number.
///
/// The operand appears in one of three forms: a single digit (`digit`), a borrowed slice
/// (`ref`), or an owned buffer (`val`).
pub(crate) trait UnaryOpDigits<T: AsDigits> {
    /// The operand is a single digit.
    fn apply_digit(operand: T::SingleDigit) -> T;

    /// The operand is a borrowed slice.
    fn apply_ref(operand: &[Digit]) -> T;

    /// The operand is an owned buffer.
    fn apply_val(operand: Digits) -> T;
}

/// Every [`UnaryOpDigits`] induces a [`UnaryOp`].
impl<T: AsDigits, Op: UnaryOpDigits<T>> UnaryOp<T> for Op {
    #[inline]
    fn apply_ref(value: &T) -> T {
        match value.try_to_digit() {
            Some(d) => <Op as UnaryOpDigits<T>>::apply_digit(d),
            None => <Op as UnaryOpDigits<T>>::apply_ref(value.as_digits()),
        }
    }

    #[inline]
    fn apply_val(value: T) -> T {
        match value.try_to_digit() {
            Some(d) => <Op as UnaryOpDigits<T>>::apply_digit(d),
            None => <Op as UnaryOpDigits<T>>::apply_val(value.into_digits()),
        }
    }
}

/// Implements a unary operator for a value type `$t`, deriving the owned and borrowed forms from
/// an [`UnaryOp`] implemented by the marker type `$op`.
///
/// `$trait`/`$method` is the operator trait; it must be in scope at the call site.
macro_rules! impl_unary_operator {
    ($t:ty, $trait:ident :: $method:ident, $op:ty) => {
        impl $trait for $t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                <$op as $crate::ops::UnaryOp<$t>>::apply_val(self)
            }
        }

        impl $trait for &$t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                <$op as $crate::ops::UnaryOp<$t>>::apply_ref(self)
            }
        }
    };
}

pub(crate) use impl_unary_operator;

/// A binary operation on values of type `T`.
///
/// Each operand appears can be borrowed or owned.
pub(crate) trait BinaryOp<T: Default> {
    /// Both operands are borrowed.
    fn apply_ref_ref(lhs: &T, rhs: &T) -> T;

    /// Left operand is borrowed, right operand owned.
    fn apply_ref_val(lhs: &T, rhs: T) -> T;

    /// Left operand is owned, right operand borrowed.
    fn apply_val_ref(lhs: T, rhs: &T) -> T;

    /// Both operands are owned.
    fn apply_val_val(lhs: T, rhs: T) -> T;
}

/// A binary operation implemented on the digit representation of a number.
///
/// Each operand appears in one of three forms: a single digit (`digit`), a borrowed slice
/// (`ref`), or an owned buffer (`val`).
pub(crate) trait BinaryOpDigits<T: AsDigits> {
    /// Both operands are single digits.
    fn apply_digit_digit(lhs: T::SingleDigit, rhs: T::SingleDigit) -> T;

    /// Left operand is a single digit, right operand a borrowed slice.
    fn apply_digit_ref(lhs: T::SingleDigit, rhs: &[Digit]) -> T;

    /// Left operand is a single digit, right operand an owned buffer.
    fn apply_digit_val(lhs: T::SingleDigit, rhs: Digits) -> T;

    /// Left operand is a borrowed slice, right operand a single digit.
    fn apply_ref_digit(lhs: &[Digit], rhs: T::SingleDigit) -> T;

    /// Both operands are borrowed slices.
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> T;

    /// Left operand is a borrowed slice, right operand an owned buffer.
    fn apply_ref_val(lhs: &[Digit], rhs: Digits) -> T;

    /// Left operand is an owned buffer, right operand a single digit.
    fn apply_val_digit(lhs: Digits, rhs: T::SingleDigit) -> T;

    /// Left operand is an owned buffer, right operand a borrowed slice.
    fn apply_val_ref(lhs: Digits, rhs: &[Digit]) -> T;

    /// Both operands are owned buffers.
    fn apply_val_val(lhs: Digits, rhs: Digits) -> T;
}

/// A commutative binary operation implemented on the digit representation of a number.
///
/// Each operand appears in one of three forms: a single digit (`digit`), a borrowed slice
/// (`ref`), or an owned buffer (`val`).
pub(crate) trait CommutativeBinaryOpDigits<T: AsDigits> {
    /// Both operands are single digits.
    fn apply_digit_digit(lhs: T::SingleDigit, rhs: T::SingleDigit) -> T;

    /// One operand is a borrowed slice, the other a single digit.
    fn apply_ref_digit(lhs: &[Digit], rhs: T::SingleDigit) -> T;

    /// Both operands are borrowed slices.
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> T;

    /// One operand is an owned buffer, the other a single digit.
    fn apply_val_digit(lhs: Digits, rhs: T::SingleDigit) -> T;

    /// One operand is an owned buffer, the other a borrowed slice.
    fn apply_val_ref(lhs: Digits, rhs: &[Digit]) -> T;

    /// Both operands are owned buffers.
    fn apply_val_val(lhs: Digits, rhs: Digits) -> T;
}

/// Every [`BinaryOpDigits`] induces a [`BinaryOp`].
impl<T: AsDigits, Op: BinaryOpDigits<T>> BinaryOp<T> for Op {
    #[inline]
    fn apply_ref_ref(lhs: &T, rhs: &T) -> T {
        match (lhs.try_to_digit(), rhs.try_to_digit()) {
            (Some(a), Some(b)) => <Op as BinaryOpDigits<T>>::apply_digit_digit(a, b),
            (Some(a), None) => <Op as BinaryOpDigits<T>>::apply_digit_ref(a, rhs.as_digits()),
            (None, Some(b)) => <Op as BinaryOpDigits<T>>::apply_ref_digit(lhs.as_digits(), b),
            (None, None) => {
                <Op as BinaryOpDigits<T>>::apply_ref_ref(lhs.as_digits(), rhs.as_digits())
            }
        }
    }

    #[inline]
    fn apply_ref_val(lhs: &T, rhs: T) -> T {
        match (lhs.try_to_digit(), rhs.try_to_digit()) {
            (Some(a), Some(b)) => <Op as BinaryOpDigits<T>>::apply_digit_digit(a, b),
            (Some(a), None) => <Op as BinaryOpDigits<T>>::apply_digit_val(a, rhs.into_digits()),
            (None, Some(b)) => <Op as BinaryOpDigits<T>>::apply_ref_digit(lhs.as_digits(), b),
            (None, None) => {
                <Op as BinaryOpDigits<T>>::apply_ref_val(lhs.as_digits(), rhs.into_digits())
            }
        }
    }

    #[inline]
    fn apply_val_ref(lhs: T, rhs: &T) -> T {
        match (lhs.try_to_digit(), rhs.try_to_digit()) {
            (Some(a), Some(b)) => <Op as BinaryOpDigits<T>>::apply_digit_digit(a, b),
            (Some(a), None) => <Op as BinaryOpDigits<T>>::apply_digit_ref(a, rhs.as_digits()),
            (None, Some(b)) => <Op as BinaryOpDigits<T>>::apply_val_digit(lhs.into_digits(), b),
            (None, None) => {
                <Op as BinaryOpDigits<T>>::apply_val_ref(lhs.into_digits(), rhs.as_digits())
            }
        }
    }

    #[inline]
    fn apply_val_val(lhs: T, rhs: T) -> T {
        match (lhs.try_to_digit(), rhs.try_to_digit()) {
            (Some(a), Some(b)) => <Op as BinaryOpDigits<T>>::apply_digit_digit(a, b),
            (Some(a), None) => <Op as BinaryOpDigits<T>>::apply_digit_val(a, rhs.into_digits()),
            (None, Some(b)) => <Op as BinaryOpDigits<T>>::apply_val_digit(lhs.into_digits(), b),
            (None, None) => {
                <Op as BinaryOpDigits<T>>::apply_val_val(lhs.into_digits(), rhs.into_digits())
            }
        }
    }
}

/// Every [`CommutativeBinaryOpDigits`] is a [`BinaryOpDigits`].
impl<T: AsDigits, Op: CommutativeBinaryOpDigits<T>> BinaryOpDigits<T> for Op {
    #[inline]
    fn apply_digit_digit(lhs: T::SingleDigit, rhs: T::SingleDigit) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_digit_digit(lhs, rhs)
    }

    #[inline]
    fn apply_digit_ref(lhs: T::SingleDigit, rhs: &[Digit]) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_ref_digit(rhs, lhs)
    }

    #[inline]
    fn apply_digit_val(lhs: T::SingleDigit, rhs: Digits) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_val_digit(rhs, lhs)
    }

    #[inline]
    fn apply_ref_digit(lhs: &[Digit], rhs: T::SingleDigit) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_ref_digit(lhs, rhs)
    }

    #[inline]
    fn apply_ref_ref(lhs: &[Digit], rhs: &[Digit]) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_ref_ref(lhs, rhs)
    }

    #[inline]
    fn apply_ref_val(lhs: &[Digit], rhs: Digits) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_val_ref(rhs, lhs)
    }

    #[inline]
    fn apply_val_digit(lhs: Digits, rhs: T::SingleDigit) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_val_digit(lhs, rhs)
    }

    #[inline]
    fn apply_val_ref(lhs: Digits, rhs: &[Digit]) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_val_ref(lhs, rhs)
    }

    #[inline]
    fn apply_val_val(lhs: Digits, rhs: Digits) -> T {
        <Op as CommutativeBinaryOpDigits<T>>::apply_val_val(lhs, rhs)
    }
}

/// Implements a binary operator and its assigning counterpart for a value type `$t`, deriving
/// every owned/borrowed operand combination from an [`BinaryOp`] implemented by the marker type
/// `$op`.
///
/// `$trait`/`$method` and `$assign_trait`/`$assign_method` are the operator and assigning-operator
/// traits; they must be in scope at the call site.
macro_rules! impl_binary_operator {
    ($t:ty, $trait:ident :: $method:ident, $assign_trait:ident :: $assign_method:ident, $op:ty) => {
        impl $trait<$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: $t) -> $t {
                <$op as $crate::ops::BinaryOp<$t>>::apply_val_val(self, rhs)
            }
        }

        impl $trait<&$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> $t {
                <$op as $crate::ops::BinaryOp<$t>>::apply_val_ref(self, rhs)
            }
        }

        impl $trait<$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: $t) -> $t {
                <$op as $crate::ops::BinaryOp<$t>>::apply_ref_val(self, rhs)
            }
        }

        impl $trait<&$t> for &$t {
            type Output = $t;

            #[inline]
            fn $method(self, rhs: &$t) -> $t {
                <$op as $crate::ops::BinaryOp<$t>>::apply_ref_ref(self, rhs)
            }
        }

        impl $assign_trait<$t> for $t {
            #[inline]
            fn $assign_method(&mut self, rhs: $t) {
                *self =
                    <$op as $crate::ops::BinaryOp<$t>>::apply_val_val(::core::mem::take(self), rhs);
            }
        }

        impl $assign_trait<&$t> for $t {
            #[inline]
            fn $assign_method(&mut self, rhs: &$t) {
                *self =
                    <$op as $crate::ops::BinaryOp<$t>>::apply_val_ref(::core::mem::take(self), rhs);
            }
        }
    };
}

pub(crate) use impl_binary_operator;
