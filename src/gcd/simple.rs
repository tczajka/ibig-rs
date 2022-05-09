use core::{ptr, cmp::Ordering};

use crate::{
    add, arch::word::Word, bits::trailing_zeros_large, cmp::cmp_same_len,
    primitive::WORD_BITS_USIZE, shift,
};

/// Naive binary GCD for two multi-digit integers
///
/// The result is stored in the low bits of lhs.
/// The word length of the result number is returned.
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
            match lhs_cur.len()
                .cmp(&rhs_cur.len())
                .then_with(|| cmp_same_len(lhs_cur, rhs_cur))
            {
                Ordering::Equal => break,
                Ordering::Greater => {
                    // lhs > rhs
                    assert!(!add::sub_in_place(lhs_cur, rhs_cur));
                    if *lhs_cur.last().unwrap() == 0 {
                        let last_pos = lhs_cur.len() - 1;
                        lhs_cur = &mut lhs_cur[..last_pos];
                    }
    
                    let zeros = trailing_zeros_large(lhs_cur);
                    lhs_cur = &mut lhs_cur[zeros / WORD_BITS_USIZE..];
                    shift::shr_in_place(lhs_cur, (zeros % WORD_BITS_USIZE) as u32);
                }
                Ordering::Less => {
                    // lhs < rhs
                    assert!(!add::sub_in_place(rhs_cur, lhs_cur));
                    if *rhs_cur.last().unwrap() == 0 {
                        let last_pos = rhs_cur.len() - 1;
                        rhs_cur = &mut rhs_cur[..last_pos];
                    }
    
                    let zeros = trailing_zeros_large(rhs_cur);
                    rhs_cur = &mut rhs_cur[zeros / WORD_BITS_USIZE..];
                    shift::shr_in_place(rhs_cur, (zeros % WORD_BITS_USIZE) as u32);
                }
            }
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
