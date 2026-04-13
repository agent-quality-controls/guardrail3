mod rs_release_filetree_01_license_file;
mod rs_release_filetree_02_release_plz_exists;
mod rs_release_filetree_03_cliff_exists;
mod rs_release_filetree_04_readme_exists;
mod rs_release_filetree_05_input_failures;
mod run;
#[cfg(test)]
mod test_support;
#[cfg(test)]
use g3rs_release_filetree_checks_assertions as _;

#[cfg(feature = "checks")]
pub use run::check;
