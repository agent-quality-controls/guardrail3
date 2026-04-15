use g3rs_fmt_filetree_checks_assertions::run::rule as assertions;
use test_support::{G3RsFmtConfigFileKind, input};

#[test]
fn run_combines_all_filetree_findings() {
    let results = super::super::rule::check(&input(
        None,
        None,
        vec![(
            "crates/api/rustfmt.toml",
            G3RsFmtConfigFileKind::RustfmtToml,
        )],
        vec![""],
    ));
    assertions::assert_combined_all_filetree_findings(&results);
}
