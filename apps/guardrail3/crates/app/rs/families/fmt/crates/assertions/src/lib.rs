use guardrail3_app_rs_family_fmt as _;

#[cfg(feature = "checks")]
pub mod rs_fmt_01_exists;
#[cfg(feature = "checks")]
pub mod rs_fmt_02_settings;
#[cfg(feature = "checks")]
pub mod rs_fmt_03_extra_settings;
#[cfg(feature = "checks")]
pub mod rs_fmt_04_nightly_keys_on_stable;
#[cfg(feature = "checks")]
pub mod rs_fmt_05_per_crate_override;
#[cfg(feature = "checks")]
pub mod rs_fmt_06_edition_mismatch;
#[cfg(feature = "checks")]
pub mod rs_fmt_07_ignore_escape_hatch;
#[cfg(feature = "checks")]
pub mod rs_fmt_08_dual_file_conflict;
