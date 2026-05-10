#[cfg(test)]
use g3rs_code_types::G3RsSourceFile;
use g3rs_code_types::{G3RsCodeParsedSourceState, G3RsCodeSourceChecksInput, G3RsCodeWaiver};

#[cfg(test)]
pub(crate) struct G3RsCodeSourceFileAst {
    pub(crate) source_file: G3RsSourceFile,
    pub(crate) source: syn::File,
}

#[derive(Debug)]
/// Struct `CodeInputFailureRuleInput` used by this module.
pub(crate) struct CodeInputFailureRuleInput {
    /// Field `rel_path`.
    pub(crate) rel_path: String,
    /// Field `message`.
    pub(crate) message: String,
}

#[derive(Clone, Copy)]
/// Struct `CodeSourceRuleInput` used by this module.
pub(crate) struct CodeSourceRuleInput<'a> {
    /// Field `rel_path`.
    pub(crate) rel_path: &'a str,
    /// Field `content`.
    pub(crate) content: &'a str,
    /// Field `source`.
    pub(crate) source: &'a syn::File,
    /// Field `is_test`.
    pub(crate) is_test: bool,
    /// Field `is_shared_crate`.
    pub(crate) is_shared_crate: bool,
    /// Field `waivers`.
    pub(crate) waivers: &'a [G3RsCodeWaiver],
    #[allow(dead_code)] // reason: retained for upcoming profile-sensitive code source rules
    /// Field `profile_name`.
    pub(crate) profile_name: Option<&'a str>,
    #[allow(dead_code)] // reason: retained for upcoming lib.rs-only code source rules
    /// Field `is_library_root`.
    pub(crate) is_library_root: bool,
}

#[cfg(test)]
impl<'a> From<&'a G3RsCodeSourceFileAst> for CodeSourceRuleInput<'a> {
    fn from(value: &'a G3RsCodeSourceFileAst) -> Self {
        Self {
            rel_path: &value.source_file.rel_path,
            content: &value.source_file.content,
            source: &value.source,
            is_test: value.source_file.is_test,
            is_shared_crate: false,
            waivers: &[],
            profile_name: value.source_file.profile_name.as_deref(),
            is_library_root: value.source_file.is_library_root,
        }
    }
}

/// Implements `has matching waiver`.
pub(crate) fn has_matching_waiver(
    input: &CodeSourceRuleInput<'_>,
    rule: &str,
    selector: &str,
) -> bool {
    input.waivers.iter().any(|waiver| {
        waiver.rule == rule && waiver.file == input.rel_path && waiver.selector == selector
    })
}

/// Implements `rule input`.
pub(crate) fn rule_input(
    input: &G3RsCodeSourceChecksInput,
) -> Result<CodeSourceRuleInput<'_>, CodeInputFailureRuleInput> {
    match &input.parsed_source {
        G3RsCodeParsedSourceState::Parsed(source) => Ok(CodeSourceRuleInput {
            rel_path: &input.source_file.rel_path,
            content: &input.source_file.content,
            source,
            is_test: input.source_file.is_test,
            is_shared_crate: input.is_shared_crate,
            waivers: &input.waivers,
            profile_name: input.source_file.profile_name.as_deref(),
            is_library_root: input.source_file.is_library_root,
        }),
        G3RsCodeParsedSourceState::Invalid { message } => Err(CodeInputFailureRuleInput {
            rel_path: input.source_file.rel_path.clone(),
            message: message.clone(),
        }),
    }
}
