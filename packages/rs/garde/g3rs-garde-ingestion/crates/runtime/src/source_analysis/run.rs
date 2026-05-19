#![expect(
    clippy::too_many_lines,
    clippy::wildcard_enum_match_arm,
    clippy::type_complexity,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "analyze_source_files orchestrates the full per-source-file pipeline (per-file syn parse, per-boundary classification, cross-file simple-name aggregation); flattening loses the parallel structure with the data model; the (has_non_primitive, has_validate_derive) tuple is the compact contract between the per-file pass and the cross-file aggregator; counters increment by 1 per occurrence and saturating wouldn't change the rule output; `states[0]` is exercised on a non-empty Vec the producer just inserted into"
)]

use std::collections::{BTreeMap, BTreeSet};

use g3rs_garde_types::{
    G3RsGardeBoundaryFieldSite, G3RsGardeBoundaryKind, G3RsGardeDerivedBoundaryTypeSite,
    G3RsGardeInputFailureSite, G3RsGardeManualDeserializeImplSite, G3RsGardeQueryAsMacroSite,
    G3RsGardeRustPolicyInput, G3RsSourceFile,
};

use super::parse;

#[derive(Debug, Clone, Default)]
/// Struct `AnalyzedGardeSource` used by this module.
pub(crate) struct AnalyzedGardeSource {
    /// Field `input_failures`.
    pub(crate) input_failures: Vec<G3RsGardeInputFailureSite>,
    /// Field `struct_targets`.
    pub(crate) struct_targets: Vec<G3RsGardeDerivedBoundaryTypeSite>,
    /// Field `enum_targets`.
    pub(crate) enum_targets: Vec<G3RsGardeDerivedBoundaryTypeSite>,
    /// Field `manual_deserialize_impls`.
    pub(crate) manual_deserialize_impls: Vec<G3RsGardeManualDeserializeImplSite>,
    /// Field `boundary_fields`.
    pub(crate) boundary_fields: Vec<G3RsGardeBoundaryFieldSite>,
    /// Field `query_as_macros`.
    pub(crate) query_as_macros: Vec<G3RsGardeQueryAsMacroSite>,
}

#[derive(Debug, Clone)]
/// Struct `ParsedSourceFile` used by this module.
struct ParsedSourceFile {
    /// Field `rel_path`.
    rel_path: String,
    /// Field `parsed`.
    parsed: parse::ParsedGardeFile,
}

