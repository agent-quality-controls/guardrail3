//! Coverage maps — show which config file covers which crate/package.
//!
//! Each sub-module handles one config file type and documents the
//! actual tool resolution rules for that file.

pub mod clippy;
pub mod cspell;
pub mod deny;
pub mod engine;
pub mod eslint;
pub mod jscpd;
pub mod npmrc;
pub mod prettier;
pub mod rust_toolchain;
pub mod rustfmt;
pub mod stylelint;
pub mod tsconfig;
