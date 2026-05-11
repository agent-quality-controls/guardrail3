#![expect(
    clippy::arithmetic_side_effects,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::excessive_nesting,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::if_same_then_else,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::too_many_lines,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_types::ast::{FunctionInfo, UseBinding};
use g3rs_test_types::{
    G3RsTestAnalyzedSourceFile, G3RsTestFileKind, G3RsTestFileTreeInputFailure, G3RsTestSourceFile,
    G3RsTestSourceInputFailure,
};
/// `analyze_source_files` function.
pub(crate) fn analyze_source_files(
    files: Vec<G3RsTestSourceFile>,
) -> (
    Vec<G3RsTestAnalyzedSourceFile>,
    Vec<G3RsTestSourceInputFailure>,
) {
    let (analyzed_files, input_failures) = analyze_files(files, "source analysis");
    (
        analyzed_files,
        input_failures
            .into_iter()
            .map(|(rel_path, message)| G3RsTestSourceInputFailure { rel_path, message })
            .collect(),
    )
}

/// `analyze_file_tree_files` function.
pub(crate) fn analyze_file_tree_files(
    files: Vec<G3RsTestSourceFile>,
) -> (
    Vec<G3RsTestAnalyzedSourceFile>,
    Vec<G3RsTestFileTreeInputFailure>,
) {
    let (analyzed_files, input_failures) = analyze_files(files, "file-tree analysis");
    (
        analyzed_files,
        input_failures
            .into_iter()
            .map(|(rel_path, message)| G3RsTestFileTreeInputFailure { rel_path, message })
            .collect(),
    )
}

/// `file_activates_test_rules` function.
pub(crate) fn file_activates_test_rules(file: &G3RsTestAnalyzedSourceFile) -> bool {
    matches!(
        file.kind,
        G3RsTestFileKind::InternalSidecarMod
            | G3RsTestFileKind::InternalSidecarSupport
            | G3RsTestFileKind::ExternalHarness
    ) || !file.parsed.test_functions.is_empty()
        || !file.parsed.cfg_test_modules.is_empty()
}

/// `analyze_files` function.
fn analyze_files(
    files: Vec<G3RsTestSourceFile>,
    context: &str,
) -> (Vec<G3RsTestAnalyzedSourceFile>, Vec<(String, String)>) {
    let mut analyzed_files = Vec::new();
    let mut input_failures = Vec::new();

    for file in files {
        let source = match crate::parse::parse_rust_file(&file.content) {
            Ok(source) => source,
            Err(err) => {
                input_failures.push((
                    file.rel_path,
                    format!("Failed to parse Rust source file for test-family {context}: {err}"),
                ));
                continue;
            }
        };
        let parsed = crate::parse::analyze(&source, &file.content);
        analyzed_files.push(G3RsTestAnalyzedSourceFile {
            rel_path: file.rel_path,
            kind: file.kind,
            owner_module_name: file.owner_module_name,
            component_rel_dir: file.component_rel_dir,
            assertions_package_name: file.assertions_package_name,
            parsed,
            local_proof_helper_functions: BTreeSet::new(),
            proof_bearing_exported_functions: BTreeSet::new(),
            proof_bearing_assertion_functions: BTreeSet::new(),
        });
    }

    let (proof_bearing_by_file, proof_bearing_by_package) =
        collect_assertions_proof_catalog(&analyzed_files);
    for file in &mut analyzed_files {
        if let Some(file_proofs) = proof_bearing_by_file.get(&file.rel_path) {
            file.proof_bearing_exported_functions = file_proofs.clone();
        }
        if let Some(package_name) = &file.assertions_package_name {
            if let Some(package_proofs) = proof_bearing_by_package.get(package_name) {
                file.proof_bearing_assertion_functions = package_proofs.clone();
            }
        }
        file.local_proof_helper_functions =
            super::proof_helpers::collect_local_proof_helper_functions(file);
    }

    input_failures.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    input_failures.dedup();
    analyzed_files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));

    (analyzed_files, input_failures)
}
/// `collect_assertions_proof_catalog` function.
fn collect_assertions_proof_catalog(
    files: &[G3RsTestAnalyzedSourceFile],
) -> (
    BTreeMap<String, BTreeSet<String>>,
    BTreeMap<String, BTreeSet<String>>,
) {
    let mut proof_bearing_by_file = BTreeMap::new();
    let mut proof_bearing_by_package = BTreeMap::new();
    let mut files_by_package: BTreeMap<String, Vec<&G3RsTestAnalyzedSourceFile>> = BTreeMap::new();

    for file in files
        .iter()
        .filter(|file| matches!(file.kind, G3RsTestFileKind::AssertionsModule))
    {
        let Some(package_name) = file.assertions_package_name.as_ref() else {
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
                let module_prefix = assertions_module_prefix(&file.rel_path);
                let direct_module_prefix = module_prefix;
                file.parsed
                    .functions
                    .iter()
                    .filter(|function| {
                        function.is_public
                            && !function.is_test
                            && function.assertions.has_assertion_macro
                    })
                    .map(move |function| {
                        qualified_assertion_name(&direct_module_prefix, &function.name)
                    })
            })
            .collect::<BTreeSet<_>>();

        for file in &package_files {
            let module_prefix = assertions_module_prefix(&file.rel_path);
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
                    .filter(|function| !function.is_test && function.assertions.has_assertion_macro)
                    .map(|function| function.name.clone())
                    .collect::<BTreeSet<_>>();
                for function in file
                    .parsed
                    .functions
                    .iter()
                    .filter(|function| function.is_public && !function.is_test)
                {
                    let module_prefix = assertions_module_prefix(&file.rel_path);
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
                changed |= collect_public_assertion_reexports(
                    file,
                    &package_name,
                    &mut proof_bearing_names,
                );
            }
            if !changed {
                break;
            }
        }

        let _ = proof_bearing_by_package.insert(package_name, proof_bearing_names.clone());
        for file in package_files {
            let module_prefix = assertions_module_prefix(&file.rel_path);
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
            let _ = proof_bearing_by_file.insert(file.rel_path.clone(), file_proofs);
        }
    }

    (proof_bearing_by_file, proof_bearing_by_package)
}

