mod rule;
pub use rule::{check};

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::helpers::canonical_clippy_toml;

// Note: canonical_clippy_toml is re-exported because facts/tests/scoped_files.rs
// references it cross-module as rs_garde_config_02_core_method_bans::canonical_clippy_toml().
