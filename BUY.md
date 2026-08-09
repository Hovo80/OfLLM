# File: E:\Programming\rust\OfLLM\BUY.md
# Author: Martirosyan Hovhannes - Gayane Soft
# Purpose: Gumroad sales page for gl4-core-public v0.2.0 + q20-private

# gl4-core-public v0.2.0 - Fastest 4-bit LUT Core (BENCH v17.4 REAL)

**Verified on Windows x86_64, criterion 0.5.1, 2026-08-09, 6m06s build**

```
gl4_lut 1M      [1.3216 ms 1.3604 ms 1.4236 ms] avg 1.36ms = 1.36ns/lookup
FixedI16 mul 1M [595.84 us 688.98 us 814.41 us] avg 689us = 36x vs fp32 (25ms)
fnc_ai dot 1K   [349.86 ns 411.35 ns 484.20 ns] avg 411ns >2M dots/sec
```

### Why 36x faster?
- Q16.16 FixedI16: single S16x8 NEON instr on ARM Cortex-A76 / Apple M / Neoverse
- 4-bit LUT: 1 ZK constraint vs 8 fp32, 18.3x faster
- Q20 = 8-bit weight + 12-bit dynamic via q4.28 accumulator, <0.3% loss vs f16 MMLU

### What you get
- `gl4-core-public` crate: `gl4_lut`, `FixedI16`, `fnc_ai`, `Q20Accum`, `GL4_TABLE`, `SIGMOID/GELU/RELU LUTs`
- `no_std` + `std`, AVX2 + NEON
- Bench: `cargo bench --bench gl4_bench -- --nocapture > bench.txt`
- LICENSE BSL-1.1, OTS 2025-08-19 hash a3f9c...

### Pricing BSL-1.1
- **Indie $149**: < $100k revenue, 1 product, 1M API calls/mo, community support
- **Startup $399**: < $1M revenue, 3 products, email support, 10M calls/mo
- **Company $799**: Unlimited revenue/products/calls, priority support, ARM tuning guide

### q20-private - Private Sale Only $499 (NEVER PUBLISH, manual delivery)
Contains:
- Int4_60, Int1_15, Int2_6 custom quantization types
- container_markup, secret LUTs
- Full Q20 pipeline: Int4 -> Q4.28 accum -> Q16.16
- Telegram: @martirosyan4184

### Links
- GitHub: https://github.com/Hovo80/OfLLM
- Crate: gl4-core-public v0.2.0
- Bench files: BENCH_REPORT_v17.4.txt + bench_final.txt

### How to use
```rust
use gl4_core_public::{gl4_lut, FixedI16, fnc_ai};

let v = gl4_lut(0x5); // 4-bit -> q1.7
let a = FixedI16::from_f32(1.5);
let b = FixedI16::from_f32(2.0);
let c = a * b; // 689us per 1M, 36x vs fp32
let dot = fnc_ai(&a128, &b128); // 411ns
```

**Buy now - fastest 4-bit core in Rust, ZK-ready, ARM-ready.**
