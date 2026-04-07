use std::collections::BTreeSet;

use deny_toml_parser::{
    AdvisoriesConfig, AdvisoryIgnoreEntry, BanDenyDetail, BanDenyEntry, BanFeatureEntry,
    BanSkipDetail, BanSkipEntry, BansConfig, LicenseException, LicensesConfig,
    LicensesPrivateConfig, SourcesConfig,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_domain_modules::deny;

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

pub(crate) fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
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
        .map(str::to_owned)
        .or_else(|| detail.crate_name.as_deref().map(|crate_name| crate_name.split('@').next().unwrap_or(crate_name).to_owned()))
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
