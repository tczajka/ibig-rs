//! Conversions between types.

use crate::{
    arch::Word,
    buffer::Buffer,
    ibig::IBig,
    primitive::{
        self, OutOfBoundsError, PrimitiveSigned, PrimitiveUnsigned, WORD_BITS, WORD_BYTES,
    },
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use alloc::vec::Vec;
use core::{
    borrow::Borrow,
    convert::{TryFrom, TryInto},
};

impl Default for UBig {
    /// Default value: 0.
    fn default() -> UBig {
        UBig::from_word(0)
    }
}

impl Default for IBig {
    /// Default value: 0.
    fn default() -> IBig {
        IBig::from(0u8)
    }
}

impl UBig {
    /// Construct from little-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(UBig::from_le_bytes(&[3, 2, 1]), ubig!(0x010203));
    /// ```
    pub fn from_le_bytes(bytes: &[u8]) -> UBig {
        if bytes.len() <= WORD_BYTES {
            // fast path
            UBig::from_word(primitive::word_from_le_bytes_partial(bytes))
        } else {
            UBig::from_le_bytes_large(bytes)
        }
    }

    fn from_le_bytes_large(bytes: &[u8]) -> UBig {
        debug_assert!(bytes.len() > WORD_BYTES);
        let mut buffer = Buffer::allocate((bytes.len() - 1) / WORD_BYTES + 1);
        let mut chunks = bytes.chunks_exact(WORD_BYTES);
        for chunk in &mut chunks {
            buffer.push(Word::from_le_bytes(chunk.try_into().unwrap()));
        }
        if !chunks.remainder().is_empty() {
            buffer.push(primitive::word_from_le_bytes_partial(chunks.remainder()));
        }
        buffer.into()
    }

    /// Construct from big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(UBig::from_be_bytes(&[1, 2, 3]), ubig!(0x010203));
    /// ```
    pub fn from_be_bytes(bytes: &[u8]) -> UBig {
        if bytes.len() <= WORD_BYTES {
            // fast path
            UBig::from_word(primitive::word_from_be_bytes_partial(bytes))
        } else {
            UBig::from_be_bytes_large(bytes)
        }
    }

    fn from_be_bytes_large(bytes: &[u8]) -> UBig {
        debug_assert!(bytes.len() > WORD_BYTES);
        let mut buffer = Buffer::allocate((bytes.len() - 1) / WORD_BYTES + 1);
        let mut chunks = bytes.rchunks_exact(WORD_BYTES);
        for chunk in &mut chunks {
            buffer.push(Word::from_be_bytes(chunk.try_into().unwrap()));
        }
        if !chunks.remainder().is_empty() {
            buffer.push(primitive::word_from_be_bytes_partial(chunks.remainder()));
        }
        buffer.into()
    }

    /// Return little-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert!(ubig!(0).to_le_bytes().is_empty());
    /// assert_eq!(ubig!(0x010203).to_le_bytes(), [3, 2, 1]);
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match self.repr() {
            Small(x) => {
                let bytes = x.to_le_bytes();
                let skip_bytes = x.leading_zeros() as usize / 8;
                bytes[..WORD_BYTES - skip_bytes].to_vec()
            }
            Large(buffer) => {
                let n = buffer.len();
                let last = buffer[n - 1];
                let skip_last_bytes = last.leading_zeros() as usize / 8;
                let mut bytes = Vec::with_capacity(n * WORD_BYTES - skip_last_bytes);
                for word in &buffer[..n - 1] {
                    bytes.extend_from_slice(&word.to_le_bytes());
                }
                let last_bytes = last.to_le_bytes();
                bytes.extend_from_slice(&last_bytes[..WORD_BYTES - skip_last_bytes]);
                bytes
            }
        }
    }

    /// Return big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert!(ubig!(0).to_be_bytes().is_empty());
    /// assert_eq!(ubig!(0x010203).to_be_bytes(), [1, 2, 3]);
    /// ```
    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self.repr() {
            Small(x) => {
                let bytes = x.to_be_bytes();
                let skip_bytes = x.leading_zeros() as usize / 8;
                bytes[skip_bytes..].to_vec()
            }
            Large(buffer) => {
                let n = buffer.len();
                let last = buffer[n - 1];
                let skip_last_bytes = last.leading_zeros() as usize / 8;
                let mut bytes = Vec::with_capacity(n * WORD_BYTES - skip_last_bytes);
                let last_bytes = last.to_be_bytes();
                bytes.extend_from_slice(&last_bytes[skip_last_bytes..]);
                for word in buffer[..n - 1].iter().rev() {
                    bytes.extend_from_slice(&word.to_be_bytes());
                }
                bytes
            }
        }
    }

    /// Convert to f32.
    ///
    /// Round to nearest, breaking ties to even last bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(134).to_f32(), 134.0f32);
    /// ```
    pub fn to_f32(&self) -> f32 {
        match self.repr() {
            Small(word) => *word as f32,
            Large(_) => match u32::try_from(self) {
                Ok(val) => val as f32,
                Err(_) => self.to_f32_slow(),
            },
        }
    }

    fn to_f32_slow(&self) -> f32 {
        let n = self.bit_len();
        debug_assert!(n > 32);

        if n > 128 {
            f32::INFINITY
        } else {
            let exponent = (n - 1) as u32;
            debug_assert!((32..128).contains(&exponent));
            let mantissa25 = u32::try_from(self >> (n - 25)).unwrap();
            let mantissa = mantissa25 >> 1;

            // value = [8 bits: exponent + 127][23 bits: mantissa without the top bit]
            let value = ((exponent + 126) << 23) + mantissa;

            // Calculate round-to-even adjustment.
            let extra_bit = self.are_low_bits_nonzero(n - 25);
            // low bit of mantissa and two extra bits
            let low_bits = ((mantissa25 & 0b11) << 1) | u32::from(extra_bit);
            let adjustment = round_to_even_adjustment(low_bits);

            // If adjustment is true, increase the mantissa.
            // If the mantissa overflows, this correctly increases the exponent and
            // sets the mantissa to 0.
            // If the exponent overflows, we correctly get the representation of infinity.
            let value = value + u32::from(adjustment);
            f32::from_bits(value)
        }
    }

    /// Convert to f64.
    ///
    /// Round to nearest, breaking ties to even last bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ubig!(134).to_f64(), 134.0f64);
    /// ```
    pub fn to_f64(&self) -> f64 {
        match self.repr() {
            Small(word) => *word as f64,
            Large(_) => match u64::try_from(self) {
                Ok(val) => val as f64,
                Err(_) => self.to_f64_slow(),
            },
        }
    }

    fn to_f64_slow(&self) -> f64 {
        let n = self.bit_len();
        debug_assert!(n > 64);

        if n > 1024 {
            f64::INFINITY
        } else {
            let exponent = (n - 1) as u64;
            debug_assert!((64..1024).contains(&exponent));
            let mantissa54 = u64::try_from(self >> (n - 54)).unwrap();
            let mantissa = mantissa54 >> 1;

            // value = [11-bits: exponent + 1023][52 bit: mantissa without the top bit]
            let value = ((exponent + 1022) << 52) + mantissa;

            // Calculate round-to-even adjustment.
            let extra_bit = self.are_low_bits_nonzero(n - 54);
            // low bit of mantissa and two extra bits
            let low_bits = (((mantissa54 & 0b11) as u32) << 1) | u32::from(extra_bit);
            let adjustment = round_to_even_adjustment(low_bits);

            // If adjustment is true, increase the mantissa.
            // If the mantissa overflows, this correctly increases the exponent and
            // sets the mantissa to 0.
            // If the exponent overflows, we correctly get the representation of infinity.
            let value = value + u64::from(adjustment);
            f64::from_bits(value)
        }
    }
}

