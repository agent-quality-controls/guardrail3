use g3rs_toolchain_filetree_checks_assertions::run as assertions;

use super::helpers::run_check;

#[test]
fn modern_only_emits_only_filetree_01_inventory() {
    assertions::assert_modern_only(&run_check(Some("rust-toolchain.toml"), None));
}

#[test]
fn legacy_only_emits_missing_modern_and_legacy_warn() {
    assertions::assert_legacy_only_without_modern(&run_check(None, Some("rust-toolchain")));
}

#[test]
fn both_files_emit_modern_inventory_and_legacy_conflict() {
    assertions::assert_both_files_present(&run_check(
        Some("rust-toolchain.toml"),
        Some("rust-toolchain"),
    ));
}

#[test]
fn neither_file_emits_only_missing_modern() {
    assertions::assert_missing_modern_only(&run_check(None, None));
}
