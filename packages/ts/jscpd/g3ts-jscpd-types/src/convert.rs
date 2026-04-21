use jscpd_json_parser::types::JscpdSnapshot;

use crate::G3TsJscpdRootSnapshot;

#[must_use]
pub fn root_snapshot(rel_path: &str, snapshot: &JscpdSnapshot) -> G3TsJscpdRootSnapshot {
    G3TsJscpdRootSnapshot {
        rel_path: rel_path.to_owned(),
        threshold: snapshot.threshold,
        min_tokens: snapshot.min_tokens,
        absolute: snapshot.absolute,
        format: snapshot.format.clone(),
        ignore: snapshot.ignore.clone(),
        extra_keys: snapshot.extra_keys.clone(),
    }
}
