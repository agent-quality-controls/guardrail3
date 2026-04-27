mod cliff_exists;
mod input_failures;
mod license_file;
mod readme_exists;
mod release_plz_exists;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
