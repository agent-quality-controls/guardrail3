use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_01_crate_level_allow::{Severity, 
    RuleFinding, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_crate_and_nested_module_wide_allows_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let crate_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let inline_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let mixed_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let test_rel = "apps/backend/tests/allow_inventory_tests.rs";
    let exempt_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let mixed_exempt_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let item_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";

    let crate_content = test_support::read_file(root, crate_rel);
    let inline_content = test_support::read_file(root, inline_rel);
    let mixed_content = test_support::read_file(root, mixed_rel);
    let exempt_content = test_support::read_file(root, exempt_rel);
    let mixed_exempt_content = test_support::read_file(root, mixed_exempt_rel);
    let item_content = test_support::read_file(root, item_rel);

    let crate_new = format!("#![allow(clippy::unwrap_used)]\n{crate_content}\n");
    let inline_new = format!(
        "{inline_content}\nmod nested_allow {{\n    #![allow(clippy::panic)]\n    pub fn helper() {{}}\n}}\n"
    );
    let mixed_new = format!(
        "#![allow(clippy::unwrap_used)]\n{mixed_content}\nmod outer {{\n    mod inner {{\n        #![allow(clippy::panic)]\n        pub fn helper() {{}}\n    }}\n}}\n"
    );
    let mixed_exempt_new = format!(
        "#![allow(unused_crate_dependencies)]\n#![allow(clippy::unwrap_used)]\n{mixed_exempt_content}\nmod exempt_inline {{\n    #![allow(unused_crate_dependencies)]\n    pub fn helper() {{}}\n}}\n"
    );
    write_file(root, crate_rel, &crate_new);
    write_file(root, inline_rel, &inline_new);
    write_file(root, mixed_rel, &mixed_new);
    write_file(
        root,
        test_rel,
        "mod nested_test_allow {\n    #![allow(clippy::expect_used)]\n    pub fn helper() {}\n}\n",
    );
    write_file(
        root,
        exempt_rel,
        &format!("#![allow(unused_crate_dependencies)]\n{exempt_content}\n"),
    );
    write_file(root, mixed_exempt_rel, &mixed_exempt_new);
    write_file(
        root,
        item_rel,
        &format!("{item_content}\n#[allow(clippy::unwrap_used)]\npub fn item_level_probe() {{}}\n"),
    );

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file.as_deref(),
                Some(path)
                    if [
                        crate_rel,
                        inline_rel,
                        mixed_rel,
                        test_rel,
                        mixed_exempt_rel,
                        exempt_rel,
                        item_rel,
                    ]
                    .contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_files(
        &relevant_results,
        BTreeSet::from([
            crate_rel.to_owned(),
            inline_rel.to_owned(),
            mixed_rel.to_owned(),
            mixed_exempt_rel.to_owned(),
            test_rel.to_owned(),
        ]),
    );
    assert_findings(
        &relevant_results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "crate-level allow",
                message: "Crate/module-wide `allow(clippy::unwrap_used)` suppresses the lint too broadly.",
                file: Some(crate_rel),
                line: crate_new
                    .lines()
                    .position(|line| line.contains("#![allow(clippy::unwrap_used)]"))
                    .map(|index| index + 1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "module-level allow in outer::inner",
                message: "Crate/module-wide `allow(clippy::panic)` suppresses the lint too broadly.",
                file: Some(mixed_rel),
                line: mixed_new
                    .lines()
                    .position(|line| line.contains("#![allow(clippy::panic)]"))
                    .map(|index| index + 1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "crate-level allow",
                message: "Crate/module-wide `allow(clippy::unwrap_used)` suppresses the lint too broadly.",
                file: Some(mixed_rel),
                line: mixed_new
                    .lines()
                    .position(|line| line.contains("#![allow(clippy::unwrap_used)]"))
                    .map(|index| index + 1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "crate-level allow",
                message: "Crate/module-wide `allow(clippy::unwrap_used)` suppresses the lint too broadly.",
                file: Some(mixed_exempt_rel),
                line: mixed_exempt_new
                    .lines()
                    .position(|line| line.contains("#![allow(clippy::unwrap_used)]"))
                    .map(|index| index + 1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "module-level allow in nested_test_allow",
                message: "Crate/module-wide allow for `clippy::expect_used` is test-file exempt.",
                file: Some(test_rel),
                line: Some(2),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "module-level allow in nested_allow",
                message: "Crate/module-wide `allow(clippy::panic)` suppresses the lint too broadly.",
                file: Some(inline_rel),
                line: inline_new
                    .lines()
                    .position(|line| line.contains("#![allow(clippy::panic)]"))
                    .map(|index| index + 1),
                inventory: false,
            },
        ],
    );
}
