use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_exception_comments_across_real_config_roots_with_exact_owned_hit_set() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let root_guardrail_rel = "guardrail3.toml";
    let backend_cargo_rel = "apps/backend/Cargo.toml";
    let worker_cargo_rel = "apps/worker/Cargo.toml";

    let root_guardrail =
        std::fs::read_to_string(root.join(root_guardrail_rel)).expect("read guardrail");
    let backend_cargo =
        std::fs::read_to_string(root.join(backend_cargo_rel)).expect("read backend cargo");
    let worker_cargo =
        std::fs::read_to_string(root.join(worker_cargo_rel)).expect("read worker cargo");

    let root_line = root_guardrail.lines().count() + 1;
    let backend_line = backend_cargo.lines().count() + 1;
    let worker_line = worker_cargo.lines().count() + 1;

    write_file(
        root,
        root_guardrail_rel,
        &format!("{root_guardrail}\n# EXCEPTION: fixture root policy note\n"),
    );
    write_file(
        root,
        backend_cargo_rel,
        &format!("{backend_cargo}\n# EXCEPTION: backend workspace lint inventory\n"),
    );
    write_file(
        root,
        worker_cargo_rel,
        &format!("{worker_cargo}\n# EXCEPTION: worker rollout inventory\n"),
    );

    let results = run_family(root);
    let rs_code_07_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-07")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-07"),
        BTreeSet::from([
            root_guardrail_rel.to_owned(),
            backend_cargo_rel.to_owned(),
            worker_cargo_rel.to_owned(),
        ])
    );
    assert_eq!(rs_code_07_results.len(), 3);
    assert!(rs_code_07_results.iter().all(|result| result.inventory));
    assert_eq!(
        rs_code_07_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line))
            .collect::<Vec<_>>(),
        vec![
            (Some("guardrail3.toml"), Some(root_line)),
            (Some("apps/backend/Cargo.toml"), Some(backend_line)),
            (Some("apps/worker/Cargo.toml"), Some(worker_line)),
        ]
    );
}