/// `collect_public_assertion_reexports` function.
fn collect_public_assertion_reexports(
    file: &G3RsTestAnalyzedSourceFile,
    package_name: &str,
    proof_bearing_names: &mut BTreeSet<String>,
) -> bool {
    let module_prefix = assertions_module_prefix(&file.rel_path);
    let mut changed = false;
    for binding in file
        .parsed
        .imports
        .iter()
        .filter(|binding| binding.is_public)
    {
        let Some(local_name) = binding.local_name.as_ref() else {
            continue;
        };
        let reexported_path =
            public_reexport_target_path(&binding.path_segments, package_name, &module_prefix);
        if proof_bearing_names.contains(&reexported_path) {
            changed |=
                proof_bearing_names.insert(qualified_assertion_name(&module_prefix, local_name));
        }
    }
    changed
}

/// `public_reexport_target_path` function.
fn public_reexport_target_path(
    path_segments: &[String],
    package_name: &str,
    module_prefix: &[String],
) -> String {
    let package_root = package_name.replace('-', "_");
    let target_segments = match path_segments.first().map(String::as_str) {
        Some("crate") => path_segments[1..].to_vec(),
        Some("self") => {
            let mut segments = module_prefix.to_vec();
            segments.extend(path_segments[1..].iter().cloned());
            segments
        }
        Some("super") => {
            let mut segments = parent_module_prefix(module_prefix);
            segments.extend(path_segments[1..].iter().cloned());
            segments
        }
        Some(first) if first == package_name => path_segments[1..].to_vec(),
        Some(first) if first == package_root => path_segments[1..].to_vec(),
        Some(_) => path_segments.to_vec(),
        None => Vec::new(),
    };
    target_segments.join("::")
}

/// `exported_assertion_function_calls_proof` function.
fn exported_assertion_function_calls_proof(
    function: &FunctionInfo,
    imports: &[UseBinding],
    file_function_names: &BTreeSet<String>,
    package_name: &str,
    module_prefix: &[String],
    proof_bearing_names: &BTreeSet<String>,
    local_proof_functions: &BTreeSet<String>,
) -> bool {
    let package_root = package_name.replace('-', "_");
    let mut root_prefixes = BTreeMap::from([
        ("crate".to_owned(), Vec::new()),
        ("self".to_owned(), module_prefix.to_vec()),
        ("super".to_owned(), parent_module_prefix(module_prefix)),
        (package_name.to_owned(), Vec::new()),
        (package_root, Vec::new()),
    ]);
    let mut bare_imported_proofs = BTreeMap::new();
    let mut glob_prefixes = Vec::new();
    let mut external_assertions_aliases = BTreeSet::new();
    let mut external_assertions_glob = false;

    for binding in imports {
        if binding
            .path_segments
            .first()
            .is_some_and(|segment| segment.ends_with("_assertions"))
        {
            if let Some(local_name) = binding.local_name.as_ref() {
                let _ = external_assertions_aliases.insert(local_name.clone());
            } else if let Some(last) = binding.path_segments.last() {
                let _ = external_assertions_aliases.insert(last.clone());
            } else {
                external_assertions_glob = true;
            }
            continue;
        }
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
        .body
        .call_paths
        .iter()
        .any(|path| match path.as_slice() {
            [name] => {
                !function.body.shadowed_idents.contains(name)
                    && ((file_function_names.contains(name)
                        && local_proof_functions.contains(name))
                        || (file_function_names.contains(name)
                            && proof_bearing_names
                                .contains(&qualified_assertion_name(module_prefix, name)))
                        || external_assertions_aliases.contains(name)
                        || (!file_function_names.contains(name)
                            && (bare_imported_proofs
                                .get(name)
                                .is_some_and(|qualified| proof_bearing_names.contains(qualified))
                                || (external_assertions_glob
                                    && !function.body.shadowed_idents.contains(name))
                                || glob_prefixes.iter().any(|prefix| {
                                    proof_bearing_names
                                        .contains(&qualified_assertion_name(prefix, name))
                                }))))
            }
            [first, rest @ ..] => {
                (first.ends_with("_assertions") || external_assertions_aliases.contains(first))
                    || root_prefixes.get(first).is_some_and(|prefix| {
                        proof_bearing_names
                            .contains(&qualified_assertion_name(prefix, &rest.join("::")))
                    })
            }
            _ => false,
        })
}

/// `assertions_module_prefix` function.
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

/// `qualified_assertion_name` function.
pub(super) fn qualified_assertion_name(module_prefix: &[String], function_name: &str) -> String {
    if module_prefix.is_empty() {
        function_name.to_owned()
    } else {
        format!("{}::{function_name}", module_prefix.join("::"))
    }
}

/// `parent_module_prefix` function.
fn parent_module_prefix(module_prefix: &[String]) -> Vec<String> {
    if module_prefix.is_empty() {
        Vec::new()
    } else {
        module_prefix[..module_prefix.len() - 1].to_vec()
    }
}

/// `normalize_relative_assertion_path` function.
fn normalize_relative_assertion_path(
    path_segments: &[String],
    base_prefix: &[String],
) -> Vec<String> {
    let mut normalized = base_prefix.to_vec();
    let mut iter = path_segments.iter();
    match iter.next().map(String::as_str) {
        Some("crate") => normalized.clear(),
        Some("self" | "super") => {
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
