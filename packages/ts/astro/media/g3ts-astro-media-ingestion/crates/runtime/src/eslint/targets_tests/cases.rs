use super::super::*;

#[test]
fn literal_file_path_probes_exact_file() {
    let targets = probe_targets_from_glob("app", "src/mdx-components.tsx");

    assert_eq!(targets.len(), 1);
    g3ts_astro_media_ingestion_assertions::eslint::targets::assert_target(
        &targets[0],
        "app/src/mdx-components.tsx",
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );
}

#[test]
fn literal_directory_path_probes_directory() {
    let targets = probe_targets_from_glob("app", "content");
    let rel_paths = targets
        .iter()
        .map(|target| target.rel_path.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        rel_paths,
        vec![
            "app/content/__g3ts_media_probe__.astro",
            "app/content/__g3ts_media_probe__.ts",
            "app/content/__g3ts_media_probe__.tsx",
            "app/content/__g3ts_media_probe__.mdx"
        ]
    );
}

#[test]
fn astro_ts_tsx_glob_probes_glob_directory() {
    let targets = probe_targets_from_glob("app", "src/pages/**/*.{astro,ts,tsx}");
    let rel_paths = targets
        .iter()
        .map(|target| target.rel_path.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        rel_paths,
        vec![
            "app/src/pages/__g3ts_media_probe__.astro",
            "app/src/pages/__g3ts_media_probe__.tsx",
            "app/src/pages/__g3ts_media_probe__.ts"
        ]
    );
}

#[test]
fn mdx_glob_probes_content_directory() {
    let targets = probe_targets_from_glob("app", "content/**/*.mdx");

    assert_eq!(targets.len(), 1);
    g3ts_astro_media_ingestion_assertions::eslint::targets::assert_target(
        &targets[0],
        "app/content/__g3ts_media_probe__.mdx",
        eslint_config_parser::types::EslintProbeKind::MdxContent,
    );
}

#[test]
fn file_like_prefix_glob_does_not_probe_under_file() {
    let targets = probe_targets_from_glob("", "src/mdx-components.tsx/**/*");

    assert!(targets.is_empty());
}

#[test]
fn file_like_prefix_with_explicit_extension_does_not_probe_under_file() {
    let targets = probe_targets_from_glob("", "src/mdx-components.tsx/**/*.tsx");

    assert!(targets.is_empty());
}

#[test]
fn duplicate_targets_are_deduped() {
    let duplicate = target(
        "",
        "src/pages/__g3ts_media_probe__.tsx",
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );
    let targets = dedupe_targets(vec![duplicate.clone(), duplicate]);

    assert_eq!(targets.len(), 1);
}
