use std::collections::BTreeSet;

use deny_toml_parser::{
    AdvisoriesConfig, AdvisoryIgnoreEntry, BanDenyDetail, BanDenyEntry, BanFeatureEntry,
    BanSkipDetail, BanSkipEntry, BanSkipTreeEntry, BanWorkspaceDependenciesConfig, BansConfig,
    GraphTargetEntry, LicenseClarification, LicenseClarificationFile, LicenseException,
    LicensesConfig, LicensesPrivateConfig, SourcesConfig,
};
use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use g3rs_deny_types::G3RsDenyRustPolicyState;
use guardrail3_rs_toml_parser::RustProfile;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::baseline as deny;

pub(crate) fn error(id: &str, title: impl Into<String>, message: impl Into<String>, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn warn(id: &str, title: impl Into<String>, message: impl Into<String>, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn info(id: &str, title: impl Into<String>, message: impl Into<String>, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn inventory(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    info(id, title, message, file).into_inventory()
}

pub(crate) fn expected_advisory_baseline() -> (String, String) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_ADVISORIES.content()).ok();
    let advisories = parsed.as_ref().and_then(|value| value.get("advisories"));
    let unmaintained = advisories
        .and_then(|value| value.get("unmaintained"))
        .and_then(toml::Value::as_str)
        .unwrap_or("workspace");
    let yanked = advisories
        .and_then(|value| value.get("yanked"))
        .and_then(toml::Value::as_str)
        .unwrap_or("warn");
    (unmaintained.to_owned(), yanked.to_owned())
}

pub(crate) fn expected_graph() -> (bool, bool) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_GRAPH.content()).ok();
    let graph = parsed.as_ref().and_then(|value| value.get("graph"));
    let all_features = graph
        .and_then(|value| value.get("all-features"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true);
    let no_default_features = graph
        .and_then(|value| value.get("no-default-features"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    (all_features, no_default_features)
}

pub(crate) fn expected_bans_settings() -> (Option<String>, bool, Option<String>) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_BANS_BASE.content()).ok();
    let bans = parsed.as_ref().and_then(|value| value.get("bans"));
    let wildcards = bans
        .and_then(|value| value.get("wildcards"))
        .and_then(toml::Value::as_str);
    let allow_wildcard_paths = bans
        .and_then(|value| value.get("allow-wildcard-paths"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true);
    let highlight = bans
        .and_then(|value| value.get("highlight"))
        .and_then(toml::Value::as_str);
    (
        wildcards.map(str::to_owned),
        allow_wildcard_paths,
        highlight.map(str::to_owned),
    )
}

#[derive(Debug, Clone)]
pub(crate) struct BanExpectation {
    pub(crate) wrappers: BTreeSet<String>,
}

pub(crate) fn expected_bans(profile: Option<&str>) -> std::collections::BTreeMap<String, BanExpectation> {
    let modules = if profile == Some("library") {
        deny::library_profile_ban_entries()
    } else {
        deny::service_profile_ban_entries()
    };

    let mut map = std::collections::BTreeMap::new();
    for module in modules {
        for (name, wrappers) in parse_ban_entries(module.content()) {
            let _ = map.insert(name, BanExpectation { wrappers });
        }
    }
    map
}

pub(crate) fn expected_ban_names(profile: Option<&str>) -> BTreeSet<String> {
    expected_bans(profile).into_keys().collect()
}

pub(crate) fn expected_licenses() -> BTreeSet<String> {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_LICENSES.content()).ok();
    parsed
        .as_ref()
        .and_then(|value| value.get("licenses"))
        .and_then(|value| value.get("allow"))
        .and_then(toml::Value::as_array)
        .map(|licenses| {
            licenses
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) const fn expected_confidence_threshold() -> f64 {
    0.8
}

pub(crate) fn expected_sources() -> (BTreeSet<String>, String, String) {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_SOURCES.content()).ok();
    let sources = parsed.as_ref().and_then(|value| value.get("sources"));
    let registries = sources
        .and_then(|value| value.get("allow-registry"))
        .and_then(toml::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let unknown_registry = sources
        .and_then(|value| value.get("unknown-registry"))
        .and_then(toml::Value::as_str)
        .unwrap_or("deny");
    let unknown_git = sources
        .and_then(|value| value.get("unknown-git"))
        .and_then(toml::Value::as_str)
        .unwrap_or("deny");
    (
        registries,
        unknown_registry.to_owned(),
        unknown_git.to_owned(),
    )
}

pub(crate) fn expected_tokio_allowed_features() -> BTreeSet<String> {
    let parsed = toml::from_str::<toml::Value>(deny::DENY_FEATURE_BANS_TOKIO.content()).ok();
    parsed
        .as_ref()
        .and_then(|value| value.get("bans"))
        .and_then(|value| value.get("features"))
        .and_then(toml::Value::as_array)
        .and_then(|entries| entries.first())
        .map(|entry| string_set_from_value(entry.get("allow")))
        .unwrap_or_default()
}

pub(crate) fn rust_policy_valid(input: &G3RsDenyConfigChecksInput) -> bool {
    !matches!(
        input.rust_policy,
        G3RsDenyRustPolicyState::Unreadable { .. } | G3RsDenyRustPolicyState::ParseError { .. }
    )
}

pub(crate) fn managed_profile_name(input: &G3RsDenyConfigChecksInput) -> Option<&'static str> {
    match input.rust_policy {
        G3RsDenyRustPolicyState::Parsed {
            profile: Some(RustProfile::Library),
            ..
        } => Some("library"),
        _ => None,
    }
}

pub(crate) fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}

