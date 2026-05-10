/// `astro-config-exists` rule.
mod astro_config_exists;
/// Top-level runtime entry point.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
