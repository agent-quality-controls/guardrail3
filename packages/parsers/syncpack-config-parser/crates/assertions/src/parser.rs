pub use syncpack_config_parser_runtime::types::SyncpackVersionGroup;

use syncpack_config_parser_runtime::types::{
    SyncpackConfigDocument, SyncpackConfigParseState,
};

pub fn assert_parsed_document(document: &SyncpackConfigDocument) {
    assert!(
        matches!(document.typed, SyncpackConfigParseState::Parsed(_)),
        "expected parsed Syncpack config document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(document: &SyncpackConfigDocument, expected_reason: &str) {
    match &document.typed {
        SyncpackConfigParseState::Invalid(reason) => assert!(
            reason.contains(expected_reason),
            "expected invalid reason to contain {expected_reason:?}, got {reason:?}"
        ),
        SyncpackConfigParseState::Parsed(_) => assert!(
            false,
            "expected invalid Syncpack config document, got: {document:#?}"
        ),
    }
}

pub fn assert_has_pinned_group(
    document: &SyncpackConfigDocument,
    dependency: &str,
    expected_version: &str,
) {
    let SyncpackConfigParseState::Parsed(snapshot) = &document.typed else {
        assert!(
            false,
            "expected parsed Syncpack config document, got: {document:#?}"
        );
        return;
    };
    assert!(
        snapshot.version_groups.iter().any(|group| {
            group.pin_version.as_deref() == Some(expected_version)
                && group.dependencies.iter().any(|item| item == dependency)
        }),
        "expected pinned group for {dependency}@{expected_version}, got: {snapshot:#?}"
    );
}

pub fn assert_source(document: &SyncpackConfigDocument, expected: &[&str]) {
    let SyncpackConfigParseState::Parsed(snapshot) = &document.typed else {
        assert!(
            false,
            "expected parsed Syncpack config document, got: {document:#?}"
        );
        return;
    };
    assert_eq!(
        snapshot.source,
        expected
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "source mismatch"
    );
}

pub fn assert_group_exact(
    document: &SyncpackConfigDocument,
    index: usize,
    expected: &SyncpackVersionGroup,
) {
    let SyncpackConfigParseState::Parsed(snapshot) = &document.typed else {
        assert!(
            false,
            "expected parsed Syncpack config document, got: {document:#?}"
        );
        return;
    };
    let actual = snapshot
        .version_groups
        .get(index)
        .expect("version group should exist");
    assert_eq!(actual, expected, "version group mismatch at index {index}");
}

pub fn assert_has_banned_group(document: &SyncpackConfigDocument, dependency: &str) {
    let SyncpackConfigParseState::Parsed(snapshot) = &document.typed else {
        assert!(
            false,
            "expected parsed Syncpack config document, got: {document:#?}"
        );
        return;
    };
    assert!(
        snapshot.version_groups.iter().any(|group| {
            group.is_banned == Some(true)
                && group.dependencies.iter().any(|item| item == dependency)
        }),
        "expected banned group for {dependency}, got: {snapshot:#?}"
    );
}
