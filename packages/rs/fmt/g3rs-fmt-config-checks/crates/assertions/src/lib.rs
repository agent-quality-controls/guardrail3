use g3rs_fmt_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod run;

#[cfg(feature = "checks")]
pub mod rs_fmt_config_01_settings;
#[cfg(feature = "checks")]
pub mod rs_fmt_config_02_extra_settings;
#[cfg(feature = "checks")]
pub mod rs_fmt_config_03_nightly_keys_on_stable;
#[cfg(feature = "checks")]
pub mod rs_fmt_config_04_edition_mismatch;
#[cfg(feature = "checks")]
pub mod rs_fmt_config_07_ignore_escape_hatch;
