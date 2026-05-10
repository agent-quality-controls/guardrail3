//! Shared support utilities for the tsconfig config-check runtime: strict
//! flag descriptors, finding builders, and snapshot accessors.

use g3ts_tsconfig_types::{
    G3TsTsconfigBoolState, G3TsTsconfigChecksInput, G3TsTsconfigExtendsState,
    G3TsTsconfigInlineStrictFlags, G3TsTsconfigState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Identifier for each tsconfig strict-mode compiler option inspected by
/// the strict-baseline rule.
#[derive(Clone, Copy)]
pub(crate) enum StrictFlag {
    /// `strict` compiler option.
    Strict,
    /// `noImplicitReturns` compiler option.
    NoImplicitReturns,
    /// `noUnusedLocals` compiler option.
    NoUnusedLocals,
    /// `noUnusedParameters` compiler option.
    NoUnusedParameters,
    /// `noUncheckedIndexedAccess` compiler option.
    NoUncheckedIndexedAccess,
    /// `exactOptionalPropertyTypes` compiler option.
    ExactOptionalPropertyTypes,
    /// `noPropertyAccessFromIndexSignature` compiler option.
    NoPropertyAccessFromIndexSignature,
    /// `noImplicitOverride` compiler option.
    NoImplicitOverride,
    /// `noFallthroughCasesInSwitch` compiler option.
    NoFallthroughCasesInSwitch,
    /// `forceConsistentCasingInFileNames` compiler option.
    ForceConsistentCasingInFileNames,
    /// `allowUnreachableCode` compiler option.
    AllowUnreachableCode,
    /// `allowUnusedLabels` compiler option.
    AllowUnusedLabels,
}

/// Pair of a strict flag and the expected boolean value the strict baseline
/// requires.
pub(crate) struct StrictFlagSpec {
    /// Which strict flag this spec applies to.
    pub flag: StrictFlag,
    /// Expected boolean value for the strict baseline.
    pub expected: bool,
}

/// Strict-baseline expectations for every tsconfig compiler option the
/// rule inspects.
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

/// Return the workspace-relative path of the root tsconfig, if any state
/// other than `Missing` is recorded.
pub(crate) fn root_rel_path(input: &G3TsTsconfigChecksInput) -> Option<&str> {
    match &input.config {
        G3TsTsconfigState::Missing => None,
        G3TsTsconfigState::Unreadable { rel_path, .. }
        | G3TsTsconfigState::ParseError { rel_path, .. }
        | G3TsTsconfigState::Parsed { rel_path, .. } => Some(rel_path),
    }
}

/// Build an inventory `Info` finding carrying `id`, `title`, `message`,
/// and the workspace-relative `file` path.
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

/// Enumerate human-readable descriptions of unresolved local `extends`
/// chain entries (missing, unreadable, or invalid).
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

/// Whether the tsconfig `extends` chain contains an external (out-of-tree)
/// base config that the strict-baseline rule cannot inspect.
pub(crate) fn has_external_extends(input: &G3TsTsconfigChecksInput) -> bool {
    let G3TsTsconfigState::Parsed { extends_chain, .. } = &input.config else {
        return false;
    };
    extends_chain
        .iter()
        .any(|entry| matches!(entry, G3TsTsconfigExtendsState::External { .. }))
}

/// Whether every local `extends` entry resolves successfully.
pub(crate) fn all_local_extends_resolved(input: &G3TsTsconfigChecksInput) -> bool {
    extends_chain_issues(input).is_empty()
}

/// List the strict flags whose inline value is missing or non-boolean.
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

/// List the strict flags whose effective value differs from the
/// strict-baseline expectation.
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

/// Whether the effective-compiler-options check can be meaningfully run
/// (parsed state with all local extends resolved and no external base).
pub(crate) fn effective_check_actionable(input: &G3TsTsconfigChecksInput) -> bool {
    let G3TsTsconfigState::Parsed { .. } = &input.config else {
        return false;
    };
    all_local_extends_resolved(input) && !has_external_extends(input)
}

impl StrictFlag {
    /// Return the JSON field name of this strict flag as it appears in
    /// `tsconfig.json` compiler options.
    const fn field_name(self) -> &'static str {
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

    /// Return the effective boolean value of this strict flag from the
    /// resolved compiler-options struct, if present.
    const fn effective_value(
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

    /// Return the inline value of this strict flag as recorded in the
    /// tsconfig snapshot.
    const fn inline_value(self, flags: &G3TsTsconfigInlineStrictFlags) -> G3TsTsconfigBoolState {
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
