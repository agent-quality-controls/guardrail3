use std::collections::BTreeSet;

use guardrail3_app_rs_family_code_assertions::rs_code_02_unused_crate_dependencies_allow::assert_files;
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_other_allow_names_inline_modules_and_item_level_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let other_allow_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let inline_other_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let item_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let module_decl_rel = "apps/devctl/crates/adapters/inbound/cli/src/lib.rs";
    let inline_exempt_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";

    let other_allow_content =
        test_support::read_file(root, other_allow_rel);
    let inline_other_content = test_support::read_file(root, inline_other_rel);
    let item_content = test_support::read_file(root, item_rel);
    let module_decl_content =
        test_support::read_file(root, module_decl_rel);
    let inline_exempt_content =
        test_support::read_file(root, inline_exempt_rel);

    write_file(
        root,
        other_allow_rel,
        &format!("#![allow(clippy::unwrap_used)]\n{other_allow_content}\n"),
    );
    write_file(
        root,
        inline_other_rel,
        &format!(
            "{inline_other_content}\nmod nested_non_exempt {{\n    #![allow(clippy::unwrap_used)]\n    pub fn helper() {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        item_rel,
        &format!(
            "{item_content}\n#[allow(unused_crate_dependencies)]\npub fn item_level_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        inline_exempt_rel,
        &format!(
            "{inline_exempt_content}\nmod nested_unused_deps {{\n    #![allow(unused_crate_dependencies)]\n    pub fn helper() {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        module_decl_rel,
        &format!(
            "{module_decl_content}\n#[allow(unused_crate_dependencies)]\nmod file_backed_unused_deps;\n"
        ),
    );
    write_file(
        root,
        "apps/devctl/crates/adapters/inbound/cli/src/file_backed_unused_deps.rs",
        "pub fn helper() {}\n",
    );

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file.as_deref(),
                Some(path)
                    if [
                        other_allow_rel,
                        inline_other_rel,
                        item_rel,
                        module_decl_rel,
                        inline_exempt_rel,
                    ]
                    .contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_files(
        &relevant_results,
        BTreeSet::from([inline_exempt_rel.to_owned()]),
    );
}
