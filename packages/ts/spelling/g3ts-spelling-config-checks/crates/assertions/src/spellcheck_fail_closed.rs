pub fn assert_error(
    input: &g3ts_spelling_types::G3TsSpellingConfigChecksInput,
    file: Option<&str>,
) {
    crate::run::assert_runtime_error(input, "g3ts-spelling/spellcheck-fail-closed", file);
}
