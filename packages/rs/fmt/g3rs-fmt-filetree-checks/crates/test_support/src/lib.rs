//! Test support utilities for the `g3rs-fmt-filetree-checks` family.

/// Rule implementation for `input`.
mod input;

#[cfg(feature = "support")]
pub use g3rs_fmt_types::G3RsFmtConfigFileKind;

#[cfg(feature = "support")]
pub use input::input;
