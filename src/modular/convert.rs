//! Conversion between Modulo, UBig and IBig.

use crate::{
    ibig::IBig,
    modular::{modulo::Modulo, modulo_ring::ModuloRing},
    sign::Sign::*,
    ubig::UBig,
};

impl ModuloRing {
    /// The ring modulus.
    ///
    /// # Example
    ///
    /// ```
    /// # use ibig::{ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// assert_eq!(ring.modulus(), ubig!(100));
    /// ```
    #[inline]
    pub fn modulus(&self) -> UBig {
        self.normalized_modulus() >> self.shift()
    }

    /// Create an element of the ring from another type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// let x = ring.from(-1234);
    /// let y = ring.from(ubig!(3366));
    /// assert!(x == y);
    /// ```
    #[inline]
    pub fn from<T: Into<IBig>>(&self, source: T) -> Modulo {
        // TODO: Avoid copy for huge numbers.
        let source = IBig::from(source);
        let (sign, mag) = source.into_sign_magnitude();
        let modulo = self.from_ubig(mag);
        match sign {
            Positive => modulo,
            Negative => -modulo,
        }
    }

    pub(crate) fn from_ubig(&self, mut value: UBig) -> Modulo {
        value <<= self.shift();
        // TODO: Optimize using fast_div_top.
        value %= self.normalized_modulus();
        self.from_normalized_value(value)
    }

    pub(crate) fn from_normalized_value(&self, normalized_value: UBig) -> Modulo {
        Modulo::from_normalized_value(normalized_value, self)
    }
}

impl Modulo {
    /// Get the residue in range `0..n` in an n-element ring.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ModuloRing, ubig};
    /// let ring = ModuloRing::new(&ubig!(100));
    /// let x = ring.from(-1234);
    /// assert_eq!(x.residue(), ubig!(66));
    /// ```
    #[inline]
    pub fn residue(&self) -> UBig {
        self.normalized() >> self.ring().shift()
    }
}
