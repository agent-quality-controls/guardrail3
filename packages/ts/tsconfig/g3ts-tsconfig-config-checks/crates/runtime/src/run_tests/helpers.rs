use g3ts_tsconfig_types::{
    G3TsTsconfigBoolState, G3TsTsconfigChecksInput, G3TsTsconfigExtendsState,
    G3TsTsconfigInlineStrictFlags, G3TsTsconfigState,
};
use tsconfig_json_parser::types::TsconfigCompilerOptions;

pub(super) fn missing() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Missing,
    }
}

pub(super) fn parse_error() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::ParseError {
            rel_path: "tsconfig.json".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
    }
}

pub(super) fn golden_extends() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            uses_extends: true,
            extends_chain: vec![G3TsTsconfigExtendsState::Resolved {
                specifier: "../../tsconfig.base.json".to_owned(),
                display_path: "/tmp/tsconfig.base.json".to_owned(),
            }],
            inline_strict_flags: missing_inline_flags(),
            effective_compiler_options: strict_baseline(),
        },
    }
}

pub(super) fn broken_chain() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            uses_extends: true,
            extends_chain: vec![G3TsTsconfigExtendsState::Missing {
                specifier: "../../tsconfig.base.json".to_owned(),
                display_path: "/tmp/tsconfig.base.json".to_owned(),
            }],
            inline_strict_flags: missing_inline_flags(),
            effective_compiler_options: TsconfigCompilerOptions::default(),
        },
    }
}

pub(super) fn standalone_missing_inline() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            uses_extends: false,
            extends_chain: Vec::new(),
            inline_strict_flags: G3TsTsconfigInlineStrictFlags {
                strict: G3TsTsconfigBoolState::Value(true),
                ..missing_inline_flags()
            },
            effective_compiler_options: TsconfigCompilerOptions {
                strict: Some(true),
                ..TsconfigCompilerOptions::default()
            },
        },
    }
}

pub(super) fn weak_effective_flags() -> G3TsTsconfigChecksInput {
    let mut effective = strict_baseline();
    effective.no_unused_locals = Some(false);

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            uses_extends: true,
            extends_chain: vec![G3TsTsconfigExtendsState::Resolved {
                specifier: "../../tsconfig.base.json".to_owned(),
                display_path: "/tmp/tsconfig.base.json".to_owned(),
            }],
            inline_strict_flags: missing_inline_flags(),
            effective_compiler_options: effective,
        },
    }
}

pub(super) fn external_extends() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            uses_extends: true,
            extends_chain: vec![G3TsTsconfigExtendsState::External {
                specifier: "@tsconfig/strictest/tsconfig.json".to_owned(),
            }],
            inline_strict_flags: missing_inline_flags(),
            effective_compiler_options: TsconfigCompilerOptions::default(),
        },
    }
}

fn missing_inline_flags() -> G3TsTsconfigInlineStrictFlags {
    G3TsTsconfigInlineStrictFlags {
        strict: G3TsTsconfigBoolState::Missing,
        no_implicit_returns: G3TsTsconfigBoolState::Missing,
        no_unused_locals: G3TsTsconfigBoolState::Missing,
        no_unused_parameters: G3TsTsconfigBoolState::Missing,
        no_unchecked_indexed_access: G3TsTsconfigBoolState::Missing,
        exact_optional_property_types: G3TsTsconfigBoolState::Missing,
        no_property_access_from_index_signature: G3TsTsconfigBoolState::Missing,
        no_implicit_override: G3TsTsconfigBoolState::Missing,
        no_fallthrough_cases_in_switch: G3TsTsconfigBoolState::Missing,
        force_consistent_casing_in_file_names: G3TsTsconfigBoolState::Missing,
        allow_unreachable_code: G3TsTsconfigBoolState::Missing,
        allow_unused_labels: G3TsTsconfigBoolState::Missing,
    }
}

fn strict_baseline() -> TsconfigCompilerOptions {
    TsconfigCompilerOptions {
        strict: Some(true),
        no_implicit_returns: Some(true),
        no_unused_locals: Some(true),
        no_unused_parameters: Some(true),
        no_unchecked_indexed_access: Some(true),
        exact_optional_property_types: Some(true),
        isolated_modules: None,
        no_property_access_from_index_signature: Some(true),
        no_implicit_override: Some(true),
        no_fallthrough_cases_in_switch: Some(true),
        force_consistent_casing_in_file_names: Some(true),
        allow_unreachable_code: Some(false),
        allow_unused_labels: Some(false),
    }
}
