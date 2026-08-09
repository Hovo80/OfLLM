// gl4-core-public/benches/gl4_bench.rs - публичная часть Q20-ARM v17.4
// Автор: Martirosyan Hovhannes - Gayane Soft
// Что внутри: Criterion бенч для LUT, fixed-point, AI ядра

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gl4_core_public::{FixedI16, fnc_ai, gl4_lut};

fn bench_lut(c: &mut Criterion) {
    c.bench_function("gl4_lut 1M", |b| {
        b.iter(|| {
            for i in 0..1_000_000 {
                black_box(gl4_lut(black_box(i as u8 % 16)));
            }
        })
    });
}

fn bench_fixed(c: &mut Criterion) {
    c.bench_function("FixedI16 mul 1M", |b| {
        b.iter(|| {
            let a = FixedI16::from_f32(1.5);
            let b_val = FixedI16::from_f32(2.5);
            for _ in 0..1_000_000 {
                black_box(a * b_val);
            }
        })
    });
}

fn bench_ai(c: &mut Criterion) {
    c.bench_function("fnc_ai dot 1K", |b| {
        let input: Vec<FixedI16> = (0..128).map(|x| FixedI16::from_f32(x as f32 * 0.1)).collect();
        let weights: Vec<FixedI16> = (0..128).map(|x| FixedI16::from_f32(x as f32 * 0.01)).collect();
        b.iter(|| {
            black_box(fnc_ai::dot_product(black_box(&input), black_box(&weights)))
        })
    });
}

criterion_group!(benches, bench_lut, bench_fixed, bench_ai);
criterion_main!(benches);
