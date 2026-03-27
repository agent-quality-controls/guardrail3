use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_04_item_level_allow_with_reason::{assert_findings, assert_value_eq, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn inventories_documented_item_level_allows_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let top_level_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let nested_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let grouped_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let module_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let impl_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let trait_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";

    let top_level_content =
        std::fs::read_to_string(root.join(top_level_rel)).expect("read top-level file");
    let nested_content = std::fs::read_to_string(root.join(nested_rel)).expect("read nested file");
    let grouped_content =
        std::fs::read_to_string(root.join(grouped_rel)).expect("read grouped file");
    let module_content = std::fs::read_to_string(root.join(module_rel)).expect("read module file");
    let impl_content = std::fs::read_to_string(root.join(impl_rel)).expect("read impl file");
    let trait_content = std::fs::read_to_string(root.join(trait_rel)).expect("read trait file");

    let top_level_new = format!(
        "{top_level_content}\n#[allow(clippy::unwrap_used)] // reason: compatibility shim\npub fn documented_query_probe() {{}}\n"
    );
    let nested_new = format!(
        "{nested_content}\nmod nested_documented {{\n    #[allow(clippy::panic)] // reason: queue adapter probe\n    pub fn helper() {{}}\n}}\n"
    );
    let grouped_new = format!(
        "{grouped_content}\n#[allow(clippy::unwrap_used, clippy::expect_used)] // reason: grouped adapter allowance\npub fn grouped_documented_probe() {{}}\n"
    );
    let module_new = format!(
        "{module_content}\n#[allow(clippy::expect_used)] // reason: documented module seam\npub mod documented_module_probe {{\n    pub fn helper() {{}}\n}}\n"
    );
    let impl_new = format!(
        "{impl_content}\nstruct DocumentedImplBoundary;\nimpl DocumentedImplBoundary {{\n    #[allow(clippy::panic)] // reason: adapter glue seam\n    fn documented_impl_probe(&self) {{}}\n}}\n"
    );
    let trait_new = format!(
        "{trait_content}\npub trait DocumentedTraitBoundary {{\n    #[allow(clippy::unwrap_used)] // reason: trait shim contract\n    fn documented_trait_probe(&self);\n}}\n"
    );

    write_file(root, top_level_rel, &top_level_new);
    write_file(root, nested_rel, &nested_new);
    write_file(root, grouped_rel, &grouped_new);
    write_file(root, module_rel, &module_new);
    write_file(root, impl_rel, &impl_new);
    write_file(root, trait_rel, &trait_new);

    let top_level_line = top_level_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::unwrap_used)] // reason: compatibility shim")
        })
        .expect("top level line")
        + 1;
    let nested_line = nested_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)] // reason: queue adapter probe"))
        .expect("nested line")
        + 1;
    let grouped_line = grouped_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::unwrap_used, clippy::expect_used)] // reason: grouped adapter allowance"))
        .expect("grouped line")
        + 1;
    let module_line = module_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::expect_used)] // reason: documented module seam")
        })
        .expect("module line")
        + 1;
    let impl_line = impl_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)] // reason: adapter glue seam"))
        .expect("impl line")
        + 1;
    let trait_line = trait_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::unwrap_used)] // reason: trait shim contract")
        })
        .expect("trait line")
        + 1;

    assert_findings(
        &run_family(root),
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::panic)] reason: adapter glue seam",
                file: Some(impl_rel),
                line: Some(impl_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::unwrap_used)] reason: compatibility shim",
                file: Some(top_level_rel),
                line: Some(top_level_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::expect_used)] reason: documented module seam",
                file: Some(module_rel),
                line: Some(module_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::unwrap_used)] reason: trait shim contract",
                file: Some(trait_rel),
                line: Some(trait_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::expect_used)] reason: grouped adapter allowance",
                file: Some(grouped_rel),
                line: Some(grouped_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::unwrap_used)] reason: grouped adapter allowance",
                file: Some(grouped_rel),
                line: Some(grouped_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::panic)] reason: queue adapter probe",
                file: Some(nested_rel),
                line: Some(nested_line),
                inventory: false,
            },
        ]
    );
}
