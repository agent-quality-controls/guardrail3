use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::{FamilyMapper, RsTestRoute};

mod discover;
mod facts;
mod inputs;
mod parse;
mod rs_test_01_inline_test_bodies;
mod rs_test_02_owned_sidecar_shape;
mod rs_test_03_runtime_assertions_split;
mod rs_test_04_ignore_reason;
mod rs_test_05_should_panic_expected;
mod rs_test_06_tautological_assertions;
mod rs_test_07_real_proof_site;
mod rs_test_08_weak_matches_assert;
mod rs_test_09_nextest_timeouts;
mod rs_test_10_input_failures;
mod rs_test_11_cargo_mutants_installed;
mod rs_test_12_mutants_toml_exists;
mod rs_test_13_mutants_profile_present;
mod rs_test_14_mutation_hook_present;
mod rs_test_15_mutants_config_sane;
mod rs_test_16_assertions_modules_prove;
mod rs_test_17_external_harnesses_use_assertions;
mod rs_test_18_test_support_generic;

use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::ToolChecker;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use self::discover::collect;
use self::facts::{DiscoveredTestFile, InputFailureFacts, TestFacts, TestFileKind, TestRootFacts};
use self::inputs::{
    AssertionsModuleInput, CfgTestModuleInput, InputFailureTestInput, RootTestInput, TestFileInput,
    TestFunctionInput, TestSupportFileInput,
};
use self::parse::{FunctionInfo, ParsedTestFile, UseBinding, parse_rust_file};

pub(crate) struct AnalyzedFile {
    pub(crate) facts: DiscoveredTestFile,
    pub(crate) parsed: ParsedTestFile,
}

#[derive(Default)]
struct RootAnalysis {
    files: Vec<AnalyzedFile>,
    has_tests: bool,
    has_tokio_tests: bool,
    input_failures: Vec<InputFailureFacts>,
    proof_bearing_assertions_by_file: BTreeMap<String, BTreeSet<String>>,
    proof_bearing_assertions_by_package: BTreeMap<String, BTreeSet<String>>,
    local_runtime_packages: BTreeSet<String>,
    local_assertions_packages: BTreeSet<String>,
}

