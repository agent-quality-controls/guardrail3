pub mod lockfile_integrity;
#[expect(
    clippy::module_name_repetitions,
    reason = "module name mirrors the migration-consistency check id and its assertion crate API surface; renaming would break the documented per-rule assertion path used by callers"
)]
pub mod migration_consistency;
