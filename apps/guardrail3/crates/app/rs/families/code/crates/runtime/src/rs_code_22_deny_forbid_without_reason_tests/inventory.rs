use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_22_deny_forbid_without_reason::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_undocumented_deny_forbid_attrs_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    let backend_line = backend_content.lines().count() + 2;
    let worker_info_line = 1;
    let worker_error_line = worker_content.lines().count() + 4;

    write_file(
        root,
        backend_rel,
        &format!("{backend_content}\n#[deny(clippy::panic)]\nfn planner_policy_probe() {{}}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "#![forbid(unsafe_code)]\n{worker_content}\n\n#[forbid(clippy::expect_used)]\nfn worker_policy_probe() {{}}\n"
        ),
    );

    let results = run_family(root);

    assert_files(
        &results,
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "#[deny]/#[forbid] without reason",
                message: "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                file: Some(backend_rel),
                line: Some(backend_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "forbid(unsafe_code)",
                message: "`forbid(unsafe_code)` strengthens the local safety boundary.",
                file: Some(worker_rel),
                line: Some(worker_info_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "#[deny]/#[forbid] without reason",
                message: "`#[forbid(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                file: Some(worker_rel),
                line: Some(worker_error_line),
                inventory: false,
            },
        ],
    );
}

#[test]
fn attacks_grouped_inner_forbid_and_trait_item_across_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let test_rel = "apps/backend/crates/app/queries/tests/lint_policy.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    write_file(
        root,
        backend_rel,
        &format!(
            "#![forbid(unsafe_code, clippy::panic)]\n{backend_content}\n\ntrait PolicyProbe {{\n    #[deny(clippy::expect_used)]\n    fn run();\n}}\n"
        ),
    );
    write_file(
        root,
        test_rel,
        "#[deny(clippy::panic)]\nfn test_probe() {}\n",
    );

    let results = run_family(root);
    let backend_trait_line = test_support::read_file(root, backend_rel)
        .lines()
        .position(|line| line.contains("#[deny(clippy::expect_used)]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(
        &results,
        BTreeSet::from([backend_rel.to_owned(), test_rel.to_owned()]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "#[deny]/#[forbid] without reason",
                message: "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                file: Some(test_rel),
                line: Some(1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "#[deny]/#[forbid] without reason",
                message: "`#[forbid(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                file: Some(backend_rel),
                line: Some(1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "forbid(unsafe_code)",
                message: "`forbid(unsafe_code)` strengthens the local safety boundary.",
                file: Some(backend_rel),
                line: Some(1),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "#[deny]/#[forbid] without reason",
                message: "`#[deny(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                file: Some(backend_rel),
                line: Some(backend_trait_line),
                inventory: false,
            },
        ],
    );
}
