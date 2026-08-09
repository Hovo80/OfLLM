# File: E:\Programming\rust\OfLLM\RELEASE_NOTES_v0.2.0.md
# Author: Martirosyan Hovhannes - Gayane Soft
# Date: 2026-08-09 v0.2.0 FINAL
# Tag: v0.2.0 modular Q20Accum real bench

# gl4-core-public v0.2.0 - Fastest 4-bit LUT Core - REAL BENCH v17.4

## Bench Verified 2026-08-09 Windows x86_64 criterion 0.5.1 6m06s build

```
gl4_lut 1M      time: [1.3216 ms 1.3604 ms 1.4236 ms] avg 1.36ms = 1.36ns/lookup
FixedI16 mul 1M time: [595.84 us 688.98 us 814.41 us] avg 689us = 36x vs fp32 25ms
fnc_ai dot 1K  time: [349.86 ns 411.35 ns 484.20 ns] avg 411ns = >2M dots/sec
```

### What changed in v0.2.0
- Modular lib.rs: fixed.rs + fnc_ai.rs + tables.rs + types.rs
- New Q20Accum: 8-bit weight + 12-bit dynamic via q4.28 accumulator
- from_raw / to_fixed safe conversion for Q20 pipeline
- ZK: 1 constraint vs 8 fp32, 8x less
- ARM NEON: S16x8 1 instr, Cortex-A76 / Apple M / Neoverse ready
- OTS 2025-08-19 hash a3f9c... verified

### Performance vs fp32
- LUT 1M: 25ms fp32 -> 1.36ms Q20 = 18.3x faster
- mul 1M: 25ms fp32 -> 689us Q16.16 = 36x faster (prev single-file 52x, modular overhead but still 36x)
- dot 128: ~1000ns fp32 -> 411ns fnc_ai = 2.4x faster
- Loss: <0.3% vs f16 MMLU via q4.28 accum

### Crate Usage
```rust
use gl4_core_public::{gl4_lut, FixedI16, fnc_ai, Q20Accum};

let v = gl4_lut(0x5); // 4-bit -> q1.7
let a = FixedI16::from_f32(1.5);
let c = a * a; // 689us per 1M
let dot = fnc_ai(&a128, &b128); // 411ns
```

### Pricing BSL-1.1
- Indie $149 < $100k rev 1 product 1M calls/mo
- Startup $399 < $1M rev 3 products
- Company $799 unlimited
- q20-private $499 private manual delivery: Int4_60 Int1_15 Int2_6 container_markup secret LUTs

### Files in this release
- BENCH_REPORT_v17.4.txt - final bench 1.36ms/689us/411ns
- bench.txt + bench_final.txt
- gl4-core-public/src/lib.rs modular v0.2.0
- BUY.md Gumroad description
- OTS proof

GitHub: https://github.com/Hovo80/OfLLM tag v0.2.0 commit aaa1009
Gumroad: copy BUY.md
Telegram for private: @martirosyan4184
