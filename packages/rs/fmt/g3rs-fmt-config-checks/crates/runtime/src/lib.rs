mod edition_mismatch;
mod extra_settings;
mod ignore_escape_hatch;
mod inputs;
mod nightly_keys_on_stable;
mod run;
mod settings;

#[cfg(feature = "checks")]
pub use run::check;
