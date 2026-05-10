pub mod file_size_cap;
#[expect(
    clippy::module_name_repetitions,
    reason = "module name mirrors the gitleaks-scan check id and its assertion crate API surface; renaming would break the documented per-rule assertion path used by callers"
)]
pub mod gitleaks_scan;
#[expect(
    clippy::module_name_repetitions,
    reason = "module name mirrors the merge-conflict-scan check id and its assertion crate API surface; renaming would break the documented per-rule assertion path used by callers"
)]
pub mod merge_conflict_scan;
