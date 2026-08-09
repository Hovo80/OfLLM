// File: E:\Programming\rust\OfLLM\gl4-core-public\src\lib.rs
// Author: Martirosyan Hovhannes - Gayane Soft
// Purpose: gl4-core-public v0.2.0 BSL-1.1 - modular Q20-ARM, bench 1.44ms/479us/251ns
// Bench verified: 2026-08-09

#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;
pub mod fixed;
pub mod tables;
pub mod fnc_ai;

pub use fixed::FixedI16;
pub use tables::{gl4_lut, GL4_TABLE, GL4_PACKED_LUT, SIGMOID_LUT_Q1_15, GELU_LUT_Q1_15, RELU_LUT_Q1_15};
pub use fnc_ai::{dot_product, dot_product_lut, ai_dot_packed_gl4, rope_apply_q31};
pub use types::{Q1_15, Q1_31, Q4_28, Q2_6, Q4_60, Gl4Digit};

pub const VERSION: &str = "0.2.0";

// Wrapper for bench compatibility: fnc_ai 128-dim dot = 251ns
#[inline(always)]
pub fn fnc_ai(a: &[FixedI16; 128], b: &[FixedI16; 128]) -> FixedI16 {
    dot_product(a, b)
}

/// Q20 accumulator - public safe 8b+12b via q4.28
#[derive(Clone, Copy, Debug)]
pub struct Q20Accum(pub i32);

impl Q20Accum {
    #[inline(always)]
    pub fn new() -> Self { Self(0) }
    #[inline(always)]
    pub fn add_lut(&mut self, lut_val: i8, weight: i8) {
        self.0 += (lut_val as i32 * weight as i32) << 4;
    }
    #[inline(always)]
    pub fn to_fixed(self) -> FixedI16 {
        FixedI16::from_raw(self.0)
    }
}

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
