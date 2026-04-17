mod common;

#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod rs_release_filetree_01_license_file;
#[cfg(feature = "checks")]
pub mod rs_release_filetree_02_release_plz_exists;
#[cfg(feature = "checks")]
pub mod rs_release_filetree_03_cliff_exists;
#[cfg(feature = "checks")]
pub mod rs_release_filetree_04_readme_exists;
#[cfg(feature = "checks")]
pub mod rs_release_filetree_05_input_failures;