pub fn check(tree: &ProjectTree, route: &RsTestRoute, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let facts = collect(tree, &route.roots, tc);
    let mut results = Vec::new();
    let discovered_root_dirs = facts
        .roots
        .iter()
        .map(|root| root.rel_dir.as_str())
        .collect::<BTreeSet<_>>();

    for failure in facts
        .input_failures
        .iter()
        .filter(|failure| failure.rel_path.ends_with("Cargo.toml"))
        .filter(|failure| !discovered_root_dirs.contains(failure.root_rel_dir.as_str()))
    {
        rs_test_10_input_failures::check(&InputFailureTestInput::new(failure), &mut results);
    }

    for root in &facts.roots {
        let analysis = analyze_root(tree, root, &facts, route.scoped_files.as_ref());
        let mutation_active =
            root.mutants_exists || root.has_mutants_profile || !root.mutation_hook_files.is_empty();
        let root_input = RootTestInput::new(
            root,
            analysis.has_tests,
            analysis.has_tokio_tests,
            facts.cargo_mutants_installed,
            &root.mutation_hook_files,
        );

        for failure in active_failures_for_root(&facts, root, &analysis, mutation_active) {
            rs_test_10_input_failures::check(&InputFailureTestInput::new(failure), &mut results);
        }

        if analysis.has_tests {
            rs_test_02_owned_sidecar_shape::collect(
                tree,
                root,
                &analysis.files,
                route.scoped_files.as_ref(),
                &mut results,
            );

            rs_test_03_runtime_assertions_split::collect(
                root,
                &analysis.files,
                route.scoped_files.as_ref(),
                &facts.local_package_names,
                &mut results,
            );

            for file in &analysis.files {
                let file_input = TestFileInput::new(&file.facts, &file.parsed);

                if matches!(file.facts.kind, TestFileKind::AssertionsModule) {
                    let empty = BTreeSet::new();
                    let proof_bearing_exported_functions = analysis
                        .proof_bearing_assertions_by_file
                        .get(&file.facts.rel_path)
                        .unwrap_or(&empty);
                    rs_test_16_assertions_modules_prove::check(
                        &AssertionsModuleInput::new(
                            &file.facts,
                            &file.parsed,
                            proof_bearing_exported_functions,
                        ),
                        &mut results,
                    );
                }

                if is_test_support_file(root, &file.facts.rel_path) {
                    rs_test_18_test_support_generic::check(
                        &TestSupportFileInput::new(
                            &file.facts,
                            &file.parsed,
                            &analysis.local_runtime_packages,
                            &analysis.local_assertions_packages,
                        ),
                        &mut results,
                    );
                }

                if matches!(file.facts.kind, TestFileKind::Source) {
                    for module in &file.parsed.cfg_test_modules {
                        rs_test_01_inline_test_bodies::check(
                            &CfgTestModuleInput::new(&file.facts, module),
                            &mut results,
                        );
                    }
                }

                rs_test_04_ignore_reason::check(&file_input, &mut results);

                for function in &file.parsed.test_functions {
                    let proof_bearing_assertion_functions = file
                        .facts
                        .assertions_package_name
                        .as_deref()
                        .and_then(|package| {
                            analysis.proof_bearing_assertions_by_package.get(package)
                        });
                    let function_input = TestFunctionInput::new(
                        &file.facts,
                        &file.parsed,
                        function,
                        proof_bearing_assertion_functions,
                    );
                    rs_test_16_assertions_modules_prove::check_sidecar_semantic_proof(
                        &function_input,
                        &mut results,
                    );
                    rs_test_05_should_panic_expected::check(&function_input, &mut results);
                    rs_test_06_tautological_assertions::check(&function_input, &mut results);
                    rs_test_07_real_proof_site::check(&function_input, &mut results);
                    rs_test_08_weak_matches_assert::check(&function_input, &mut results);
                    rs_test_17_external_harnesses_use_assertions::check(
                        &function_input,
                        &mut results,
                    );
                }
            }

            rs_test_09_nextest_timeouts::check(&root_input, &mut results);
        }

        if mutation_active {
            rs_test_11_cargo_mutants_installed::check(&root_input, &mut results);
            rs_test_12_mutants_toml_exists::check(&root_input, &mut results);
            rs_test_13_mutants_profile_present::check(&root_input, &mut results);
            rs_test_14_mutation_hook_present::check(&root_input, &mut results);
            rs_test_15_mutants_config_sane::check(&root_input, &mut results);
        }
    }

    results
}

pub fn check_test_tree(tree: &ProjectTree, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Test]));
    let route = FamilyMapper::new(tree, &scope, None, &selection, None).map_rs_test();
    check(tree, &route, tc)
}

fn analyze_root(
    tree: &ProjectTree,
    root: &TestRootFacts,
    facts: &TestFacts,
    scoped_files: Option<&BTreeSet<String>>,
) -> RootAnalysis {
    let mut analysis = RootAnalysis::default();

    for file in facts
        .files
        .iter()
        .filter(|file| file.root_rel_dir == root.rel_dir)
    {
        if matches!(
            file.kind,
            TestFileKind::InternalSidecarMod
                | TestFileKind::InternalSidecarSupport
                | TestFileKind::ExternalHarness
        ) {
            analysis.has_tests = true;
        }

        let content = match guardrail3_shared_fs::read_file_err(&tree.abs_path(&file.rel_path)) {
            Ok(content) => content,
            Err(read_error) => {
                analysis.input_failures.push(InputFailureFacts {
                    root_rel_dir: root.rel_dir.clone(),
                    rel_path: file.rel_path.clone(),
                    message: format!(
                        "Failed to read Rust source file for test-family analysis: {read_error}"
                    ),
                });
                continue;
            }
        };

        let ast = match parse_rust_file(&content) {
            Ok(ast) => ast,
            Err(parse_error) => {
                analysis.input_failures.push(InputFailureFacts {
                    root_rel_dir: root.rel_dir.clone(),
                    rel_path: file.rel_path.clone(),
                    message: format!(
                        "Failed to parse Rust source file for test-family analysis: {parse_error}"
                    ),
                });
                continue;
            }
        };

        let parsed = parse::analyze(&ast, &content);
        analysis.has_tests |= !parsed.test_functions.is_empty();
        analysis.has_tokio_tests |= parsed
            .test_functions
            .iter()
            .any(|function| function.uses_tokio_test_attr);
        if scoped_files.is_some_and(|paths| !paths.contains(&file.rel_path)) {
            continue;
        }
        analysis.files.push(AnalyzedFile {
            facts: file.clone(),
            parsed,
        });
    }

    analysis.local_runtime_packages = root
        .components
        .iter()
        .filter_map(|component| component.runtime_package_name.clone())
        .collect();
    analysis.local_assertions_packages = root
        .components
        .iter()
        .filter_map(|component| component.assertions_package_name.clone())
        .collect();
    (
        analysis.proof_bearing_assertions_by_file,
        analysis.proof_bearing_assertions_by_package,
    ) = collect_assertions_proof_catalog(&analysis.files);

    analysis
}