impl IBig {
    /// Convert to f32.
    ///
    /// Round to nearest, breaking ties to even last bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-134).to_f32(), -134.0f32);
    /// ```
    pub fn to_f32(&self) -> f32 {
        let val = self.magnitude().to_f32();
        match self.sign() {
            Positive => val,
            Negative => -val,
        }
    }

    /// Convert to f64.
    ///
    /// Round to nearest, breaking ties to even last bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(ibig!(-134).to_f64(), -134.0f64);
    /// ```
    pub fn to_f64(&self) -> f64 {
        let val = self.magnitude().to_f64();
        match self.sign() {
            Positive => val,
            Negative => -val,
        }
    }
}

/// Round to even floating point adjustment, based on the bottom
/// bit of mantissa and additional 2 bits (i.e. 3 bits in units of ULP/4).
fn round_to_even_adjustment(bits: u32) -> bool {
    bits >= 0b110 || bits == 0b011
}

/// Implement `impl From<U> for T` using a function.
macro_rules! impl_from {
    (impl From<$a:ty> for $b:ty as $f:ident) => {
        impl From<$a> for $b {
            fn from(value: $a) -> $b {
                $f(value)
            }
        }
    };
}

/// Implement `impl TryFrom<U> for T` using a function.
macro_rules! impl_try_from {
    (impl TryFrom<$a:ty> for $b:ty as $f:ident) => {
        impl TryFrom<$a> for $b {
            type Error = OutOfBoundsError;

            fn try_from(value: $a) -> Result<$b, OutOfBoundsError> {
                $f(value)
            }
        }
    };
}

