use crate::types::{G3TsTsconfigBoolState, G3TsTsconfigInlineStrictFlags};
use tsconfig_json_parser::bool_field_state;
use tsconfig_json_parser::types::{TsconfigBoolFieldState, TsconfigDocument};

#[must_use]
pub fn inline_strict_flags(document: &TsconfigDocument) -> G3TsTsconfigInlineStrictFlags {
    G3TsTsconfigInlineStrictFlags {
        strict: map_bool_state(bool_field_state(document, "strict")),
        no_implicit_returns: map_bool_state(bool_field_state(document, "noImplicitReturns")),
        no_unused_locals: map_bool_state(bool_field_state(document, "noUnusedLocals")),
        no_unused_parameters: map_bool_state(bool_field_state(document, "noUnusedParameters")),
        no_unchecked_indexed_access: map_bool_state(bool_field_state(
            document,
            "noUncheckedIndexedAccess",
        )),
        exact_optional_property_types: map_bool_state(bool_field_state(
            document,
            "exactOptionalPropertyTypes",
        )),
        no_property_access_from_index_signature: map_bool_state(bool_field_state(
            document,
            "noPropertyAccessFromIndexSignature",
        )),
        no_implicit_override: map_bool_state(bool_field_state(document, "noImplicitOverride")),
        no_fallthrough_cases_in_switch: map_bool_state(bool_field_state(
            document,
            "noFallthroughCasesInSwitch",
        )),
        force_consistent_casing_in_file_names: map_bool_state(bool_field_state(
            document,
            "forceConsistentCasingInFileNames",
        )),
        allow_unreachable_code: map_bool_state(bool_field_state(document, "allowUnreachableCode")),
        allow_unused_labels: map_bool_state(bool_field_state(document, "allowUnusedLabels")),
    }
}

/// Map an internal tsconfig bool field state to the public contract enum.
const fn map_bool_state(state: TsconfigBoolFieldState<'_>) -> G3TsTsconfigBoolState {
    match state {
        TsconfigBoolFieldState::Missing => G3TsTsconfigBoolState::Missing,
        TsconfigBoolFieldState::Value(value) => G3TsTsconfigBoolState::Value(value),
        TsconfigBoolFieldState::WrongType(_) => G3TsTsconfigBoolState::WrongType,
    }
}
