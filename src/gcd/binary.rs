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
    // TODO: constraint lhs > rhs, so that the result will be always stored in rhs
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

            // lhs -= rhs
            assert!(!add::sub_in_place(lhs_cur, rhs_cur));
            while *lhs_cur.last().unwrap() == 0 {
                let last_pos = lhs_cur.len() - 1;
                lhs_cur = &mut lhs_cur[..last_pos];
            }

            // truncate trailing zeros
            let zeros = trailing_zeros_large(lhs_cur);
            lhs_cur = &mut lhs_cur[zeros / WORD_BITS_USIZE..];
            shift::shr_in_place(lhs_cur, (zeros % WORD_BITS_USIZE) as u32);

            // TODO: delegate to single word version when lhs_cur.len() = rhs_cur.len() = 1
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

    let (lhs_pos, rhs_pos) = (lhs_zeros / WORD_BITS_USIZE, rhs_zeros / WORD_BITS_USIZE);
    shift::shr_in_place(&mut lhs[lhs_pos..], (lhs_zeros % WORD_BITS_USIZE) as u32);
    shift::shr_in_place(&mut rhs[rhs_pos..], (rhs_zeros % WORD_BITS_USIZE) as u32);

    // allocate memory for coefficients
    let (mut s0, mut memory) = memory.allocate_slice_fill::<Word>(lhs.len() + 1, 0);
    let (mut s1, mut memory) = memory.allocate_slice_fill::<Word>(lhs.len() + 1, 0);
    let (mut t0, mut memory) = memory.allocate_slice_fill::<Word>(rhs.len() + 1, 0);
    let (mut t1, _) = memory.allocate_slice_fill::<Word>(rhs.len() + 1, 0);

    // initialize s, t
    let (init_zeros, mut shift) = match lhs_zeros.cmp(&rhs_zeros) {
        Ordering::Equal => (lhs_zeros, 0),
        Ordering::Greater => {
            let shift = lhs_zeros - rhs_zeros;
            *s0.first_mut().unwrap() = 1;
            *t1.get_mut(shift / WORD_BITS_USIZE).unwrap() |= 1 << (shift % WORD_BITS_USIZE);
            (rhs_zeros, shift)
        },
        Ordering::Less => {
            let shift = lhs_zeros - rhs_zeros;
            *s0.get_mut(shift / WORD_BITS_USIZE).unwrap() |= 1 << (shift % WORD_BITS_USIZE);
            *t1.first_mut().unwrap() = 1;
            (lhs_zeros, shift)
        },
    };

    // Use the binary GCD algorithm from GMP. Right shift operations are performed inplace just like gcd_in_place
    {
        let (mut lhs_cur, mut rhs_cur) = (&mut lhs[lhs_pos..], &mut rhs[rhs_pos..]);

        loop {
            // keep lhs > rhs, swap if needed
            match lhs_cur
                .len()
                .cmp(&rhs_cur.len())
                .then_with(|| cmp_same_len(lhs_cur, rhs_cur))
            {
                Ordering::Equal => break,
                Ordering::Less => {
                    mem::swap(&mut lhs_cur, &mut rhs_cur);
                    mem::swap(&mut t0, &mut t1);
                    mem::swap(&mut s0, &mut s1);
                },
                Ordering::Greater => {}
            }

            // lhs -= rhs
            assert!(!add::sub_in_place(lhs_cur, rhs_cur));
            while *lhs_cur.last().unwrap() == 0 {
                let last_pos = lhs_cur.len() - 1;
                lhs_cur = &mut lhs_cur[..last_pos];
            }

            // truncate trailing zeros
            let zeros = trailing_zeros_large(lhs_cur);
            lhs_cur = &mut lhs_cur[zeros / WORD_BITS_USIZE..];
            shift::shr_in_place(lhs_cur, (zeros % WORD_BITS_USIZE) as u32);
            shift += zeros;

            assert!(!add::add_in_place(t0, t1));
            assert!(shift::shl_in_place(t1, zeros as u32) == 0); // TODO: full shift
            assert!(!add::add_in_place(s0, s1));
            assert!(shift::shl_in_place(s1, zeros as u32) == 0);

            // TODO: delegate to single word version when lhs_cur.len() = rhs_cur.len() = 1
        }

        // copy result from lhs to g
        let carry = shift::shl_in_place(lhs_cur, (init_zeros % WORD_BITS_USIZE) as u32);
        let shift_words = init_zeros / WORD_BITS_USIZE;
        g[shift_words..shift_words + lhs_cur.len()].copy_from_slice(lhs_cur);
        if carry > 0 {
            g[shift_words + lhs_cur.len()] = carry;
        }
    };

    // now u = v = g = gcd (lhs,rhs). Compute U/g (store in t1) and V/g (store in s1)
    assert!(!add::add_in_place(s1, s0));
    assert!(!add::add_in_place(t1, t0));

    // now 2^shift * g = s0 * U - t0 * V. Get rid of the power of two, using
    // s0 * U - t0 * V = (s0 + V/g) U - (t0 + U/g) V.
    for _ in 0..shift {
        if (s0.first().unwrap() | t0.first().unwrap()) & 1 > 0 {
            assert!(!add::add_in_place(s0, s1));
            assert!(!add::add_in_place(s0, s1));
        }
        shift::shr_in_place(s0, 1);
        shift::shr_in_place(t0, 1);
    }

    // move a to lhs, b to rhs
    lhs.copy_from_slice(&s0[..lhs.len()]);
    rhs.copy_from_slice(&t0[..rhs.len()]);
    (Sign::Positive, Sign::Positive)
}
