use g3ts_astro_media_types::G3TsAstroMediaPolicySurfaceState;

#[cfg(test)]
#[path = "targets_tests/mod.rs"]
// reason: keep private target-generation tests in the owned sidecar directory.
mod targets_tests;

/// `probe_targets`: probe targets.
pub(crate) fn probe_targets(
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMediaPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    public_probe_targets(app_root_rel_path, astro_policy)
}

/// `public_probe_targets`: public probe targets.
pub(crate) fn public_probe_targets(
    app_root_rel_path: &str,
    policy: &G3TsAstroMediaPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } = policy else {
        return fallback_public_probe_targets(app_root_rel_path);
    };

    let targets = snapshot
        .public_source_globs
        .iter()
        .flat_map(|glob| probe_targets_from_glob(app_root_rel_path, glob))
        .collect::<Vec<_>>();
    if targets.is_empty() {
        fallback_public_probe_targets(app_root_rel_path)
    } else {
        dedupe_targets(targets)
    }
}

/// `fallback_public_probe_targets`: fallback public probe targets.
fn fallback_public_probe_targets(
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        target(
            app_root_rel_path,
            "src/pages/__g3ts_media_probe__.astro",
            eslint_config_parser::types::EslintProbeKind::AstroSource,
        ),
        target(
            app_root_rel_path,
            "src/pages/__g3ts_media_probe__.ts",
            eslint_config_parser::types::EslintProbeKind::TsSource,
        ),
        target(
            app_root_rel_path,
            "src/pages/__g3ts_media_probe__.tsx",
            eslint_config_parser::types::EslintProbeKind::TsxSource,
        ),
    ]
}

/// `probe_targets_from_glob`: probe targets from glob.
fn probe_targets_from_glob(
    app_root_rel_path: &str,
    glob: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    if !has_glob_meta(glob) {
        return literal_probe_targets(app_root_rel_path, glob);
    }

    extensions_from_glob(glob)
        .into_iter()
        .filter_map(|extension| {
            let kind = probe_kind_for_extension(extension)?;
            Some(target(
                app_root_rel_path,
                &probe_from_glob(glob, extension)?,
                kind,
            ))
        })
        .collect()
}

/// `literal_probe_targets`: literal probe targets.
fn literal_probe_targets(
    app_root_rel_path: &str,
    rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    if let Some(extension) = source_file_extension(rel_path) {
        let Some(kind) = probe_kind_for_extension(extension) else {
            return Vec::new();
        };

        return vec![target(app_root_rel_path, rel_path, kind)];
    }

    ["astro", "ts", "tsx", "mdx"]
        .into_iter()
        .filter_map(|extension| {
            Some(target(
                app_root_rel_path,
                &format!("{rel_path}/__g3ts_media_probe__.{extension}"),
                probe_kind_for_extension(extension)?,
            ))
        })
        .collect()
}

/// `has_glob_meta`: has glob meta.
fn has_glob_meta(glob: &str) -> bool {
    glob.chars()
        .any(|character| matches!(character, '*' | '?' | '[' | ']' | '{' | '}'))
}

/// `extensions_from_glob`: extensions from glob.
fn extensions_from_glob(glob: &str) -> Vec<&'static str> {
    let glob = glob_after_first_meta(glob);
    let mut extensions = Vec::new();
    if glob.contains(".astro") || glob.contains("{astro") || glob.contains(",astro") {
        extensions.push("astro");
    }
    if glob.contains(".mdx") || glob.contains("{mdx") || glob.contains(",mdx") {
        extensions.push("mdx");
    }
    if glob.contains(".tsx") || glob.contains("{tsx") || glob.contains(",tsx") {
        extensions.push("tsx");
    }
    if glob.contains(".ts") && !glob.contains(".tsx")
        || glob.contains("{ts,")
        || glob.contains("{ts}")
        || glob.contains(",ts,")
        || glob.contains(",ts}")
    {
        extensions.push("ts");
    }
    extensions
}

/// `glob_after_first_meta`: glob after first meta.
fn glob_after_first_meta(glob: &str) -> &str {
    glob.find(['*', '?', '[', '{'])
        .and_then(|index| glob.get(index..))
        .unwrap_or(glob)
}

/// `probe_kind_for_extension`: probe kind for extension.
fn probe_kind_for_extension(
    extension: &str,
) -> Option<eslint_config_parser::types::EslintProbeKind> {
    match extension {
        "astro" => Some(eslint_config_parser::types::EslintProbeKind::AstroSource),
        "mdx" => Some(eslint_config_parser::types::EslintProbeKind::MdxContent),
        "ts" => Some(eslint_config_parser::types::EslintProbeKind::TsSource),
        "tsx" => Some(eslint_config_parser::types::EslintProbeKind::TsxSource),
        _ => None,
    }
}

/// `target`: target.
fn target(
    app_root_rel_path: &str,
    local_rel_path: &str,
    probe: eslint_config_parser::types::EslintProbeKind,
) -> eslint_config_parser::types::EslintProbeTarget {
    eslint_config_parser::types::EslintProbeTarget {
        probe,
        rel_path: g3ts_astro_check_support::surfaces::scoped_rel_path(
            app_root_rel_path,
            local_rel_path,
        ),
    }
}

/// `probe_from_glob`: probe from glob.
fn probe_from_glob(glob: &str, extension: &str) -> Option<String> {
    let prefix = glob_prefix(glob)?;
    let prefix = glob_prefix_directory(prefix, glob_after_first_meta(glob))?;

    Some(format!("{prefix}/__g3ts_media_probe__.{extension}"))
}

/// `glob_prefix`: glob prefix.
fn glob_prefix(glob: &str) -> Option<&str> {
    glob.split(['*', '?', '[', '{'])
        .next()
        .map(str::trim)
        .filter(|prefix| !prefix.is_empty())
}

/// `glob_prefix_directory`: glob prefix directory.
fn glob_prefix_directory(prefix: &str, glob_suffix: &str) -> Option<String> {
    let had_trailing_slash = prefix.ends_with('/');
    let prefix = prefix.trim_end_matches('/');
    if source_file_extension(prefix).is_some() && suffix_declares_extension(glob_suffix) {
        return None;
    }
    if had_trailing_slash {
        return Some(prefix.to_owned());
    }
    if source_file_extension(prefix).is_some() {
        if suffix_declares_extension(glob_suffix) {
            return None;
        }

        return std::path::Path::new(prefix)
            .parent()
            .and_then(std::path::Path::to_str)
            .filter(|parent| !parent.is_empty())
            .map(str::to_owned);
    }

    std::path::Path::new(prefix)
        .parent()
        .and_then(std::path::Path::to_str)
        .filter(|parent| !parent.is_empty())
        .map(str::to_owned)
        .or_else(|| Some(prefix.to_owned()))
}

/// `suffix_declares_extension`: suffix declares extension.
fn suffix_declares_extension(glob_suffix: &str) -> bool {
    [".astro", ".ts", ".tsx", ".mdx"]
        .into_iter()
        .any(|extension| glob_suffix.contains(extension))
}

/// `source_file_extension`: source file extension.
fn source_file_extension(rel_path: &str) -> Option<&str> {
    let extension = rel_path.rsplit_once('.').map(|(_, extension)| extension)?;
    probe_kind_for_extension(extension).map(|_| extension)
}

/// `dedupe_targets`: dedupe targets.
fn dedupe_targets(
    targets: Vec<eslint_config_parser::types::EslintProbeTarget>,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let mut seen = std::collections::BTreeSet::new();
    targets
        .into_iter()
        .filter(|target| seen.insert((target.probe, target.rel_path.clone())))
        .collect()
}
