use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::Layer;
use super::inputs::MemberDependencyHexarchInput;

const ID: &str = "RS-HEXARCH-21";
const BUILTIN_ALLOWED: &[&str] = &[
    "serde",
    "serde_json",
    "thiserror",
    "chrono",
    "uuid",
    "time",
    "bytes",
];

pub fn check(input: &MemberDependencyHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let member = input.member;
    if member.layer != Some(Layer::Domain) {
        return;
    }

    let allowed = BUILTIN_ALLOWED
        .iter()
        .map(|value| (*value).to_owned())
        .chain(member.allowed_deps.iter().cloned())
        .collect::<BTreeSet<_>>();

    for edge in input.edges.iter().filter(|edge| !edge.kind.is_dev()) {
        if edge.source_app_root_rel_dir.is_some()
            && edge.target_app_root_rel_dir.is_some()
            && edge.source_app_root_rel_dir != edge.target_app_root_rel_dir
        {
            continue;
        }
        if let Some(target_layer) = edge.target_layer {
            if edge.resolved_target_is_member
                && matches!(target_layer, Layer::Domain | Layer::Ports)
            {
                continue;
            }
            if edge.resolved_target_rel_dir.is_some()
                && !matches!(target_layer, Layer::Domain | Layer::Ports)
            {
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: format!("domain crate `{}` depends on non-pure layer", member.name),
                    message: format!(
                        "Domain crate `{}` depends on {} layer `{}` via `{}`.",
                        member.name,
                        target_layer.label(),
                        edge.dep_package_name,
                        edge.section_label
                    ),
                    file: Some(member.cargo_rel_path.clone()),
                    line: None,
                    inventory: false,
                });
                continue;
            }
            if edge.resolved_target_rel_dir.is_some()
                && matches!(target_layer, Layer::Domain | Layer::Ports)
            {
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: format!(
                        "domain crate `{}` depends on non-workspace pure-layer crate",
                        member.name
                    ),
                    message: format!(
                        "Domain crate `{}` depends on {}-layer path `{}` via `{}`, but that target is not a discovered workspace member.",
                        member.name,
                        target_layer.label(),
                        edge.dep_package_name,
                        edge.section_label
                    ),
                    file: Some(member.cargo_rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
            continue;
        }

        if !allowed.contains(&edge.dep_package_name) {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: format!(
                    "domain crate `{}` depends on disallowed external crate `{}`",
                    member.name, edge.dep_package_name
                ),
                message: format!(
                    "Domain crate `{}` may only use pure allowlisted external crates. `{}` is not in the built-in allowlist or `allowed_deps`.",
                    member.name, edge.dep_package_name
                ),
                file: Some(member.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_hexarch_21_domain_purity_tests/mod.rs"]
mod rs_hexarch_21_domain_purity_tests;
