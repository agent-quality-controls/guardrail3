mod inputs;
mod rs_fmt_02_settings;
mod rs_fmt_03_extra_settings;
mod rs_fmt_04_nightly_keys_on_stable;
mod rs_fmt_06_edition_mismatch;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
