use g3rs_clippy_types::{G3RsClippyFileTreeChecksInput, G3RsClippyShadowedConfig};

/// Pair of `(rel_path, preferred_rel_path)` describing one shadowed config site.
pub type ShadowedConfigPair<'a> = (&'a str, &'a str);

/// Build an input fixture for the file-tree checks.
#[must_use]
pub fn input(
    preferred_root_config_rel_path: Option<&str>,
    shadowed_same_root_configs: &[ShadowedConfigPair<'_>],
) -> G3RsClippyFileTreeChecksInput {
    G3RsClippyFileTreeChecksInput {
        preferred_root_config_rel_path: preferred_root_config_rel_path.map(str::to_owned),
        shadowed_same_root_configs: shadowed_same_root_configs
            .iter()
            .map(|(rel_path, preferred_rel_path)| G3RsClippyShadowedConfig {
                rel_path: (*rel_path).to_owned(),
                preferred_rel_path: (*preferred_rel_path).to_owned(),
            })
            .collect(),
    }
}
