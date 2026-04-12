mod inputs;
mod rs_fmt_config_01_settings;
mod rs_fmt_config_02_extra_settings;
mod rs_fmt_config_03_nightly_keys_on_stable;
mod rs_fmt_config_04_edition_mismatch;
mod rs_fmt_config_07_ignore_escape_hatch;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
