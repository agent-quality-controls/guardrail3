use g3rs_fmt_filetree_checks_assertions::{
    rs_fmt_filetree_01_exists as exists_assertions,
    rs_fmt_filetree_05_per_crate_override as override_assertions,
    rs_fmt_filetree_08_dual_file_conflict as conflict_assertions,
};
use g3rs_fmt_filetree_checks_types::G3RsFmtConfigFileKind;

use crate::{run::check, test_support::input};

#[test]
fn run_combines_all_filetree_findings() {
    let results = check(&input(
        None,
        None,
        vec![(
            "crates/api/rustfmt.toml",
            G3RsFmtConfigFileKind::RustfmtToml,
        )],
        vec![""],
    ));

    exists_assertions::assert_findings(
        &results,
        &[exists_assertions::error(
            "rustfmt config missing",
            "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.",
            "rustfmt.toml",
            false,
        )],
    );
    override_assertions::assert_findings(
        &results,
        &[override_assertions::error(
            "Illegal nested rustfmt config",
            "`rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
            "crates/api/rustfmt.toml",
            false,
        )],
    );
    conflict_assertions::assert_findings(
        &results,
        &[conflict_assertions::warn(
            "Conflicting rustfmt config files",
            "Both `rustfmt.toml` and `.rustfmt.toml` exist in `.`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.",
            "rustfmt.toml",
            false,
        )],
    );
}
