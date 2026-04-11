use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_file_tree_checks_types::G3RsTestFileTreeChecksInput;
use g3rs_test_types::{
    G3RsTestFileKind, G3RsTestFileTreeInputFailure, G3RsTestSourceFile,
};

use crate::parse::{FunctionInfo, ParsedTestFile, UseBinding, analyze, parse_rust_file};

pub(crate) struct AnalyzedFile {
    pub(crate) file: G3RsTestSourceFile,
    pub(crate) parsed: ParsedTestFile,
}

#[derive(Default)]
pub(crate) struct RootAnalysis {
    pub(crate) files: Vec<AnalyzedFile>,
    pub(crate) has_tests: bool,
    pub(crate) input_failures: Vec<G3RsTestFileTreeInputFailure>,
    pub(crate) proof_bearing_assertions_by_file: BTreeMap<String, BTreeSet<String>>,
    pub(crate) proof_bearing_assertions_by_package: BTreeMap<String, BTreeSet<String>>,
    pub(crate) local_runtime_packages: BTreeSet<String>,
    pub(crate) local_assertions_packages: BTreeSet<String>,
}

pub(crate) fn analyze_root(input: &G3RsTestFileTreeChecksInput) -> RootAnalysis {
    let mut analysis = RootAnalysis::default();

    analysis.local_runtime_packages = input
        .components
        .iter()
        .filter_map(|component| component.runtime_package_name.clone())
        .collect();
    analysis.local_assertions_packages = input
        .components
        .iter()
        .filter_map(|component| component.assertions_package_name.clone())
        .collect();
    analysis.input_failures.extend(input.input_failures.clone());
    analysis.has_tests = input.components.iter().any(|component| {
        !component.sidecars.is_empty() || !component.external_harnesses.is_empty()
    });

    for file in &input.files {
        let source = match parse_rust_file(&file.content) {
            Ok(source) => source,
            Err(err) => {
                analysis.input_failures.push(G3RsTestFileTreeInputFailure {
                    rel_path: file.rel_path.clone(),
                    message: format!(
                        "Failed to parse Rust source file for test-family analysis: {err}"
                    ),
                });
                continue;
            }
        };
        let parsed = analyze(&source, &file.content);
        analysis.has_tests |= file_activates_test_rules(file, &parsed);
        analysis.files.push(AnalyzedFile {
            file: file.clone(),
            parsed,
        });
    }

    (
        analysis.proof_bearing_assertions_by_file,
        analysis.proof_bearing_assertions_by_package,
    ) = collect_assertions_proof_catalog(&analysis.files);

    analysis
}

pub(crate) struct TestSupportFileInput<'a> {
    pub(crate) file: &'a G3RsTestSourceFile,
    pub(crate) parsed: &'a ParsedTestFile,
    pub(crate) local_runtime_packages: &'a BTreeSet<String>,
    pub(crate) local_assertions_packages: &'a BTreeSet<String>,
}

impl<'a> TestSupportFileInput<'a> {
    pub(crate) const fn new(
        file: &'a G3RsTestSourceFile,
        parsed: &'a ParsedTestFile,
        local_runtime_packages: &'a BTreeSet<String>,
        local_assertions_packages: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            file,
            parsed,
            local_runtime_packages,
            local_assertions_packages,
        }
    }
}

fn file_activates_test_rules(file: &G3RsTestSourceFile, parsed: &ParsedTestFile) -> bool {
    matches!(
        file.kind,
        G3RsTestFileKind::InternalSidecarMod
            | G3RsTestFileKind::InternalSidecarSupport
            | G3RsTestFileKind::ExternalHarness
    ) || !parsed.test_functions.is_empty()
        || !parsed.cfg_test_modules.is_empty()
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
        .filter(|file| matches!(file.file.kind, G3RsTestFileKind::AssertionsModule))
    {
        let Some(package_name) = file.file.assertions_package_name.as_ref() else {
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
                let module_prefix = assertions_module_prefix(&file.file.rel_path);
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

                direct_functions
            })
            .collect::<BTreeSet<_>>();

        for file in &package_files {
            let module_prefix = assertions_module_prefix(&file.file.rel_path);
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
                    let module_prefix = assertions_module_prefix(&file.file.rel_path);
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
            let module_prefix = assertions_module_prefix(&file.file.rel_path);
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
            let _ = proof_bearing_by_file.insert(file.file.rel_path.clone(), file_proofs);
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
