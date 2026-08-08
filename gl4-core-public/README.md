# gl4-core-public - Q20-ARM v17.4 core

Production-ready 4-bit LUT core. Часть проекта Q20-ARM v17.4.

**Автор:** Martirosyan Hovhannes (Gayane Soft) - Yerevan, AM
**Лицензия:** GPL-3.0-only - бесплатно для open-source, платно для closed-source (dual-license).

## Что внутри
- `Gl4Digit = u8` - 4-битное значение 0..15
- `Q1_15`, `Q1_31`, `Q4_28`, `Q2_6` - fixed-point типы
- `ai_dot_packed_gl4` - 2 тетрады в байте, 256B LUT в L1, &15 защита от OOB
- LUT активаций: SIGMOID, GELU, RELU - 16 значений + линейная интерполяция
- `no_std` совместимо, работает на ARM NEON и x86 AVX2

## Почему это важно для ZKML
Halo2 / Plonky3 цепь: умножение fp32 = 32 constraints. GL4 LUT = 1 lookup. 
Наша модель дает 8x меньше constraints чем int8.

## Бенчмарк (на Raspberry Pi 4)
- dot 4096 dim GL4: ~ 4.2x быстрее чем llama.cpp int8
- RAM: 6MB vs 50MB fp32

## OpenTimestamp
Каждый релиз штампуется в Bitcoin через OpenTimestamps для доказательства авторства.

## Dual-license
Хочешь использовать в закрытом продукте - пиши: martirosyan4184 (Instagram) / Gayane Soft.

(c) 2026 Gayane Soft