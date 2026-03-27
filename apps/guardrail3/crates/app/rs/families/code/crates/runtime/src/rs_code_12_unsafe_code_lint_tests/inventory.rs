use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn distinguishes_deny_and_forbid_workspace_lint_levels_across_real_manifests() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/Cargo.toml";
    let devctl_rel = "apps/devctl/Cargo.toml";
    let worker_rel = "apps/worker/Cargo.toml";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend cargo");
    let worker_content = std::fs::read_to_string(root.join(worker_rel)).expect("read worker cargo");

    write_file(
        root,
        backend_rel,
        &backend_content.replace("unsafe_code = \"forbid\"", "unsafe_code = \"deny\""),
    );
    write_file(
        root,
        worker_rel,
        &worker_content.replace("unsafe_code = \"forbid\"", "unsafe_code = \"forbid\""),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-12"),
        BTreeSet::from([
            backend_rel.to_owned(),
            devctl_rel.to_owned(),
            worker_rel.to_owned(),
        ])
    );
    let mut actual = results
        .iter()
        .filter(|result| result.id == "RS-CODE-12")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.severity,
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    actual.sort_by_key(|(file, severity, inventory)| {
        (file.clone(), format!("{severity:?}"), *inventory)
    });
    assert_eq!(
        actual,
        vec![
            (backend_rel.to_owned(), Severity::Error, false),
            (devctl_rel.to_owned(), Severity::Info, true),
            (worker_rel.to_owned(), Severity::Info, true),
        ]
    );
}
