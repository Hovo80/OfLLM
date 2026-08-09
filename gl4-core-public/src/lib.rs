#![allow(dead_code)]
// gl4-core-public/src/lib.rs v0.1.0 - production-ready, no_std
// Автор: Martirosyan Hovhannes (Gayane Soft)
// Лицензия: GPL-3.0-only, dual-license для closed-source

#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;
pub mod fixed;
pub mod tables;
pub mod fnc_ai;

pub use fixed::FixedI16;
pub use tables::{gl4_lut, GL4_TABLE, GL4_PACKED_LUT, SIGMOID_LUT_Q1_15, GELU_LUT_Q1_15, RELU_LUT_Q1_15};
pub use fnc_ai::{dot_product, dot_product_lut, ai_dot_packed_gl4, rope_apply_q31};
pub use types::{Q1_15, Q1_31, Q4_28, Q2_6, Q4_60, Gl4Digit};

pub const VERSION: &str = "0.1.0";

#[cfg(feature = "neon")]
pub mod neon {
    use crate::fixed::FixedI16;
    #[inline(always)]
    pub fn dot_product_neon(a: &[FixedI16], b: &[FixedI16]) -> FixedI16 {
        crate::fnc_ai::dot_product(a, b)
    }
    #[inline(always)]
    pub fn ai_dot_packed_gl4_neon(a: &[FixedI16], b_packed: &[u8]) -> FixedI16 {
        crate::fnc_ai::ai_dot_packed_gl4(a, b_packed)
    }
}
