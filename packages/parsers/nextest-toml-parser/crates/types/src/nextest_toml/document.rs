use std::collections::BTreeMap;

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use toml::Value;

use super::basics::{ExperimentalFeature, NextestVersionConfig, StoreConfig};
use super::execution::TestGroupConfig;
use super::profile::NextestProfile;
use super::scripts::{ScriptsConfig, SetupScriptConfig};

/// Parsed representation of a `.config/nextest.toml` configuration file.
///
/// Known nextest keys are typed where the contract is explicit in nextest's
/// configuration reference. Unknown keys are preserved in `extra` because
/// nextest itself warns on unknown configuration and otherwise ignores it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NextestToml {
    pub store: Option<StoreConfig>,
    pub nextest_version: Option<NextestVersionConfig>,
    #[serde(default)]
    pub experimental: Vec<ExperimentalFeature>,
    #[serde(default)]
    pub test_groups: BTreeMap<String, TestGroupConfig>,
    #[serde(default, rename = "script")]
    pub script: BTreeMap<String, SetupScriptConfig>,
    pub scripts: Option<ScriptsConfig>,
    #[serde(default)]
    pub profile: BTreeMap<String, NextestProfile>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for NextestToml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct RawNextestToml {
            store: Option<StoreConfig>,
            nextest_version: Option<NextestVersionConfig>,
            #[serde(default)]
            experimental: Vec<ExperimentalFeature>,
            #[serde(default)]
            test_groups: BTreeMap<String, TestGroupConfig>,
            #[serde(default, rename = "script")]
            script: BTreeMap<String, SetupScriptConfig>,
            scripts: Option<ScriptsConfig>,
            #[serde(default)]
            profile: BTreeMap<String, NextestProfile>,
            #[serde(flatten)]
            extra: BTreeMap<String, Value>,
        }

        let raw = RawNextestToml::deserialize(deserializer)?;
        let scripts = raw.scripts.as_ref();
        let has_legacy_setup = !raw.script.is_empty();
        let has_setup_scripts = scripts.is_some_and(|scripts| !scripts.setup.is_empty());
        let has_wrapper_scripts = scripts.is_some_and(|scripts| !scripts.wrapper.is_empty());
        let has_experimental_setup = raw
            .experimental
            .contains(&ExperimentalFeature::SetupScripts);
        let has_experimental_wrapper = raw
            .experimental
            .contains(&ExperimentalFeature::WrapperScripts);

        if has_legacy_setup && has_setup_scripts {
            return Err(de::Error::custom(
                "invalid nextest.toml: [script.*] cannot be used together with [scripts.setup.*]",
            ));
        }

        if (has_legacy_setup || has_setup_scripts) && !has_experimental_setup {
            return Err(de::Error::custom(
                "invalid nextest.toml: setup scripts require experimental = [\"setup-scripts\"]",
            ));
        }

        if has_wrapper_scripts && !has_experimental_wrapper {
            return Err(de::Error::custom(
                "invalid nextest.toml: wrapper scripts require experimental = [\"wrapper-scripts\"]",
            ));
        }

        Ok(Self {
            store: raw.store,
            nextest_version: raw.nextest_version,
            experimental: raw.experimental,
            test_groups: raw.test_groups,
            script: raw.script,
            scripts: raw.scripts,
            profile: raw.profile,
            extra: raw.extra,
        })
    }
}
