#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTopologyDescendantGuardrail3TsToml {
    pub rel_dir: String,
    pub toml_rel_path: String,
    pub has_sibling_package_json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTopologyFileTreeInputFailure {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTopologyNestedGuardrail3TsTomlInput {
    pub rel_dir: String,
    pub toml_rel_path: String,
    pub parent_unit_rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTopologyFileTreeChecksInput {
    pub unit_root_rel_dir: String,
    pub unit_root_package_json_rel_path: String,
    pub unit_root_guardrail3_ts_toml_rel_path: String,
    pub descendant_guardrail3_ts_tomls: Vec<G3TsTopologyDescendantGuardrail3TsToml>,
    pub input_failures: Vec<G3TsTopologyFileTreeInputFailure>,
    pub nested_guardrail3_ts_tomls: Vec<G3TsTopologyNestedGuardrail3TsTomlInput>,
}
