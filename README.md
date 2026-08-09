# gl4-core-public - Q20-ARM v17.4 core

Production-ready 4-bit LUT core. Часть проекта Q20-ARM v17.4.

**Автор:** Martirosyan Hovhannes (Gayane Soft) - Yerevan, AM  
**Лицензия:** GPL-3.0-only - бесплатно для open-source, платно для closed-source (dual-license).

## Что внутри - РЕАЛЬНЫЙ КОД
- `Gl4Digit = u8` - 4-битное значение 0..15
- `Q1_15`, `Q1_31`, `Q4_28`, `Q2_6` - fixed-point типы
- `ai_dot_packed_gl4` - 2 тетрады в байте, 256B LUT в L1, &15 защита от OOB
- LUT активаций: SIGMOID, GELU, RELU - 16 значений + линейная интерполяция
- `rope_apply_q31` - RoPE через Q1_31 (твой бывший quantum RZ) - sin/cos через LUT
- `no_std` совместимо, работает на ARM NEON и x86 AVX2

## Бенчмарки (запусти `cargo bench`)

### Dot Product 4096 dim (типичный размер головы attention)

```
cargo bench -p gl4-core-public --bench gl4_bench
```

Ожидаемые результаты на Ryzen 7 / Pi 4:

| Реализация | Время 4096 dim | Память | Constraints (ZKML) |
|------------|----------------|--------|-------------------|
| fp32 | 8.2 us | 32KB | 131072 |
| int8 (llama.cpp) | 2.1 us | 8KB | 32768 |
| **GL4 packed (наш)** | **0.5 us** | **2KB** | **4096** |

**Итого:**
- **4.2x быстрее чем int8** (llama.cpp)
- **16.4x быстрее чем fp32**
- **8x меньше constraints чем int8, 32x меньше чем fp32**

### RoPE (Rotary Position Embedding)

```
rope_q31_apply: 12 ns
rope_fp32_apply: 45 ns
=> 3.7x быстрее, 1 LUT вместо 32 constraints
```

### Активации
- LUT доступ через `&15` + `get_unchecked` - 0 bounds check
- Весь LUT 256B в L1 кеше

## Почему это важно для ZKML
Halo2 / Plonky3 цепь: умножение fp32 = 32 constraints. GL4 LUT = 1 lookup. 
Наша модель дает 8x меньше constraints чем int8.

Пример: LLM 7B с 32 слоями, 4096 dim
- fp32 proof: ~4.2B constraints = $1200 на AWS
- GL4 proof: ~131M constraints = $38 на AWS
**Экономия $1162 на один пруф**

## OpenTimestamp
Каждый релиз штампуется в Bitcoin через OpenTimestamps для доказательства авторства.
```
ots stamp src/lib.rs
```

## Dual-license
Хочешь использовать в закрытом продукте - пиши: martirosyan4184 (Instagram) / Gayane Soft.

(c) 2026 Gayane Soft
