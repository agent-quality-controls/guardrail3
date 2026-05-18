use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackagePnpmWorkspaceSnapshot, G3TsPackagePnpmWorkspaceState,
};
use guardrail3_check_types::G3CheckResult;
use js_semver::{Range, Version};

use crate::support::{error, info, parsed_root};

/// `ID` constant.
const ID: &str = "g3ts-package/node-engine-install-contract";

/// `check`: check.
pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(root) = parsed_root(input) else {
        return;
    };
    let Some(engines_node) = root.engines_node.as_deref() else {
        results.push(error(
            ID,
            "node engine install contract is missing",
            "The root package manifest must declare `engines.node` before pnpm can verify \
             dependency engines against the supported Node floor."
                .to_owned(),
            &root.rel_path,
        ));
        return;
    };

    let workspace = match &input.pnpm_workspace {
        G3TsPackagePnpmWorkspaceState::Parsed { snapshot } => snapshot,
        G3TsPackagePnpmWorkspaceState::Missing { rel_path } => {
            results.push(error(
                ID,
                "node engine install contract is missing",
                format!(
                    "`{rel_path}` must exist with exact `nodeVersion` and `engineStrict: true` \
                     so pnpm checks dependency engines against the lowest supported Node."
                ),
                rel_path,
            ));
            return;
        }
        G3TsPackagePnpmWorkspaceState::Unreadable { rel_path, reason }
        | G3TsPackagePnpmWorkspaceState::ParseError { rel_path, reason } => {
            results.push(error(
                ID,
                "node engine install contract is unreadable",
                format!(
                    "`{rel_path}` must parse before pnpm engine policy can be verified: {reason}."
                ),
                rel_path,
            ));
            return;
        }
        G3TsPackagePnpmWorkspaceState::NotRequired => return,
    };

    let violations = contract_violations(workspace, engines_node);
    if violations.is_empty() {
        results.push(info(
            ID,
            "node engine install contract is enforced",
            format!(
                "`{}` pins `nodeVersion` and `engineStrict: true`; `engines.node` accepts that \
                 pinned Node version.",
                workspace.rel_path
            ),
            &workspace.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "node engine install contract is not enforced",
        violations.join(" "),
        &workspace.rel_path,
    ));
}

/// Return every policy violation in the pnpm workspace runtime contract.
fn contract_violations(
    workspace: &G3TsPackagePnpmWorkspaceSnapshot,
    engines_node: &str,
) -> Vec<String> {
    let mut violations = Vec::new();
    if workspace.engine_strict != Some(true) {
        violations.push("Set `engineStrict: true` in `pnpm-workspace.yaml`.".to_owned());
    }

    let Some(node_version) = workspace.node_version.as_deref() else {
        violations.push(
            "Set exact `nodeVersion`, for example `24.0.0`, in `pnpm-workspace.yaml`.".to_owned(),
        );
        return violations;
    };
    let version = match Version::parse(node_version) {
        Ok(version) => version,
        Err(error) => {
            violations.push(format!(
                "`pnpm-workspace.yaml` `nodeVersion` must be an exact semantic version; \
                 `{node_version}` failed to parse: {error}."
            ));
            return violations;
        }
    };
    let range = match Range::parse(engines_node) {
        Ok(range) => range,
        Err(error) => {
            violations.push(format!(
                "`package.json` `engines.node` must be a valid npm semver range; \
                 `{engines_node}` failed to parse: {error}."
            ));
            return violations;
        }
    };
    if !range.satisfies(&version) {
        violations.push(format!(
            "`package.json` `engines.node` must accept `pnpm-workspace.yaml` `nodeVersion`; \
             `{engines_node}` does not accept `{node_version}`."
        ));
    }

    violations
}
