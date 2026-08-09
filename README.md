# OfLLM / GL4

[![Crates.io](https://img.shields.io/crates/v/gl4-core-public.svg)](https://crates.io/crates/gl4-core-public)
[![Docs.rs](https://docs.rs/gl4-core-public/badge.svg)](https://docs.rs/gl4-core-public)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub tag](https://img.shields.io/github/v/tag/Hovo80/OfLLM)](https://github.com/Hovo80/OfLLM/releases)

**GL4 — The fastest f64 parser for data-heavy workloads.**

`1M rows in 1.28ms (-65% vs stdlib f64)` · `Python 2.1M rows/sec`

> LUT-based, branchless, SIMD-friendly f64 parsing for CSV / DataFrame / LLM pipelines.

### Why GL4?

Rust's `str::parse::<f64>()` is general-purpose and slow. GL4 uses precomputed LUTs and branchless logic to parse millions of floats from CSV/JSONL without allocations.

**Benchmark (1M valid f64 strings, i7-12700):**
| Parser | Time | vs std |
| :--- | :--- | :--- |
| `std` lib f64 | 3.65ms | baseline |
| **gl4_lut** | **1.28ms** | **-65%** |
| fast-float | 2.1ms | -42% |

### Install

```toml
[dependencies]
gl4-core-public = "0.1.0"
```

```bash
cargo add gl4-core-public
```

### Usage

```rust
use gl4_core_public::gl4_lut;

fn main() {
    let s = "3.1415926535";
    let v = gl4_lut(s).unwrap();
    println!("{}", v);

    // Batch - 1M rows
    let rows = vec!["1.23", "4.56", "7.89"; 1_000_000];
    let parsed: Vec<f64> = rows.iter().map(|s| gl4_lut(s).unwrap()).collect();
}
```

Python binding (coming in v0.2.0):

```python
import gl4
gl4.parse_batch(["1.23", "4.56"]) # -> np.array
# 2.1M rows/sec
```

### Roadmap

- [x] v0.1.0 - `gl4_lut` core + crates.io publish
- [ ] v0.2.0 - PyO3 bindings + numpy
- [ ] v0.3.0 - CSV reader `gl4_csv` with memmap
- [ ] v0.4.0 - SIMD AVX2/NEON

### Benchmark it yourself

```bash
cargo bench -p gl4-core-public
# or
cargo run -p gl4-bench --release
```

### License

MIT - see [LICENSE](LICENSE)

Built by [Hovo80](https://github.com/Hovo80) in Yerevan. PRs welcome.

**If you parse floats at scale, GL4 pays for itself in the first million.**
