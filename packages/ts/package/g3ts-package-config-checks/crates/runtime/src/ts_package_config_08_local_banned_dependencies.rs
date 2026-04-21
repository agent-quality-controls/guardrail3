use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, local_banned_dependencies, local_parse_blockers};

const ID: &str = "TS-PACKAGE-CONFIG-08";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.locals.is_empty() {
        return;
    }

    let parse_blockers = local_parse_blockers(input);
    let banned_dependencies = local_banned_dependencies(input);

    for (rel_path, reason) in parse_blockers {
        results.push(error(
            ID,
            "local package.json parse blocker",
            format!("Failed to prove local manifest policy for `{rel_path}`: {reason}"),
            rel_path,
        ));
    }

    for (snapshot, dependency) in &banned_dependencies {
        results.push(error(
            ID,
            "banned dependency declared",
            format!(
                "Local manifest `{}` declares banned dependency `{dependency}`.",
                snapshot.rel_path
            ),
            &snapshot.rel_path,
        ));
    }

    if results
        .iter()
        .any(|result| result.id() == ID && !result.inventory())
    {
        return;
    }

    results.push(info(
        ID,
        "local manifests avoid banned dependencies",
        format!(
            "Checked {} local package manifests without banned dependency declarations.",
            input.locals.len()
        ),
        input
            .locals
            .first()
            .map(|state| match state {
                g3ts_package_types::G3TsPackageLocalState::Unreadable { rel_path, .. }
                | g3ts_package_types::G3TsPackageLocalState::ParseError { rel_path, .. } => {
                    rel_path.as_str()
                }
                g3ts_package_types::G3TsPackageLocalState::Parsed { snapshot } => {
                    snapshot.rel_path.as_str()
                }
            })
            .unwrap_or("package.json"),
    ));
}
