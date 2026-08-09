#![allow(dead_code)]
// File: src/lib.rs (корень воркспейса E:\Programming\rust\OfLLM\src\lib.rs)
// Description: Корневой lib - НЕ ПУБЛИЧНЫЙ, НЕ ПРИВАТНЫЙ, просто holder воркспейса
// Автор: Martirosyan Hovhannes - Gayane Soft
// Цель: Чтобы workspace собирался, но весь код живет в gl4-core-public и q20-private
// Этот файл НЕ публикуется на crates.io, publish = false в корневом Cargo.toml если он пакет

//! OfLLM - Q20-ARM v17.4 workspace root
//! - gl4-core-public v0.1.0 (GPL-3.0) - публичный крючок, LUT ядро
//! - q20-private v17.4.0-private (proprietary) - закрытая монетизация, компилятор, ARM kernels, ZK

pub const VERSION: &str = "17.4.0-root";
pub const WORKSPACE: &str = "OfLLM Q20-ARM v17.4";

/// Корневой модуль - пустой, чтобы `cargo check` в корне не падал
/// Весь AI код - в пакетах ниже
pub mod workspace_info {
    pub const MEMBERS: [&str; 2] = ["gl4-core-public", "q20-private"];
    pub const PUBLIC_API: &str = "gl4-core-public::FixedI16, gl4_lut, dot_product";
    pub const PRIVATE_API: &str = "q20-private::custom_types::Int4_60, Int1_15, Int2_6 + container_markup";
}

/// Проверка что публичный пакет компилится
#[cfg(test)]
mod tests {
    #[test]
    fn test_workspace_members_exist() {
        // просто чтобы lib.rs не был пустым для cargo test
        assert_eq!(super::workspace_info::MEMBERS.len(), 2);
    }
}
