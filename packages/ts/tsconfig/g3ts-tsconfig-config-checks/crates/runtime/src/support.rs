use g3ts_tsconfig_types::{
    G3TsTsconfigBoolState, G3TsTsconfigChecksInput, G3TsTsconfigExtendsState,
    G3TsTsconfigInlineStrictFlags, G3TsTsconfigState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Copy)]
pub(crate) enum StrictFlag {
    Strict,
    NoImplicitReturns,
    NoUnusedLocals,
    NoUnusedParameters,
    NoUncheckedIndexedAccess,
    ExactOptionalPropertyTypes,
    NoPropertyAccessFromIndexSignature,
    NoImplicitOverride,
    NoFallthroughCasesInSwitch,
    ForceConsistentCasingInFileNames,
    AllowUnreachableCode,
    AllowUnusedLabels,
}

pub(crate) struct StrictFlagSpec {
    pub flag: StrictFlag,
    pub expected: bool,
}

pub(crate) const STRICT_FLAGS: [StrictFlagSpec; 12] = [
    StrictFlagSpec {
        flag: StrictFlag::Strict,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoImplicitReturns,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoUnusedLocals,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoUnusedParameters,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoUncheckedIndexedAccess,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::ExactOptionalPropertyTypes,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoPropertyAccessFromIndexSignature,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoImplicitOverride,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::NoFallthroughCasesInSwitch,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::ForceConsistentCasingInFileNames,
        expected: true,
    },
    StrictFlagSpec {
        flag: StrictFlag::AllowUnreachableCode,
        expected: false,
    },
    StrictFlagSpec {
        flag: StrictFlag::AllowUnusedLabels,
        expected: false,
    },
];

pub(crate) fn root_rel_path(input: &G3TsTsconfigChecksInput) -> Option<&str> {
    match &input.config {
        G3TsTsconfigState::Missing => None,
        G3TsTsconfigState::Unreadable { rel_path, .. }
        | G3TsTsconfigState::ParseError { rel_path, .. }
        | G3TsTsconfigState::Parsed { rel_path, .. } => Some(rel_path),
    }
}

pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

pub(crate) fn extends_chain_issues(input: &G3TsTsconfigChecksInput) -> Vec<String> {
    let G3TsTsconfigState::Parsed { extends_chain, .. } = &input.config else {
        return Vec::new();
    };

    extends_chain
        .iter()
        .filter_map(|entry| match entry {
            G3TsTsconfigExtendsState::Missing {
                specifier,
                display_path,
            } => Some(format!(
                "`{specifier}` resolved to missing path `{display_path}`"
            )),
            G3TsTsconfigExtendsState::Unreadable {
                specifier,
                display_path,
                reason,
            } => Some(format!(
                "`{specifier}` resolved to unreadable path `{display_path}`: {reason}"
            )),
            G3TsTsconfigExtendsState::ParseError {
                specifier,
                display_path,
                reason,
            } => Some(format!(
                "`{specifier}` resolved to invalid config `{display_path}`: {reason}"
            )),
            G3TsTsconfigExtendsState::External { .. }
            | G3TsTsconfigExtendsState::Resolved { .. } => None,
        })
        .collect()
}

pub(crate) fn has_external_extends(input: &G3TsTsconfigChecksInput) -> bool {
    let G3TsTsconfigState::Parsed { extends_chain, .. } = &input.config else {
        return false;
    };
    extends_chain
        .iter()
        .any(|entry| matches!(entry, G3TsTsconfigExtendsState::External { .. }))
}

pub(crate) fn all_local_extends_resolved(input: &G3TsTsconfigChecksInput) -> bool {
    extends_chain_issues(input).is_empty()
}

pub(crate) fn missing_inline_flags(input: &G3TsTsconfigChecksInput) -> Vec<String> {
    let G3TsTsconfigState::Parsed {
        inline_strict_flags,
        ..
    } = &input.config
    else {
        return Vec::new();
    };
    STRICT_FLAGS
        .iter()
        .filter_map(|spec| match spec.flag.inline_value(inline_strict_flags) {
            G3TsTsconfigBoolState::Value(_) => None,
            G3TsTsconfigBoolState::Missing => Some(spec.flag.field_name().to_owned()),
            G3TsTsconfigBoolState::WrongType => {
                Some(format!("{} (non-boolean)", spec.flag.field_name()))
            }
        })
        .collect()
}