impl_from!(impl From<u8> for UBig as ubig_from_unsigned);
impl_from!(impl From<u16> for UBig as ubig_from_unsigned);
impl_from!(impl From<u32> for UBig as ubig_from_unsigned);
impl_from!(impl From<u64> for UBig as ubig_from_unsigned);
impl_from!(impl From<u128> for UBig as ubig_from_unsigned);
impl_from!(impl From<usize> for UBig as ubig_from_unsigned);

impl_try_from!(impl TryFrom<i8> for UBig as ubig_from_signed);
impl_try_from!(impl TryFrom<i16> for UBig as ubig_from_signed);
impl_try_from!(impl TryFrom<i32> for UBig as ubig_from_signed);
impl_try_from!(impl TryFrom<i64> for UBig as ubig_from_signed);
impl_try_from!(impl TryFrom<i128> for UBig as ubig_from_signed);
impl_try_from!(impl TryFrom<isize> for UBig as ubig_from_signed);

impl_try_from!(impl TryFrom<UBig> for u8 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<UBig> for u16 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<UBig> for u32 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<UBig> for u64 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<UBig> for u128 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<UBig> for usize as unsigned_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for u8 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for u16 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for u32 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for u64 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for u128 as unsigned_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for usize as unsigned_from_ubig);

impl_try_from!(impl TryFrom<UBig> for i8 as signed_from_ubig);
impl_try_from!(impl TryFrom<UBig> for i16 as signed_from_ubig);
impl_try_from!(impl TryFrom<UBig> for i32 as signed_from_ubig);
impl_try_from!(impl TryFrom<UBig> for i64 as signed_from_ubig);
impl_try_from!(impl TryFrom<UBig> for i128 as signed_from_ubig);
impl_try_from!(impl TryFrom<UBig> for isize as signed_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for i8 as signed_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for i16 as signed_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for i32 as signed_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for i64 as signed_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for i128 as signed_from_ubig);
impl_try_from!(impl TryFrom<&UBig> for isize as signed_from_ubig);

impl From<bool> for UBig {
    fn from(b: bool) -> UBig {
        u8::from(b).into()
    }
}

impl_from!(impl From<u8> for IBig as ibig_from_unsigned);
impl_from!(impl From<u16> for IBig as ibig_from_unsigned);
impl_from!(impl From<u32> for IBig as ibig_from_unsigned);
impl_from!(impl From<u64> for IBig as ibig_from_unsigned);
impl_from!(impl From<u128> for IBig as ibig_from_unsigned);
impl_from!(impl From<usize> for IBig as ibig_from_unsigned);

impl_from!(impl From<i8> for IBig as ibig_from_signed);
impl_from!(impl From<i16> for IBig as ibig_from_signed);
impl_from!(impl From<i32> for IBig as ibig_from_signed);
impl_from!(impl From<i64> for IBig as ibig_from_signed);
impl_from!(impl From<i128> for IBig as ibig_from_signed);
impl_from!(impl From<isize> for IBig as ibig_from_signed);

impl_try_from!(impl TryFrom<IBig> for u8 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<IBig> for u16 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<IBig> for u32 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<IBig> for u64 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<IBig> for u128 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<IBig> for usize as unsigned_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for u8 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for u16 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for u32 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for u64 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for u128 as unsigned_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for usize as unsigned_from_ibig);

impl_try_from!(impl TryFrom<IBig> for i8 as signed_from_ibig);
impl_try_from!(impl TryFrom<IBig> for i16 as signed_from_ibig);
impl_try_from!(impl TryFrom<IBig> for i32 as signed_from_ibig);
impl_try_from!(impl TryFrom<IBig> for i64 as signed_from_ibig);
impl_try_from!(impl TryFrom<IBig> for i128 as signed_from_ibig);
impl_try_from!(impl TryFrom<IBig> for isize as signed_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for i8 as signed_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for i16 as signed_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for i32 as signed_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for i64 as signed_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for i128 as signed_from_ibig);
impl_try_from!(impl TryFrom<&IBig> for isize as signed_from_ibig);

impl From<bool> for IBig {
    fn from(b: bool) -> IBig {
        u8::from(b).into()
    }
}

impl From<UBig> for IBig {
    fn from(x: UBig) -> IBig {
        IBig::from_sign_magnitude(Positive, x)
    }
}

impl From<&UBig> for IBig {
    fn from(x: &UBig) -> IBig {
        IBig::from(x.clone())
    }
}

impl TryFrom<IBig> for UBig {
    type Error = OutOfBoundsError;

