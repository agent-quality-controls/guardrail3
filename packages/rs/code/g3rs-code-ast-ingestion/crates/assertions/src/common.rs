use g3rs_code_ast_checks_types::G3RsCodeAstChecksInput;

/// Find one ingested source file by repo-relative path.
pub fn require_source_file<'a>(
    inputs: &'a [G3RsCodeAstChecksInput],
    rel_path: &str,
) -> &'a G3RsCodeAstChecksInput {
    inputs
        .iter()
        .find(|input| input.source_file.rel_path == rel_path)
        .unwrap_or_else(|| panic!("missing ingested source file {rel_path}; inputs: {inputs:#?}"))
}

/// Assert one ingested source file matches the expected metadata and content.
pub fn assert_source_file(
    input: &G3RsCodeAstChecksInput,
    rel_path: &str,
    is_test: bool,
    profile_name: Option<&str>,
    is_library_root: bool,
    content: &str,
) {
    assert_eq!(input.source_file.rel_path, rel_path, "unexpected rel_path");
    assert_eq!(input.source_file.is_test, is_test, "unexpected is_test");
    assert_eq!(
        input.source_file.profile_name.as_deref(),
        profile_name,
        "unexpected profile_name"
    );
    assert_eq!(
        input.source_file.is_library_root, is_library_root,
        "unexpected is_library_root"
    );
    assert_eq!(input.source_file.content, content, "unexpected content");
}