pub(crate) fn ban_name(entry: &BanDenyEntry) -> Option<String> {
    match entry {
        BanDenyEntry::Simple(name) => normalized_name(name),
        BanDenyEntry::Detailed(detail) => deny_detail_name(detail),
    }
}

pub(crate) fn allow_name(entry: &deny_toml_parser::BanAllowEntry) -> Option<String> {
    match entry {
        deny_toml_parser::BanAllowEntry::Simple(name) => normalized_name(name),
        deny_toml_parser::BanAllowEntry::Detailed(detail) => detail
            .name
            .as_deref()
            .and_then(normalized_name)
            .or_else(|| {
                detail
                    .crate_name
                    .as_deref()
                    .and_then(|crate_name| normalized_name(crate_name.split('@').next().unwrap_or(crate_name)))
            }),
    }
}

pub(crate) fn wrappers(entry: &BanDenyEntry) -> BTreeSet<String> {
    match entry {
        BanDenyEntry::Simple(_) => BTreeSet::new(),
        BanDenyEntry::Detailed(detail) => detail.wrappers.iter().cloned().collect(),
    }
}

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

pub(crate) fn known_section_keys(section: &str) -> BTreeSet<&'static str> {
    match section {
        "graph" => BTreeSet::from([
            "all-features",
            "no-default-features",
            "targets",
            "exclude",
            "features",
            "exclude-dev",
            "exclude-unpublished",
        ]),
        "graph-target" => BTreeSet::from(["triple", "features"]),
        "bans" => BTreeSet::from([
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
        ]),
        "allow" => BTreeSet::from(["name", "crate", "version", "reason"]),
        "skip-tree" => BTreeSet::from(["name", "crate", "version", "depth", "reason"]),
        "workspace-dependencies" => {
            BTreeSet::from(["duplicates", "include-path-dependencies", "unused"])
        }
        "build" => BTreeSet::from([
            "allow-build-scripts",
            "executables",
            "interpreted",
            "script-extensions",
            "enable-builtin-globs",
            "include-dependencies",
            "include-workspace",
            "include-archives",
            "bypass",
        ]),
        "build-allow-build-scripts" => BTreeSet::from(["name", "crate", "version"]),
        "build-bypass" => BTreeSet::from([
            "name",
            "crate",
            "version",
            "build-script",
            "required-features",
            "allow-globs",
            "allow",
        ]),
        "build-bypass-allow" => BTreeSet::from(["path", "checksum"]),
        "licenses" => BTreeSet::from([
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
        ]),
        "clarify" => BTreeSet::from(["name", "crate", "version", "expression", "license-files"]),
        "clarify-file" => BTreeSet::from(["path", "hash"]),
        "advisories" => BTreeSet::from([
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
        ]),
        "sources" => BTreeSet::from([
            "unknown-registry",
            "unknown-git",
            "required-git-spec",
            "allow-registry",
            "allow-git",
            "private",
            "unused-allowed-source",
            "allow-org",
        ]),
        "private" => BTreeSet::from(["ignore", "registries", "ignore-sources"]),
        "exception" => BTreeSet::from(["allow", "name", "crate", "version", "reason"]),
        "skip" => BTreeSet::from(["name", "crate", "version", "reason"]),
        "ignore" => BTreeSet::from(["id", "crate", "name", "version", "reason"]),
        "feature" => BTreeSet::from(["name", "crate", "version", "deny", "allow", "reason", "exact"]),
        "allow-org" => BTreeSet::from(["github", "gitlab", "bitbucket"]),
        "output" => BTreeSet::from(["feature-depth"]),
        _ => BTreeSet::new(),
    }
}

