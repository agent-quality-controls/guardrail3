use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};
use guardrail3_domain_report::Severity;

fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warn => 1,
        Severity::Info => 2,
    }
}

#[test]
fn attacks_undocumented_deny_forbid_attrs_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

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
    let mut rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_22_results.sort_by_key(|(file, line, severity, title, message, inventory)| {
        (
            file.clone(),
            *line,
            severity_rank(*severity),
            title.clone(),
            message.clone(),
            *inventory,
        )
    });

    assert_eq!(
        files_for_rule(&results, "RS-CODE-22"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(files_for_rule(&results, "RS-CODE-03"), BTreeSet::new());
    assert_eq!(files_for_rule(&results, "RS-CODE-04"), BTreeSet::new());
    assert_eq!(
        rs_code_22_results,
        vec![
            (
                Some(backend_rel.to_owned()),
                Some(backend_line),
                Severity::Error,
                "#[deny]/#[forbid] without reason".to_owned(),
                "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.".to_owned(),
                false,
            ),
            (
                Some(worker_rel.to_owned()),
                Some(worker_info_line),
                Severity::Info,
                "forbid(unsafe_code)".to_owned(),
                "`forbid(unsafe_code)` strengthens the local safety boundary.".to_owned(),
                true,
            ),
            (
                Some(worker_rel.to_owned()),
                Some(worker_error_line),
                Severity::Error,
                "#[deny]/#[forbid] without reason".to_owned(),
                "`#[forbid(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.".to_owned(),
                false,
            ),
        ]
    );
}

#[test]
fn attacks_grouped_inner_forbid_and_trait_item_across_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let test_rel = "apps/backend/crates/app/queries/tests/lint_policy.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
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
    let backend_trait_line = std::fs::read_to_string(root.join(backend_rel))
        .expect("read mutated backend source")
        .lines()
        .position(|line| line.contains("#[deny(clippy::expect_used)]"))
        .expect("trait item deny line")
        + 1;
    let mut rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_22_results.sort_by_key(|(file, line, severity, title, message, inventory)| {
        (
            file.clone(),
            *line,
            severity_rank(*severity),
            title.clone(),
            message.clone(),
            *inventory,
        )
    });

    assert_eq!(
        files_for_rule(&results, "RS-CODE-22"),
        BTreeSet::from([backend_rel.to_owned(), test_rel.to_owned()])
    );
    assert_eq!(
        rs_code_22_results,
        vec![
            (
                Some(test_rel.to_owned()),
                Some(1),
                Severity::Error,
                "#[deny]/#[forbid] without reason".to_owned(),
                "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.".to_owned(),
                false,
            ),
            (
                Some(backend_rel.to_owned()),
                Some(1),
                Severity::Error,
                "#[deny]/#[forbid] without reason".to_owned(),
                "`#[forbid(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.".to_owned(),
                false,
            ),
            (
                Some(backend_rel.to_owned()),
                Some(1),
                Severity::Info,
                "forbid(unsafe_code)".to_owned(),
                "`forbid(unsafe_code)` strengthens the local safety boundary.".to_owned(),
                true,
            ),
            (
                Some(backend_rel.to_owned()),
                Some(backend_trait_line),
                Severity::Error,
                "#[deny]/#[forbid] without reason".to_owned(),
                "`#[deny(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line.".to_owned(),
                false,
            ),
        ]
    );
}
