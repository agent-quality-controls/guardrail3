use g3rs_clippy_types::{G3RsClippyFileTreeChecksInput, G3RsClippyShadowedConfig};

pub fn input(
    preferred_root_config_rel_path: Option<&str>,
    shadowed_same_root_configs: &[(&str, &str)],
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
