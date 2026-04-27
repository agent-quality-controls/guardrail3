mod common;

#[cfg(feature = "checks")]
pub mod cliff_exists;
#[cfg(feature = "checks")]
pub mod input_failures;
#[cfg(feature = "checks")]
pub mod license_file;
#[cfg(feature = "checks")]
pub mod readme_exists;
#[cfg(feature = "checks")]
pub mod release_plz_exists;
#[cfg(feature = "checks")]
pub mod run;
