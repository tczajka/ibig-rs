//! Binary GCD algorithms.

use core::{cmp::Ordering, mem, ptr};

use crate::{
    add,
    arch::word::Word,
    bits::trailing_zeros_large,
    cmp::cmp_same_len,
    memory::{self, Memory},
    primitive::WORD_BITS_USIZE,
    shift,
    sign::Sign,
};

use alloc::alloc::Layout;

/// Naive binary GCD for two multi-digit integers
pub(crate) fn gcd_in_place(lhs: &mut [Word], rhs: &mut [Word]) -> usize {
    // find common factors of 2
    let lhs_zeros = trailing_zeros_large(lhs);
    let rhs_zeros = trailing_zeros_large(rhs);
    let shift = lhs_zeros.min(rhs_zeros);

    let (lhs_pos, rhs_pos) = (lhs_zeros / WORD_BITS_USIZE, rhs_zeros / WORD_BITS_USIZE);
    shift::shr_in_place(&mut lhs[lhs_pos..], (lhs_zeros % WORD_BITS_USIZE) as u32);
    shift::shr_in_place(&mut rhs[rhs_pos..], (rhs_zeros % WORD_BITS_USIZE) as u32);

    // Use the binary GCD algorithm. Right shift operations are performed inplace,
    // we keep track of the valid range of words in lhs and rhs using lhs_cur and rhs_cur respectively.
    // The scope below is used to constraint the mutable borrow.

    let (result_ptr, result_len, carry) = {
        let (mut lhs_cur, mut rhs_cur) = (&mut lhs[lhs_pos..], &mut rhs[rhs_pos..]);

        loop {
            // keep lhs > rhs, swap if needed
            match lhs_cur
                .len()
                .cmp(&rhs_cur.len())
                .then_with(|| cmp_same_len(lhs_cur, rhs_cur))
            {
                Ordering::Equal => break,
                Ordering::Less => mem::swap(&mut lhs_cur, &mut rhs_cur),
                Ordering::Greater => {}
            }

            assert!(!add::sub_in_place(lhs_cur, rhs_cur));
            while *lhs_cur.last().unwrap() == 0 {
                let last_pos = lhs_cur.len() - 1;
                lhs_cur = &mut lhs_cur[..last_pos];
            }

            let zeros = trailing_zeros_large(lhs_cur);
            lhs_cur = &mut lhs_cur[zeros / WORD_BITS_USIZE..];
            shift::shr_in_place(lhs_cur, (zeros % WORD_BITS_USIZE) as u32);
        }

        let carry = shift::shl_in_place(lhs_cur, (shift % WORD_BITS_USIZE) as u32);
        (lhs_cur.as_ptr(), lhs_cur.len(), carry)
    };

    // move the result from high bits to low bits, with shift taken into account
    let shift_words = shift / WORD_BITS_USIZE;
    unsafe {
        ptr::copy(result_ptr, lhs.as_mut_ptr().add(shift_words), result_len);
    }
    lhs[..shift_words].fill(0);

    let mut final_size = result_len + shift_words;
    if carry > 0 {
        lhs[final_size] = carry;
        final_size += 1;
    }
    final_size
}

/// Temporary memory required for extended gcd.
pub(crate) fn memory_requirement_up_to(lhs_len: usize, rhs_len: usize) -> Layout {
    // Required memory:
    // - a copy of lhs and rhs
    // - two numbers (a & c) with almost the same size as rhs
    // - two numbers (b & d) with almost the same size as lhs
    let num_words = 3 * (lhs_len + rhs_len) + 4;
    memory::array_layout::<Word>(num_words)
}

