use guardrail3_app_rs_family_code_assertions::rs_code_02_unused_crate_dependencies_allow::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_other_allow_names_inline_modules_and_item_level_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let other_allow_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let inline_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let item_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let module_decl_rel = "apps/devctl/crates/adapters/inbound/cli/src/lib.rs";

    let other_allow_content =
        std::fs::read_to_string(root.join(other_allow_rel)).expect("read other allow file");
    let inline_content = std::fs::read_to_string(root.join(inline_rel)).expect("read inline file");
    let item_content = std::fs::read_to_string(root.join(item_rel)).expect("read item file");
    let module_decl_content =
        std::fs::read_to_string(root.join(module_decl_rel)).expect("read module decl file");

    write_file(
        root,
        other_allow_rel,
        &format!("#![allow(clippy::unwrap_used)]\n{other_allow_content}\n"),
    );
    write_file(
        root,
        inline_rel,
        &format!(
            "{inline_content}\nmod nested_unused_deps {{\n    #![allow(unused_crate_dependencies)]\n    pub fn helper() {{}}\n}}\n"
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

    assert_no_hits(&results);
}
