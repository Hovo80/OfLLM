// File: gl4-core-public/src/types.rs
// Description: Fixed-point типы для GL4 - все вычисления без fp32, для ZKML 1 constraint вместо 32
// Автор: Martirosyan Hovhannes - Gayane Soft
// Типы: Gl4Digit (4-bit), Q1_15, Q1_31 (для RoPE), Q4_28, Q2_6 (KV-cache), Q4_60 (Adam)

#![allow(dead_code)]
/// Базовые типы. Все fixed-point - без float для ZKML и edge AI.

/// Gl4Digit - 4-битное значение 0..15, хранит Q1.3 или знаковое -8..7
/// Упаковка: 2 тетрады в 1 байте = 2x экономия RAM
pub type Gl4Digit = u8; // 0..15, хранит Q1.3

/// Q1_15: 1 знак + 15 дробных, диапазон -1..0.999, SCALE=32767
/// Используется для весов attention
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q1_15(pub i16);

/// Q1_31: 1 знак + 31 дробных, SCALE=2147483647
/// Твой тип из quantum32.txt - для RoPE sin/cos
/// 1 mul = 1 constraint в ZKML вместо 32 для fp32
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q1_31(pub i32);

/// Q4_28: 4 целых + 28 дробных, диапазон -8..7.999
/// Для аккумуляции dot product
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q4_28(pub i32);

/// Q2_6: 2 целых + 6 дробных, диапазон -2..1.98, 1 байт i8
/// Для KV-cache - экономия 4x RAM
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q2_6(pub i8);

/// Q4_60: 4 целых + 60 дробных, i64, для Adam моментов, только train
/// В инференсе не используется, только в q20-private
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Q4_60(pub i64);

impl Q1_31 {
    pub const SCALE: i64 = 2147483647; // 2^31-1
    pub const ONE: Self = Self(2147483647);
    
    /// Конверт f32 -> Q1_31 с clamp
    #[inline(always)] pub fn from_f32(f: f32) -> Self { 
        Self((f.clamp(-1.0,0.999) * Self::SCALE as f32) as i32) 
    }
    
    /// Конверт Q1_31 -> f32 для дебага
    #[inline(always)] pub fn to_f32(self) -> f32 { 
        self.0 as f32 / Self::SCALE as f32 
    }
    
    /// Умножение Q1_31 * Q1_31 -> Q1_31 с сдвигом >>31
    /// 1 constraint в ZKML
    #[inline(always)] pub fn mul(self, other: Self) -> Self {
        // (a * b) >> 31 - сохраняет Q1_31 формат
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
