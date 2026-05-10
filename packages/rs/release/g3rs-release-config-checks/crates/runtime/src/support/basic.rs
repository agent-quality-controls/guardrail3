use std::collections::BTreeSet;

use cargo_toml_parser::types::{
    InheritableValue, IntegerOrString, PackageSection, ProfileConfig, StringOrBool, VecStringOrBool,
};
use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigEdge, G3RsReleaseConfigRepo};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use semver::{Version, VersionReq};

/// Optional inheritable list of strings (e.g. `keywords`, `categories`).
type InheritableStringList<'a> = Option<&'a InheritableValue<Vec<String>>>;

/// TOML map value used for nested metadata tables.
type TomlMap = toml::map::Map<String, toml::Value>;

/// `error` function.
pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

/// `warn` function.
pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

/// `info` function.
pub(crate) fn info(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

/// `message_covers_prefix` function.
pub(crate) fn message_covers_prefix(message: &str, prefix: &str) -> bool {
    if message == prefix {
        return true;
    }

    let Some(prefix_body) = prefix.strip_prefix('^') else {
        return false;
    };
    let Some(message_body) = message.strip_prefix('^') else {
        return false;
    };

    if let Some(stripped) = message_body.strip_prefix(prefix_body) {
        return has_valid_commit_suffix(stripped);
    }

    let Some(grouped) = message_body.strip_prefix('(') else {
        return false;
    };
    let Some(group_end) = grouped.find(')') else {
        return false;
    };
    let Some(heads) = grouped.get(..group_end) else {
        return false;
    };
    let Some(suffix) = grouped.get(group_end.saturating_add(1)..) else {
        return false;
    };

    heads.split('|').any(|head| head == prefix_body) && has_valid_commit_suffix(suffix)
}

/// `crate_publish_declared` function.
pub(crate) fn crate_publish_declared(krate: &G3RsReleaseConfigCrate) -> bool {
    crate_package(krate)
        .and_then(|package| package.publish.as_ref())
        .is_some()
}

/// `crate_publishable` function.
pub(crate) fn crate_publishable(krate: &G3RsReleaseConfigCrate) -> bool {
    let Some(package) = crate_package(krate) else {
        return false;
    };

    match package.publish.as_ref() {
        None | Some(InheritableValue::Value(VecStringOrBool::Bool(false))) => false,
        Some(InheritableValue::Value(VecStringOrBool::VecString(values))) => !values.is_empty(),
        Some(InheritableValue::Value(VecStringOrBool::Bool(true))) => true,
        Some(InheritableValue::Inherit(_)) => {
            match krate
                .workspace_package
                .as_ref()
                .and_then(|workspace| workspace.publish.as_ref())
            {
                None | Some(VecStringOrBool::Bool(false)) => false,
                Some(VecStringOrBool::VecString(values)) => !values.is_empty(),
                Some(VecStringOrBool::Bool(true)) => true,
            }
        }
    }
}

/// `crate_description_present` function.
pub(crate) fn crate_description_present(krate: &G3RsReleaseConfigCrate) -> bool {
    inherited_string_present(
        crate_package(krate).and_then(|package| package.description.as_ref()),
        krate
            .workspace_package
            .as_ref()
            .and_then(|workspace| workspace.description.as_deref()),
    )
}

/// `crate_license_present` function.
pub(crate) fn crate_license_present(krate: &G3RsReleaseConfigCrate) -> bool {
    inherited_string_present(
        crate_package(krate).and_then(|package| package.license.as_ref()),
        krate
            .workspace_package
            .as_ref()
            .and_then(|workspace| workspace.license.as_deref()),
    ) || inherited_string_present(
        crate_package(krate).and_then(|package| package.license_file.as_ref()),
        krate
            .workspace_package
            .as_ref()
            .and_then(|workspace| workspace.license_file.as_deref()),
    )
}

/// `crate_repository_present` function.
pub(crate) fn crate_repository_present(krate: &G3RsReleaseConfigCrate) -> bool {
    inherited_string_present(
        crate_package(krate).and_then(|package| package.repository.as_ref()),
        krate
            .workspace_package
            .as_ref()
            .and_then(|workspace| workspace.repository.as_deref()),
    )
}

/// `crate_keywords_count` function.
pub(crate) fn crate_keywords_count(krate: &G3RsReleaseConfigCrate) -> Option<usize> {
    inherited_vec_count(
        crate_package(krate).and_then(|package| package.keywords.as_ref()),
        krate
            .workspace_package
            .as_ref()
            .map(|workspace| workspace.keywords.as_slice()),
    )
}

/// `crate_categories_count` function.
pub(crate) fn crate_categories_count(krate: &G3RsReleaseConfigCrate) -> Option<usize> {
    inherited_vec_count(
        crate_package(krate).and_then(|package| package.categories.as_ref()),
        krate
            .workspace_package
            .as_ref()
            .map(|workspace| workspace.categories.as_slice()),
    )
}

/// `crate_version_string` function.
pub(crate) fn crate_version_string(krate: &G3RsReleaseConfigCrate) -> Option<String> {
    match crate_package(krate).and_then(|package| package.version.as_ref()) {
        Some(InheritableValue::Value(value)) => Some(value.clone()),
        Some(InheritableValue::Inherit(_)) => krate
            .workspace_package
            .as_ref()
            .and_then(|workspace| workspace.version.clone()),
        None => None,
    }
}

/// `crate_version_valid` function.
pub(crate) fn crate_version_valid(krate: &G3RsReleaseConfigCrate) -> bool {
    crate_version_string(krate)
        .as_deref()
        .is_some_and(|version| Version::parse(version).is_ok())
}

/// `crate_docs_rs_present` function.
pub(crate) fn crate_docs_rs_present(krate: &G3RsReleaseConfigCrate) -> bool {
    crate_package(krate)
        .and_then(|package| package.metadata.as_ref())
        .and_then(docs_rs_table)
        .is_some_and(has_supported_docs_rs_settings)
}

/// `crate_include_exclude_present` function.
pub(crate) fn crate_include_exclude_present(krate: &G3RsReleaseConfigCrate) -> bool {
    crate_package(krate).is_some_and(|package| {
        package.include.as_ref().is_some_and(non_empty_values)
            || package.exclude.as_ref().is_some_and(non_empty_values)
    })
}

/// `crate_has_binstall_metadata` function.
pub(crate) fn crate_has_binstall_metadata(krate: &G3RsReleaseConfigCrate) -> bool {
    crate_package(krate)
        .and_then(|package| package.metadata.as_ref())
        .and_then(|metadata| metadata.get("binstall"))
        .and_then(|value| value.as_table())
        .is_some()
}

/// `repo_publishable_count` function.
pub(crate) fn repo_publishable_count(crates: &[G3RsReleaseConfigCrate]) -> usize {
    crates
        .iter()
        .filter(|krate| crate_publishable(krate))
        .count()
}

/// `repo_non_publishable_count` function.
pub(crate) fn repo_non_publishable_count(crates: &[G3RsReleaseConfigCrate]) -> usize {
    crates.len().saturating_sub(repo_publishable_count(crates))
}

/// `repo_publishable_crate_names` function.
pub(crate) fn repo_publishable_crate_names(crates: &[G3RsReleaseConfigCrate]) -> BTreeSet<String> {
    crates
        .iter()
        .filter(|krate| crate_publishable(krate))
        .map(|krate| krate.name.clone())
        .collect()
}

/// `repo_binary_crate_count` function.
pub(crate) fn repo_binary_crate_count(crates: &[G3RsReleaseConfigCrate]) -> usize {
    crates.iter().filter(|krate| krate.is_binary).count()
}

/// `repo_release_plz_package_names` function.
pub(crate) fn repo_release_plz_package_names(repo: &G3RsReleaseConfigRepo) -> BTreeSet<String> {
    repo.release_plz
        .as_ref()
        .map(|release_plz| {
            release_plz
                .package
                .iter()
                .filter_map(|package| package.name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// `repo_publish_setting` function.
pub(crate) fn repo_publish_setting(repo: &G3RsReleaseConfigRepo) -> Option<String> {
    let publish = repo
        .cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.publish.as_ref())
        .cloned()
        .or_else(|| {
            repo.cargo.package.as_ref().and_then(|package| {
                package.publish.as_ref().map(|publish| match publish {
                    InheritableValue::Value(value) => value.clone(),
                    InheritableValue::Inherit(_) => VecStringOrBool::Bool(false),
                })
            })
        })?;

    Some(match publish {
        VecStringOrBool::Bool(value) => value.to_string(),
        VecStringOrBool::VecString(values) => format!(
            "[{}]",
            values
                .iter()
                .map(|value| format!("\"{value}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    })
}

/// `repo_release_profile_settings` function.
pub(crate) fn repo_release_profile_settings(repo: &G3RsReleaseConfigRepo) -> Vec<String> {
    let Some(profile) = repo.cargo.profile.get("release") else {
        return Vec::new();
    };
    profile_settings(profile)
}

/// `edge_source_publishable` function.
pub(crate) fn edge_source_publishable(edge: &G3RsReleaseConfigEdge) -> bool {
    crate_publishable(&edge.source)
}

/// `edge_target_publishable` function.
pub(crate) fn edge_target_publishable(edge: &G3RsReleaseConfigEdge) -> bool {
    edge.target.as_ref().is_some_and(crate_publishable)
}

/// `edge_target_version` function.
pub(crate) fn edge_target_version(edge: &G3RsReleaseConfigEdge) -> Option<String> {
    edge.target.as_ref().and_then(crate_version_string)
}

/// `edge_version_satisfied` function.
pub(crate) fn edge_version_satisfied(edge: &G3RsReleaseConfigEdge) -> bool {
    let Some(version_req) = &edge.version_req else {
        return true;
    };
    let Some(actual_version) = edge_target_version(edge) else {
        return false;
    };
    version_requirement_satisfied(&actual_version, version_req)
}

/// `fn` function.
const fn crate_package(krate: &G3RsReleaseConfigCrate) -> Option<&PackageSection> {
    krate.cargo.package.as_ref()
}

/// `non_empty_values` function.
fn non_empty_values(value: &InheritableValue<Vec<String>>) -> bool {
    matches!(value, InheritableValue::Value(values) if !values.is_empty())
}

/// `inherited_string_present` function.
fn inherited_string_present(
    value: Option<&InheritableValue<String>>,
    workspace_value: Option<&str>,
) -> bool {
    match value {
        Some(InheritableValue::Value(declared)) => !declared.trim().is_empty(),
        Some(InheritableValue::Inherit(_)) => {
            workspace_value.is_some_and(|inherited| !inherited.trim().is_empty())
        }
        None => false,
    }
}

/// `inherited_vec_count` function.
fn inherited_vec_count(
    value: InheritableStringList<'_>,
    workspace_values: Option<&[String]>,
) -> Option<usize> {
    match value {
        Some(InheritableValue::Value(values)) => Some(values.len()),
        Some(InheritableValue::Inherit(_)) => workspace_values.map(<[std::string::String]>::len),
        None => None,
    }
}

/// `docs_rs_table` function.
fn docs_rs_table(metadata: &toml::Value) -> Option<&TomlMap> {
    metadata
        .get("docs.rs")
        .and_then(|value| value.as_table())
        .or_else(|| {
            metadata
                .get("docs")
                .and_then(|docs| docs.as_table())
                .and_then(|docs| docs.get("rs"))
                .and_then(|value| value.as_table())
        })
}

/// `has_supported_docs_rs_settings` function.
fn has_supported_docs_rs_settings(table: &toml::map::Map<String, toml::Value>) -> bool {
    [
        "all-features",
        "features",
        "no-default-features",
        "default-target",
        "targets",
        "rustdoc-args",
        "cargo-args",
    ]
    .iter()
    .any(|key| table.contains_key(*key))
}

/// `profile_settings` function.
fn profile_settings(profile: &ProfileConfig) -> Vec<String> {
    let mut settings = Vec::new();
    if let Some(value) = &profile.opt_level {
        settings.push(format!(
            "opt-level = {}",
            match value {
                IntegerOrString::Integer(value) => value.to_string(),
                IntegerOrString::String(value) => format!("\"{value}\""),
            }
        ));
    }
    if let Some(value) = &profile.lto {
        settings.push(format!(
            "lto = {}",
            match value {
                StringOrBool::String(value) => format!("\"{value}\""),
                StringOrBool::Bool(value) => value.to_string(),
            }
        ));
    }
    if let Some(value) = profile.codegen_units {
        settings.push(format!("codegen-units = {value}"));
    }
    if let Some(value) = profile.debug_assertions {
        settings.push(format!("debug-assertions = {value}"));
    }
    if let Some(value) = profile.strip.as_ref() {
        settings.push(format!(
            "strip = {}",
            match value {
                StringOrBool::String(value) => format!("\"{value}\""),
                StringOrBool::Bool(value) => value.to_string(),
            }
        ));
    }
    if let Some(value) = profile.trim_paths.as_ref() {
        settings.push(format!("trim-paths = {value:?}"));
    }
    settings.extend(profile.extra.keys().map(|key| format!("{key} = ...")));
    settings
}

/// `has_valid_commit_suffix` function.
fn has_valid_commit_suffix(suffix: &str) -> bool {
    suffix.starts_with(':') || (suffix.starts_with('(') && suffix.ends_with(':'))
}

/// `version_requirement_satisfied` function.
fn version_requirement_satisfied(actual: &str, req: &str) -> bool {
    let Ok(actual) = Version::parse(actual) else {
        return false;
    };
    let normalized = if req.trim_start().starts_with(['^', '~', '>', '<', '=']) {
        req.trim().to_owned()
    } else {
        format!("^{req}")
    };
    let Ok(parsed_req) = VersionReq::parse(&normalized) else {
        return false;
    };
    parsed_req.matches(&actual)
}
