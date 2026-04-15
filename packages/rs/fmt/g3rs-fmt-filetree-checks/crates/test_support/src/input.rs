use g3rs_fmt_types::{
    G3RsFmtConfigFileKind, G3RsFmtFileTreeChecksInput, G3RsFmtNestedConfigFile,
};

pub fn input(
    root_rustfmt_toml_rel_path: Option<&str>,
    root_dot_rustfmt_toml_rel_path: Option<&str>,
    nested_config_files: Vec<(&str, G3RsFmtConfigFileKind)>,
    dual_conflict_dirs: Vec<&str>,
) -> G3RsFmtFileTreeChecksInput {
    G3RsFmtFileTreeChecksInput {
        root_rustfmt_toml_rel_path: root_rustfmt_toml_rel_path.map(str::to_owned),
        root_dot_rustfmt_toml_rel_path: root_dot_rustfmt_toml_rel_path.map(str::to_owned),
        nested_config_files: nested_config_files
            .into_iter()
            .map(|(rel_path, kind)| G3RsFmtNestedConfigFile {
                rel_path: rel_path.to_owned(),
                kind,
            })
            .collect(),
        dual_conflict_dirs: dual_conflict_dirs.into_iter().map(str::to_owned).collect(),
    }
}
