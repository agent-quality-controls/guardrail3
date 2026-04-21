use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootState};

pub fn assert_root_missing(input: &G3TsJscpdChecksInput) {
    match &input.root {
        G3TsJscpdRootState::Missing => {}
        other => {
            assert!(
                false,
                "expected missing root .jscpd.json state, got: {other:?}"
            );
        }
    }
}

pub fn assert_root_parse_error(input: &G3TsJscpdChecksInput, expected_rel_path: &str) {
    match &input.root {
        G3TsJscpdRootState::ParseError { rel_path, .. } => {
            assert_eq!(
                rel_path, expected_rel_path,
                "root parse error path mismatch"
            );
        }
        other => {
            assert!(false, "expected root parse error state, got: {other:?}");
        }
    }
}

pub fn assert_root_parsed(input: &G3TsJscpdChecksInput, expected_rel_path: &str) {
    match &input.root {
        G3TsJscpdRootState::Parsed { snapshot } => {
            assert_eq!(
                snapshot.rel_path, expected_rel_path,
                "parsed root path mismatch"
            );
        }
        other => {
            assert!(
                false,
                "expected parsed root .jscpd.json state, got: {other:?}"
            );
        }
    }
}
