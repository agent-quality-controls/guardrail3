#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use g3rs_code_types as code_types;

pub fn assert_root_cargo_paths(input: &code_types::G3RsCodeFileTreeChecksInput, expected: &[&str]) {
    let actual = input
        .roots
        .iter()
        .map(|root| root.cargo_rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{input:#?}");
}

pub fn assert_single_zero_structural_root(
    input: &code_types::G3RsCodeFileTreeChecksInput,
    cargo_rel_path: &str,
) {
    assert_eq!(input.roots.len(), 1, "{input:#?}");
    assert_eq!(input.roots[0].cargo_rel_path, cargo_rel_path);
    assert_eq!(input.roots[0].max_module_depth, 0);
    assert_eq!(input.roots[0].max_sibling_dirs, 0);
    assert_eq!(input.roots[0].max_sibling_rs_files, 0);
}