pub(crate) fn deny_entry_name(entry: &BanDenyEntry) -> Option<String> {
    match entry {
        BanDenyEntry::Simple(name) => Some(name.clone()),
        BanDenyEntry::Detailed(detail) => deny_detail_name(detail),
    }
}

pub(crate) fn skip_entry_name(entry: &BanSkipEntry) -> Option<String> {
    match entry {
        BanSkipEntry::Simple(name) => Some(name.clone()),
        BanSkipEntry::Detailed(detail) => skip_detail_name(detail),
    }
}

pub(crate) fn skip_entry_reason(entry: &BanSkipEntry) -> Option<&str> {
    match entry {
        BanSkipEntry::Simple(_) => None,
        BanSkipEntry::Detailed(detail) => detail.reason.as_deref(),
    }
}

pub(crate) fn normalized_skip_identity(entry: &BanSkipEntry) -> Option<String> {
    match entry {
        BanSkipEntry::Simple(name) => {
            let name = name.trim();
            (!name.is_empty()).then(|| name.to_owned())
        }
        BanSkipEntry::Detailed(detail) => {
            let Some(name) = skip_detail_name(detail) else {
                return None;
            };
            let version = skip_detail_version(detail).map(str::trim).filter(|value| !value.is_empty());
            Some(match version {
                Some(version) => format!("{name}@{version}"),
                None => name,
            })
        }
    }
}

pub(crate) fn feature_entry_name(entry: &BanFeatureEntry) -> Option<String> {
    entry
        .name
        .as_deref()
        .map(str::to_owned)
        .or_else(|| {
            entry
                .crate_name
                .as_deref()
                .map(|crate_name| crate_name.split('@').next().unwrap_or(crate_name).to_owned())
        })
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

pub(crate) fn license_exception_name(entry: &LicenseException) -> Option<String> {
    entry
        .name
        .as_deref()
        .map(str::to_owned)
        .or_else(|| entry.crate_name.as_deref().map(str::to_owned))
}

pub(crate) fn license_exception_unknown_keys(entry: &LicenseException) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("exception").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn allow_entry_unknown_keys(entry: &deny_toml_parser::BanAllowEntry) -> Vec<String> {
    match entry {
        deny_toml_parser::BanAllowEntry::Simple(_) => Vec::new(),
        deny_toml_parser::BanAllowEntry::Detailed(detail) => detail
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

pub(crate) fn advisory_ignore_identity(entry: &AdvisoryIgnoreEntry) -> Option<String> {
    match entry {
        AdvisoryIgnoreEntry::Simple(id) => {
            let id = id.trim();
            (!id.is_empty()).then(|| id.to_owned())
        }
        AdvisoryIgnoreEntry::Detailed(detail) => {
            if let Some(id) = detail.id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
                return Some(id.to_owned());
            }
            if let Some(crate_name) = detail
                .crate_name
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                return Some(crate_name.to_owned());
            }
            let name = detail
                .name
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())?;
            let version = detail
                .version
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty());
            Some(match version {
                Some(version) => format!("{name}@{version}"),
                None => name.to_owned(),
            })
        }
    }
}

