use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    error, forbidden_syncpack_deps_message, info, parsed_root, root_has_dependency,
    root_invokes_tool,
};

/// `ID` constant.
const ID: &str = "g3ts-package/local-banned-dependencies";
/// `SYNCPACK_DEPENDENCY` constant.
const SYNCPACK_DEPENDENCY: &str = "syncpack";

/// `check`: check.
pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(root) = parsed_root(input) else {
        if matches!(
            input.syncpack_config,
            g3ts_package_types::G3TsPackageSyncpackConfigState::NotRequired
        ) && !input.locals.is_empty()
        {
            results.push(info(
                ID,
                "Syncpack dependency policy is not required",
                "No pnpm package-manager root was detected, so the package-family Syncpack dependency policy is not applied."
                    .to_owned(),
                "package.json",
            ));
        }
        return;
    };

    let has_dependency = root_has_dependency(root, SYNCPACK_DEPENDENCY);
    let has_safe_script = root.safely_runs_syncpack_lint;
    let invokes_script = root_invokes_tool(root, "syncpack", "lint");

    if !has_dependency || !has_safe_script {
        results.push(error(
            ID,
            "Syncpack package policy validator is not installed and wired",
            syncpack_setup_message(root, has_dependency, has_safe_script, invokes_script),
            &root.rel_path,
        ));
    }

    check_syncpack_config(input, results);

    if results
        .iter()
        .any(|result| result.id() == ID && !result.inventory())
    {
        return;
    }

    results.push(info(
        ID,
        "Syncpack bans forbidden package dependencies",
        format!(
            "`{}` lists `syncpack`, runs `syncpack lint` fail-closed, and `.syncpackrc` bans forbidden package deps through Syncpack: {}.",
            root.rel_path,
            forbidden_syncpack_deps_message(input)
        ),
        &root.rel_path,
    ));
}

/// `syncpack_setup_message`: syncpack setup message.
fn syncpack_setup_message(
    root: &g3ts_package_types::G3TsPackageRootSnapshot,
    has_dependency: bool,
    has_safe_script: bool,
    invokes_script: bool,
) -> String {
    match (has_dependency, has_safe_script, invokes_script) {
        (false, false, true) => format!(
            "`{}` invokes `syncpack lint`, but not in a supported fail-closed root script position, and does not list `syncpack` in dependencies or devDependencies. Add `syncpack` and remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden.",
            root.rel_path
        ),
        (false, false, false) => format!(
            "`{}` does not list `syncpack` in dependencies or devDependencies and does not run `syncpack lint` in any root script. Add `syncpack` and wire `syncpack lint` so dependency policy is enforced by Syncpack instead of G3TS scanning package manifests.",
            root.rel_path
        ),
        (false, true, _) => format!(
            "`{}` runs `syncpack lint` but does not list `syncpack` in dependencies or devDependencies. Add `syncpack` so the repo uses the pinned validator.",
            root.rel_path
        ),
        (true, false, true) => format!(
            "`{}` invokes `syncpack lint`, but not in a supported fail-closed root script position. Remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden.",
            root.rel_path
        ),
        (true, false, false) => format!(
            "`{}` lists `syncpack` but does not run `syncpack lint` in any root script. Add `syncpack lint` to the root script surface so the package policy validator actually runs.",
            root.rel_path
        ),
        (true, true, _) => format!(
            "`{}` has the required Syncpack package-policy setup, but the success path was not reached. Check the package Syncpack contract implementation.",
            root.rel_path
        ),
    }
}

/// `check_syncpack_config`: check syncpack config.
fn check_syncpack_config(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.syncpack_config {
        g3ts_package_types::G3TsPackageSyncpackConfigState::NotRequired => {}
        g3ts_package_types::G3TsPackageSyncpackConfigState::Missing { rel_path } => {
            push_syncpack_unavailable_error(input, rel_path, "is missing", results);
        }
        g3ts_package_types::G3TsPackageSyncpackConfigState::Unreadable { rel_path, reason }
        | g3ts_package_types::G3TsPackageSyncpackConfigState::ParseError { rel_path, reason } => {
            push_syncpack_unavailable_error(input, rel_path, reason, results);
        }
        g3ts_package_types::G3TsPackageSyncpackConfigState::Parsed { snapshot } => {
            if !snapshot.missing_source_entries.is_empty() {
                results.push(error(
                    ID,
                    "Syncpack does not cover package manifests",
                    format!(
                        "`{}` is missing exact `source` entries for package manifests: {}. Add the listed paths so `syncpack lint` can reject forbidden dependencies everywhere the package family applies.",
                        snapshot.rel_path,
                        snapshot
                            .missing_source_entries
                            .iter()
                            .map(|source| format!("`{source}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    &snapshot.rel_path,
                ));
            }

            if !snapshot.missing_forbidden_bans.is_empty() {
                results.push(error(
                    ID,
                    "Syncpack does not ban forbidden package dependencies",
                    format!(
                        "`{}` is missing Syncpack banned versionGroups for: {}. Add one canonical banned versionGroup per listed dependency before any repo-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\", \"optional\", \"peer\"]`, `isBanned: true`, and no `packages` or `specifierTypes`.",
                        snapshot.rel_path,
                        snapshot
                            .missing_forbidden_bans
                            .iter()
                            .map(|dependency| format!("`{dependency}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    &snapshot.rel_path,
                ));
            }
        }
    }
}

/// `push_syncpack_unavailable_error`: push syncpack unavailable error.
fn push_syncpack_unavailable_error(
    input: &G3TsPackageChecksInput,
    rel_path: &str,
    reason: &str,
    results: &mut Vec<G3CheckResult>,
) {
    let Some(root) = parsed_root(input) else {
        return;
    };
    results.push(error(
        ID,
        "Syncpack does not ban forbidden package dependencies",
        format!(
            "`{rel_path}` {reason}, so the package family cannot prove Syncpack bans forbidden package deps for `{}`. Add a parseable `{rel_path}` with exact `source` entries and canonical `isBanned: true` versionGroups for {}.",
            root.rel_path,
            forbidden_syncpack_deps_message(input)
        ),
        rel_path,
    ));
}
