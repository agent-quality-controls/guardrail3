use g3_garde_ast_checks::{G3AstFile, G3GardeAstChecksInput};
use g3_garde_content_checks::{G3GardeClippyBanChecksInput, G3GardeDependencyCheckInput};
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
        run_dependency_check(&input, &mut results);

        if !root.garde_dependency_present {
            continue;
        }

        run_clippy_ban_checks(&input, &mut results);
    }

    run_ast_checks(surface, route, &facts, &mut results);

    results
}

fn run_dependency_check(input: &crate::inputs::GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(cargo) = input.root.cargo_parsed_typed.clone() else {
        crate::root_policy::rs_garde_01_dependency_present::check(input, results);
        return;
    };

    let package_input = G3GardeDependencyCheckInput {
        cargo_rel_path: input.root.cargo_rel_path.clone(),
        cargo,
    };
    results.extend(
        g3_garde_content_checks::check_dependency_present(&package_input)
            .into_iter()
            .map(convert_check_result),
    );
}

fn run_clippy_ban_checks(
    input: &crate::inputs::GardeRootInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    let (Some(clippy_rel_path), Some(clippy)) = (
        input.root.clippy_rel_path.clone(),
        input.root.clippy_parsed_typed.clone(),
    ) else {
        crate::root_policy::rs_garde_02_core_method_bans::check(input, results);
        crate::root_policy::rs_garde_03_extractor_type_bans::check(input, results);
        crate::root_policy::rs_garde_04_reqwest_json_ban::check(input, results);
        crate::root_policy::rs_garde_06_additional_method_bans::check(input, results);
        return;
    };

    let package_input = G3GardeClippyBanChecksInput {
        clippy_rel_path,
        clippy,
    };
    results.extend(
        g3_garde_content_checks::check_clippy_bans(&package_input)
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
            .filter_map(|rel_path| {
                surface.abs_path(&rel_path).map(|abs_path| G3AstFile {
                    rel_path,
                    abs_path,
                })
            })
            .collect::<Vec<_>>();

        if source_files.is_empty() {
            continue;
        }

        let package_input = G3GardeAstChecksInput {
            source_files,
            guardrail_toml: G3AstFile {
                rel_path: guardrail_rel_path,
                abs_path: guardrail_abs_path,
            },
        };
        results.extend(
            g3_garde_ast_checks::check(&package_input)
                .into_iter()
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
