use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_exception_comments_across_real_config_roots_with_exact_owned_hit_set() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let root_guardrail_rel = "guardrail3.toml";
    let root_rustfmt_rel = "rustfmt.toml";
    let root_toolchain_rel = "rust-toolchain.toml";
    let backend_cargo_rel = "apps/backend/Cargo.toml";
    let worker_cargo_rel = "apps/worker/Cargo.toml";
    let nested_cargo_rel = "apps/backend/crates/app/queries/Cargo.toml";

    let root_guardrail =
        std::fs::read_to_string(root.join(root_guardrail_rel)).expect("read guardrail");
    let root_rustfmt = std::fs::read_to_string(root.join(root_rustfmt_rel)).unwrap_or_default();
    let root_toolchain = std::fs::read_to_string(root.join(root_toolchain_rel)).unwrap_or_default();
    let backend_cargo =
        std::fs::read_to_string(root.join(backend_cargo_rel)).expect("read backend cargo");
    let worker_cargo =
        std::fs::read_to_string(root.join(worker_cargo_rel)).expect("read worker cargo");
    let nested_cargo =
        std::fs::read_to_string(root.join(nested_cargo_rel)).expect("read nested cargo");

    let root_line = root_guardrail.lines().count() + 2;
    let rustfmt_first_line = root_rustfmt.lines().count() + 1;
    let rustfmt_second_line = root_rustfmt.lines().count() + 2;
    let toolchain_line = root_toolchain.lines().count() + 1;
    let backend_line = backend_cargo.lines().count() + 2;
    let worker_line = worker_cargo.lines().count() + 2;
    let nested_line = nested_cargo.lines().count() + 2;

    write_file(
        root,
        root_guardrail_rel,
        &format!("{root_guardrail}\n# EXCEPTION: fixture root policy note\n"),
    );
    write_file(
        root,
        root_rustfmt_rel,
        &format!(
            "{root_rustfmt}# EXCEPTION: rustfmt policy inventory\n# EXCEPTION: rustfmt repeated inventory\n"
        ),
    );
    write_file(
        root,
        root_toolchain_rel,
        &format!("{root_toolchain}// EXCEPTION: toolchain rollout inventory\n"),
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
    write_file(
        root,
        nested_cargo_rel,
        &format!("{nested_cargo}\n# EXCEPTION: nested crate lint inventory\n"),
    );

    let results = run_family(root);
    let mut rs_code_07_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-07")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_07_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-07"),
        BTreeSet::from([
            root_guardrail_rel.to_owned(),
            root_rustfmt_rel.to_owned(),
            root_toolchain_rel.to_owned(),
            backend_cargo_rel.to_owned(),
            worker_cargo_rel.to_owned(),
            nested_cargo_rel.to_owned(),
        ])
    );
    assert_eq!(
        rs_code_07_results,
        vec![
            (
                backend_cargo_rel.to_owned(),
                Some(backend_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: # EXCEPTION: backend workspace lint inventory"
                    .to_owned(),
                true,
            ),
            (
                nested_cargo_rel.to_owned(),
                Some(nested_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: # EXCEPTION: nested crate lint inventory".to_owned(),
                true,
            ),
            (
                worker_cargo_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: # EXCEPTION: worker rollout inventory".to_owned(),
                true,
            ),
            (
                root_guardrail_rel.to_owned(),
                Some(root_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: # EXCEPTION: fixture root policy note".to_owned(),
                true,
            ),
            (
                root_toolchain_rel.to_owned(),
                Some(toolchain_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: // EXCEPTION: toolchain rollout inventory".to_owned(),
                true,
            ),
            (
                root_rustfmt_rel.to_owned(),
                Some(rustfmt_first_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: # EXCEPTION: rustfmt policy inventory".to_owned(),
                true,
            ),
            (
                root_rustfmt_rel.to_owned(),
                Some(rustfmt_second_line),
                format!("{:?}", Severity::Info),
                "EXCEPTION comment inventory".to_owned(),
                "Config exception comment: # EXCEPTION: rustfmt repeated inventory".to_owned(),
                true,
            ),
        ]
    );
}
