/// Typed model definitions for `rust-toolchain.toml`.
mod rust_toolchain_toml;

#[cfg(feature = "api")]
pub use rust_toolchain_toml::{RustToolchainToml, ToolchainSection};
