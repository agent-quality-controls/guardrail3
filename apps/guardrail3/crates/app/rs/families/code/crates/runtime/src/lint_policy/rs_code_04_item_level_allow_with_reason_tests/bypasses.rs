use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_04_item_level_allow_with_reason::{
    RuleFinding, Severity, assert_findings,
};
use test_support::write_file;

#[test]
fn reports_documented_item_level_expects_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let top_level_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let grouped_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let module_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";

    let top_level_content = test_support::read_file(root, top_level_rel);
    let grouped_content = test_support::read_file(root, grouped_rel);
    let module_content = test_support::read_file(root, module_rel);

    let top_level_new = format!(
        "{top_level_content}\n#[expect(clippy::unwrap_used)] // reason: compatibility probe\npub fn documented_expect_probe() {{}}\n"
    );
    let grouped_new = format!(
        "{grouped_content}\n#[expect(clippy::unwrap_used, clippy::expect_used)] // reason: grouped expectation surface\npub fn grouped_expect_probe() {{}}\n"
    );
    let module_new = format!(
        "{module_content}\n#[expect(clippy::panic)] // reason: documented extern-facing probe\npub mod documented_expect_module_probe {{\n    pub fn helper() {{}}\n}}\n"
    );

    write_file(root, top_level_rel, &top_level_new);
    write_file(root, grouped_rel, &grouped_new);
    write_file(root, module_rel, &module_new);

    let top_level_line = top_level_new
        .lines()
        .position(|line| {
            line.contains("#[expect(clippy::unwrap_used)] // reason: compatibility probe")
        })
        .map(|index| index + 1)
        .unwrap_or_default();
    let grouped_line = grouped_new
        .lines()
        .position(|line| {
            line.contains(
                "#[expect(clippy::unwrap_used, clippy::expect_used)] // reason: grouped expectation surface",
            )
        })
        .map(|index| index + 1)
        .unwrap_or_default();
    let module_line = module_new
        .lines()
        .position(|line| {
            line.contains("#[expect(clippy::panic)] // reason: documented extern-facing probe")
        })
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[
            RuleFinding::new(
                Severity::Warn,
                "item-level expect with reason",
                "#[expect(clippy::unwrap_used)] reason: compatibility probe",
                Some(top_level_rel),
                Some(top_level_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "item-level expect with reason",
                "#[expect(clippy::expect_used)] reason: grouped expectation surface",
                Some(grouped_rel),
                Some(grouped_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "item-level expect with reason",
                "#[expect(clippy::unwrap_used)] reason: grouped expectation surface",
                Some(grouped_rel),
                Some(grouped_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "item-level expect with reason",
                "#[expect(clippy::panic)] reason: documented extern-facing probe",
                Some(module_rel),
                Some(module_line),
                false,
            ),
        ],
    );
}