fn active_failures_for_root<'a>(
    facts: &'a TestFacts,
    root: &'a TestRootFacts,
    analysis: &'a RootAnalysis,
    mutation_active: bool,
) -> Vec<&'a InputFailureFacts> {
    let async_active =
        analysis.has_tests && (root.tokio_dependency_present || analysis.has_tokio_tests);

    facts
        .input_failures
        .iter()
        .filter(|failure| failure.root_rel_dir == root.rel_dir)
        .chain(analysis.input_failures.iter())
        .filter(|failure| {
            if failure.rel_path.ends_with("nextest.toml") {
                return async_active;
            }
            if failure.rel_path.ends_with("mutants.toml") {
                return mutation_active;
            }
            true
        })
        .collect::<Vec<_>>()
}

fn collect_assertions_proof_catalog(
    files: &[AnalyzedFile],
) -> (
    BTreeMap<String, BTreeSet<String>>,
    BTreeMap<String, BTreeSet<String>>,
) {
    let mut proof_bearing_by_file = BTreeMap::new();
    let mut proof_bearing_by_package = BTreeMap::new();
    let mut files_by_package: BTreeMap<String, Vec<&AnalyzedFile>> = BTreeMap::new();

    for file in files
        .iter()
        .filter(|file| matches!(file.facts.kind, TestFileKind::AssertionsModule))
    {
        let Some(package_name) = file.facts.assertions_package_name.as_ref() else {
            continue;
        };
        files_by_package
            .entry(package_name.clone())
            .or_default()
            .push(file);
    }

    for (package_name, package_files) in files_by_package {
        let mut proof_bearing_names = package_files
            .iter()
            .flat_map(|file| {
                let module_prefix = assertions_module_prefix(&file.facts.rel_path);
                file.parsed
                    .functions
                    .iter()
                    .filter(|function| {
                        function.is_public && !function.is_test && function.has_assertion_macro
                    })
                    .map(move |function| qualified_assertion_name(&module_prefix, &function.name))
            })
            .collect::<BTreeSet<_>>();

        loop {
            let mut changed = false;
            for file in &package_files {
                let candidates = file
                    .parsed
                    .functions
                    .iter()
                    .filter(|function| function.is_public && !function.is_test)
                    .collect::<Vec<_>>();
                for function in candidates {
                    let module_prefix = assertions_module_prefix(&file.facts.rel_path);
                    let qualified_name = qualified_assertion_name(&module_prefix, &function.name);
                    if proof_bearing_names.contains(&qualified_name) {
                        continue;
                    }
                    if exported_assertion_function_calls_proof(
                        function,
                        &file.parsed.imports,
                        &file.parsed.file_function_names,
                        &package_name,
                        &module_prefix,
                        &proof_bearing_names,
                    ) {
                        changed |= proof_bearing_names.insert(qualified_name);
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let _ = proof_bearing_by_package.insert(package_name, proof_bearing_names.clone());
        for file in package_files {
            let module_prefix = assertions_module_prefix(&file.facts.rel_path);
            let file_proofs = file
                .parsed
                .functions
                .iter()
                .filter(|function| {
                    function.is_public
                        && !function.is_test
                        && proof_bearing_names
                            .contains(&qualified_assertion_name(&module_prefix, &function.name))
                })
                .map(|function| function.name.clone())
                .collect();
            let _ = proof_bearing_by_file.insert(file.facts.rel_path.clone(), file_proofs);
        }
    }

    (proof_bearing_by_file, proof_bearing_by_package)
}

fn exported_assertion_function_calls_proof(
    function: &FunctionInfo,
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
    package_name: &str,
    module_prefix: &[String],
    proof_bearing_names: &BTreeSet<String>,
) -> bool {
    let mut root_prefixes = BTreeMap::from([
        ("crate".to_owned(), Vec::new()),
        ("self".to_owned(), module_prefix.to_vec()),
        ("super".to_owned(), parent_module_prefix(module_prefix)),
        (package_name.to_owned(), Vec::new()),
    ]);
    let mut bare_imported_proofs = BTreeMap::new();
    let mut glob_prefixes = Vec::new();

    for binding in imports {
        if !binding
            .path_segments
            .first()
            .is_some_and(|segment| root_prefixes.contains_key(segment))
        {
            continue;
        }
        let Some(first) = binding.path_segments.first() else {
            continue;
        };
        let Some(base_prefix) = root_prefixes.get(first).cloned() else {
            continue;
        };
        let relative_segments =
            normalize_relative_assertion_path(&binding.path_segments, &base_prefix);

        if let Some(local_name) = binding.local_name.as_ref() {
            let _ = root_prefixes.insert(local_name.clone(), relative_segments.clone());
            let _ = bare_imported_proofs.insert(local_name.clone(), relative_segments.join("::"));
        } else {
            glob_prefixes.push(relative_segments);
        }
    }

    function
        .call_paths
        .iter()
        .any(|path| match path.as_slice() {
            [name] => {
                !function.shadowed_idents.contains(name)
                    && !file_function_names.contains(name)
                    && (bare_imported_proofs
                        .get(name)
                        .is_some_and(|qualified| proof_bearing_names.contains(qualified))
                        || glob_prefixes.iter().any(|prefix| {
                            proof_bearing_names.contains(&qualified_assertion_name(prefix, name))
                        }))
            }
            [first, rest @ ..] => root_prefixes.get(first).is_some_and(|prefix| {
                proof_bearing_names.contains(&qualified_assertion_name(prefix, &rest.join("::")))
            }),
            _ => false,
        })
}

fn assertions_module_prefix(rel_path: &str) -> Vec<String> {
    let rel_after_src = rel_path
        .split_once("/src/")
        .map_or(rel_path, |(_, suffix)| suffix);
    let rel_without_ext = rel_after_src.strip_suffix(".rs").unwrap_or(rel_after_src);
    let mut segments = rel_without_ext
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.last().is_some_and(|segment| *segment == "lib") {
        let _ = segments.pop();
    } else if segments.last().is_some_and(|segment| *segment == "mod") {
        let _ = segments.pop();
    }
    segments.into_iter().map(str::to_owned).collect()
}

fn qualified_assertion_name(module_prefix: &[String], function_name: &str) -> String {
    if module_prefix.is_empty() {
        function_name.to_owned()
    } else {
        format!("{}::{function_name}", module_prefix.join("::"))
    }
}

fn parent_module_prefix(module_prefix: &[String]) -> Vec<String> {
    if module_prefix.is_empty() {
        Vec::new()
    } else {
        module_prefix[..module_prefix.len() - 1].to_vec()
    }
}

fn normalize_relative_assertion_path(
    path_segments: &[String],
    base_prefix: &[String],
) -> Vec<String> {
    let mut normalized = base_prefix.to_vec();
    let mut iter = path_segments.iter();
    match iter.next().map(String::as_str) {
        Some("crate") => normalized.clear(),
        Some("self") | Some("super") => {
            for segment in iter.by_ref() {
                match segment.as_str() {
                    "self" => {}
                    "super" => {
                        let _ = normalized.pop();
                    }
                    _ => {
                        normalized.push(segment.clone());
                        break;
                    }
                }
            }
        }
        Some(_) => {
            normalized.clear();
        }
        None => {}
    }
    normalized.extend(iter.cloned());
    normalized
}

fn is_test_support_file(root: &TestRootFacts, rel_path: &str) -> bool {
    let test_support_src = discover::join_under_root(&root.rel_dir, "test_support/src");
    rel_path == test_support_src || discover::path_is_under(rel_path, &test_support_src)
}
