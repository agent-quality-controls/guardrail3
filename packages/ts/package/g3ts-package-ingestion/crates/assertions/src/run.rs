use g3ts_package_types::{G3TsPackageChecksInput, G3TsPackageLocalState, G3TsPackageRootState};

pub fn assert_root_missing(input: &G3TsPackageChecksInput) {
    match &input.root {
        G3TsPackageRootState::Missing => {}
        other => assert!(false, "expected missing root package state, got: {other:?}"),
    }
}

pub fn assert_root_not_package_manager_root(input: &G3TsPackageChecksInput) {
    match &input.root {
        G3TsPackageRootState::NotPackageManagerRoot => {}
        other => assert!(
            false,
            "expected non-package-manager root state, got: {other:?}"
        ),
    }
}

pub fn assert_root_parse_error(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    match &input.root {
        G3TsPackageRootState::ParseError { rel_path, .. } => {
            assert_eq!(
                rel_path, expected_rel_path,
                "root parse error path mismatch"
            );
        }
        other => assert!(false, "expected root parse error state, got: {other:?}"),
    }
}

pub fn assert_root_parsed(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    match &input.root {
        G3TsPackageRootState::Parsed { snapshot } => {
            assert_eq!(
                snapshot.rel_path, expected_rel_path,
                "parsed root path mismatch"
            );
        }
        other => assert!(false, "expected parsed root package state, got: {other:?}"),
    }
}

pub fn assert_root_script_policy(
    input: &G3TsPackageChecksInput,
    expected_only_allow_pnpm: bool,
    expected_syncpack_lint: bool,
) {
    match &input.root {
        G3TsPackageRootState::Parsed { snapshot } => {
            assert_eq!(
                snapshot.safely_runs_only_allow_pnpm, expected_only_allow_pnpm,
                "root only-allow pnpm script policy mismatch"
            );
            assert_eq!(
                snapshot.safely_runs_syncpack_lint, expected_syncpack_lint,
                "root syncpack lint script policy mismatch"
            );
        }
        other => assert!(false, "expected parsed root package state, got: {other:?}"),
    }
}

pub fn assert_local_paths(input: &G3TsPackageChecksInput, expected: &[&str]) {
    let actual = input
        .locals
        .iter()
        .map(|state| match state {
            G3TsPackageLocalState::Unreadable { rel_path, .. }
            | G3TsPackageLocalState::ParseError { rel_path, .. } => rel_path.clone(),
            G3TsPackageLocalState::Parsed { snapshot } => snapshot.rel_path.clone(),
        })
        .collect::<Vec<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "local manifest path mismatch");
}

pub fn assert_local_parse_error(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    let Some(matching) = input.locals.iter().find(|state| match state {
        G3TsPackageLocalState::Unreadable { rel_path, .. }
        | G3TsPackageLocalState::ParseError { rel_path, .. } => rel_path == expected_rel_path,
        G3TsPackageLocalState::Parsed { snapshot } => snapshot.rel_path == expected_rel_path,
    }) else {
        assert!(
            false,
            "missing local manifest state for `{expected_rel_path}`"
        );
        return;
    };

    match matching {
        G3TsPackageLocalState::ParseError { .. } => {}
        other => assert!(false, "expected local parse error state, got: {other:?}"),
    }
}

pub fn assert_local_dependency_names(
    input: &G3TsPackageChecksInput,
    expected_rel_path: &str,
    expected_dependencies: &[&str],
) {
    let Some(matching) = input.locals.iter().find(|state| match state {
        G3TsPackageLocalState::Unreadable { rel_path, .. }
        | G3TsPackageLocalState::ParseError { rel_path, .. } => rel_path == expected_rel_path,
        G3TsPackageLocalState::Parsed { snapshot } => snapshot.rel_path == expected_rel_path,
    }) else {
        assert!(
            false,
            "missing local manifest state for `{expected_rel_path}`"
        );
        return;
    };

    let G3TsPackageLocalState::Parsed { snapshot } = matching else {
        assert!(
            false,
            "expected parsed local package state, got: {matching:?}"
        );
        return;
    };

    let expected = expected_dependencies
        .iter()
        .map(|dependency| (*dependency).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        snapshot.dependencies, expected,
        "local dependency list mismatch for `{expected_rel_path}`"
    );
}

pub fn assert_syncpack_not_required(input: &G3TsPackageChecksInput) {
    match &input.syncpack_config {
        g3ts_package_types::G3TsPackageSyncpackConfigState::NotRequired => {}
        other => assert!(
            false,
            "expected Syncpack not-required state, got: {other:?}"
        ),
    }
}

pub fn assert_syncpack_missing(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    match &input.syncpack_config {
        g3ts_package_types::G3TsPackageSyncpackConfigState::Missing { rel_path } => {
            assert_eq!(
                rel_path, expected_rel_path,
                "Syncpack missing path mismatch"
            );
        }
        other => assert!(false, "expected missing Syncpack state, got: {other:?}"),
    }
}

pub fn assert_syncpack_missing_source_entries(input: &G3TsPackageChecksInput, expected: &[&str]) {
    match &input.syncpack_config {
        g3ts_package_types::G3TsPackageSyncpackConfigState::Parsed { snapshot } => {
            let expected = expected
                .iter()
                .map(|entry| (*entry).to_owned())
                .collect::<Vec<_>>();
            assert_eq!(
                snapshot.missing_source_entries, expected,
                "Syncpack missing source entries mismatch"
            );
        }
        other => assert!(false, "expected parsed Syncpack state, got: {other:?}"),
    }
}

pub fn assert_syncpack_missing_forbidden_ban(
    input: &G3TsPackageChecksInput,
    expected_dependency: &str,
) {
    match &input.syncpack_config {
        g3ts_package_types::G3TsPackageSyncpackConfigState::Parsed { snapshot } => {
            assert!(
                snapshot
                    .missing_forbidden_bans
                    .iter()
                    .any(|dependency| dependency == expected_dependency),
                "expected missing Syncpack forbidden ban for `{expected_dependency}`, got: {:?}",
                snapshot.missing_forbidden_bans
            );
        }
        other => assert!(false, "expected parsed Syncpack state, got: {other:?}"),
    }
}
