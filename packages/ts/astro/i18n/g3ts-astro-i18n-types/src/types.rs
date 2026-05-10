use std::collections::BTreeMap;

/// Map of plugin specifier to the package names that resolve from it.
///
/// Keyed by the plugin string used in `eslint.config` and valued by the list
/// of npm package names contributed by that plugin entry.
pub type G3TsAstroI18nPluginPackageNames = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    pub rel_path: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroPackageSurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroI18nPolicySnapshot {
    pub rel_path: String,
    pub locales: Vec<String>,
    pub default_locale: Option<String>,
    pub require_locale_prefix_for_content_routes: bool,
    pub allowed_unprefixed_routes: Vec<String>,
    pub content_route_prefixes: Vec<String>,
    pub checked_internal_link_helpers: Vec<String>,
    pub approved_internal_link_helpers: Vec<String>,
    pub approved_localized_link_components: Vec<String>,
    pub approved_date_format_helpers: Vec<String>,
    pub approved_number_format_helpers: Vec<String>,
    pub public_source_globs: Vec<String>,
    pub helper_source_globs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::large_enum_variant,
    reason = "Parsed snapshot is the dominant runtime variant; boxing would force \
              construction-site changes across consumer crates outside this workspace"
)]
pub enum G3TsAstroI18nPolicySurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    MissingAstroPolicy {
        rel_path: String,
    },
    MissingI18nPolicy {
        rel_path: String,
    },
    Parsed {
        snapshot: G3TsAstroI18nPolicySnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each bool corresponds to a distinct named field in eslint config probe \
              detection; merging into bitflags would obscure the field-level surface \
              consumed by checks across multiple crates outside this workspace"
)]
pub struct G3TsAstroI18nEslintSurfaceSnapshot {
    pub rel_path: String,
    pub public_probe_present: bool,
    pub public_probe_ignored: bool,
    pub helper_probe_present: bool,
    pub helper_probe_ignored: bool,
    pub public_plugins: Vec<String>,
    pub public_plugin_package_names: G3TsAstroI18nPluginPackageNames,
    pub public_error_rules: Vec<String>,
    pub public_restricted_disable_patterns: Vec<String>,
    pub public_i18n_policy_rules: Vec<String>,
    pub public_no_restricted_syntax_selectors: Vec<String>,
    pub helper_no_restricted_syntax_selectors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroI18nEslintSurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        snapshot: G3TsAstroI18nEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroI18nIntegrationContractInput {
    pub app_root_rel_path: String,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_policy: G3TsAstroI18nPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroI18nEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroI18nEslintSurfaceState,
    pub astro_policy: G3TsAstroI18nPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroI18nConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroI18nIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroI18nEslintPluginContractInput>,
}
