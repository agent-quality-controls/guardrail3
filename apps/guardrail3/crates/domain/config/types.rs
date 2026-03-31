use std::collections::BTreeMap;

use guardrail3_validation_model::RustValidateFamily;
use serde::Deserialize;

/// Type alias for crate configuration map.
type CrateMap = BTreeMap<String, CrateConfig>;

/// Type alias for TypeScript app configuration map.
type TsAppMap = BTreeMap<String, TsAppConfig>;

#[derive(Debug, Deserialize, garde::Validate)]
pub struct GuardrailConfig {
    #[garde(inner(length(min = 1)))] // reason: version string must be non-empty when present
    version: Option<String>,
    #[garde(dive)] // reason: recursively validate nested ProfileConfig
    profile: Option<ProfileConfig>,
    #[garde(dive)] // reason: recursively validate nested RustConfig
    rust: Option<RustConfig>,
    #[garde(dive)] // reason: recursively validate nested TypeScriptConfig
    typescript: Option<TypeScriptConfig>,
    #[garde(dive)] // reason: recursively validate nested HooksConfig
    hooks: Option<HooksConfig>,
    #[garde(dive)] // reason: each shared escape hatch entry must validate independently
    escape_hatches: Option<Vec<EscapeHatchConfig>>,
}

impl GuardrailConfig {
    #[must_use]
    pub const fn new(
        version: Option<String>,
        profile: Option<ProfileConfig>,
        rust: Option<RustConfig>,
        typescript: Option<TypeScriptConfig>,
        hooks: Option<HooksConfig>,
    ) -> Self {
        Self {
            version,
            profile,
            rust,
            typescript,
            hooks,
            escape_hatches: None,
        }
    }

    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    #[must_use]
    pub const fn profile(&self) -> Option<&ProfileConfig> {
        self.profile.as_ref()
    }

    #[must_use]
    pub const fn rust(&self) -> Option<&RustConfig> {
        self.rust.as_ref()
    }

    #[must_use]
    pub const fn typescript(&self) -> Option<&TypeScriptConfig> {
        self.typescript.as_ref()
    }

    #[must_use]
    pub const fn hooks(&self) -> Option<&HooksConfig> {
        self.hooks.as_ref()
    }

    #[must_use]
    pub fn escape_hatches(&self) -> &[EscapeHatchConfig] {
        self.escape_hatches.as_deref().unwrap_or(&[])
    }

    #[must_use]
    pub fn escape_hatch_reason(
        &self,
        family: &str,
        file: &str,
        kind: &str,
        selector: &str,
    ) -> Option<&str> {
        self.escape_hatches()
            .iter()
            .find(|entry| {
                entry.family() == family
                    && entry.file() == file
                    && entry.kind() == kind
                    && entry.selector() == selector
            })
            .map(EscapeHatchConfig::reason)
    }
}

#[derive(Debug, Clone, Deserialize, garde::Validate)]
pub struct EscapeHatchConfig {
    #[garde(length(min = 1))] // reason: family discriminator must be non-empty
    family: String,
    #[garde(length(min = 1))] // reason: file selector must be non-empty
    file: String,
    #[garde(length(min = 1))] // reason: escape hatch kind must be non-empty
    kind: String,
    #[garde(length(min = 1))] // reason: selector key must be non-empty
    selector: String,
    #[garde(length(min = 1))] // reason: reason text must be present before policy validation
    reason: String,
}

