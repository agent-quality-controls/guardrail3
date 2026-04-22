use g3rs_code_source_checks_assertions::run as assertions;
use g3rs_code_types::{G3RsCodeParsedSourceState, G3RsCodeSourceChecksInput, G3RsSourceFile};

#[test]
fn run_dispatches_prebound_parsed_source() {
    let input = G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path: "src/lib.rs".to_owned(),
            content: "not valid rust".to_owned(),
            is_test: false,
            profile_name: None,
            is_library_root: true,
        },
        parsed_source: G3RsCodeParsedSourceState::Parsed(
            syn::parse_file("#![allow(dead_code)]\npub fn run() {}\n")
                .expect("fixture source should parse"),
        ),
        is_shared_crate: false,
        waivers: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "RS-CODE-SOURCE-01");
    assertions::assert_missing_finding_id(&results, "RS-CODE-SOURCE-30");
}

#[test]
fn run_dispatches_prebound_parse_failure() {
    let input = G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path: "src/lib.rs".to_owned(),
            content: "not valid rust".to_owned(),
            is_test: false,
            profile_name: None,
            is_library_root: true,
        },
        parsed_source: G3RsCodeParsedSourceState::Invalid {
            message: "Failed to parse Rust source file: expected item".to_owned(),
        },
        is_shared_crate: false,
        waivers: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "RS-CODE-SOURCE-30");
}
