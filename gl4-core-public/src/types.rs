#![allow(dead_code)]
/// Базовые типы. Все fixed-point.

pub type Gl4Digit = u8; // 0..15, хранит Q1.3

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q1_15(pub i16); // int1.15 : 1 знак + 15 дробных, -1..0.999

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q1_31(pub i32); // int1.31 : SCALE = 2147483647, твой тип из quantum32

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q4_28(pub i32); // int4.28 : 4 целых + 28 дробных, -8..7.999

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q2_6(pub i8); // int2.6 : для KV-cache, -2..1.98

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q4_60(pub i64); // int4.60 : для Adam моментов, только train

impl Q1_31 {
    pub const SCALE: i64 = 2147483647;
    pub const ONE: Self = Self(2147483647);
    #[inline(always)] pub fn from_f32(f: f32) -> Self { Self((f.clamp(-1.0,0.999) * Self::SCALE as f32) as i32) }
    #[inline(always)] pub fn to_f32(self) -> f32 { self.0 as f32 / Self::SCALE as f32 }
    #[inline(always)] pub fn mul(self, other: Self) -> Self {
        // (a * b) >> 31
        Self(((self.0 as i64 * other.0 as i64) >> 31) as i32)
    }
}

impl Q4_28 {
    #[inline(always)] pub fn from_f32(f: f32) -> Self { Self((f * (1<<28) as f32) as i32) }
    #[inline(always)] pub fn to_f32(self) -> f32 { self.0 as f32 / (1<<28) as f32 }
}

impl Q1_15 {
    #[inline(always)] pub fn from_f32(f: f32) -> Self { Self((f.clamp(-1.0,0.9999) * 32767.0) as i16) }
    #[inline(always)] pub fn to_f32(self) -> f32 { self.0 as f32 / 32767.0 }
}