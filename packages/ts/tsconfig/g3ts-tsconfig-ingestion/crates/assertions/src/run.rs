use g3ts_tsconfig_types::G3TsTsconfigState;

pub fn assert_missing(input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput) {
    match &input.config {
        G3TsTsconfigState::Missing => {}
        other => assert!(false, "expected missing tsconfig state, got: {other:?}"),
    }
}

pub fn assert_parse_error(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_rel_path: &str,
) {
    match &input.config {
        G3TsTsconfigState::ParseError { rel_path, .. } => {
            assert_eq!(rel_path, expected_rel_path, "parse error path mismatch");
        }
        other => assert!(false, "expected parse error state, got: {other:?}"),
    }
}

pub fn assert_parsed_root_rel_path(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_rel_path: &str,
) {
    match &input.config {
        G3TsTsconfigState::Parsed { rel_path, .. } => {
            assert_eq!(rel_path, expected_rel_path, "parsed root path mismatch");
        }
        other => assert!(false, "expected parsed tsconfig state, got: {other:?}"),
    }
}

pub fn assert_effective_flags(
    input: &g3ts_tsconfig_types::G3TsTsconfigChecksInput,
    expected_pairs: &[(&str, Option<bool>)],
) {
    let G3TsTsconfigState::Parsed {
        effective_compiler_options,
        ..
    } = &input.config
    else {
        assert!(
            false,
            "expected parsed tsconfig state, got: {:?}",
            input.config
        );
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
                assert!(false, "unsupported effective flag assertion: {other}");
                return;
            }
        };
        assert_eq!(actual, *expected, "effective flag mismatch for {field}");
    }
}
