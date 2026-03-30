use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_11_use_count_warn::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content)
}

fn count_top_level_use_imports(ast: &syn::File) -> usize {
    ast.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Use(item_use) => Some(count_use_tree_imports(&item_use.tree)),
            _ => None,
        })
        .sum()
}

fn count_use_tree_imports(tree: &syn::UseTree) -> usize {
    match tree {
        syn::UseTree::Path(path) => count_use_tree_imports(&path.tree),
        syn::UseTree::Group(group) => group.items.iter().map(count_use_tree_imports).sum(),
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) | syn::UseTree::Glob(_) => 1,
    }
}

#[test]
fn warns_at_threshold_band_in_real_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let imports = (0..16)
        .map(|index| format!("use crate::warn_{index};"))
        .collect::<Vec<_>>()
        .join("\n");
    write_file(root, rel, &format!("{imports}\n{content}"));
    let updated = test_support::read_file(root, rel);
    let ast = parse_rust_file(&updated).unwrap_or_else(|error| panic!("valid rust: {error}"));
    let total_use_count = count_top_level_use_imports(&ast);

    let results = run_family(root);
    let expected_message = format!("{total_use_count} top-level use imports (warn at 16, max 20).");

    assert_files(&results, BTreeSet::from([rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding::new(
            Severity::Warn,
            "many use imports",
            &expected_message,
            Some(rel),
            None,
            false,
        )],
    );
}
