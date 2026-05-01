use g3ts_style_types::G3TsStyleContractInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn check_syncpack_style_policy_pin(
    contract: &G3TsStyleContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = parsed_syncpack(&contract.syncpack_config) else {
        results.push(error(
            "g3ts-style/syncpack-style-policy-pin",
            "Style Syncpack policy is missing",
            format!(
                "`{}` must pin `g3ts-eslint-plugin-style-policy` through Syncpack. G3TS does not parse dependency ranges from `package.json`; Syncpack owns the style package floor.",
                rel_path.unwrap_or(".syncpackrc")
            ),
            rel_path,
        ));
        return;
    };

    if snapshot.source_covers_package_manifest && snapshot.missing_required_pins.is_empty() {
        results.push(info(
            "g3ts-style/syncpack-style-policy-pin",
            "Style package floor is pinned by Syncpack",
            format!(
                "`{}` covers `package.json` and pins `g3ts-eslint-plugin-style-policy` to the required floor.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    } else {
        let missing = snapshot
            .missing_required_pins
            .iter()
            .map(|pin| format!("`{} = {}`", pin.dependency, pin.version))
            .collect::<Vec<_>>();
        results.push(error(
            "g3ts-style/syncpack-style-policy-pin",
            "Style package floor is not pinned by Syncpack",
            format!(
                "`{}` must have `source: [\"package.json\"]` and a canonical Syncpack pin group for {}. G3TS does not parse dependency ranges from `package.json`; Syncpack owns the style package floor.",
                snapshot.rel_path,
                if missing.is_empty() {
                    "`g3ts-eslint-plugin-style-policy`".to_owned()
                } else {
                    missing.join(", ")
                }
            ),
            Some(&snapshot.rel_path),
        ));
    }
}

fn parsed_syncpack(
    config: &g3ts_style_types::G3TsStyleSyncpackSurfaceState,
) -> Option<&g3ts_style_types::G3TsStyleSyncpackSnapshot> {
    match config {
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Missing { .. }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::Unreadable { .. }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::ParseError { .. } => None,
    }
}

fn syncpack_rel_path(config: &g3ts_style_types::G3TsStyleSyncpackSurfaceState) -> Option<&str> {
    match config {
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Missing { rel_path }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::Unreadable { rel_path, .. }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::ParseError { rel_path, .. } => {
            Some(rel_path)
        }
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Parsed { snapshot } => {
            Some(&snapshot.rel_path)
        }
    }
}

fn info(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
    .into_inventory()
}

fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}
