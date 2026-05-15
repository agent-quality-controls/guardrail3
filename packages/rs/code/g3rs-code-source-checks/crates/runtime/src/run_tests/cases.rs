use g3rs_code_source_checks_assertions::run as assertions;
use g3rs_code_types::{G3RsCodeSourceChecksInput, G3RsSourceFile};

#[test]
fn run_parses_source_once_before_dispatch() {
    let input = G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path: "src/lib.rs".to_owned(),
            content: "#![allow(dead_code)]\npub fn run() {}\n".to_owned(),
            is_test: false,
            profile_name: None,
            is_library_root: true,
        },
        is_shared_crate: false,
        waivers: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "g3rs-code/crate-level-allow");
    assertions::assert_missing_finding_id(&results, "g3rs-code/input-failures");
}

#[test]
fn run_reports_parse_failure_from_source_content() {
    let input = G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path: "src/lib.rs".to_owned(),
            content: "not valid rust".to_owned(),
            is_test: false,
            profile_name: None,
            is_library_root: true,
        },
        is_shared_crate: false,
        waivers: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "g3rs-code/input-failures");
}
