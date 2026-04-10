use std::collections::{BTreeMap, BTreeSet};

use g3rs_garde_ast_checks_types::G3RsGardeAstChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_domain_config::types::GuardrailConfig;

use crate::parse::{self, BoundaryKind, GuardrailConfigParseKind, ParsedGardeFile};

#[derive(Debug, Clone)]
pub(crate) struct DerivedBoundaryTypeSite {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) boundary_kind: BoundaryKind,
    pub(crate) boundary_macros: Vec<String>,
    pub(crate) has_validate: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ManualDeserializeImplSite {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) type_name: String,
    pub(crate) needs_validate: bool,
    pub(crate) has_validate: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct QueryAsMacroSite {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) escape_hatch_reason: Option<String>,
    pub(crate) policy_available: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct BoundaryFieldSite {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) boundary_name: String,
    pub(crate) field_name: String,
    pub(crate) field_type: String,
    pub(crate) requires_field_validation: bool,
    pub(crate) nested_validated: bool,
    pub(crate) has_garde_skip: bool,
    pub(crate) has_garde_dive: bool,
    pub(crate) has_meaningful_garde_rule: bool,
    pub(crate) uses_context: bool,
    pub(crate) boundary_has_context: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct GuardrailConfigValidationSite {
    pub(crate) rel_path: String,
    pub(crate) line: usize,
    pub(crate) parse_kind: GuardrailConfigParseKind,
}

#[derive(Debug, Clone)]
pub(crate) struct InputFailureSite {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct GardeAstAnalysis {
    pub(crate) input_failures: Vec<InputFailureSite>,
    pub(crate) struct_targets: Vec<DerivedBoundaryTypeSite>,
    pub(crate) enum_targets: Vec<DerivedBoundaryTypeSite>,
    pub(crate) manual_deserialize_impls: Vec<ManualDeserializeImplSite>,
    pub(crate) boundary_fields: Vec<BoundaryFieldSite>,
    pub(crate) query_as_macros: Vec<QueryAsMacroSite>,
    pub(crate) guardrail_config_validation_sites: Vec<GuardrailConfigValidationSite>,
}

#[derive(Debug, Clone)]
struct ParsedSourceFile {
    rel_path: String,
    parsed: ParsedGardeFile,
}

pub(crate) fn analyze_input(input: &G3RsGardeAstChecksInput) -> GardeAstAnalysis {
    let mut input_failures = Vec::new();
    let guardrail_config = match std::fs::read_to_string(&input.guardrail_toml.abs_path) {
        Ok(content) => match toml::from_str::<GuardrailConfig>(&content) {
            Ok(config) => Some(config),
            Err(parse_error) => {
                input_failures.push(InputFailureSite {
                    rel_path: input.guardrail_toml.rel_path.clone(),
                    message: format!(
                        "Failed to parse guardrail3.toml for garde policy resolution: {parse_error}"
                    ),
                });
                None
            }
        },
        Err(read_error) => {
            input_failures.push(InputFailureSite {
                rel_path: input.guardrail_toml.rel_path.clone(),
                message: format!(
                    "Failed to read guardrail3.toml for garde policy resolution: {read_error}"
                ),
            });
            None
        }
    };

    let mut parsed_files = Vec::new();
    let mut source_files = input.source_files.clone();
    source_files.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

    for source_file in &source_files {
        let content = match std::fs::read_to_string(&source_file.abs_path) {
            Ok(content) => content,
            Err(read_error) => {
                input_failures.push(InputFailureSite {
                    rel_path: source_file.rel_path.clone(),
                    message: format!(
                        "Failed to read Rust source file for garde checks: {read_error}"
                    ),
                });
                continue;
            }
        };
        let ast = match parse::parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                input_failures.push(InputFailureSite {
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
            parsed: parse::analyze(&ast),
        });
    }

    let garde_applicable = input.garde_dependency_present
        || parsed_files
            .iter()
            .any(|parsed_file| parsed_file_shows_garde_adoption(&parsed_file.parsed));

    if !garde_applicable {
        return GardeAstAnalysis {
            input_failures,
            ..GardeAstAnalysis::default()
        };
    }

    let mut global_type_validation_map = BTreeMap::<String, (bool, bool)>::new();
    let mut simple_type_validation_map = BTreeMap::<String, Vec<(bool, bool)>>::new();
    for parsed_file in &parsed_files {
        for (type_name, state) in &parsed_file.parsed.type_validation_map {
            let _ = global_type_validation_map.insert(type_name.clone(), *state);
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
    let mut guardrail_config_validation_sites = Vec::new();

    for parsed_file in parsed_files {
        for target in parsed_file.parsed.derived_types {
            let site = DerivedBoundaryTypeSite {
                rel_path: parsed_file.rel_path.clone(),
                line: target.line,
                name: target.name,
                boundary_kind: target.boundary_kind,
                boundary_macros: target.boundary_macros,
                has_validate: target.has_validate_derive,
            };
            match target.boundary_kind {
                BoundaryKind::Struct if target.has_non_primitive_fields => struct_targets.push(site),
                BoundaryKind::Enum if target.has_non_primitive_fields => enum_targets.push(site),
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
            let needs_validate = resolved.map_or(true, |(has_non_primitive, _)| has_non_primitive);
            let has_validate =
                resolved.is_some_and(|(_, has_validate)| has_validate) || has_manual_validate;
            manual_deserialize_impls.push(ManualDeserializeImplSite {
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

            boundary_fields.push(BoundaryFieldSite {
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
            let selector = format!("{}@L{}", macro_use.macro_name, macro_use.line);
            query_as_macros.push(QueryAsMacroSite {
                rel_path: parsed_file.rel_path.clone(),
                line: macro_use.line,
                macro_name: macro_use.macro_name,
                policy_available: guardrail_config.is_some(),
                escape_hatch_reason: guardrail_config.as_ref().and_then(|config| {
                    config
                        .escape_hatch_reason(
                            "garde",
                            &parsed_file.rel_path,
                            "sqlx_query_as",
                            &selector,
                        )
                        .map(str::to_owned)
                }),
            });
        }

        for site in parsed_file.parsed.guardrail_config_validation_sites {
            guardrail_config_validation_sites.push(GuardrailConfigValidationSite {
                rel_path: parsed_file.rel_path.clone(),
                line: site.line,
                parse_kind: site.parse_kind,
            });
        }
    }

    struct_targets.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    enum_targets.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    manual_deserialize_impls.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    boundary_fields.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    query_as_macros.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));
    guardrail_config_validation_sites
        .sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.line.cmp(&b.line)));

    GardeAstAnalysis {
        input_failures,
        struct_targets,
        enum_targets,
        manual_deserialize_impls,
        boundary_fields,
        query_as_macros,
        guardrail_config_validation_sites,
    }
}

fn parsed_file_shows_garde_adoption(parsed: &ParsedGardeFile) -> bool {
    !parsed.derived_types.is_empty()
        || !parsed.manual_deserialize_impls.is_empty()
        || !parsed.manual_validate_impls.is_empty()
        || parsed
            .type_validation_map
            .values()
            .any(|(_has_non_primitive_fields, has_validate)| *has_validate)
}

fn resolve_validation_state(
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

pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        line,
    )
}

pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: Option<&str>,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        file.map(str::to_owned),
        line,
    )
}
