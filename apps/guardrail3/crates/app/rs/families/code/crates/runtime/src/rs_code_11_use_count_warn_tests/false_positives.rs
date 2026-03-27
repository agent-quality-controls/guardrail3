use guardrail3_app_rs_family_code_assertions::rs_code_11_use_count_warn::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_below_threshold_and_test_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let test_rel = "apps/backend/crates/app/commands/tests/use_band_tests.rs";
    let lower_rel = "apps/backend/crates/app/commands/src/use_band_probe.rs";
    let nested_rel = "apps/backend/crates/app/commands/src/nested_use_band_probe.rs";

    let test_imports = (0..20)
        .map(|index| format!("use crate::test_{index};"))
        .collect::<Vec<_>>()
        .join("\n");
    let lower_imports = (0..15)
        .map(|index| format!("use crate::prod_{index};"))
        .collect::<Vec<_>>()
        .join("\n");
    let nested_uses = (0..20)
        .map(|index| format!("    use crate::nested_{index};"))
        .collect::<Vec<_>>()
        .join("\n");

    write_file(root, test_rel, &test_imports);
    write_file(
        root,
        lower_rel,
        &format!("{lower_imports}\nfn probe() {{}}\n"),
    );
    write_file(
        root,
        nested_rel,
        &format!("mod nested {{\n{nested_uses}\n    pub fn probe() {{}}\n}}\n"),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
