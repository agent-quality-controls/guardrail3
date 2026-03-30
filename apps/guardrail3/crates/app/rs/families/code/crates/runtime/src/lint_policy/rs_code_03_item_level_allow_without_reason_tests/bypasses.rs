use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_03_item_level_allow_without_reason::{
    RuleFinding, Severity, assert_files, assert_findings,
};
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

    let top_level_content = test_support::read_file(root, top_level_rel);
    let nested_content = test_support::read_file(root, nested_rel);
    let grouped_content = test_support::read_file(root, grouped_rel);
    let module_content = test_support::read_file(root, module_rel);
    let trait_content = test_support::read_file(root, trait_rel);
    let impl_content = test_support::read_file(root, impl_rel);

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
        .map(|index| index + 1)
        .unwrap_or_default();
    let nested_line = nested_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let grouped_line = grouped_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::unwrap_used, clippy::expect_used)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let module_line = module_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let trait_line = trait_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::expect_used)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let impl_line = impl_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file(),
                Some(path)
                    if [
                        top_level_rel,
                        nested_rel,
                        grouped_rel,
                        module_rel,
                        trait_rel,
                        impl_rel,
                    ]
                    .contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_files(
        &relevant_results,
        BTreeSet::from([
            top_level_rel.to_owned(),
            nested_rel.to_owned(),
            grouped_rel.to_owned(),
            module_rel.to_owned(),
            trait_rel.to_owned(),
            impl_rel.to_owned(),
        ]),
    );
    assert_findings(
        &relevant_results,
        &[
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::panic)]` requires `// reason:` on the same line.",
                Some(impl_rel),
                Some(impl_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::unwrap_used)]` requires `// reason:` on the same line.",
                Some(top_level_rel),
                Some(top_level_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::panic)]` requires `// reason:` on the same line.",
                Some(module_rel),
                Some(module_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::expect_used)]` requires `// reason:` on the same line.",
                Some(trait_rel),
                Some(trait_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::expect_used)]` requires `// reason:` on the same line.",
                Some(grouped_rel),
                Some(grouped_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::unwrap_used)]` requires `// reason:` on the same line.",
                Some(grouped_rel),
                Some(grouped_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level allow without reason",
                "`#[allow(clippy::panic)]` requires `// reason:` on the same line.",
                Some(nested_rel),
                Some(nested_line),
                false,
            ),
        ],
    );
}

#[test]
fn detects_undocumented_item_level_expects_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let top_level_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let grouped_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let module_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";

    let top_level_content = test_support::read_file(root, top_level_rel);
    let grouped_content = test_support::read_file(root, grouped_rel);
    let module_content = test_support::read_file(root, module_rel);

    let top_level_new = format!(
        "{top_level_content}\n#[expect(clippy::unwrap_used)]\npub fn undocumented_expect_probe() {{}}\n"
    );
    let grouped_new = format!(
        "{grouped_content}\n#[expect(clippy::unwrap_used, clippy::expect_used)]\npub fn grouped_expect_probe() {{}}\n"
    );
    let module_new = format!(
        "{module_content}\n#[expect(clippy::panic)]\npub mod undocumented_expect_module_probe {{\n    pub fn helper() {{}}\n}}\n"
    );

    write_file(root, top_level_rel, &top_level_new);
    write_file(root, grouped_rel, &grouped_new);
    write_file(root, module_rel, &module_new);

    let top_level_line = top_level_new
        .lines()
        .position(|line| line.contains("#[expect(clippy::unwrap_used)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let grouped_line = grouped_new
        .lines()
        .position(|line| line.contains("#[expect(clippy::unwrap_used, clippy::expect_used)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let module_line = module_new
        .lines()
        .position(|line| line.contains("#[expect(clippy::panic)]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file(),
                Some(path) if [top_level_rel, grouped_rel, module_rel].contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_files(
        &relevant_results,
        BTreeSet::from([
            top_level_rel.to_owned(),
            grouped_rel.to_owned(),
            module_rel.to_owned(),
        ]),
    );
    assert_findings(
        &relevant_results,
        &[
            RuleFinding::new(
                Severity::Error,
                "item-level expect without reason",
                "`#[expect(clippy::unwrap_used)]` requires `// reason:` on the same line.",
                Some(top_level_rel),
                Some(top_level_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level expect without reason",
                "`#[expect(clippy::expect_used)]` requires `// reason:` on the same line.",
                Some(grouped_rel),
                Some(grouped_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level expect without reason",
                "`#[expect(clippy::unwrap_used)]` requires `// reason:` on the same line.",
                Some(grouped_rel),
                Some(grouped_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "item-level expect without reason",
                "`#[expect(clippy::panic)]` requires `// reason:` on the same line.",
                Some(module_rel),
                Some(module_line),
                false,
            ),
        ],
    );
}
