use std::collections::{BTreeMap, BTreeSet};

pub(super) fn resolve_validation_state(
    candidate_names: &[String],
    global_type_validation_map: &BTreeMap<String, (bool, bool)>,
    simple_type_validation_map: &BTreeMap<String, Vec<(bool, bool)>>,
    global_manual_validate_types: &BTreeSet<String>,
    simple_manual_validate_counts: &BTreeMap<String, usize>,
) -> Option<(bool, bool)> {
    for candidate_name in candidate_names {
        if let Some((has_non_primitive, has_validate_derive)) = resolve_exact_validation_state(
            candidate_name,
            global_type_validation_map,
            global_manual_validate_types,
        ) {
            return Some((has_non_primitive, has_validate_derive));
        }

        let stripped_candidate = strip_local_path_prefixes(candidate_name);
        if let Some((has_non_primitive, has_validate_derive)) =
            stripped_candidate.and_then(|local_candidate| {
                resolve_exact_validation_state(
                    local_candidate,
                    global_type_validation_map,
                    global_manual_validate_types,
                )
            })
        {
            return Some((has_non_primitive, has_validate_derive));
        }

        if candidate_name.contains("::") && stripped_candidate.is_none() {
            continue;
        }

        let simple_name = stripped_candidate
            .unwrap_or(candidate_name.as_str())
            .rsplit("::")
            .next()
            .unwrap_or(candidate_name.as_str());
        if let Some(states) = simple_type_validation_map.get(simple_name) {
            if states.len() == 1 {
                return Some((
                    states[0].0,
                    states[0].1
                        || simple_manual_validate_counts.get(simple_name).copied() == Some(1),
                ));
            }
        }
    }
    None
}

fn resolve_exact_validation_state(
    candidate_name: &str,
    global_type_validation_map: &BTreeMap<String, (bool, bool)>,
    global_manual_validate_types: &BTreeSet<String>,
) -> Option<(bool, bool)> {
    if let Some((has_non_primitive, has_validate_derive)) =
        global_type_validation_map.get(candidate_name)
    {
        return Some((
            *has_non_primitive,
            *has_validate_derive || global_manual_validate_types.contains(candidate_name),
        ));
    }
    None
}

fn strip_local_path_prefixes(candidate_name: &str) -> Option<&str> {
    let mut stripped = candidate_name;
    let mut changed = false;
    loop {
        if let Some(rest) = stripped.strip_prefix("crate::") {
            stripped = rest;
            changed = true;
            continue;
        }
        if let Some(rest) = stripped.strip_prefix("self::") {
            stripped = rest;
            changed = true;
            continue;
        }
        if let Some(rest) = stripped.strip_prefix("super::") {
            stripped = rest;
            changed = true;
            continue;
        }
        break;
    }
    changed.then_some(stripped)
}
