use g3ts_npmrc_types::{G3TsNpmrcChecksInput, G3TsNpmrcRootState};

pub fn assert_root_missing(input: &G3TsNpmrcChecksInput) {
    match &input.root {
        G3TsNpmrcRootState::Missing => {}
        other => assert!(false, "expected missing root .npmrc state, got: {other:?}"),
    }
}

pub fn assert_root_not_package_manager_root(input: &G3TsNpmrcChecksInput) {
    match &input.root {
        G3TsNpmrcRootState::NotPackageManagerRoot => {}
        other => assert!(
            false,
            "expected non-package-manager root state, got: {other:?}"
        ),
    }
}

pub fn assert_root_parse_error(input: &G3TsNpmrcChecksInput, expected_rel_path: &str) {
    match &input.root {
        G3TsNpmrcRootState::ParseError { rel_path, .. } => {
            assert_eq!(
                rel_path, expected_rel_path,
                "root parse error path mismatch"
            );
        }
        other => assert!(false, "expected root parse error state, got: {other:?}"),
    }
}

pub fn assert_root_parsed(input: &G3TsNpmrcChecksInput, expected_rel_path: &str) {
    match &input.root {
        G3TsNpmrcRootState::Parsed { snapshot } => {
            assert_eq!(
                snapshot.rel_path, expected_rel_path,
                "parsed root path mismatch"
            );
        }
        other => assert!(false, "expected parsed root .npmrc state, got: {other:?}"),
    }
}
