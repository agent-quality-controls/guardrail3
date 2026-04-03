use std::collections::BTreeSet;

use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_22_deny_forbid_without_reason::{
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
            RuleFinding::new(
                Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some(backend_rel),
                Some(backend_line),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "forbid(unsafe_code)",
                "`forbid(unsafe_code)` strengthens the local safety boundary.",
                Some(worker_rel),
                Some(worker_info_line),
                true,
            ),
            RuleFinding::new(
                Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[forbid(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some(worker_rel),
                Some(worker_error_line),
                false,
            ),
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
            RuleFinding::new(
                Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some(test_rel),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[forbid(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some(backend_rel),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "forbid(unsafe_code)",
                "`forbid(unsafe_code)` strengthens the local safety boundary.",
                Some(backend_rel),
                Some(1),
                true,
            ),
            RuleFinding::new(
                Severity::Error,
                "#[deny]/#[forbid] without reason",
                "`#[deny(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                Some(backend_rel),
                Some(backend_trait_line),
                false,
            ),
        ],
    );
}