    fn try_from(x: IBig) -> Result<UBig, OutOfBoundsError> {
        match x.into_sign_magnitude() {
            (Positive, mag) => Ok(mag),
            (Negative, _) => Err(OutOfBoundsError),
        }
    }
}

impl TryFrom<&IBig> for UBig {
    type Error = OutOfBoundsError;

    fn try_from(x: &IBig) -> Result<UBig, OutOfBoundsError> {
        match x.sign() {
            Positive => Ok(x.magnitude().clone()),
            Negative => Err(OutOfBoundsError),
        }
    }
}

/// Convert an unsigned primitive to `UBig`.
fn ubig_from_unsigned<T>(x: T) -> UBig
where
    T: PrimitiveUnsigned,
{
    match x.try_into() {
        Ok(w) => UBig::from_word(w),
        Err(_) => {
            let repr = x.to_le_bytes();
            UBig::from_le_bytes(repr.as_ref())
        }
    }
}

/// Try to convert a signed primitive to `UBig`.
fn ubig_from_signed<T>(x: T) -> Result<UBig, OutOfBoundsError>
where
    T: PrimitiveSigned,
{
    match T::Unsigned::try_from(x) {
        Ok(u) => Ok(ubig_from_unsigned(u)),
        Err(_) => Err(OutOfBoundsError),
    }
}

/// Try to convert `UBig` to an unsigned primitive.
fn unsigned_from_ubig<T, B>(num: B) -> Result<T, OutOfBoundsError>
where
    T: PrimitiveUnsigned,
    B: Borrow<UBig>,
{
    match num.borrow().repr() {
        Small(w) => match T::try_from(*w) {
            Ok(val) => Ok(val),
            Err(_) => Err(OutOfBoundsError),
        },
        Large(buffer) => unsigned_from_words(buffer),
    }
}

/// Try to convert `Word`s to an unsigned primitive.
fn unsigned_from_words<T>(words: &[Word]) -> Result<T, OutOfBoundsError>
where
    T: PrimitiveUnsigned,
{
    debug_assert!(words.len() >= 2);
    let t_words = T::BYTE_SIZE / WORD_BYTES;
    if t_words <= 1 || words.len() > t_words {
        Err(OutOfBoundsError)
    } else {
        assert!(
            T::BIT_SIZE % WORD_BITS == 0,
            "A large primitive type not a multiple of word size."
        );
        let mut repr = T::default().to_le_bytes();
        let bytes: &mut [u8] = repr.as_mut();
        for (idx, w) in words.iter().enumerate() {
            let pos = idx * WORD_BYTES;
            bytes[pos..pos + WORD_BYTES].copy_from_slice(&w.to_le_bytes());
        }
        Ok(T::from_le_bytes(repr))
    }
}

/// Try to convert `UBig` to a signed primitive.
fn signed_from_ubig<T, B>(num: B) -> Result<T, OutOfBoundsError>
where
    T: PrimitiveSigned,
    B: Borrow<UBig>,
{
    match num.borrow().repr() {
        Small(w) => T::try_from(*w).map_err(|_| OutOfBoundsError),
        Large(buffer) => {
            let u: T::Unsigned = unsigned_from_words(buffer)?;
            u.try_into().map_err(|_| OutOfBoundsError)
        }
    }
}

/// Convert an unsigned primitive to `IBig`.
fn ibig_from_unsigned<T>(x: T) -> IBig
where
    T: PrimitiveUnsigned,
{
    IBig::from(ubig_from_unsigned(x))
}

/// Convert a signed primitive to `IBig`.
fn ibig_from_signed<T>(x: T) -> IBig
where
    T: PrimitiveSigned,
{
    let (sign, mag) = x.to_sign_magnitude();
    IBig::from_sign_magnitude(sign, ubig_from_unsigned(mag))
}

/// Try to convert `IBig` to an unsigned primitive.
fn unsigned_from_ibig<T, B>(num: B) -> Result<T, OutOfBoundsError>
where
    T: PrimitiveUnsigned,
    B: Borrow<IBig>,
{
    let num = num.borrow();
    match num.sign() {
        Positive => unsigned_from_ubig(num.magnitude()),
        Negative => Err(OutOfBoundsError),
    }
}

/// Try to convert `IBig` to an signed primitive.
fn signed_from_ibig<T, B>(num: B) -> Result<T, OutOfBoundsError>
where
    T: PrimitiveSigned,
    B: Borrow<IBig>,
{
    let num = num.borrow();
    let u: T::Unsigned = unsigned_from_ubig(num.magnitude())?;
    T::try_from_sign_magnitude(num.sign(), u)
}
