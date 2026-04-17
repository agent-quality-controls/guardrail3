use std::collections::BTreeSet;

use deny_toml_parser::types::{
    AdvisoriesConfig, AdvisoryIgnoreEntry, BanAllowEntry, BanBuildAllowBuildScriptEntry,
    BanBuildBypassAllowEntry, BanBuildBypassEntry, BanBuildConfig, BanFeatureEntry,
    BanSkipTreeEntry, BanWorkspaceDependenciesConfig, BansConfig, GraphConfig, GraphTargetEntry,
    LicenseClarification, LicenseClarificationFile, LicenseException, LicensesConfig,
    LicensesPrivateConfig, OutputConfig, SourcesAllowOrg, SourcesConfig,
};

pub(crate) fn known_top_level_keys() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "graph",
        "bans",
        "licenses",
        "exceptions",
        "advisories",
        "sources",
        "output",
    ])
}

pub(crate) fn feature_entry_unknown_keys(entry: &BanFeatureEntry) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("feature").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn graph_target_unknown_keys(entry: &GraphTargetEntry) -> Vec<String> {
    match entry {
        GraphTargetEntry::Simple(_) => Vec::new(),
        GraphTargetEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("graph-target").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

pub(crate) fn license_clarification_unknown_keys(entry: &LicenseClarification) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("clarify").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn license_clarification_file_unknown_keys(
    entry: &LicenseClarificationFile,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("clarify-file").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn license_exception_unknown_keys(entry: &LicenseException) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("exception").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn allow_entry_unknown_keys(entry: &BanAllowEntry) -> Vec<String> {
    match entry {
        BanAllowEntry::Simple(_) => Vec::new(),
        BanAllowEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("allow").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

pub(crate) fn advisory_ignore_unknown_keys(entry: &AdvisoryIgnoreEntry) -> Vec<String> {
    match entry {
        AdvisoryIgnoreEntry::Simple(_) => Vec::new(),
        AdvisoryIgnoreEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("ignore").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

pub(crate) fn private_unknown_keys(config: &LicensesPrivateConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("private").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn graph_unknown_keys(graph: &GraphConfig) -> Vec<String> {
    graph
        .extra
        .keys()
        .filter(|key| !known_section_keys("graph").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn advisories_unknown_keys(config: &AdvisoriesConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("advisories").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn bans_unknown_keys(config: &BansConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("bans").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn licenses_unknown_keys(config: &LicensesConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("licenses").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn sources_unknown_keys(config: &SourcesConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("sources").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn skip_tree_unknown_keys(entry: &BanSkipTreeEntry) -> Vec<String> {
    match entry {
        BanSkipTreeEntry::Simple(_) => Vec::new(),
        BanSkipTreeEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("skip-tree").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

pub(crate) fn workspace_dependencies_unknown_keys(
    config: &BanWorkspaceDependenciesConfig,
) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("workspace-dependencies").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn build_unknown_keys(config: &BanBuildConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("build").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn build_allow_build_script_unknown_keys(
    entry: &BanBuildAllowBuildScriptEntry,
) -> Vec<String> {
    match entry {
        BanBuildAllowBuildScriptEntry::Simple(_) => Vec::new(),
        BanBuildAllowBuildScriptEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("build-allow-build-scripts").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

pub(crate) fn build_bypass_unknown_keys(
    entry: &BanBuildBypassEntry,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("build-bypass").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn build_bypass_allow_unknown_keys(
    entry: &BanBuildBypassAllowEntry,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("build-bypass-allow").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn output_unknown_keys(config: &OutputConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("output").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn allow_org_unknown_keys(config: &SourcesAllowOrg) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("allow-org").contains(key.as_str()))
        .cloned()
        .collect()
}

fn known_section_keys(section: &str) -> BTreeSet<&'static str> {
    SECTION_KEYS
        .iter()
        .find(|(name, _)| *name == section)
        .map(|(_, keys)| keys.iter().copied().collect())
        .unwrap_or_default()
}

const GRAPH_KEYS: &[&str] = &[
    "all-features",
    "no-default-features",
    "targets",
    "exclude",
    "features",
    "exclude-dev",
    "exclude-unpublished",
];
const GRAPH_TARGET_KEYS: &[&str] = &["triple", "features"];
const BANS_KEYS: &[&str] = &[
    "multiple-versions",
    "multiple-versions-include-dev",
    "wildcards",
    "allow-wildcard-paths",
    "highlight",
    "workspace-default-features",
    "external-default-features",
    "allow-workspace",
    "deny",
    "skip",
    "allow",
    "skip-tree",
    "features",
    "workspace-dependencies",
    "build",
];
const ALLOW_KEYS: &[&str] = &["name", "crate", "version", "reason"];
const SKIP_TREE_KEYS: &[&str] = &["name", "crate", "version", "depth", "reason"];
const WORKSPACE_DEPENDENCIES_KEYS: &[&str] = &["duplicates", "include-path-dependencies", "unused"];
const BUILD_KEYS: &[&str] = &[
    "allow-build-scripts",
    "executables",
    "interpreted",
    "script-extensions",
    "enable-builtin-globs",
    "include-dependencies",
    "include-workspace",
    "include-archives",
    "bypass",
];
const BUILD_ALLOW_BUILD_SCRIPTS_KEYS: &[&str] = &["name", "crate", "version"];
const BUILD_BYPASS_KEYS: &[&str] = &[
    "name",
    "crate",
    "version",
    "build-script",
    "required-features",
    "allow-globs",
    "allow",
];
const BUILD_BYPASS_ALLOW_KEYS: &[&str] = &["path", "checksum"];
const LICENSES_KEYS: &[&str] = &[
    "version",
    "include-dev",
    "include-build",
    "unused-allowed-license",
    "unused-license-exception",
    "allow",
    "confidence-threshold",
    "private",
    "exceptions",
    "clarify",
];
const CLARIFY_KEYS: &[&str] = &["name", "crate", "version", "expression", "license-files"];
const CLARIFY_FILE_KEYS: &[&str] = &["path", "hash"];
const PRIVATE_KEYS: &[&str] = &["ignore", "registries", "ignore-sources"];
const EXCEPTION_KEYS: &[&str] = &["allow", "name", "crate", "version", "reason"];
const ADVISORIES_KEYS: &[&str] = &[
    "db-path",
    "db-urls",
    "unmaintained",
    "yanked",
    "unused-ignored-advisory",
    "version",
    "ignore",
    "vulnerability",
    "notice",
    "unsound",
];
const IGNORE_KEYS: &[&str] = &["id", "crate", "name", "version", "reason"];
const SOURCES_KEYS: &[&str] = &[
    "unknown-registry",
    "unknown-git",
    "required-git-spec",
    "allow-registry",
    "allow-git",
    "private",
    "unused-allowed-source",
    "allow-org",
];
const SKIP_KEYS: &[&str] = &["name", "crate", "version", "reason"];
const FEATURE_KEYS: &[&str] = &[
    "name", "crate", "version", "deny", "allow", "reason", "exact",
];
const ALLOW_ORG_KEYS: &[&str] = &["github", "gitlab", "bitbucket"];
const OUTPUT_KEYS: &[&str] = &["feature-depth"];
const SECTION_KEYS: &[(&str, &[&str])] = &[
    ("graph", GRAPH_KEYS),
    ("graph-target", GRAPH_TARGET_KEYS),
    ("bans", BANS_KEYS),
    ("allow", ALLOW_KEYS),
    ("skip-tree", SKIP_TREE_KEYS),
    ("workspace-dependencies", WORKSPACE_DEPENDENCIES_KEYS),
    ("build", BUILD_KEYS),
    ("build-allow-build-scripts", BUILD_ALLOW_BUILD_SCRIPTS_KEYS),
    ("build-bypass", BUILD_BYPASS_KEYS),
    ("build-bypass-allow", BUILD_BYPASS_ALLOW_KEYS),
    ("licenses", LICENSES_KEYS),
    ("clarify", CLARIFY_KEYS),
    ("clarify-file", CLARIFY_FILE_KEYS),
    ("private", PRIVATE_KEYS),
    ("exception", EXCEPTION_KEYS),
    ("advisories", ADVISORIES_KEYS),
    ("ignore", IGNORE_KEYS),
    ("sources", SOURCES_KEYS),
    ("skip", SKIP_KEYS),
    ("feature", FEATURE_KEYS),
    ("allow-org", ALLOW_ORG_KEYS),
    ("output", OUTPUT_KEYS),
];
