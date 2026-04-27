use crate::support_nuasite;
use g3ts_astro_types::{
    G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroContentIntegrationContractInput,
    G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroMdxIntegrationContractInput, G3TsAstroOutputMode,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState, G3TsAstroPolicySnapshot,
    G3TsAstroPolicySurfaceState, G3TsAstroSeoIntegrationContractInput,
    G3TsAstroSetupIntegrationContractInput, G3TsAstroStaticValue,
    G3TsAstroSyncpackRequiredPin,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub trait G3TsAstroPackageContract {
    fn package(&self) -> &G3TsAstroPackageSurfaceState;
}

pub trait G3TsAstroSyncpackPinsContract {
    fn required_syncpack_pins(&self) -> &[G3TsAstroSyncpackRequiredPin];
}

pub trait G3TsAstroForbiddenDepsContract {
    fn forbidden_syncpack_deps(&self) -> &[String];
}

pub trait G3TsAstroConfigContract {
    fn astro_config(&self) -> &G3TsAstroConfigSurfaceState;
}

pub trait G3TsAstroPolicyContract {
    fn astro_policy(&self) -> &G3TsAstroPolicySurfaceState;
}

impl G3TsAstroPackageContract for G3TsAstroSetupIntegrationContractInput {
    fn package(&self) -> &G3TsAstroPackageSurfaceState {
        &self.package
    }
}

impl G3TsAstroPackageContract for G3TsAstroContentIntegrationContractInput {
    fn package(&self) -> &G3TsAstroPackageSurfaceState {
        &self.package
    }
}

impl G3TsAstroPackageContract for G3TsAstroMdxIntegrationContractInput {
    fn package(&self) -> &G3TsAstroPackageSurfaceState {
        &self.package
    }
}

impl G3TsAstroPackageContract for G3TsAstroSeoIntegrationContractInput {
    fn package(&self) -> &G3TsAstroPackageSurfaceState {
        &self.package
    }
}

impl G3TsAstroSyncpackPinsContract for G3TsAstroSetupIntegrationContractInput {
    fn required_syncpack_pins(&self) -> &[G3TsAstroSyncpackRequiredPin] {
        &self.required_syncpack_pins
    }
}

impl G3TsAstroForbiddenDepsContract for G3TsAstroSetupIntegrationContractInput {
    fn forbidden_syncpack_deps(&self) -> &[String] {
        &self.forbidden_syncpack_deps
    }
}

impl G3TsAstroConfigContract for G3TsAstroSetupIntegrationContractInput {
    fn astro_config(&self) -> &G3TsAstroConfigSurfaceState {
        &self.astro_config
    }
}

impl G3TsAstroConfigContract for G3TsAstroSeoIntegrationContractInput {
    fn astro_config(&self) -> &G3TsAstroConfigSurfaceState {
        &self.astro_config
    }
}

impl G3TsAstroPolicyContract for G3TsAstroContentIntegrationContractInput {
    fn astro_policy(&self) -> &G3TsAstroPolicySurfaceState {
        &self.astro_policy
    }
}

impl G3TsAstroPolicyContract for G3TsAstroMdxIntegrationContractInput {
    fn astro_policy(&self) -> &G3TsAstroPolicySurfaceState {
        &self.astro_policy
    }
}

impl G3TsAstroPolicyContract for G3TsAstroSeoIntegrationContractInput {
    fn astro_policy(&self) -> &G3TsAstroPolicySurfaceState {
        &self.astro_policy
    }
}

#[must_use]
pub fn parsed_package<T: G3TsAstroPackageContract>(
    contract: &T,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match contract.package() {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub fn package_rel_path<T: G3TsAstroPackageContract>(contract: &T) -> Option<&str> {
    match contract.package() {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub fn package_has_dependency(
    contract: &impl G3TsAstroPackageContract,
    dependency_name: &str,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

#[must_use]
pub fn package_safely_runs_astro_check(contract: &impl G3TsAstroPackageContract) -> bool {
    parsed_package(contract).is_some_and(|snapshot| snapshot.safely_runs_astro_check)
}

#[must_use]
pub fn package_safely_runs_astro_build(contract: &impl G3TsAstroPackageContract) -> bool {
    parsed_package(contract).is_some_and(|snapshot| snapshot.safely_runs_astro_build)
}

#[must_use]
pub fn expected_syncpack_source_entry(
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> Option<String> {
    let _syncpack_rel_path = syncpack_rel_path;
    let _package_rel_path = package_rel_path;
    Some("package.json".to_owned())
}

#[must_use]
pub fn required_syncpack_pins_message(contract: &impl G3TsAstroSyncpackPinsContract) -> String {
    contract
        .required_syncpack_pins()
        .iter()
        .map(|pin| format!("`{}` -> `{}`", pin.dependency, pin.version))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
#[cfg(test)]
pub fn forbidden_syncpack_deps_message(contract: &impl G3TsAstroForbiddenDepsContract) -> String {
    contract
        .forbidden_syncpack_deps()
        .iter()
        .map(|dependency| format!("`{dependency}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
pub fn parsed_eslint_surface(
    contract: &G3TsAstroEslintPluginContractInput,
) -> Option<&G3TsAstroEslintSurfaceSnapshot> {
    match &contract.config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroEslintSurfaceState::Missing { .. }
        | G3TsAstroEslintSurfaceState::Unreadable { .. }
        | G3TsAstroEslintSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub fn parsed_astro_config(
    contract: &impl G3TsAstroConfigContract,
) -> Option<&G3TsAstroConfigSurfaceSnapshot> {
    match contract.astro_config() {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub fn parsed_astro_policy(
    contract: &impl G3TsAstroPolicyContract,
) -> Option<&G3TsAstroPolicySnapshot> {
    match contract.astro_policy() {
        G3TsAstroPolicySurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPolicySurfaceState::Missing { .. }
        | G3TsAstroPolicySurfaceState::Unreadable { .. }
        | G3TsAstroPolicySurfaceState::ParseError { .. }
        | G3TsAstroPolicySurfaceState::MissingAstroPolicy { .. } => None,
    }
}

#[must_use]
pub fn astro_policy_rel_path(contract: &impl G3TsAstroPolicyContract) -> Option<&str> {
    match contract.astro_policy() {
        G3TsAstroPolicySurfaceState::Missing { rel_path }
        | G3TsAstroPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroPolicySurfaceState::MissingAstroPolicy { rel_path } => Some(rel_path),
        G3TsAstroPolicySurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub fn astro_config_rel_path(contract: &impl G3TsAstroConfigContract) -> Option<&str> {
    match contract.astro_config() {
        G3TsAstroConfigSurfaceState::Missing { rel_path }
        | G3TsAstroConfigSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroConfigSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub fn astro_config_is_static(contract: &impl G3TsAstroConfigContract) -> bool {
    parsed_astro_config(contract)
        .is_some_and(|snapshot| snapshot.output == Some(G3TsAstroOutputMode::Static))
}

#[must_use]
pub fn astro_config_site_is_https(snapshot: &G3TsAstroConfigSurfaceSnapshot) -> bool {
    snapshot.site.as_deref().is_some_and(|site| {
        url::Url::parse(site).is_ok_and(|url| url.scheme() == "https" && url.host_str().is_some())
    })
}

#[must_use]
pub fn astro_config_has_zero_arg_integration(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
    module: &str,
    accepted_imported_names: &[Option<&str>],
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some(module)
            && integration.call.is_some()
            && integration
                .call
                .as_ref()
                .is_some_and(|call| call.first_arg.is_none())
            && accepted_imported_names
                .iter()
                .any(|expected| integration.imported_name.as_deref() == *expected)
    })
}

#[must_use]
pub fn astro_config_has_object_arg_integration(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
    module: &str,
    accepted_imported_names: &[Option<&str>],
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some(module)
            && integration
                .call
                .as_ref()
                .is_some_and(|call| matches!(call.first_arg, Some(G3TsAstroStaticValue::Object(_))))
            && accepted_imported_names
                .iter()
                .any(|expected| integration.imported_name.as_deref() == *expected)
    })
}

#[must_use]
pub fn astro_config_has_nuasite_checks_with_required_options(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some("@nuasite/checks")
            && integration.call.is_some()
            && matches!(integration.imported_name.as_deref(), None | Some("checks"))
            && integration
                .call
                .as_ref()
                .and_then(|call| call.first_arg.as_ref())
                .is_some_and(support_nuasite::checks_options_are_fail_closed)
    })
}

#[must_use]
pub fn checks_options_include_structured_data_check(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some("@nuasite/checks")
            && integration.call.is_some()
            && matches!(integration.imported_name.as_deref(), None | Some("checks"))
            && integration
                .call
                .as_ref()
                .and_then(|call| call.first_arg.as_ref())
                .is_some_and(support_nuasite::checks_options_have_structured_data_custom_check)
    })
}

#[must_use]
pub fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

#[must_use]
pub fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}
