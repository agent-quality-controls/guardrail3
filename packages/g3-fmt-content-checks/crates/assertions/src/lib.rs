use g3_fmt_content_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_fmt_02_settings;
#[cfg(feature = "checks")]
pub mod rs_fmt_03_extra_settings;
#[cfg(feature = "checks")]
pub mod rs_fmt_04_nightly_keys_on_stable;
#[cfg(feature = "checks")]
pub mod rs_fmt_06_edition_mismatch;
