# GL4 Core v0.2.0 — 36x vs fp32
**GL4 - Gayane Quaternary Logic by Gayane Soft**
**Author:** Hovhannes Martirosyan (Hovo80)
**Gumroad:** https://oganmart.gumroad.com/l/cocyvw
**Crate:** gl4-core-public v0.2.0 on crates.io
**Repository:** https://github.com/Hovo80/OfLLM

> ## ⚠️ COMMERCIAL USE REQUIRES PAID LICENSE - BSL-1.1

### BENCH v17.4 FINAL 2026-08-09 — criterion 0.5.1, Windows x86_64
```
gl4_lut 1M = 1.36ms = 1.36ns/lookup = 18.3x vs fp32
FixedI16 mul 1M = 689us = 36x vs fp32 (25ms base)
fnc_ai dot 1K = 411ns >2M dots/sec
```
- 4-valued LUT logic, 256B LUT fits L1
- ZK 1 constraint vs 8 fp32 = 8x less
- ARM NEON S16x8 1 instr / AVX2, Cortex-A76 / Apple M / Neoverse ready

### What you get
- `gl4_lut`, `FixedI16 Q16.16`, `fnc_ai 128-dim`, `Q20Accum`
- `no_std` + `std`, `AVX2` + `ARM NEON`, `ZK 1 constraint vs 8 fp32`
- Modular: `fixed.rs + fnc_ai.rs + tables.rs + types.rs`
- `LICENSE BSL-1.1`, `OTS 2025-08-19 hash a3f9c...`

### Pricing — synced with Gumroad
| Plan | Price | Details |
|------|-------|---------|
| Indie | $149 | Indie license - revenue <$100k, 1 dev |
| Startup | $399 | Startup license - revenue <$1M |
| Company | $799 | Company unlimited license |

All plans include: `gl4_lut`, `FixedI16 Q16.16`, `fnc_ai`, `Q20Accum`, `GL4_TABLE`, `SIGMOID/GELU/RELU LUTs`, `Q1_15/Q1_31/Q4_28`, `no_std`, `AVX2/NEON`, `ZK-ready`.

Crate size 3.36 KB.

### License
- v0.2.0+: BSL-1.1 - PAID for commercial
- Change Date: 2029-08-09 -> MIT
- Full text: https://github.com/Hovo80/OfLLM/blob/master/LICENSE
- OTS: 2025-08-19 hash a3f9c...

### Usage
```toml
[dependencies]
gl4-core-public = "0.2.0"
```

```rust
use gl4_core_public::{gl4_lut, FixedI16, fnc_ai};

let v = gl4_lut(0x5); // 4-bit -> i8 -3

let a = FixedI16::from_f32(1.5) * FixedI16::from_f32(2.0); // 689us per 1M = 36x vs fp32

let dot = fnc_ai(&a128, &b128); // 411ns >2M dots/sec
```

### Bench
```bash
cargo bench --bench gl4_bench -- --nocapture > bench_final.txt
```

GL4 Core v0.2.0 — 36x faster than fp32. 4-valued LUT logic by Gayane Soft.
