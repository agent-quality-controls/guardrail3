#![allow(
    clippy::missing_docs_in_private_items,
    reason = "this file mirrors clippy.toml schema directly; private helper details are implementation scaffolding for exact parsing"
)]

use std::collections::BTreeSet;

use serde::de::{self, Deserializer};
use serde::ser;
use serde::{Deserialize, Serialize};

/// Parsed representation of a `clippy.toml` / `.clippy.toml` configuration file.
///
/// Known Clippy configuration keys are mapped to typed fields. Unknown keys are
/// rejected because upstream Clippy reports them as configuration errors rather
/// than preserving them for forward compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ClippyToml {
    #[serde(default)]
    pub absolute_paths_allowed_crates: Vec<String>,
    pub absolute_paths_max_segments: Option<u64>,
    pub accept_comment_above_attributes: Option<bool>,
    pub accept_comment_above_statement: Option<bool>,
    pub allow_comparison_to_zero: Option<bool>,
    pub allow_dbg_in_tests: Option<bool>,
    pub allow_exact_repetitions: Option<bool>,
    pub allow_expect_in_consts: Option<bool>,
    pub allow_expect_in_tests: Option<bool>,
    pub allow_indexing_slicing_in_tests: Option<bool>,
    pub allow_large_stack_frames_in_tests: Option<bool>,
    pub allow_mixed_uninlined_format_args: Option<bool>,
    pub allow_one_hash_in_raw_strings: Option<bool>,
    pub allow_panic_in_tests: Option<bool>,
    pub allow_print_in_tests: Option<bool>,
    pub allow_private_module_inception: Option<bool>,
    #[serde(default)]
    pub allow_renamed_params_for: Vec<String>,
    pub allow_unwrap_in_consts: Option<bool>,
    pub allow_unwrap_in_tests: Option<bool>,
    #[serde(default)]
    pub allow_unwrap_types: Vec<String>,
    pub allow_useless_vec_in_tests: Option<bool>,
    #[serde(default)]
    pub allowed_dotfiles: Vec<String>,
    #[serde(default)]
    pub allowed_duplicate_crates: Vec<String>,
    #[serde(default)]
    pub allowed_idents_below_min_chars: Vec<String>,
    #[serde(default)]
    pub allowed_prefixes: Vec<String>,
    #[serde(default)]
    pub allowed_scripts: Vec<String>,
    #[serde(default)]
    pub allowed_wildcard_imports: Vec<String>,
    #[serde(default)]
    pub arithmetic_side_effects_allowed: Vec<String>,
    #[serde(default)]
    pub arithmetic_side_effects_allowed_binary: Vec<ArithmeticSideEffectsBinaryEntry>,
    #[serde(default)]
    pub arithmetic_side_effects_allowed_unary: Vec<String>,
    pub array_size_threshold: Option<u64>,
    pub avoid_breaking_exported_api: Option<bool>,
    #[serde(default)]
    pub await_holding_invalid_types: Vec<AwaitHoldingInvalidType>,
    #[serde(default)]
    pub blacklisted_names: Vec<String>,
    pub cargo_ignore_publish: Option<bool>,
    pub check_incompatible_msrv_in_tests: Option<bool>,
    pub check_inconsistent_struct_field_initializers: Option<bool>,
    pub check_private_items: Option<bool>,
    pub cognitive_complexity_threshold: Option<u64>,
    pub const_literal_digits_threshold: Option<u64>,
    pub cyclomatic_complexity_threshold: Option<u64>,
    #[serde(default)]
    pub disallowed_fields: Vec<DisallowedField>,
    #[serde(default)]
    pub disallowed_macros: Vec<DisallowedPath>,
    #[serde(default)]
    pub disallowed_methods: Vec<DisallowedPath>,
    #[serde(default)]
    pub disallowed_names: Vec<String>,
    #[serde(default)]
    pub disallowed_types: Vec<DisallowedPath>,
    #[serde(default)]
    pub doc_valid_idents: Vec<String>,
    pub enable_raw_pointer_heuristic_for_send: Option<bool>,
    pub enforce_iter_loop_reborrow: Option<bool>,
    #[serde(default)]
    pub enforced_import_renames: Vec<RenameEntry>,
    pub enum_variant_name_threshold: Option<u64>,
    pub enum_variant_size_threshold: Option<u64>,
    pub excessive_nesting_threshold: Option<u64>,
    pub future_size_threshold: Option<u64>,
    #[serde(default)]
    pub ignore_interior_mutability: Vec<String>,
    pub inherent_impl_lint_scope: Option<InherentImplLintScope>,
    #[serde(default)]
    pub large_error_ignored: Vec<String>,
    pub large_error_threshold: Option<u64>,
    pub lint_commented_code: Option<bool>,
    pub lint_inconsistent_struct_field_initializers: Option<bool>,
    pub literal_representation_threshold: Option<u64>,
    pub matches_for_let_else: Option<MatchLintBehaviour>,
    pub max_fn_params_bools: Option<u64>,
    pub max_include_file_size: Option<u64>,
    pub max_struct_bools: Option<u64>,
    pub max_suggested_slice_pattern_length: Option<u64>,
    pub max_trait_bounds: Option<u64>,
    pub min_ident_chars_threshold: Option<u64>,
    pub missing_docs_allow_unused: Option<bool>,
    pub missing_docs_in_crate_items: Option<bool>,
    pub module_item_order_groupings: Option<SourceItemOrderingModuleItemGroupings>,
    pub module_items_ordered_within_groupings: Option<SourceItemOrderingWithinModuleItemGroupings>,
    pub msrv: Option<String>,
    pub pass_by_value_size_limit: Option<u64>,
    pub pub_underscore_fields_behavior: Option<PubUnderscoreFieldsBehaviour>,
    pub recursive_self_in_type_definitions: Option<bool>,
    pub semicolon_inside_block_ignore_singleline: Option<bool>,
    pub semicolon_outside_block_ignore_multiline: Option<bool>,
    pub single_char_binding_names_threshold: Option<u64>,
    pub source_item_ordering: Option<SourceItemOrdering>,
    pub stack_size_threshold: Option<u64>,
    #[serde(default)]
    pub standard_macro_braces: Vec<MacroBraceEntry>,
    pub struct_field_name_threshold: Option<u64>,
    pub suppress_restriction_lint_in_const: Option<bool>,
    pub too_large_for_stack: Option<u64>,
    pub too_many_arguments_threshold: Option<u64>,
    pub too_many_lines_threshold: Option<u64>,
    pub trait_assoc_item_kinds_order: Option<SourceItemOrderingTraitAssocItemKinds>,
    pub trivial_copy_size_limit: Option<u64>,
    pub type_complexity_threshold: Option<u64>,
    pub unnecessary_box_size: Option<u64>,
    pub unreadable_literal_lint_fractions: Option<bool>,
    pub upper_case_acronyms_aggressive: Option<bool>,
    pub vec_box_size_threshold: Option<u64>,
    pub verbose_bit_mask_threshold: Option<u64>,
    pub warn_on_all_wildcard_imports: Option<bool>,
    pub warn_unsafe_macro_metavars_in_private_macros: Option<bool>,
}

