use std::collections::BTreeSet;

use deny_toml_parser::types as deny;

/// Implements `known top level keys`.
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

/// Implements `feature entry unknown keys`.
pub(crate) fn feature_entry_unknown_keys(entry: &deny::BanFeatureEntry) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("feature").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `graph target unknown keys`.
pub(crate) fn graph_target_unknown_keys(entry: &deny::GraphTargetEntry) -> Vec<String> {
    match entry {
        deny::GraphTargetEntry::Simple(_) => Vec::new(),
        deny::GraphTargetEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("graph-target").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

/// Implements `license clarification unknown keys`.
pub(crate) fn license_clarification_unknown_keys(
    entry: &deny::LicenseClarification,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("clarify").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `license clarification file unknown keys`.
pub(crate) fn license_clarification_file_unknown_keys(
    entry: &deny::LicenseClarificationFile,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("clarify-file").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `license exception unknown keys`.
pub(crate) fn license_exception_unknown_keys(entry: &deny::LicenseException) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("exception").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `allow entry unknown keys`.
pub(crate) fn allow_entry_unknown_keys(entry: &deny::BanAllowEntry) -> Vec<String> {
    match entry {
        deny::BanAllowEntry::Simple(_) => Vec::new(),
        deny::BanAllowEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("allow").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

/// Implements `advisory ignore unknown keys`.
pub(crate) fn advisory_ignore_unknown_keys(entry: &deny::AdvisoryIgnoreEntry) -> Vec<String> {
    match entry {
        deny::AdvisoryIgnoreEntry::Simple(_) => Vec::new(),
        deny::AdvisoryIgnoreEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("ignore").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

/// Implements `private unknown keys`.
pub(crate) fn private_unknown_keys(config: &deny::LicensesPrivateConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("private").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `graph unknown keys`.
pub(crate) fn graph_unknown_keys(graph: &deny::GraphConfig) -> Vec<String> {
    graph
        .extra
        .keys()
        .filter(|key| !known_section_keys("graph").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `advisories unknown keys`.
pub(crate) fn advisories_unknown_keys(config: &deny::AdvisoriesConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("advisories").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `bans unknown keys`.
pub(crate) fn bans_unknown_keys(config: &deny::BansConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("bans").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `licenses unknown keys`.
pub(crate) fn licenses_unknown_keys(config: &deny::LicensesConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("licenses").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `sources unknown keys`.
pub(crate) fn sources_unknown_keys(config: &deny::SourcesConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("sources").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `skip tree unknown keys`.
pub(crate) fn skip_tree_unknown_keys(entry: &deny::BanSkipTreeEntry) -> Vec<String> {
    match entry {
        deny::BanSkipTreeEntry::Simple(_) => Vec::new(),
        deny::BanSkipTreeEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("skip-tree").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

/// Implements `workspace dependencies unknown keys`.
pub(crate) fn workspace_dependencies_unknown_keys(
    config: &deny::BanWorkspaceDependenciesConfig,
) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("workspace-dependencies").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `build unknown keys`.
pub(crate) fn build_unknown_keys(config: &deny::BanBuildConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("build").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `build allow build script unknown keys`.
pub(crate) fn build_allow_build_script_unknown_keys(
    entry: &deny::BanBuildAllowBuildScriptEntry,
) -> Vec<String> {
    match entry {
        deny::BanBuildAllowBuildScriptEntry::Simple(_) => Vec::new(),
        deny::BanBuildAllowBuildScriptEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("build-allow-build-scripts").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

/// Implements `build bypass unknown keys`.
pub(crate) fn build_bypass_unknown_keys(entry: &deny::BanBuildBypassEntry) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("build-bypass").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `build bypass allow unknown keys`.
pub(crate) fn build_bypass_allow_unknown_keys(
    entry: &deny::BanBuildBypassAllowEntry,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("build-bypass-allow").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `output unknown keys`.
pub(crate) fn output_unknown_keys(config: &deny::OutputConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("output").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `allow org unknown keys`.
pub(crate) fn allow_org_unknown_keys(config: &deny::SourcesAllowOrg) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("allow-org").contains(key.as_str()))
        .cloned()
        .collect()
}

/// Implements `known section keys`.
fn known_section_keys(section: &str) -> BTreeSet<&'static str> {
    SECTION_KEYS
        .iter()
        .find(|(name, _)| *name == section)
        .map(|(_, keys)| keys.iter().copied().collect())
        .unwrap_or_default()
}

/// Constant value used by the surrounding module.
const GRAPH_KEYS: &[&str] = &[
    "all-features",
    "no-default-features",
    "targets",
    "exclude",
    "features",
    "exclude-dev",
    "exclude-unpublished",
];
/// Constant value used by the surrounding module.
const GRAPH_TARGET_KEYS: &[&str] = &["triple", "features"];
/// Constant value used by the surrounding module.
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
/// Constant value used by the surrounding module.
const ALLOW_KEYS: &[&str] = &["name", "crate", "version", "reason"];
/// Constant value used by the surrounding module.
const SKIP_TREE_KEYS: &[&str] = &["name", "crate", "version", "depth", "reason"];
/// Constant value used by the surrounding module.
const WORKSPACE_DEPENDENCIES_KEYS: &[&str] = &["duplicates", "include-path-dependencies", "unused"];
/// Constant value used by the surrounding module.
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
/// Constant value used by the surrounding module.
const BUILD_ALLOW_BUILD_SCRIPTS_KEYS: &[&str] = &["name", "crate", "version"];
/// Constant value used by the surrounding module.
const BUILD_BYPASS_KEYS: &[&str] = &[
    "name",
    "crate",
    "version",
    "build-script",
    "required-features",
    "allow-globs",
    "allow",
];
/// Constant value used by the surrounding module.
const BUILD_BYPASS_ALLOW_KEYS: &[&str] = &["path", "checksum"];
/// Constant value used by the surrounding module.
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
/// Constant value used by the surrounding module.
const CLARIFY_KEYS: &[&str] = &["name", "crate", "version", "expression", "license-files"];
/// Constant value used by the surrounding module.
const CLARIFY_FILE_KEYS: &[&str] = &["path", "hash"];
/// Constant value used by the surrounding module.
const PRIVATE_KEYS: &[&str] = &["ignore", "registries", "ignore-sources"];
/// Constant value used by the surrounding module.
const EXCEPTION_KEYS: &[&str] = &["allow", "name", "crate", "version", "reason"];
/// Constant value used by the surrounding module.
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
/// Constant value used by the surrounding module.
const IGNORE_KEYS: &[&str] = &["id", "crate", "name", "version", "reason"];
/// Constant value used by the surrounding module.
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
/// Constant value used by the surrounding module.
const SKIP_KEYS: &[&str] = &["name", "crate", "version", "reason"];
/// Constant value used by the surrounding module.
const FEATURE_KEYS: &[&str] = &[
    "name", "crate", "version", "deny", "allow", "reason", "exact",
];
/// Constant value used by the surrounding module.
const ALLOW_ORG_KEYS: &[&str] = &["github", "gitlab", "bitbucket"];
/// Constant value used by the surrounding module.
const OUTPUT_KEYS: &[&str] = &["feature-depth"];
/// Mapping from a deny.toml section name to its allowlist of recognized keys.
type SectionKeyAllowlist = (&'static str, &'static [&'static str]);

/// Constant value used by the surrounding module.
const SECTION_KEYS: &[SectionKeyAllowlist] = &[
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
