/// Rule: a `cliff.toml` must exist for changelog generation.
mod cliff_exists;
/// Rule: surface input-failure findings for the file-tree input.
mod input_failures;
/// Rule: a recognized `LICENSE` file must exist.
mod license_file;
/// Rule: a `README.md` must exist for publishable crates.
mod readme_exists;
/// Rule: a `release-plz.toml` must exist.
mod release_plz_exists;
/// Family runner that dispatches to per-rule check modules.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
