use g3rs_garde_source_checks::{G3RsGardeSourceChecksInput, G3RsSourceFile};
use g3rs_garde_config_checks::G3RsGardeConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::collect;

pub fn check(surface: &FamilyView, route: &RsGardeRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for failure in &facts.input_failures {
        crate::root_policy::rs_garde_10_input_failures::check(
            &crate::inputs::GardeInputFailureInput::new(failure),
            &mut results,
        );
    }

    for root in &facts.roots {
        if !root.garde_applicable {
            continue;
        }
        let input = crate::inputs::GardeRootInput::new(root);
        run_config_checks(&input, &mut results);

        if !root.garde_dependency_present {
            continue;
        }
    }

    run_ast_checks(surface, route, &facts, &mut results);

    results
}

fn run_config_checks(input: &crate::inputs::GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(cargo) = input.root.cargo_parsed_typed.clone() else {
        crate::root_policy::rs_garde_config_01_dependency_present::check(input, results);
        crate::root_policy::rs_garde_config_02_core_method_bans::check(input, results);
        crate::root_policy::rs_garde_config_03_extractor_type_bans::check(input, results);
        crate::root_policy::rs_garde_config_04_reqwest_json_ban::check(input, results);
        crate::root_policy::rs_garde_config_05_additional_method_bans::check(input, results);
        return;
    };

    let (clippy_rel_path, clippy) = match (
        input.root.clippy_rel_path.clone(),
        input.root.clippy_parsed_typed.clone(),
    ) {
        (Some(rel_path), Some(parsed)) => (Some(rel_path), Some(parsed)),
        _ => {
            // Cargo parsed OK but clippy config missing/unparseable — fall back to
            // app-level checks for the clippy ban rules only; the dependency check
            // can still run through the extracted package.
            let package_input = G3RsGardeConfigChecksInput {
                cargo_rel_path: input.root.cargo_rel_path.clone(),
                cargo,
                clippy_rel_path: None,
                clippy: None,
            };
            results.extend(
                g3rs_garde_config_checks::check(&package_input)
                    .into_iter()
                    .map(convert_check_result),
            );
            crate::root_policy::rs_garde_config_02_core_method_bans::check(input, results);
            crate::root_policy::rs_garde_config_03_extractor_type_bans::check(input, results);
            crate::root_policy::rs_garde_config_04_reqwest_json_ban::check(input, results);
            crate::root_policy::rs_garde_config_05_additional_method_bans::check(input, results);
            return;
        }
    };

    let package_input = G3RsGardeConfigChecksInput {
        cargo_rel_path: input.root.cargo_rel_path.clone(),
        cargo,
        clippy_rel_path,
        clippy,
    };
    results.extend(
        g3rs_garde_config_checks::check(&package_input)
            .into_iter()
            .map(convert_check_result),
    );
}

fn run_ast_checks(
    surface: &FamilyView,
    route: &RsGardeRoute,
    facts: &crate::facts::GardeFacts,
    results: &mut Vec<CheckResult>,
) {
    let root_dirs = facts
        .roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<Vec<_>>();

    for root in facts.roots.iter().filter(|root| root.garde_applicable) {
        let Some(guardrail_rel_path) = route.family_files().iter().find_map(|file| {
            (file.kind() == RustFamilyFileKind::GuardrailToml
                && file.exact_rust_root_owner()
                && file.logical_owner_rel() == root.rel_dir)
                .then(|| file.rel_path().to_owned())
        }) else {
            continue;
        };
        let Some(guardrail_abs_path) = surface.abs_path(&guardrail_rel_path) else {
            continue;
        };
        if facts
            .input_failures
            .iter()
            .any(|failure| failure.rel_path == guardrail_rel_path)
        {
            continue;
        }

        let source_files = crate::discover::rust_file_rels(surface)
            .into_iter()
            .filter(|rel_path| {
                route
                    .scoped_files()
                    .is_none_or(|files| files.contains(rel_path))
            })
            .filter(|rel_path| owning_root_dir(rel_path, &root_dirs) == Some(root.rel_dir.as_str()))
            .filter_map(|rel_path| surface.abs_path(&rel_path).map(|abs_path| G3RsSourceFile {
                rel_path,
                abs_path,
            }))
            .collect::<Vec<_>>();

        if source_files.is_empty() {
            continue;
        }

        let package_input = G3RsGardeSourceChecksInput {
            garde_dependency_present: root.garde_dependency_present,
            source_files,
            guardrail_toml: G3RsSourceFile {
                rel_path: guardrail_rel_path,
                abs_path: guardrail_abs_path,
            },
        };
        results.extend(
            g3rs_garde_source_checks::check(&package_input)
                .into_iter()
                .filter(|result| result.id() != "RS-GARDE-10")
                .map(convert_check_result),
        );
    }
}

fn owning_root_dir<'a>(rel_path: &str, root_dirs: &'a [String]) -> Option<&'a str> {
    let parent = rel_path.rsplit_once('/').map_or("", |(prefix, _)| prefix);
    root_dirs
        .iter()
        .filter(|root| {
            root.is_empty()
                || parent == root.as_str()
                || parent
                    .strip_prefix(root.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|root| root.len())
        .map(String::as_str)
}

fn convert_check_result(result: G3CheckResult) -> CheckResult {
    CheckResult::from_parts(
        result.id().to_owned(),
        convert_severity(result.severity()),
        result.title().to_owned(),
        result.message().to_owned(),
        result.file().map(str::to_owned),
        result.line(),
        result.inventory(),
    )
}

fn convert_severity(severity: G3Severity) -> Severity {
    match severity {
        G3Severity::Error => Severity::Error,
        G3Severity::Warn => Severity::Warn,
        G3Severity::Info => Severity::Info,
    }
}
