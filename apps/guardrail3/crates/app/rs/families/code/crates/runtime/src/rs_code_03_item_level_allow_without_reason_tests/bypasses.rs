use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_03_item_level_allow_without_reason::{assert_files, assert_findings, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn detects_undocumented_item_level_allows_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let top_level_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let nested_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let grouped_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let module_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let trait_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let impl_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";

    let top_level_content =
        std::fs::read_to_string(root.join(top_level_rel)).expect("read top level file");
    let nested_content = std::fs::read_to_string(root.join(nested_rel)).expect("read nested file");
    let grouped_content =
        std::fs::read_to_string(root.join(grouped_rel)).expect("read grouped file");
    let module_content = std::fs::read_to_string(root.join(module_rel)).expect("read module file");
    let trait_content = std::fs::read_to_string(root.join(trait_rel)).expect("read trait file");
    let impl_content = std::fs::read_to_string(root.join(impl_rel)).expect("read impl file");

    let top_level_new = format!(
        "{top_level_content}\n#[allow(clippy::unwrap_used)]\npub fn undocumented_query_probe() {{}}\n"
    );
    let nested_new = format!(
        "{nested_content}\nmod nested_undocumented {{\n    #[allow(clippy::panic)]\n    pub fn helper() {{}}\n}}\n"
    );
    let grouped_new = format!(
        "{grouped_content}\n#[allow(clippy::unwrap_used, clippy::expect_used)]\npub fn grouped_probe() {{}}\n"
    );
    let module_new = format!(
        "{module_content}\n#[allow(clippy::panic)]\npub mod undocumented_module_probe {{\n    pub fn helper() {{}}\n}}\n"
    );
    let trait_new = format!(
        "{trait_content}\npub trait UndocumentedTraitBoundary {{\n    #[allow(clippy::expect_used)]\n    fn undocumented_trait_probe(&self);\n}}\n"
    );
    let impl_new = format!(
        "{impl_content}\nstruct UndocumentedImplBoundary;\nimpl UndocumentedImplBoundary {{\n    #[allow(clippy::panic)]\n    fn undocumented_impl_probe(&self) {{}}\n}}\n"
    );

    write_file(root, top_level_rel, &top_level_new);
    write_file(root, nested_rel, &nested_new);
    write_file(root, grouped_rel, &grouped_new);
    write_file(root, module_rel, &module_new);
    write_file(root, trait_rel, &trait_new);
    write_file(root, impl_rel, &impl_new);

    let top_level_line = top_level_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::unwrap_used)]"))
        .expect("top level allow line")
        + 1;
    let nested_line = nested_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)]"))
        .expect("nested allow line")
        + 1;
    let grouped_line = grouped_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::unwrap_used, clippy::expect_used)]"))
        .expect("grouped allow line")
        + 1;
    let module_line = module_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)]"))
        .expect("module allow line")
        + 1;
    let trait_line = trait_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::expect_used)]"))
        .expect("trait allow line")
        + 1;
    let impl_line = impl_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)]"))
        .expect("impl allow line")
        + 1;

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([
            top_level_rel.to_owned(),
            nested_rel.to_owned(),
            grouped_rel.to_owned(),
            module_rel.to_owned(),
            trait_rel.to_owned(),
            impl_rel.to_owned(),
        ]));
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::panic)]` requires `// reason:` on the same line.",
                file: Some(impl_rel),
                line: Some(impl_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::unwrap_used)]` requires `// reason:` on the same line.",
                file: Some(top_level_rel),
                line: Some(top_level_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::panic)]` requires `// reason:` on the same line.",
                file: Some(module_rel),
                line: Some(module_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::expect_used)]` requires `// reason:` on the same line.",
                file: Some(trait_rel),
                line: Some(trait_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::expect_used)]` requires `// reason:` on the same line.",
                file: Some(grouped_rel),
                line: Some(grouped_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::unwrap_used)]` requires `// reason:` on the same line.",
                file: Some(grouped_rel),
                line: Some(grouped_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "item-level allow without reason",
                message: "`#[allow(clippy::panic)]` requires `// reason:` on the same line.",
                file: Some(nested_rel),
                line: Some(nested_line),
                inventory: false,
            },
        ]
    );
}
