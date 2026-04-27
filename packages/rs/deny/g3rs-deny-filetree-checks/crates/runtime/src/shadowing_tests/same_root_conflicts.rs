use g3rs_deny_filetree_checks_assertions::shadowing as assertions;
use test_support::input;

#[test]
fn errors_on_same_root_precedence_conflict() {
    let input = input(
        Some("deny.toml"),
        vec!["deny.toml", ".deny.toml", ".cargo/deny.toml"],
        Vec::new(),
    );
    let mut results = Vec::new();

    crate::shadowing::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml.",
            ".cargo/deny.toml",
            false,
        )],
    );
}
