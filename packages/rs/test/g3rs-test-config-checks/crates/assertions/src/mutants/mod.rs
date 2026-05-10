#![expect(
    clippy::module_name_repetitions,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
pub mod cargo_mutants_installed;
pub mod mutants_config_sane;
pub mod mutants_profile_present;
pub mod mutants_toml_exists;
pub mod mutation_hook_present;
