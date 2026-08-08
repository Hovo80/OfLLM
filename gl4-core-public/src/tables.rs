#![allow(dead_code)]
//! LUT таблицы - 100% L1 кэш. Все доступы через &15

/// Умножение знаковое GL4: 16x16 = 256B
pub static LUT_SIGNED_MUL_FLAT: [i8; 256] = {
    let mut t = [0i8; 256];
    let mut i = 0;
    while i < 256 {
        let a = (i >> 4) as u8 & 15;
        let b = (i & 15) as u8 & 15;
        let sa = if a >= 8 { a as i8 - 16 } else { a as i8 };
        let sb = if b >= 8 { b as i8 - 16 } else { b as i8 };
        t[i] = sa * sb;
        i += 1;
    }
    t
};

/// SAT ADD: 16x16
pub static SAT_ADD_FLAT: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let a = (i >> 4) & 15;
        let b = (i & 15) & 15;
        let sum = a + b;
        t[i] = if sum > 15 { 15 } else { sum as u8 };
        i += 1;
    }
    t
};

/// Активации - 16 значений. Для production расширь до 256 + интерполяция
pub static SIGMOID_TABLE: [u8; 16] = [0,0,1,2,3,5,7,9,11,13,14,14,15,15,15,15];
pub static RELU_TABLE: [u8; 16] = [0,0,0,0,0,0,0,0,8,9,10,11,12,13,14,15];
pub static GELU_TABLE: [u8; 16] = [0,0,0,1,2,4,6,8,10,11,13,14,14,15,15,15];
pub static LUT_SQUARE_I8: [u8; 16] = [0,1,4,9,16,25,36,49,64,49,36,25,16,9,4,1];