impl EscapeHatchConfig {
    #[must_use]
    pub const fn new(
        family: String,
        file: String,
        kind: String,
        selector: String,
        reason: String,
    ) -> Self {
        Self {
            family,
            file,
            kind,
            selector,
            reason,
        }
    }

    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    #[must_use]
    pub fn selector(&self) -> &str {
        &self.selector
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct ProfileConfig {
    #[garde(length(min = 1))] // reason: profile name must be non-empty
    name: String,
}

impl ProfileConfig {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self { name }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct RustConfig {
    #[garde(inner(length(min = 1)))] // reason: workspace root path must be non-empty when present
    workspace_root: Option<String>,
    #[garde(inner(inner(length(min = 1))))] // reason: each workspace path must be non-empty
    workspaces: Option<Vec<String>>,
    #[garde(skip)] // reason: map values are validated by layer_from_config
    apps: Option<CrateMap>,
    #[garde(dive)] // reason: recursively validate nested CrateConfig for packages
    packages: Option<CrateConfig>,
    #[garde(dive)] // reason: recursively validate nested RustChecksConfig
    checks: Option<RustChecksConfig>,
}

impl RustConfig {
    #[must_use]
    pub const fn new(
        workspace_root: Option<String>,
        workspaces: Option<Vec<String>>,
        apps: Option<CrateMap>,
        packages: Option<CrateConfig>,
        checks: Option<RustChecksConfig>,
    ) -> Self {
        Self {
            workspace_root,
            workspaces,
            apps,
            packages,
            checks,
        }
    }

    #[must_use]
    pub fn workspace_root(&self) -> Option<&str> {
        self.workspace_root.as_deref()
    }

    #[must_use]
    pub fn workspaces(&self) -> Option<&[String]> {
        self.workspaces.as_deref()
    }

    #[must_use]
    pub const fn apps(&self) -> Option<&BTreeMap<String, CrateConfig>> {
        self.apps.as_ref()
    }

    #[must_use]
    pub const fn packages(&self) -> Option<&CrateConfig> {
        self.packages.as_ref()
    }

    #[must_use]
    pub const fn checks(&self) -> Option<&RustChecksConfig> {
        self.checks.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize, garde::Validate)]
pub struct CrateConfig {
    #[garde(inner(length(min = 1)))] // reason: layer name must be non-empty when present
    layer: Option<String>,
    #[garde(inner(length(min = 1)))] // reason: profile name must be non-empty when present
    profile: Option<String>,
    /// App type — unified alias for `profile` (matches TS convention)
    #[serde(rename = "type")]
    #[garde(inner(length(min = 1)))] // reason: type name must be non-empty when present
    type_: Option<String>,
    #[garde(inner(inner(length(min = 1))))] // reason: each allowed dep name must be non-empty
    allowed_deps: Option<Vec<String>>,
    #[garde(dive)] // reason: recursively validate nested RustChecksConfig
    checks: Option<RustChecksConfig>,
}

impl CrateConfig {
    #[must_use]
    pub const fn new(
        layer: Option<String>,
        profile: Option<String>,
        type_: Option<String>,
        allowed_deps: Option<Vec<String>>,
        checks: Option<RustChecksConfig>,
    ) -> Self {
        Self {
            layer,
            profile,
            type_,
            allowed_deps,
            checks,
        }
    }

    #[must_use]
    pub fn layer(&self) -> Option<&str> {
        self.layer.as_deref()
    }

    #[must_use]
    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }

    #[must_use]
    pub fn type_(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    #[must_use]
    pub fn allowed_deps(&self) -> Option<&[String]> {
        self.allowed_deps.as_deref()
    }

    #[must_use]
    pub const fn checks(&self) -> Option<&RustChecksConfig> {
        self.checks.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize, garde::Validate)]
#[serde(deny_unknown_fields)]
pub struct RustChecksConfig {
    #[garde(skip)] // reason: Option<bool> — inherently valid
    topology: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    fmt: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    toolchain: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    clippy: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    deny: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    cargo: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    code: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    hexarch: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    libarch: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    deps: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    garde: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    test: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    release: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    hooks_shared: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    hooks_rs: Option<bool>,
}

impl RustChecksConfig {
    #[must_use]
    #[allow(clippy::too_many_arguments)] // reason: config value object mirrors serialized family toggle set
    pub const fn new(
        topology: Option<bool>,
        fmt: Option<bool>,
        toolchain: Option<bool>,
        clippy: Option<bool>,
        deny: Option<bool>,
        cargo: Option<bool>,
        code: Option<bool>,
        hexarch: Option<bool>,
        libarch: Option<bool>,
        deps: Option<bool>,
        garde: Option<bool>,
        test: Option<bool>,
        release: Option<bool>,
        hooks_shared: Option<bool>,
        hooks_rs: Option<bool>,
    ) -> Self {
        Self {
            topology,
            fmt,
            toolchain,
            clippy,
            deny,
            cargo,
            code,
            hexarch,
            libarch,
            deps,
            garde,
            test,
            release,
            hooks_shared,
            hooks_rs,
        }
    }

    #[must_use]
    pub const fn family_enabled(&self, family: RustValidateFamily) -> Option<bool> {
        match family {
            RustValidateFamily::Topology => self.topology,
            RustValidateFamily::Fmt => self.fmt,
            RustValidateFamily::Toolchain => self.toolchain,
            RustValidateFamily::Clippy => self.clippy,
            RustValidateFamily::Deny => self.deny,
            RustValidateFamily::Cargo => self.cargo,
            RustValidateFamily::Code => self.code,
            RustValidateFamily::Hexarch => self.hexarch,
            RustValidateFamily::Libarch => self.libarch,
            RustValidateFamily::Deps => self.deps,
            RustValidateFamily::Garde => self.garde,
            RustValidateFamily::Test => self.test,
            RustValidateFamily::Release => self.release,
            RustValidateFamily::HooksShared => self.hooks_shared,
            RustValidateFamily::HooksRs => self.hooks_rs,
        }
    }

    #[must_use]
    pub const fn topology(&self) -> Option<bool> {
        self.topology
    }

    #[must_use]
    pub const fn fmt(&self) -> Option<bool> {
        self.fmt
    }

    #[must_use]
    pub const fn toolchain(&self) -> Option<bool> {
        self.toolchain
    }

    #[must_use]
    pub const fn clippy(&self) -> Option<bool> {
        self.clippy
    }

    #[must_use]
    pub const fn deny(&self) -> Option<bool> {
        self.deny
    }

    #[must_use]
    pub const fn cargo(&self) -> Option<bool> {
        self.cargo
    }

    #[must_use]
    pub const fn code(&self) -> Option<bool> {
        self.code
    }

    #[must_use]
    pub const fn hexarch(&self) -> Option<bool> {
        self.hexarch
    }

    #[must_use]
    pub const fn libarch(&self) -> Option<bool> {
        self.libarch
    }

    #[must_use]
    pub const fn deps(&self) -> Option<bool> {
        self.deps
    }

    #[must_use]
    pub const fn garde(&self) -> Option<bool> {
        self.garde
    }

    #[must_use]
    pub const fn test(&self) -> Option<bool> {
        self.test
    }

    #[must_use]
    pub const fn release(&self) -> Option<bool> {
        self.release
    }

    #[must_use]
    pub const fn hooks_shared(&self) -> Option<bool> {
        self.hooks_shared
    }

    #[must_use]
    pub const fn hooks_rs(&self) -> Option<bool> {
        self.hooks_rs
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TsChecksConfig {
    #[garde(skip)] // reason: Option<bool> — inherently valid
    topology: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    content: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid
    tests: Option<bool>,
}

impl TsChecksConfig {
    #[must_use]
    pub const fn new(
        topology: Option<bool>,
        content: Option<bool>,
        tests: Option<bool>,
    ) -> Self {
        Self {
            topology,
            content,
            tests,
        }
    }

    #[must_use]
    pub const fn topology(&self) -> Option<bool> {
        self.topology
    }

    #[must_use]
    pub const fn content(&self) -> Option<bool> {
        self.content
    }

    #[must_use]
    pub const fn tests(&self) -> Option<bool> {
        self.tests
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TsAppConfig {
    /// App type: "service", "content", or "library"
    #[serde(rename = "type")]
    #[garde(inner(length(min = 1)))] // reason: type name must be non-empty when present
    type_: Option<String>,
    #[garde(dive)] // reason: recursively validate nested TsChecksConfig
    checks: Option<TsChecksConfig>,
}

impl TsAppConfig {
    #[must_use]
    pub const fn new(type_: Option<String>, checks: Option<TsChecksConfig>) -> Self {
        Self { type_, checks }
    }

    #[must_use]
    pub fn type_(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    #[must_use]
    pub const fn checks(&self) -> Option<&TsChecksConfig> {
        self.checks.as_ref()
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct TypeScriptConfig {
    #[garde(skip)] // reason: map values are validated by type resolution
    apps: Option<TsAppMap>,
    #[garde(inner(length(min = 1)))] // reason: migrations path must be non-empty when present
    migrations: Option<String>,
    #[garde(dive)] // reason: recursively validate nested EslintConfig
    eslint: Option<EslintConfig>,
    #[garde(dive)] // reason: recursively validate nested CanonicalConfig
    canonical: Option<CanonicalConfig>,
    #[garde(dive)] // reason: recursively validate nested TsChecksConfig
    checks: Option<TsChecksConfig>,
}

impl TypeScriptConfig {
    #[must_use]
    pub const fn new(
        apps: Option<TsAppMap>,
        migrations: Option<String>,
        eslint: Option<EslintConfig>,
        canonical: Option<CanonicalConfig>,
        checks: Option<TsChecksConfig>,
    ) -> Self {
        Self {
            apps,
            migrations,
            eslint,
            canonical,
            checks,
        }
    }

    #[must_use]
    pub const fn apps(&self) -> Option<&BTreeMap<String, TsAppConfig>> {
        self.apps.as_ref()
    }

    #[must_use]
    pub fn migrations(&self) -> Option<&str> {
        self.migrations.as_deref()
    }

    #[must_use]
    pub const fn eslint(&self) -> Option<&EslintConfig> {
        self.eslint.as_ref()
    }

    #[must_use]
    pub const fn canonical(&self) -> Option<&CanonicalConfig> {
        self.canonical.as_ref()
    }

    #[must_use]
    pub const fn checks(&self) -> Option<&TsChecksConfig> {
        self.checks.as_ref()
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct EslintConfig {
    #[garde(inner(length(min = 1)))] // reason: eslint mode must be non-empty when present
    mode: Option<String>,
}

impl EslintConfig {
    #[must_use]
    pub const fn new(mode: Option<String>) -> Self {
        Self { mode }
    }

    #[must_use]
    pub fn mode(&self) -> Option<&str> {
        self.mode.as_deref()
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct CanonicalConfig {
    #[garde(skip)] // reason: Option<bool> — inherently valid, no string validation needed
    npmrc: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid, no string validation needed
    tsconfig_base: Option<bool>,
    #[garde(skip)] // reason: Option<bool> — inherently valid, no string validation needed
    jscpd: Option<bool>,
}

impl CanonicalConfig {
    #[must_use]
    pub const fn new(
        npmrc: Option<bool>,
        tsconfig_base: Option<bool>,
        jscpd: Option<bool>,
    ) -> Self {
        Self {
            npmrc,
            tsconfig_base,
            jscpd,
        }
    }

    #[must_use]
    pub const fn npmrc(&self) -> Option<bool> {
        self.npmrc
    }

    #[must_use]
    pub const fn tsconfig_base(&self) -> Option<bool> {
        self.tsconfig_base
    }

    #[must_use]
    pub const fn jscpd(&self) -> Option<bool> {
        self.jscpd
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct HooksConfig {
    #[garde(inner(length(min = 1)))] // reason: extra_dir path must be non-empty when present
    extra_dir: Option<String>,
}

impl HooksConfig {
    #[must_use]
    pub const fn new(extra_dir: Option<String>) -> Self {
        Self { extra_dir }
    }

    #[must_use]
    pub fn extra_dir(&self) -> Option<&str> {
        self.extra_dir.as_deref()
    }
}
