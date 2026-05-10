#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use g3rs_code_types as code_types;

#[must_use]
pub fn require_source_file<'a>(
    inputs: &'a [code_types::G3RsCodeSourceChecksInput],
    rel_path: &str,
) -> &'a code_types::G3RsCodeSourceChecksInput {
    let index = inputs
        .iter()
        .position(|input| input.source_file.rel_path == rel_path);
    assert!(
        index.is_some(),
        "missing ingested source file {rel_path}; inputs: {inputs:#?}"
    );
    &inputs[index.unwrap_or(0)]
}

pub fn assert_source_file(
    input: &code_types::G3RsCodeSourceChecksInput,
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
    assert!(
        matches!(
            input.parsed_source,
            code_types::G3RsCodeParsedSourceState::Parsed(_)
        ),
        "unexpected parsed_source: {input:#?}"
    );
}

pub fn assert_shared_crate(input: &code_types::G3RsCodeSourceChecksInput) {
    assert!(input.is_shared_crate, "{input:#?}");
}

pub fn assert_not_shared_crate(input: &code_types::G3RsCodeSourceChecksInput) {
    assert!(!input.is_shared_crate, "{input:#?}");
}

pub fn assert_source_parse_failure(input: &code_types::G3RsCodeSourceChecksInput, rel_path: &str) {
    assert_eq!(input.source_file.rel_path, rel_path, "unexpected rel_path");
    assert!(
        matches!(
            input.parsed_source,
            code_types::G3RsCodeParsedSourceState::Invalid { .. }
        ),
        "unexpected parsed_source: {input:#?}"
    );
}

pub fn assert_source_waiver(
    input: &code_types::G3RsCodeSourceChecksInput,
    rule: &str,
    file: &str,
    selector: &str,
    reason: &str,
) {
    let waiver = input
        .waivers
        .iter()
        .find(|waiver| waiver.rule == rule && waiver.file == file && waiver.selector == selector);
    assert!(waiver.is_some(), "{input:#?}");
    let waiver =
        waiver.expect("assert_source_waiver should only unwrap an asserted-present waiver");
    assert_eq!(waiver.reason, reason, "{input:#?}");
}
