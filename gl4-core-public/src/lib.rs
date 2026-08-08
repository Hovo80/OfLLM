#![cfg_attr(not(feature = "std"), no_std)]
//! gl4-core-public - production core of Q20-ARM v17.4
//! GPL-3.0-only

pub mod types;
pub mod tables;
pub mod fnc_ai;
pub mod fixed;

pub use types::{Gl4Digit, Q1_15, Q1_31, Q4_28, Q2_6};
pub use fnc_ai::{ai_dot_packed_gl4, ai_dot_gl4, ai_sigmoid, ai_gelu, ai_relu};

/// Версия для OpenTimestamp
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "Martirosyan Hovhannes - Gayane Soft - Q20-ARM v17.4";