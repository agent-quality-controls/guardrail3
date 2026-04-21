use npmrc_parser::types::NpmrcSnapshot;

use crate::{G3TsNpmrcRootSnapshot, G3TsNpmrcSetting};

#[must_use]
pub fn root_snapshot(rel_path: &str, snapshot: &NpmrcSnapshot) -> G3TsNpmrcRootSnapshot {
    G3TsNpmrcRootSnapshot {
        rel_path: rel_path.to_owned(),
        settings: snapshot
            .settings
            .iter()
            .map(|setting| G3TsNpmrcSetting {
                key: setting.key.clone(),
                value: setting.value.clone(),
            })
            .collect(),
        duplicate_keys: snapshot.duplicate_keys.clone(),
    }
}