type ModuleItemGroupingEntries = Vec<(String, Vec<SourceItemOrderingModuleItemKind>)>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArithmeticSideEffectsBinaryEntry {
    pub lhs: String,
    pub rhs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AwaitHoldingInvalidType {
    pub path: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisallowedPath {
    Simple(String),
    Detailed(DisallowedPathDetail),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisallowedPathDetail {
    pub path: String,
    pub reason: Option<String>,
    pub replacement: Option<String>,
    #[serde(rename = "allow-invalid")]
    pub allow_invalid: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisallowedField {
    Simple(String),
    Detailed(DisallowedFieldDetail),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisallowedFieldDetail {
    pub path: String,
    pub reason: Option<String>,
    #[serde(rename = "allow-invalid")]
    pub allow_invalid: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameEntry {
    pub path: String,
    pub rename: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroBraceEntry {
    pub name: String,
    pub brace: char,
}

impl<'de> Deserialize<'de> for MacroBraceEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawMacroBraceEntry {
            name: String,
            brace: char,
        }

        let raw = RawMacroBraceEntry::deserialize(deserializer)?;
        match raw.brace {
            '(' | '{' | '[' => Ok(Self {
                name: raw.name,
                brace: raw.brace,
            }),
            other => Err(de::Error::custom(format!(
                "expected one of `(`, `{{`, `[` found `{other}`"
            ))),
        }
    }
}

impl Serialize for MacroBraceEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        #[derive(Serialize)]
        struct RawMacroBraceEntry<'a> {
            name: &'a str,
            brace: char,
        }

        RawMacroBraceEntry {
            name: &self.name,
            brace: self.brace,
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum MatchLintBehaviour {
    AllTypes,
    WellKnownTypes,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum PubUnderscoreFieldsBehaviour {
    PubliclyExported,
    AllPubFields,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InherentImplLintScope {
    Crate,
    File,
    Module,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceItemOrderingCategory {
    Enum,
    Impl,
    Module,
    Struct,
    Trait,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceItemOrdering(pub Vec<SourceItemOrderingCategory>);

impl<'de> Deserialize<'de> for SourceItemOrdering {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items = Vec::<SourceItemOrderingCategory>::deserialize(deserializer)?;
        let mut items_set = BTreeSet::new();

        for item in &items {
            if !items_set.insert(*item) {
                return Err(de::Error::custom(format!(
                    "The category \"{item:?}\" was enabled more than once in the source ordering configuration."
                )));
            }
        }

        Ok(Self(items))
    }
}

impl Serialize for SourceItemOrdering {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceItemOrderingModuleItemKind {
    ExternCrate,
    Mod,
    ForeignMod,
    Use,
    Macro,
    GlobalAsm,
    Static,
    Const,
    TyAlias,
    Enum,
    Struct,
    Union,
    Trait,
    TraitAlias,
    Impl,
    Fn,
}

impl SourceItemOrderingModuleItemKind {
    const ALL: [Self; 16] = [
        Self::ExternCrate,
        Self::Mod,
        Self::ForeignMod,
        Self::Use,
        Self::Macro,
        Self::GlobalAsm,
        Self::Static,
        Self::Const,
        Self::TyAlias,
        Self::Enum,
        Self::Struct,
        Self::Union,
        Self::Trait,
        Self::TraitAlias,
        Self::Impl,
        Self::Fn,
    ];
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceItemOrderingModuleItemGroupings(pub ModuleItemGroupingEntries);

impl<'de> Deserialize<'de> for SourceItemOrderingModuleItemGroupings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let groups = ModuleItemGroupingEntries::deserialize(deserializer)?;
        let items_total: usize = groups.iter().map(|(_, v)| v.len()).sum();
        let mut seen = BTreeSet::new();

        for (_, items) in &groups {
            for item in items {
                let _ = seen.insert(*item);
            }
        }

        let all_items = SourceItemOrderingModuleItemKind::ALL;
        if seen.len() == all_items.len() && items_total == all_items.len() {
            let use_group = groups
                .iter()
                .find(|(_, items)| items.contains(&SourceItemOrderingModuleItemKind::Use))
                .ok_or_else(|| de::Error::custom("Error in internal LUT."))?;

            if use_group.1.len() > 1 {
                return Err(de::Error::custom(
                    "The group containing the \"use\" item kind may not contain any other item kinds. The \"use\" items will (generally) be sorted by rustfmt already. Therefore it makes no sense to implement linting rules that may conflict with rustfmt.",
                ));
            }

            Ok(Self(groups))
        } else if items_total != all_items.len() {
            Err(de::Error::custom(format!(
                "Some module item kinds were configured more than once, or were missing, in the source ordering configuration. The module item kinds are: {all_items:?}"
            )))
        } else {
            Err(de::Error::custom(format!(
                "Not all module item kinds were part of the configured source ordering rule. All item kinds must be provided in the config, otherwise the required source ordering would remain ambiguous. The module item kinds are: {all_items:?}"
            )))
        }
    }
}

impl Serialize for SourceItemOrderingModuleItemGroupings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceItemOrderingWithinModuleItemGroupings {
    All,
    None,
    Custom(Vec<String>),
}

impl<'de> Deserialize<'de> for SourceItemOrderingWithinModuleItemGroupings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrVec {
            String(String),
            Vec(Vec<String>),
        }

        let description = "The available options for configuring an ordering within module item groups are: \"all\", \"none\", or a list of module item group names (as configured with the `module-item-order-groupings` configuration option).";

        match StringOrVec::deserialize(deserializer) {
            Ok(StringOrVec::String(preset)) if preset == "all" => Ok(Self::All),
            Ok(StringOrVec::String(preset)) if preset == "none" => Ok(Self::None),
            Ok(StringOrVec::String(preset)) => Err(de::Error::custom(format!(
                "Unknown configuration option: {preset}.\n{description}"
            ))),
            Ok(StringOrVec::Vec(groupings)) => Ok(Self::Custom(groupings)),
            Err(error) => Err(de::Error::custom(format!("{error}\n{description}"))),
        }
    }
}

impl Serialize for SourceItemOrderingWithinModuleItemGroupings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::None => serializer.serialize_str("none"),
            Self::Custom(groups) => groups.serialize(serializer),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceItemOrderingTraitAssocItemKind {
    Const,
    Fn,
    Type,
}

impl SourceItemOrderingTraitAssocItemKind {
    const ALL: [Self; 3] = [Self::Const, Self::Fn, Self::Type];
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceItemOrderingTraitAssocItemKinds(pub Vec<SourceItemOrderingTraitAssocItemKind>);

impl<'de> Deserialize<'de> for SourceItemOrderingTraitAssocItemKinds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items = Vec::<SourceItemOrderingTraitAssocItemKind>::deserialize(deserializer)?;
        let mut expected_items = SourceItemOrderingTraitAssocItemKind::ALL.to_vec();
        for item in &items {
            expected_items.retain(|i| i != item);
        }

        let all_items = SourceItemOrderingTraitAssocItemKind::ALL;
        if expected_items.is_empty() && items.len() == all_items.len() {
            Ok(Self(items))
        } else if items.len() != all_items.len() {
            Err(de::Error::custom(format!(
                "Some trait associated item kinds were configured more than once, or were missing, in the source ordering configuration. The trait associated item kinds are: {all_items:?}",
            )))
        } else {
            Err(de::Error::custom(format!(
                "Not all trait associated item kinds were part of the configured source ordering rule. All item kinds must be provided in the config, otherwise the required source ordering would remain ambiguous. The trait associated item kinds are: {all_items:?}"
            )))
        }
    }
}

impl Serialize for SourceItemOrderingTraitAssocItemKinds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}
