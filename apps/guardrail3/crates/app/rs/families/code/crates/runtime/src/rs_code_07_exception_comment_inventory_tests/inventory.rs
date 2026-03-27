use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_07_exception_comment_inventory::{
    RuleFinding, assert_files, assert_findings,
};
use test_support::write_file;

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

    let root_guardrail = test_support::read_file(root, root_guardrail_rel);
    let root_rustfmt = test_support::read_file(root, root_rustfmt_rel);
    let root_toolchain = test_support::read_file(root, root_toolchain_rel);
    let backend_cargo = test_support::read_file(root, backend_cargo_rel);
    let worker_cargo = test_support::read_file(root, worker_cargo_rel);
    let nested_cargo = test_support::read_file(root, nested_cargo_rel);

    let root_line = root_guardrail.lines().count() + 2;
    let rustfmt_first_line = root_rustfmt.lines().count() + 1;
    let rustfmt_second_line = root_rustfmt.lines().count() + 2;
    let rustfmt_inline_line = root_rustfmt.lines().count() + 3;
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
            "{root_rustfmt}# EXCEPTION: rustfmt policy inventory\n# EXCEPTION: rustfmt repeated inventory\nuse_small_heuristics = \"Default\" # EXCEPTION: rustfmt inline inventory\n"
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

    assert_files(
        &results,
        BTreeSet::from([
            root_guardrail_rel.to_owned(),
            root_rustfmt_rel.to_owned(),
            root_toolchain_rel.to_owned(),
            backend_cargo_rel.to_owned(),
            worker_cargo_rel.to_owned(),
            nested_cargo_rel.to_owned(),
        ]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: backend workspace lint inventory",
                file: Some(backend_cargo_rel),
                line: Some(backend_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: nested crate lint inventory",
                file: Some(nested_cargo_rel),
                line: Some(nested_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: worker rollout inventory",
                file: Some(worker_cargo_rel),
                line: Some(worker_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: fixture root policy note",
                file: Some(root_guardrail_rel),
                line: Some(root_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: // EXCEPTION: toolchain rollout inventory",
                file: Some(root_toolchain_rel),
                line: Some(toolchain_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: rustfmt policy inventory",
                file: Some(root_rustfmt_rel),
                line: Some(rustfmt_first_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: rustfmt repeated inventory",
                file: Some(root_rustfmt_rel),
                line: Some(rustfmt_second_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "EXCEPTION comment inventory",
                message: "Config exception comment: # EXCEPTION: rustfmt inline inventory",
                file: Some(root_rustfmt_rel),
                line: Some(rustfmt_inline_line),
                inventory: true,
            },
        ],
    );
}
