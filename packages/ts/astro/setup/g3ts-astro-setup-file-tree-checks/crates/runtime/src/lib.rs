mod astro_config_exists;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
