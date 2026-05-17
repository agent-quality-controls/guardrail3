use std::collections::BTreeSet;

use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchCratePurityChecksInput, G3RsApparchLayer,
    G3RsApparchRustPolicyState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/logic-purity";
const BUILTIN_ALLOWED: &[&str] = &[
    "serde",
    "serde_json",
    "thiserror",
    "chrono",
    "uuid",
    "time",
    "bytes",
];

pub(crate) fn check(input: &G3RsApparchCratePurityChecksInput, results: &mut Vec<G3CheckResult>) {
    let krate = &input.krate;
    if krate.layer != Some(G3RsApparchLayer::Logic) {
        return;
    }

    let Some(allowed) = allowed_dependencies(&input.rust_policy, krate, results) else {
        return;
    };
    let violating = input
        .external_dependencies
        .iter()
        .filter(|dep| !dep.kind.is_dev())
        .filter(|dep| !allowed.contains(dep.dep_name.as_str()))
        .collect::<Vec<_>>();

    if violating.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!("logic crate `{}` stays pure", crate::run::display_crate(krate)),
                format!(
                    "Logic crate `{}` uses only built-in pure dependencies or explicitly allowlisted externals.",
                    crate::run::display_crate(krate)
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for dep in violating {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!(
                "logic crate `{}` depends on impure external crate `{}`",
                crate::run::display_crate(krate),
                dep.dep_name
            ),
            format!(
                "Logic crate `{}` may only use built-in pure externals or `allowed_deps` from guardrail3-rs.toml. `{}` was found via `{}`.",
                crate::run::display_crate(krate),
                dep.dep_name,
                dep.kind.label()
            ),
            Some(krate.cargo_rel_path.clone()),
            None,
        ));
    }
}

fn allowed_dependencies(
    rust_policy: &G3RsApparchRustPolicyState,
    krate: &G3RsApparchCrate,
    results: &mut Vec<G3CheckResult>,
) -> Option<BTreeSet<String>> {
    let mut allowed = BUILTIN_ALLOWED
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<BTreeSet<_>>();
    match rust_policy {
        G3RsApparchRustPolicyState::Missing => Some(allowed),
        G3RsApparchRustPolicyState::Parsed { allowed_deps, .. } => {
            for dep in allowed_deps {
                let _ = allowed.insert(dep.clone());
            }
            Some(allowed)
        }
        G3RsApparchRustPolicyState::Unreadable { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!("cannot validate purity for `{}`", crate::run::display_crate(krate)),
                format!(
                    "`{}` has external dependencies that require allowlist evaluation, but `{}` is unreadable: {}.",
                    crate::run::display_crate(krate),
                    rel_path,
                    reason
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            ));
            None
        }
        G3RsApparchRustPolicyState::ParseError { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!("cannot validate purity for `{}`", crate::run::display_crate(krate)),
                format!(
                    "`{}` has external dependencies that require allowlist evaluation, but `{}` could not be parsed: {}.",
                    crate::run::display_crate(krate),
                    rel_path,
                    reason
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            ));
            None
        }
    }
}
