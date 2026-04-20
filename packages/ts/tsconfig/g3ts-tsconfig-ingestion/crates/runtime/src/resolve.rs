use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use g3ts_tsconfig_types::G3TsTsconfigExtendsState;
use tsconfig_json_parser::types::{TsconfigCompilerOptions, TsconfigDocument};
use tsconfig_json_parser::{extends_entries, from_path_document, parse_error_reason, typed};

pub(crate) fn build_extends_chain(
    workspace_root: &Path,
    root_abs_path: &Path,
    document: &TsconfigDocument,
) -> (Vec<G3TsTsconfigExtendsState>, TsconfigCompilerOptions) {
    let mut seen = BTreeSet::new();
    let mut chain = Vec::new();
    let mut effective = TsconfigCompilerOptions::default();

    for specifier in extends_entries(document) {
        resolve_entry(
            workspace_root,
            root_abs_path,
            specifier,
            &mut seen,
            &mut chain,
            &mut effective,
        );
    }

    if let Some(root_typed) = typed(document) {
        merge_compiler_options(&mut effective, &root_typed.compiler_options);
    }

    (chain, effective)
}

fn resolve_entry(
    workspace_root: &Path,
    current_config_abs_path: &Path,
    specifier: &str,
    seen: &mut BTreeSet<PathBuf>,
    chain: &mut Vec<G3TsTsconfigExtendsState>,
    effective: &mut TsconfigCompilerOptions,
) {
    if !is_local_extends(specifier) {
        chain.push(G3TsTsconfigExtendsState::External {
            specifier: specifier.to_owned(),
        });
        return;
    }

    let Some(resolved_abs_path) = resolve_local_extends_path(current_config_abs_path, specifier)
    else {
        chain.push(G3TsTsconfigExtendsState::Missing {
            specifier: specifier.to_owned(),
            display_path: specifier.to_owned(),
        });
        return;
    };

    let display_path = display_path(workspace_root, &resolved_abs_path);

    if !seen.insert(resolved_abs_path.clone()) {
        chain.push(G3TsTsconfigExtendsState::ParseError {
            specifier: specifier.to_owned(),
            display_path,
            reason: "circular tsconfig extends chain detected".to_owned(),
        });
        return;
    }

    let document = match from_path_document(&resolved_abs_path) {
        Ok(document) => document,
        Err(err) => {
            chain.push(G3TsTsconfigExtendsState::ParseError {
                specifier: specifier.to_owned(),
                display_path,
                reason: err.to_string(),
            });
            return;
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        chain.push(G3TsTsconfigExtendsState::ParseError {
            specifier: specifier.to_owned(),
            display_path,
            reason: reason.to_owned(),
        });
        return;
    }

    for parent_specifier in extends_entries(&document) {
        resolve_entry(
            workspace_root,
            &resolved_abs_path,
            parent_specifier,
            seen,
            chain,
            effective,
        );
    }

    if let Some(parent_typed) = typed(&document) {
        merge_compiler_options(effective, &parent_typed.compiler_options);
    }

    chain.push(G3TsTsconfigExtendsState::Parsed {
        specifier: specifier.to_owned(),
        display_path,
        document,
    });
}

fn is_local_extends(specifier: &str) -> bool {
    specifier.starts_with("./")
        || specifier.starts_with("../")
        || Path::new(specifier).is_absolute()
}

fn resolve_local_extends_path(current_config_abs_path: &Path, specifier: &str) -> Option<PathBuf> {
    let base_dir = current_config_abs_path.parent()?;
    let joined = if Path::new(specifier).is_absolute() {
        PathBuf::from(specifier)
    } else {
        base_dir.join(specifier)
    };

    let candidates = if joined.extension().is_some() {
        vec![joined]
    } else {
        vec![
            joined.clone(),
            joined.with_extension("json"),
            joined.join("tsconfig.json"),
        ]
    };

    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn display_path(workspace_root: &Path, abs_path: &Path) -> String {
    match abs_path.strip_prefix(workspace_root) {
        Ok(rel_path) => rel_path.display().to_string(),
        Err(_) => abs_path.display().to_string(),
    }
}

fn merge_compiler_options(target: &mut TsconfigCompilerOptions, overlay: &TsconfigCompilerOptions) {
    merge_bool(&mut target.strict, overlay.strict);
    merge_bool(&mut target.no_implicit_returns, overlay.no_implicit_returns);
    merge_bool(&mut target.no_unused_locals, overlay.no_unused_locals);
    merge_bool(
        &mut target.no_unused_parameters,
        overlay.no_unused_parameters,
    );
    merge_bool(
        &mut target.no_unchecked_indexed_access,
        overlay.no_unchecked_indexed_access,
    );
    merge_bool(
        &mut target.exact_optional_property_types,
        overlay.exact_optional_property_types,
    );
    merge_bool(&mut target.isolated_modules, overlay.isolated_modules);
    merge_bool(
        &mut target.no_property_access_from_index_signature,
        overlay.no_property_access_from_index_signature,
    );
    merge_bool(
        &mut target.no_implicit_override,
        overlay.no_implicit_override,
    );
    merge_bool(
        &mut target.no_fallthrough_cases_in_switch,
        overlay.no_fallthrough_cases_in_switch,
    );
    merge_bool(
        &mut target.force_consistent_casing_in_file_names,
        overlay.force_consistent_casing_in_file_names,
    );
    merge_bool(
        &mut target.allow_unreachable_code,
        overlay.allow_unreachable_code,
    );
    merge_bool(&mut target.allow_unused_labels, overlay.allow_unused_labels);
}

fn merge_bool(target: &mut Option<bool>, overlay: Option<bool>) {
    if let Some(value) = overlay {
        *target = Some(value);
    }
}
