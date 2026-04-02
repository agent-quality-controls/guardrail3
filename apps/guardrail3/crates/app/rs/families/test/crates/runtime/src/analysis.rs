use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use crate::discover;
use crate::facts::{DiscoveredTestFile, InputFailureFacts, TestFacts, TestFileKind, TestRootFacts};
use crate::parse::{FunctionInfo, ParsedTestFile, UseBinding, parse_rust_file};

pub(crate) struct AnalyzedFile {
    pub(crate) facts: DiscoveredTestFile,
    pub(crate) parsed: ParsedTestFile,
}

#[derive(Default)]
pub(crate) struct RootAnalysis {
    pub(crate) files: Vec<AnalyzedFile>,
    pub(crate) has_tests: bool,
    pub(crate) has_tokio_tests: bool,
    pub(crate) input_failures: Vec<InputFailureFacts>,
    pub(crate) proof_bearing_assertions_by_file: BTreeMap<String, BTreeSet<String>>,
    pub(crate) proof_bearing_assertions_by_package: BTreeMap<String, BTreeSet<String>>,
    pub(crate) local_runtime_packages: BTreeSet<String>,
    pub(crate) local_assertions_packages: BTreeSet<String>,
}

pub(crate) fn analyze_root(
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
        let Some(abs) = tree.abs_path(&file.rel_path) else { continue };
        let content = match guardrail3_shared_fs::read_file_err(&abs) {
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

        let parsed = crate::parse::analyze(&ast, &content);
        if scoped_files.is_some_and(|paths| !paths.contains(&file.rel_path)) {
            continue;
        }
        analysis.has_tests |= file_activates_test_rules(root, file, &parsed);
        analysis.has_tokio_tests |= parsed
            .test_functions
            .iter()
            .any(|function| function.uses_tokio_test_attr);
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

fn file_activates_test_rules(
    root: &TestRootFacts,
    file: &DiscoveredTestFile,
    parsed: &ParsedTestFile,
) -> bool {
    matches!(
        file.kind,
        TestFileKind::InternalSidecarMod
            | TestFileKind::InternalSidecarSupport
            | TestFileKind::ExternalHarness
            | TestFileKind::AssertionsModule
    ) || is_test_support_file(root, &file.rel_path)
        || !parsed.test_functions.is_empty()
        || !parsed.cfg_test_modules.is_empty()
}

pub(crate) fn active_failures_for_root<'a>(
    facts: &'a TestFacts,
    root: &'a TestRootFacts,
    analysis: &'a RootAnalysis,
    mutation_active: bool,
) -> Vec<&'a InputFailureFacts> {
    let async_active =
        analysis.has_tests && (root.tokio_dependency_present || analysis.has_tokio_tests);
    let mut seen_paths = BTreeSet::new();

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
        .filter(|failure| seen_paths.insert(failure.rel_path.clone()))
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
                let direct_module_prefix = module_prefix.clone();
                let direct_functions = file
                    .parsed
                    .functions
                    .iter()
                    .filter(|function| {
                        function.is_public && !function.is_test && function.has_assertion_macro
                    })
                    .map(move |function| {
                        qualified_assertion_name(&direct_module_prefix, &function.name)
                    });
                let _ = module_prefix;

                direct_functions
            })
            .collect::<BTreeSet<_>>();

        for file in &package_files {
            let module_prefix = assertions_module_prefix(&file.facts.rel_path);
            for function_name in &file.parsed.macro_defined_proof_functions {
                let _ = proof_bearing_names
                    .insert(qualified_assertion_name(&module_prefix, function_name));
            }
        }

        loop {
            let mut changed = false;
            for file in &package_files {
                let local_proof_functions = file
                    .parsed
                    .functions
                    .iter()
                    .filter(|function| !function.is_test && function.has_assertion_macro)
                    .map(|function| function.name.clone())
                    .collect::<BTreeSet<_>>();
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
                        &local_proof_functions,
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
    local_proof_functions: &BTreeSet<String>,
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
                    && ((file_function_names.contains(name)
                        && local_proof_functions.contains(name))
                        || (file_function_names.contains(name)
                            && proof_bearing_names
                                .contains(&qualified_assertion_name(module_prefix, name)))
                        || (!file_function_names.contains(name)
                            && (bare_imported_proofs
                                .get(name)
                                .is_some_and(|qualified| proof_bearing_names.contains(qualified))
                                || glob_prefixes.iter().any(|prefix| {
                                    proof_bearing_names
                                        .contains(&qualified_assertion_name(prefix, name))
                                }))))
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

pub(crate) fn is_test_support_file(root: &TestRootFacts, rel_path: &str) -> bool {
    [
        discover::join_under_root(&root.rel_dir, "test_support/src"),
        discover::join_under_root(&root.rel_dir, "crates/test_support/src"),
    ]
    .into_iter()
    .any(|test_support_src| {
        rel_path == test_support_src || discover::path_is_under(rel_path, &test_support_src)
    })
}