/// Implements `analyze source files`.
pub(crate) fn analyze_source_files(
    source_files: &[G3RsSourceFile],
    rust_policy: &G3RsGardeRustPolicyInput,
) -> AnalyzedGardeSource {
    let mut input_failures = Vec::new();
    let policy_resolved = resolve_rust_policy(rust_policy, &mut input_failures);

    let mut parsed_files = Vec::new();
    let mut ordered = source_files.to_vec();
    ordered.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

    for source_file in &ordered {
        let content = match crate::fs::read_to_string(&source_file.abs_path) {
            Ok(content) => content,
            Err(read_error) => {
                input_failures.push(G3RsGardeInputFailureSite {
                    rel_path: source_file.rel_path.clone(),
                    message: format!(
                        "Failed to read Rust source file for garde checks: {read_error}"
                    ),
                });
                continue;
            }
        };
        let source = match parse::parse_rust_file(&content) {
            Ok(source) => source,
            Err(parse_error) => {
                input_failures.push(G3RsGardeInputFailureSite {
                    rel_path: source_file.rel_path.clone(),
                    message: format!(
                        "Failed to parse Rust source file for garde checks: {parse_error}"
                    ),
                });
                continue;
            }
        };
        parsed_files.push(ParsedSourceFile {
            rel_path: source_file.rel_path.clone(),
            parsed: parse::analyze(&source),
        });
    }

    let mut global_type_validation_map = BTreeMap::<String, Vec<(bool, bool)>>::new();
    let mut simple_type_validation_map = BTreeMap::<String, Vec<(bool, bool)>>::new();
    for parsed_file in &parsed_files {
        for (type_name, state) in &parsed_file.parsed.type_validation_map {
            global_type_validation_map
                .entry(type_name.clone())
                .or_default()
                .push(*state);
            let simple_name = type_name
                .rsplit("::")
                .next()
                .unwrap_or(type_name.as_str())
                .to_owned();
            simple_type_validation_map
                .entry(simple_name)
                .or_default()
                .push(*state);
        }
    }

    let global_manual_validate_types: BTreeSet<_> = parsed_files
        .iter()
        .flat_map(|parsed_file| parsed_file.parsed.manual_validate_impls.iter().cloned())
        .collect();
    let mut simple_manual_validate_counts = BTreeMap::<String, usize>::new();
    for type_name in &global_manual_validate_types {
        let simple_name = type_name
            .rsplit("::")
            .next()
            .unwrap_or(type_name.as_str())
            .to_owned();
        *simple_manual_validate_counts
            .entry(simple_name)
            .or_insert(0) += 1;
    }

    let mut struct_targets = Vec::new();
    let mut enum_targets = Vec::new();
    let mut manual_deserialize_impls = Vec::new();
    let mut boundary_fields = Vec::new();
    let mut query_as_macros = Vec::new();

    for parsed_file in parsed_files {
        for target in parsed_file.parsed.derived_types {
            let boundary_kind = match target.boundary_kind {
                parse::BoundaryKind::Struct => G3RsGardeBoundaryKind::Struct,
                parse::BoundaryKind::Enum => G3RsGardeBoundaryKind::Enum,
            };
            let site = G3RsGardeDerivedBoundaryTypeSite {
                rel_path: parsed_file.rel_path.clone(),
                line: target.line,
                name: target.name,
                boundary_kind,
                boundary_macros: target.boundary_macros,
                has_validate: target.has_validate_derive,
            };
            match boundary_kind {
                G3RsGardeBoundaryKind::Struct if target.has_non_primitive_fields => {
                    struct_targets.push(site);
                }
                G3RsGardeBoundaryKind::Enum if target.has_non_primitive_fields => {
                    enum_targets.push(site);
                }
                _ => {}
            }
        }

        for manual_impl in parsed_file.parsed.manual_deserialize_impls {
            let resolved = resolve_validation_state(
                std::slice::from_ref(&manual_impl.type_name),
                &global_type_validation_map,
                &simple_type_validation_map,
                &global_manual_validate_types,
                &simple_manual_validate_counts,
            );
            let has_manual_validate = parsed_file
                .parsed
                .manual_validate_impls
                .contains(&manual_impl.type_name);
            let needs_validate = resolved.is_none_or(|(has_non_primitive, _)| has_non_primitive);
            let has_validate =
                resolved.is_some_and(|(_, has_validate)| has_validate) || has_manual_validate;
            manual_deserialize_impls.push(G3RsGardeManualDeserializeImplSite {
                rel_path: parsed_file.rel_path.clone(),
                line: manual_impl.line,
                type_name: manual_impl.type_name,
                needs_validate,
                has_validate,
            });
        }

        for field in parsed_file.parsed.boundary_fields {
            let boundary_has_validate = field.boundary_has_validate_derive
                || resolve_validation_state(
                    std::slice::from_ref(&field.boundary_name),
                    &global_type_validation_map,
                    &simple_type_validation_map,
                    &global_manual_validate_types,
                    &simple_manual_validate_counts,
                )
                .is_some_and(|(_, has_validate)| has_validate);
            if !boundary_has_validate {
                continue;
            }

            let nested_validated = resolve_validation_state(
                &field.candidate_type_names,
                &global_type_validation_map,
                &simple_type_validation_map,
                &global_manual_validate_types,
                &simple_manual_validate_counts,
            )
            .is_some_and(|(needs_validate, has_validate)| needs_validate && has_validate);

            boundary_fields.push(G3RsGardeBoundaryFieldSite {
                rel_path: parsed_file.rel_path.clone(),
                line: field.line,
                boundary_name: field.boundary_name,
                field_name: field.field_name,
                field_type: field.field_type,
                requires_field_validation: field.requires_field_validation,
                nested_validated,
                has_garde_skip: field.has_garde_skip,
                has_garde_dive: field.has_garde_dive,
                has_meaningful_garde_rule: field.has_meaningful_garde_rule,
                uses_context: field.uses_context,
                boundary_has_context: field.boundary_has_context,
            });
        }

        for macro_use in parsed_file.parsed.query_as_macros {
            query_as_macros.push(G3RsGardeQueryAsMacroSite {
                rel_path: parsed_file.rel_path.clone(),
                line: macro_use.line,
                macro_name: macro_use.macro_name,
                policy_resolved,
            });
        }
    }

    struct_targets.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    enum_targets.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    manual_deserialize_impls.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    boundary_fields.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    query_as_macros.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));

    AnalyzedGardeSource {
        input_failures,
        struct_targets,
        enum_targets,
        manual_deserialize_impls,
        boundary_fields,
        query_as_macros,
    }
}

/// Implements `resolve validation state`.
fn resolve_validation_state(
    candidate_names: &[String],
    global_type_validation_map: &BTreeMap<String, Vec<(bool, bool)>>,
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

/// Implements `resolve exact validation state`.
fn resolve_exact_validation_state(
    candidate_name: &str,
    global_type_validation_map: &BTreeMap<String, Vec<(bool, bool)>>,
    global_manual_validate_types: &BTreeSet<String>,
) -> Option<(bool, bool)> {
    let states = global_type_validation_map.get(candidate_name)?;
    if states.len() != 1 {
        return None;
    }

    let (has_non_primitive, has_validate_derive) = states[0];
    Some((
        has_non_primitive,
        has_validate_derive || global_manual_validate_types.contains(candidate_name),
    ))
}

/// Implements `strip local path prefixes`.
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

/// Implements `resolve rust policy`.
fn resolve_rust_policy(
    input: &G3RsGardeRustPolicyInput,
    input_failures: &mut Vec<G3RsGardeInputFailureSite>,
) -> bool {
    match input {
        G3RsGardeRustPolicyInput::Missing | G3RsGardeRustPolicyInput::Parsed { .. } => true,
        G3RsGardeRustPolicyInput::Invalid { rel_path, message } => {
            input_failures.push(G3RsGardeInputFailureSite {
                rel_path: rel_path.clone(),
                message: message.clone(),
            });
            false
        }
    }
}
