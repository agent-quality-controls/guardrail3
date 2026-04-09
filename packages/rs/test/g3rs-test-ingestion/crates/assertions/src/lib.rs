use g3rs_test_types::G3RsTestConfigChecksInput;

pub fn require_root<'a>(
    inputs: &'a [G3RsTestConfigChecksInput],
    root_rel_dir: &str,
) -> &'a G3RsTestConfigChecksInput {
    inputs
        .iter()
        .find(|input| input.root_rel_dir == root_rel_dir)
        .unwrap_or_else(|| panic!("missing test root input {root_rel_dir}; inputs: {inputs:#?}"))
}
