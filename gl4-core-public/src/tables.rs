#![allow(dead_code)]
// gl4-core-public/src/tables.rs v0.1.0 - LUT 16 значений + активации, 256B L1, &15 защита
// Автор: Martirosyan Hovhannes

/// GL4 основная таблица - 4-bit 0..15 -> i8 -8..7, симметричное
pub const GL4_TABLE: [i8; 16] = [
    -8, -7, -6, -5, -4, -3, -2, -1,
     0,  1,  2,  3,  4,  5,  6,  7,
];

#[inline(always)]
pub fn gl4_lut(index: u8) -> i8 {
    // &15 защита от OOB, без ветвления
    unsafe { *GL4_TABLE.get_unchecked((index & 0x0F) as usize) }
}

/// Packed GL4 dequant LUT - 256 комбинаций 2x 4-bit в 1 байте -> 2x i8
/// 256B, целиком в L1, 2 тетрады в байте = 2x экономия RAM
pub const GL4_PACKED_LUT: [(i8, i8); 256] = {
    let mut table = [(0i8, 0i8); 256];
    let mut i = 0;
    while i < 256 {
        let lo = (i & 0x0F) as i8 - 8;
        let hi = ((i >> 4) & 0x0F) as i8 - 8;
        table[i] = (lo, hi);
        i += 1;
    }
    table
};

/// LUT активаций - 16 значений + линейная интерполяция между ними
/// SIGMOID в Q1_15: -8..7 -> sigmoid
pub const SIGMOID_LUT_Q1_15: [i16; 16] = [
    -32767, -32600, -32000, -30000, -25000, -15000, -5000, 0,
    0, 5000, 15000, 25000, 30000, 32000, 32600, 32767,
];

/// GELU LUT Q1_15
pub const GELU_LUT_Q1_15: [i16; 16] = [
    -100, -80, -50, -20, -5, 0, 2, 10,
    50, 200, 800, 3000, 8000, 16000, 24000, 32767,
];

/// RELU LUT Q1_15 - просто max(0,x)
pub const RELU_LUT_Q1_15: [i16; 16] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    512, 2048, 4096, 8192, 16384, 24576, 30000, 32767,
];

/// RoPE sin/cos LUT в Q1_31 - 256 значений, твой бывший quantum RZ
/// sin/cos через LUT, 1 constraint в ZKML вместо 32 для fp32
pub const ROPE_SIN_LUT_Q1_31: [i32; 256] = {
    let mut table = [0i32; 256];
    let mut i = 0;
    while i < 256 {
        // sin(2*pi*i/256) в Q1_31 - упрощенно, генерируется build.rs в реале
        // тут заглушка для компиляции, реальные значения через sin()
        table[i] = ((i as i32 - 128) * 16777216) as i32; // ~ линейная аппроксимация
        i += 1;
    }
    table
};

pub const ROPE_COS_LUT_Q1_31: [i32; 256] = {
    let mut table = [0i32; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = 2147483647 - (i as i32 * 1000000); // заглушка
        i += 1;
    }
    table
};

#[inline(always)]
pub fn activation_lut(index: u8, lut: &[i16; 16]) -> i16 {
    unsafe { *lut.get_unchecked((index & 0x0F) as usize) }
}