/// Extended binary GCD for two multi-digits numbers
pub(crate) fn xgcd_in_place(
    lhs: &mut [Word],
    rhs: &mut [Word],
    g: &mut [Word],
    memory: &mut Memory,
) -> (Sign, Sign) {
    // find common factors of 2
    let lhs_zeros = trailing_zeros_large(lhs);
    let rhs_zeros = trailing_zeros_large(rhs);
    let shift = lhs_zeros.min(rhs_zeros);

    let (lhs_pos, rhs_pos) = (lhs_zeros / WORD_BITS_USIZE, rhs_zeros / WORD_BITS_USIZE);
    shift::shr_in_place(&mut lhs[lhs_pos..], (lhs_zeros % WORD_BITS_USIZE) as u32);
    shift::shr_in_place(&mut rhs[rhs_pos..], (rhs_zeros % WORD_BITS_USIZE) as u32);

    let (x, mut memory) = memory.allocate_slice_copy(&lhs[lhs_pos..]);
    let (y, mut memory) = memory.allocate_slice_copy(&rhs[rhs_pos..]);

    let (mut a, mut memory) = memory.allocate_slice_fill::<Word>(y.len() + 1, 0);
    let (mut b, mut memory) = memory.allocate_slice_fill::<Word>(x.len() + 1, 0);
    let (mut asign, mut bsign) = (Sign::Positive, Sign::Positive);
    *a.first_mut().unwrap() = 1;
    let (mut c, mut memory) = memory.allocate_slice_fill::<Word>(y.len() + 1, 0);
    let (mut d, _) = memory.allocate_slice_fill::<Word>(x.len() + 1, 0);
    let (mut csign, mut dsign) = (Sign::Positive, Sign::Positive);
    *d.first_mut().unwrap() = 1;

    #[inline]
    fn signed_sub(l: &mut [Word], sl: &mut Sign, r: &[Word], sr: &Sign) {
        match (*sl, *sr) {
            (Sign::Positive, Sign::Positive) => {
                *sl = add::sub_in_place_with_sign(l, r);
            }
            (Sign::Negative, Sign::Negative) => {
                *sl = -add::sub_in_place_with_sign(l, r);
            }
            (_, _) => {
                assert!(!add::add_in_place(l, r));
            }
        }
    }

    // Use the binary GCD algorithm. Right shift operations are performed inplace just like gcd_in_place
    // The algorithm is from Alg 14.61 of Handbook of Applied Cryptography.
    {
        let (mut lhs_cur, mut rhs_cur) = (&mut lhs[lhs_pos..], &mut rhs[rhs_pos..]);

        loop {
            // step 4/5
            let zeros = trailing_zeros_large(lhs_cur);
            lhs_cur = &mut lhs_cur[zeros / WORD_BITS_USIZE..];
            shift::shr_in_place(lhs_cur, (zeros % WORD_BITS_USIZE) as u32);
            for _ in 0..zeros {
                if a.first().unwrap() | b.first().unwrap() & 1 > 0 {
                    assert!(!add::add_in_place(a, y));
                    assert!(!add::add_in_place(b, x));
                }
                shift::shr_in_place(a, 1); // TODO: optimize?
                shift::shr_in_place(b, 1);
            }

            // // step 5
            // let zeros = trailing_zeros_large(rhs_cur);
            // rhs_cur = &mut rhs_cur[zeros / WORD_BITS_USIZE..];
            // shift::shr_in_place(rhs_cur, (zeros % WORD_BITS_USIZE) as u32);
            // for _ in 0..zeros {
            //     if c.first().unwrap() | d.first().unwrap() & 1 > 0 {
            //         assert!(!add::add_in_place(c, y));
            //         assert!(!add::add_in_place(d, x));
            //     }
            //     shift::shr_in_place(c, 1);
            //     shift::shr_in_place(d, 1);
            // }

            // step 6
            // keep lhs > rhs, swap if needed
            match lhs_cur
                .len()
                .cmp(&rhs_cur.len())
                .then_with(|| cmp_same_len(lhs_cur, rhs_cur))
            {
                Ordering::Equal => break,
                Ordering::Less => {
                    mem::swap(&mut lhs_cur, &mut rhs_cur);
                    mem::swap(&mut a, &mut c);
                    mem::swap(&mut asign, &mut csign);
                    mem::swap(&mut b, &mut d);
                    mem::swap(&mut bsign, &mut dsign);
                }
                Ordering::Greater => {}
            }

            assert!(!add::sub_in_place(lhs_cur, rhs_cur));
            while *lhs_cur.last().unwrap() == 0 {
                let last_pos = lhs_cur.len() - 1;
                lhs_cur = &mut lhs_cur[..last_pos];
            }
            signed_sub(a, &mut asign, c, &csign);
            signed_sub(b, &mut bsign, d, &dsign);
        }

        let carry = shift::shl_in_place(lhs_cur, (shift % WORD_BITS_USIZE) as u32);
        let shift_words = shift / WORD_BITS_USIZE;
        g[shift_words..shift_words + lhs_cur.len()].copy_from_slice(lhs_cur);
        if carry > 0 {
            g[shift_words + lhs_cur.len()] = carry;
        }
    };

    // move a to lhs, b to rhs
    lhs.copy_from_slice(&a[..lhs.len()]);
    rhs.copy_from_slice(&b[..rhs.len()]);
    (asign, bsign)
}
