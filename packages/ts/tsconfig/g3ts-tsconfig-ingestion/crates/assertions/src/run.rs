use g3ts_tsconfig_types::G3TsTsconfigState;

/// Discriminator describing which `G3TsTsconfigState` variant the input
/// holds, used to produce assertion failure messages without panicking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateKind {
    /// The `Missing` variant.
    Missing,
    /// The `Unreadable` variant.
    Unreadable,
    /// The `ParseError` variant.
    ParseError,
    /// The `Parsed` variant.
    Parsed,
}

impl StateKind {
    /// Inspect a `G3TsTsconfigState` value and return its variant kind.
    const fn of(state: &G3TsTsconfigState) -> Self {
        match state {
            G3TsTsconfigState::Missing => Self::Missing,
            G3TsTsconfigState::Unreadable { .. } => Self::Unreadable,
            G3TsTsconfigState::ParseError { .. } => Self::ParseError,
            G3TsTsconfigState::Parsed { .. } => Self::Parsed,
        }
    }
}

/// Compare the actual `StateKind` of `input.config` against `expected_kind`.
fn assert_state_kind(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_kind: StateKind,
    label: &str,
) {
    let actual_kind = StateKind::of(&input.config);
    assert_eq!(
        actual_kind, expected_kind,
        "expected {label} tsconfig state, got: {:?}",
        input.config
    );
}

/// Assert that the ingested tsconfig state is `Missing`.
///
/// # Panics
///
/// Panics if `input.config` is any variant other than
/// [`G3TsTsconfigState::Missing`].
pub fn assert_missing(input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput) {
    assert_state_kind(input, StateKind::Missing, "missing");
}

/// Assert that the ingested tsconfig state is `ParseError` at
/// `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.config` is not a `ParseError` variant or the recorded
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_parse_error(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_rel_path: &str,
) {
    assert_state_kind(input, StateKind::ParseError, "parse-error");
    if let G3TsTsconfigState::ParseError { rel_path, .. } = &input.config {
        assert_eq!(rel_path, expected_rel_path, "parse error path mismatch");
    }
}

/// Assert that the ingested tsconfig state is `Parsed` at `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.config` is not a `Parsed` variant or the recorded
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_parsed_root_rel_path(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_rel_path: &str,
) {
    assert_state_kind(input, StateKind::Parsed, "parsed");
    if let G3TsTsconfigState::Parsed { rel_path, .. } = &input.config {
        assert_eq!(rel_path, expected_rel_path, "parsed root path mismatch");
    }
}

/// Assert that, for a `Parsed` tsconfig, each `(field, expected)` pair in
/// `expected_pairs` matches the effective compiler-options value of that
/// field.
///
/// # Panics
///
/// Panics if `input.config` is not `Parsed`, if `field` names an
/// unsupported compiler option, or if any field value differs from its
/// expected value.
#[expect(
    clippy::type_complexity,
    reason = "Test input shape: each entry is (compiler-option name, expected boolean value); \
              a named alias would not add semantics over the tuple at the call site"
)]
#[expect(
    clippy::panic,
    reason = "Unsupported compiler-option names indicate a test fixture bug; panic here \
              surfaces the bug at the assertion site rather than silently skipping"
)]
pub fn assert_effective_flags(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_pairs: &[(&str, Option<bool>)],
) {
    assert_state_kind(input, StateKind::Parsed, "parsed");
    let G3TsTsconfigState::Parsed {
        effective_compiler_options,
        ..
    } = &input.config
    else {
        return;
    };

    for (field, expected) in expected_pairs {
        let actual = match *field {
            "strict" => effective_compiler_options.strict,
            "noImplicitReturns" => effective_compiler_options.no_implicit_returns,
            "noUnusedLocals" => effective_compiler_options.no_unused_locals,
            "noUnusedParameters" => effective_compiler_options.no_unused_parameters,
            "allowUnreachableCode" => effective_compiler_options.allow_unreachable_code,
            other => {
                panic!("unsupported effective flag assertion: {other}");
            }
        };
        assert_eq!(actual, *expected, "effective flag mismatch for {field}");
    }
}
