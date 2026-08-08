#![allow(dead_code)]
use crate::tables::{LUT_SIGNED_MUL_FLAT, SIGMOID_TABLE, GELU_TABLE, RELU_TABLE, SAT_ADD_FLAT};
use crate::types::Gl4Digit;

/// Безопасный LUT доступ - всегда &15
#[inline(always)] pub fn ai_sigmoid(x: Gl4Digit) -> Gl4Digit { unsafe { *SIGMOID_TABLE.get_unchecked((x & 15) as usize) } }
#[inline(always)] pub fn ai_relu(x: Gl4Digit) -> Gl4Digit { unsafe { *RELU_TABLE.get_unchecked((x & 15) as usize) } }
#[inline(always)] pub fn ai_gelu(x: Gl4Digit) -> Gl4Digit { unsafe { *GELU_TABLE.get_unchecked((x & 15) as usize) } }

#[inline(always)]
pub fn ai_dot_gl4(a: &[Gl4Digit], b: &[Gl4Digit]) -> i32 {
    debug_assert_eq!(a.len(), b.len());
    let mut acc = 0i32;
    for i in 0..a.len() {
        let idx = ((a[i] & 15) as usize) << 4 | (b[i] & 15) as usize;
        acc += unsafe { *LUT_SIGNED_MUL_FLAT.get_unchecked(idx) } as i32;
    }
    acc
}

/// Упакованный dot - 2 GL4 в байте, 8 GL4 в u32 - для NEON
#[inline(always)]
pub fn ai_dot_packed_gl4(a_packed: &[u8], b_packed: &[u8]) -> i32 {
    let mut acc = 0i32;
    let len = a_packed.len().min(b_packed.len());
    let mut i = 0;
    while i < len {
        let qa = a_packed[i]; let qb = b_packed[i];
        let al = (qa & 15) as usize; let ah = ((qa >> 4) & 15) as usize;
        let bl = (qb & 15) as usize; let bh = ((qb >> 4) & 15) as usize;
        unsafe {
            acc += *LUT_SIGNED_MUL_FLAT.get_unchecked((al << 4) | bl) as i32;
            acc += *LUT_SIGNED_MUL_FLAT.get_unchecked((ah << 4) | bh) as i32;
        }
        i += 1;
    }
    acc
}

#[inline(always)] pub fn weight_add(a: Gl4Digit, b: Gl4Digit) -> Gl4Digit { unsafe { *SAT_ADD_FLAT.get_unchecked(((a & 15) as usize)<<4 | (b & 15) as usize) } }