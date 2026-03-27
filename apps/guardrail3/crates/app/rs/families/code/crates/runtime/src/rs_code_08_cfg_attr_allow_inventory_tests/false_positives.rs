use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_always_true_and_non_cfg_attr_allow_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let always_true_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let plain_item_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let crate_level_rel = "apps/worker/crates/app/processor/src/lib.rs";
    let grouped_conditional_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let negated_always_true_rel = "apps/backend/crates/app/commands/src/lib.rs";

    let always_true_content =
        std::fs::read_to_string(root.join(always_true_rel)).expect("read always true file");
    let plain_item_content =
        std::fs::read_to_string(root.join(plain_item_rel)).expect("read plain item file");
    let crate_level_content =
        std::fs::read_to_string(root.join(crate_level_rel)).expect("read crate level file");
    let grouped_conditional_content = std::fs::read_to_string(root.join(grouped_conditional_rel))
        .expect("read grouped conditional file");
    let negated_always_true_content = std::fs::read_to_string(root.join(negated_always_true_rel))
        .expect("read negated always true file");

    write_file(
        root,
        always_true_rel,
        &format!(
            "{always_true_content}\n#[cfg_attr(any(unix, windows), allow(clippy::unwrap_used))]\nfn always_true_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        plain_item_rel,
        &format!(
            "{plain_item_content}\n#[allow(clippy::unwrap_used)]\nfn plain_allow_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        crate_level_rel,
        &format!("#![allow(clippy::expect_used)]\n{crate_level_content}\n"),
    );
    write_file(
        root,
        grouped_conditional_rel,
        &format!(
            "{grouped_conditional_content}\n#[cfg_attr(all(), allow(clippy::panic))]\nfn grouped_always_true_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        negated_always_true_rel,
        &format!(
            "{negated_always_true_content}\n#[cfg_attr(not(never_target), allow(clippy::expect_used))]\nfn negated_always_true_probe() {{}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_08_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-08")
        .collect::<Vec<_>>();
    let rs_code_18_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-18")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-08"), BTreeSet::new());
    assert!(rs_code_08_results.is_empty());
    assert_eq!(
        files_for_rule(&results, "RS-CODE-18"),
        BTreeSet::from([
            always_true_rel.to_owned(),
            grouped_conditional_rel.to_owned(),
            negated_always_true_rel.to_owned(),
        ])
    );
    assert_eq!(rs_code_18_results.len(), 3);
}
