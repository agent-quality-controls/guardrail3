use g3ts_astro_i18n_types::G3TsAstroI18nPolicySurfaceState;

pub(crate) fn probe_targets(
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroI18nPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let mut targets = public_probe_targets(app_root_rel_path, astro_policy);
    targets.extend(helper_probe_targets(app_root_rel_path, astro_policy));
    dedupe_targets(targets)
}

pub(crate) fn public_probe_targets(
    app_root_rel_path: &str,
    policy: &G3TsAstroI18nPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } = policy else {
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

pub(crate) fn helper_probe_targets(
    app_root_rel_path: &str,
    policy: &G3TsAstroI18nPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } = policy else {
        return fallback_helper_probe_targets(app_root_rel_path);
    };

    let targets = snapshot
        .helper_source_globs
        .iter()
        .flat_map(|glob| probe_targets_from_glob(app_root_rel_path, glob))
        .collect::<Vec<_>>();
    if targets.is_empty() {
        fallback_helper_probe_targets(app_root_rel_path)
    } else {
        dedupe_targets(targets)
    }
}

fn fallback_public_probe_targets(
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        target(
            app_root_rel_path,
            "src/pages/__g3ts_i18n_probe__.astro",
            eslint_config_parser::types::EslintProbeKind::AstroSource,
        ),
        target(
            app_root_rel_path,
            "src/pages/__g3ts_i18n_probe__.ts",
            eslint_config_parser::types::EslintProbeKind::TsSource,
        ),
        target(
            app_root_rel_path,
            "src/pages/__g3ts_i18n_probe__.tsx",
            eslint_config_parser::types::EslintProbeKind::TsxSource,
        ),
    ]
}

fn fallback_helper_probe_targets(
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![target(
        app_root_rel_path,
        "src/i18n/__g3ts_i18n_helper_probe__.ts",
        eslint_config_parser::types::EslintProbeKind::TsSource,
    )]
}

fn probe_targets_from_glob(
    app_root_rel_path: &str,
    glob: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
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

fn extensions_from_glob(glob: &str) -> Vec<&'static str> {
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

fn probe_from_glob(glob: &str, extension: &str) -> Option<String> {
    let prefix = glob
        .split('*')
        .next()
        .map(str::trim)
        .filter(|prefix| !prefix.is_empty())?
        .trim_end_matches('/')
        .to_owned();

    Some(format!("{prefix}/__g3ts_i18n_probe__.{extension}"))
}

fn dedupe_targets(
    targets: Vec<eslint_config_parser::types::EslintProbeTarget>,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let mut seen = std::collections::BTreeSet::new();
    targets
        .into_iter()
        .filter(|target| seen.insert((target.probe, target.rel_path.clone())))
        .collect()
}
