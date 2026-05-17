/// `exists` module.
mod exists;
/// `legacy_file` module.
mod legacy_file;
/// `run` module.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
