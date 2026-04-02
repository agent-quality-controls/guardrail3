use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};


use crate::dependency_facts::Layer;
use crate::inputs::MemberDependencyHexarchInput;
use crate::inventory::push_success;

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
    let before = results.len();

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
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    format!("domain crate `{}` depends on non-pure layer", member.name),
                    format!(
                        "Domain crate `{}` depends on {} layer `{}` via `{}`.",
                        member.name,
                        target_layer.label(),
                        edge.dep_package_name,
                        edge.section_label
                    ),
                    Some(member.cargo_rel_path.clone()),
                    None,
                    false,
                ));
                continue;
            }
            if edge.resolved_target_rel_dir.is_some()
                && matches!(target_layer, Layer::Domain | Layer::Ports)
            {
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    format!(
                        "domain crate `{}` depends on non-workspace pure-layer crate",
                        member.name
                    ),
                    format!(
                        "Domain crate `{}` depends on {}-layer path `{}` via `{}`, but that target is not a discovered workspace member.",
                        member.name,
                        target_layer.label(),
                        edge.dep_package_name,
                        edge.section_label
                    ),
                    Some(member.cargo_rel_path.clone()),
                    None,
                    false,
                ));
            }
            continue;
        }

        if !allowed.contains(&edge.dep_package_name) {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!(
                    "domain crate `{}` depends on disallowed external crate `{}`",
                    member.name, edge.dep_package_name
                ),
                format!(
                    "Domain crate `{}` may only use pure allowlisted external crates. `{}` is not in the built-in allowlist or `allowed_deps`.",
                    member.name, edge.dep_package_name
                ),
                Some(member.cargo_rel_path.clone()),
                None,
                false,
            ));
        }
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            format!("domain crate `{}` stays pure", member.name),
            format!(
                "Domain crate `{}` uses only allowed pure-layer or allowlisted dependencies.",
                member.name
            ),
            Some(member.cargo_rel_path.clone()),
        );
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DomainPurityEdgeKindForTest {
    Dependency,
    DevDependency,
    BuildDependency,
    TargetDependency,
    TargetBuildDependency,
}

