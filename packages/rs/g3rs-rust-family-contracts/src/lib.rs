//! Facade crate for the rust-family hook contracts shared across `g3rs` families.

#[cfg(feature = "api")]
pub use g3rs_rust_family_contracts_runtime::{RustFamily, family_hook_contract};