pub(crate) fn advisory_ignore_reason(entry: &AdvisoryIgnoreEntry) -> Option<&str> {
    match entry {
        AdvisoryIgnoreEntry::Simple(_) => None,
        AdvisoryIgnoreEntry::Detailed(detail) => detail.reason.as_deref(),
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

pub(crate) fn graph_unknown_keys(graph: &deny_toml_parser::GraphConfig) -> Vec<String> {
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

pub(crate) fn build_unknown_keys(config: &deny_toml_parser::BanBuildConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("build").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn build_allow_build_script_unknown_keys(
    entry: &deny_toml_parser::BanBuildAllowBuildScriptEntry,
) -> Vec<String> {
    match entry {
        deny_toml_parser::BanBuildAllowBuildScriptEntry::Simple(_) => Vec::new(),
        deny_toml_parser::BanBuildAllowBuildScriptEntry::Detailed(detail) => detail
            .extra
            .keys()
            .filter(|key| !known_section_keys("build-allow-build-scripts").contains(key.as_str()))
            .cloned()
            .collect(),
    }
}

pub(crate) fn build_bypass_unknown_keys(entry: &deny_toml_parser::BanBuildBypassEntry) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("build-bypass").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn build_bypass_allow_unknown_keys(
    entry: &deny_toml_parser::BanBuildBypassAllowEntry,
) -> Vec<String> {
    entry
        .extra
        .keys()
        .filter(|key| !known_section_keys("build-bypass-allow").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn output_unknown_keys(config: &deny_toml_parser::OutputConfig) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("output").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn allow_org_unknown_keys(config: &deny_toml_parser::SourcesAllowOrg) -> Vec<String> {
    config
        .extra
        .keys()
        .filter(|key| !known_section_keys("allow-org").contains(key.as_str()))
        .cloned()
        .collect()
}

pub(crate) fn string_set_from_value(value: Option<&toml::Value>) -> BTreeSet<String> {
    value
        .and_then(toml::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn deny_detail_name(detail: &BanDenyDetail) -> Option<String> {
    detail
        .name
        .as_deref()
        .and_then(normalized_name)
        .or_else(|| {
            detail
                .crate_name
                .as_deref()
                .and_then(|crate_name| normalized_name(crate_name.split('@').next().unwrap_or(crate_name)))
        })
}

fn normalized_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

fn parse_ban_entries(content: &str) -> Vec<(String, BTreeSet<String>)> {
    let wrapped = format!("bans = {{ deny = [{content}] }}");
    let Ok(parsed) = toml::from_str::<toml::Value>(&wrapped) else {
        return Vec::new();
    };
    parsed
        .get("bans")
        .and_then(|value| value.get("deny"))
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let name = entry.get("name").and_then(toml::Value::as_str)?;
                    Some((name.to_owned(), string_set_from_value(entry.get("wrappers"))))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn skip_detail_name(detail: &BanSkipDetail) -> Option<String> {
    detail
        .name
        .as_deref()
        .map(str::to_owned)
        .or_else(|| detail.crate_name.as_deref().map(|crate_name| crate_name.split('@').next().unwrap_or(crate_name).to_owned()))
}

fn skip_detail_version(detail: &BanSkipDetail) -> Option<&str> {
    detail.version.as_deref()
}
