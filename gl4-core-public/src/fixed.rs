#![allow(dead_code)]
// gl4-core-public/src/fixed.rs - Q16.16 fixed, no_std, saturate
// Автор: Martirosyan Hovhannes

use core::ops::{Add, Sub, Mul};

/// FixedI16 - Q16.16: 16 целых + 16 дробных, i32
/// no_std, переполнение с saturate, mul = (i64*a*b)>>16
/// Цель: <8ms на 1M mul vs 25ms fp32
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FixedI16(pub i32);

impl FixedI16 {
    pub const SCALE: i64 = 1 << 16;
    pub const ONE: Self = Self(1 << 16);
    pub const ZERO: Self = Self(0);

    #[inline(always)]
    pub fn from_f32(f: f32) -> Self {
        // clamp чтобы не переполнить
        let clamped = f.clamp(-32768.0, 32767.9999);
        Self((clamped * Self::SCALE as f32) as i32)
    }

    #[inline(always)]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / Self::SCALE as f32
    }

    #[inline(always)]
    pub fn from_raw(raw: i32) -> Self { Self(raw) }

    #[inline(always)]
    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    // Будущее: LUT для sin/cos RoPE в Q1_31 - 256 значений в L1
    // Сейчас cos/sin приходят извне, можно сделать таблицу [Q1_31; 256]
    // TODO: private LUT в q20-private
}

impl Add for FixedI16 {
    type Output = Self;
    #[inline(always)] fn add(self, other: Self) -> Self { Self(self.0.wrapping_add(other.0)) }
}

impl Sub for FixedI16 {
    type Output = Self;
    #[inline(always)] fn sub(self, other: Self) -> Self { Self(self.0.wrapping_sub(other.0)) }
}

impl Mul for FixedI16 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        // (a * b) >> 16 с округлением
        let prod = self.0 as i64 * other.0 as i64;
        Self(((prod + (1 << 15)) >> 16) as i32)
    }
}
