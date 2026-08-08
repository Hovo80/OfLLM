#![allow(dead_code)]
use crate::types::Q1_31;

/// Умножение Q1.31 * Q1.31 -> Q1.31 с округлением
#[inline(always)] pub fn q31_mul(a: Q1_31, b: Q1_31) -> Q1_31 { a.mul(b) }

/// RoPE через RZ LUT - используй таблицы из quantum32
#[inline(always)]
pub fn rope_apply_q31(re: Q1_31, im: Q1_31, cos: Q1_31, sin: Q1_31) -> (Q1_31, Q1_31) {
    // (re + i*im) * (cos + i*sin)
    // real = re*cos - im*sin
    // imag = re*sin + im*cos
    let r = Q1_31(((re.0 as i64 * cos.0 as i64 - im.0 as i64 * sin.0 as i64) >> 31) as i32);
    let i = Q1_31(((re.0 as i64 * sin.0 as i64 + im.0 as i64 * cos.0 as i64) >> 31) as i32);
    (r, i)
}