pub(crate) fn effective_flag_mismatches(input: &G3TsTsconfigChecksInput) -> Vec<String> {
    let G3TsTsconfigState::Parsed {
        effective_compiler_options,
        ..
    } = &input.config
    else {
        return Vec::new();
    };

    STRICT_FLAGS
        .iter()
        .filter_map(|spec| {
            let actual = spec.flag.effective_value(effective_compiler_options);
            match actual {
                Some(value) if value == spec.expected => None,
                Some(value) => Some(format!(
                    "{}={} (expected {})",
                    spec.flag.field_name(),
                    value,
                    spec.expected
                )),
                None => Some(format!("{} missing", spec.flag.field_name())),
            }
        })
        .collect()
}

pub(crate) fn effective_check_actionable(input: &G3TsTsconfigChecksInput) -> bool {
    let G3TsTsconfigState::Parsed { .. } = &input.config else {
        return false;
    };
    all_local_extends_resolved(input) && !has_external_extends(input)
}

impl StrictFlag {
    fn field_name(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::NoImplicitReturns => "noImplicitReturns",
            Self::NoUnusedLocals => "noUnusedLocals",
            Self::NoUnusedParameters => "noUnusedParameters",
            Self::NoUncheckedIndexedAccess => "noUncheckedIndexedAccess",
            Self::ExactOptionalPropertyTypes => "exactOptionalPropertyTypes",
            Self::NoPropertyAccessFromIndexSignature => "noPropertyAccessFromIndexSignature",
            Self::NoImplicitOverride => "noImplicitOverride",
            Self::NoFallthroughCasesInSwitch => "noFallthroughCasesInSwitch",
            Self::ForceConsistentCasingInFileNames => "forceConsistentCasingInFileNames",
            Self::AllowUnreachableCode => "allowUnreachableCode",
            Self::AllowUnusedLabels => "allowUnusedLabels",
        }
    }

    fn effective_value(
        self,
        options: &tsconfig_json_parser::types::TsconfigCompilerOptions,
    ) -> Option<bool> {
        match self {
            Self::Strict => options.strict,
            Self::NoImplicitReturns => options.no_implicit_returns,
            Self::NoUnusedLocals => options.no_unused_locals,
            Self::NoUnusedParameters => options.no_unused_parameters,
            Self::NoUncheckedIndexedAccess => options.no_unchecked_indexed_access,
            Self::ExactOptionalPropertyTypes => options.exact_optional_property_types,
            Self::NoPropertyAccessFromIndexSignature => {
                options.no_property_access_from_index_signature
            }
            Self::NoImplicitOverride => options.no_implicit_override,
            Self::NoFallthroughCasesInSwitch => options.no_fallthrough_cases_in_switch,
            Self::ForceConsistentCasingInFileNames => options.force_consistent_casing_in_file_names,
            Self::AllowUnreachableCode => options.allow_unreachable_code,
            Self::AllowUnusedLabels => options.allow_unused_labels,
        }
    }

    fn inline_value(self, flags: &G3TsTsconfigInlineStrictFlags) -> G3TsTsconfigBoolState {
        match self {
            Self::Strict => flags.strict,
            Self::NoImplicitReturns => flags.no_implicit_returns,
            Self::NoUnusedLocals => flags.no_unused_locals,
            Self::NoUnusedParameters => flags.no_unused_parameters,
            Self::NoUncheckedIndexedAccess => flags.no_unchecked_indexed_access,
            Self::ExactOptionalPropertyTypes => flags.exact_optional_property_types,
            Self::NoPropertyAccessFromIndexSignature => {
                flags.no_property_access_from_index_signature
            }
            Self::NoImplicitOverride => flags.no_implicit_override,
            Self::NoFallthroughCasesInSwitch => flags.no_fallthrough_cases_in_switch,
            Self::ForceConsistentCasingInFileNames => flags.force_consistent_casing_in_file_names,
            Self::AllowUnreachableCode => flags.allow_unreachable_code,
            Self::AllowUnusedLabels => flags.allow_unused_labels,
        }
    }
}
