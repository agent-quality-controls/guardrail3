#![expect(
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::arithmetic_side_effects,
    clippy::shadow_unrelated,
    reason = "boundary-field summary captures four orthogonal field facts that each map to distinct rule findings (skip, dive, validation, context); the typed (bool, bool) entries in the type-validation map encode the (has_non_primitive, has_validate_derive) pair the rules consume; `cfg_test_depth += 1` increments a small recursion counter where a saturating add would mask a bug; the `name` rebinding inside an iterator closure is intentional shadowing of the outer canonical name with the closure-local name being filtered"
)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum `BoundaryKind` used by this module.
pub(crate) enum BoundaryKind {
    /// Variant `Struct`.
    Struct,
    /// Variant `Enum`.
    Enum,
}

#[derive(Debug, Clone)]
/// Struct `DerivedBoundaryType` used by this module.
pub(crate) struct DerivedBoundaryType {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `name`.
    pub(crate) name: String,
    /// Field `boundary_kind`.
    pub(crate) boundary_kind: BoundaryKind,
    /// Field `boundary_macros`.
    pub(crate) boundary_macros: Vec<String>,
    /// Field `has_validate_derive`.
    pub(crate) has_validate_derive: bool,
    /// Field `has_non_primitive_fields`.
    pub(crate) has_non_primitive_fields: bool,
}

#[derive(Debug, Clone)]
/// Struct `ManualImpl` used by this module.
pub(crate) struct ManualImpl {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `type_name`.
    pub(crate) type_name: String,
}

#[derive(Debug, Clone)]
/// Struct `QueryAsMacro` used by this module.
pub(crate) struct QueryAsMacro {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `macro_name`.
    pub(crate) macro_name: String,
}

#[derive(Debug, Clone)]
/// Struct `BoundaryField` used by this module.
pub(crate) struct BoundaryField {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `boundary_name`.
    pub(crate) boundary_name: String,
    /// Field `field_name`.
    pub(crate) field_name: String,
    /// Field `field_type`.
    pub(crate) field_type: String,
    /// Field `candidate_type_names`.
    pub(crate) candidate_type_names: Vec<String>,
    /// Field `boundary_has_validate_derive`.
    pub(crate) boundary_has_validate_derive: bool,
    /// Field `boundary_has_context`.
    pub(crate) boundary_has_context: bool,
    /// Field `requires_field_validation`.
    pub(crate) requires_field_validation: bool,
    /// Field `has_garde_skip`.
    pub(crate) has_garde_skip: bool,
    /// Field `has_garde_dive`.
    pub(crate) has_garde_dive: bool,
    /// Field `has_meaningful_garde_rule`.
    pub(crate) has_meaningful_garde_rule: bool,
    /// Field `uses_context`.
    pub(crate) uses_context: bool,
}

#[derive(Debug, Clone, Default)]
/// Struct `ParsedGardeFile` used by this module.
pub(crate) struct ParsedGardeFile {
    /// Field `derived_types`.
    pub(crate) derived_types: Vec<DerivedBoundaryType>,
    /// Field `manual_deserialize_impls`.
    pub(crate) manual_deserialize_impls: Vec<ManualImpl>,
    /// Field `manual_validate_impls`.
    pub(crate) manual_validate_impls: std::collections::BTreeSet<String>,
    /// Field `type_validation_map`.
    pub(crate) type_validation_map: std::collections::BTreeMap<String, (bool, bool)>,
    /// Field `boundary_fields`.
    pub(crate) boundary_fields: Vec<BoundaryField>,
    /// Field `query_as_macros`.
    pub(crate) query_as_macros: Vec<QueryAsMacro>,
}

/// Implements `parse rust file`.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub(crate) fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

/// Implements `analyze`.
pub(crate) fn analyze(source: &syn::File) -> ParsedGardeFile {
    let mut visitor = GardeVisitor::default();
    syn::visit::Visit::visit_file(&mut visitor, source);
    visitor.finish()
}

#[derive(Default)]
/// Struct `GardeVisitor` used by this module.
struct GardeVisitor {
    /// Field `derived_types`.
    derived_types: Vec<DerivedBoundaryType>,
    /// Field `manual_deserialize_impls`.
    manual_deserialize_impls: Vec<ManualImpl>,
    /// Field `manual_validate_impls`.
    manual_validate_impls: std::collections::BTreeSet<String>,
    /// Field `type_validation_map`.
    type_validation_map: std::collections::BTreeMap<String, (bool, bool)>,
    /// Field `boundary_fields`.
    boundary_fields: Vec<BoundaryField>,
    /// Field `query_as_macros`.
    query_as_macros: Vec<QueryAsMacro>,
    /// Field `module_stack`.
    module_stack: Vec<String>,
    /// Field `boundary_derive_aliases`.
    boundary_derive_aliases: std::collections::BTreeSet<String>,
    /// Field `deserialize_aliases`.
    deserialize_aliases: std::collections::BTreeSet<String>,
    /// Field `validate_aliases`.
    validate_aliases: std::collections::BTreeSet<String>,
    /// Field `query_as_aliases`.
    query_as_aliases: std::collections::BTreeSet<String>,
    /// Field `module_path_aliases`.
    module_path_aliases: std::collections::BTreeMap<String, String>,
    /// Field `cfg_test_depth`.
    cfg_test_depth: usize,
}

impl GardeVisitor {
    /// Implements `finish`.
    fn finish(self) -> ParsedGardeFile {
        ParsedGardeFile {
            derived_types: self.derived_types,
            manual_deserialize_impls: self.manual_deserialize_impls,
            manual_validate_impls: self.manual_validate_impls,
            type_validation_map: self.type_validation_map,
            boundary_fields: self.boundary_fields,
            query_as_macros: self.query_as_macros,
        }
    }

    /// Implements `qualified name`.
    fn qualified_name(&self, name: &str) -> String {
        if self.module_stack.is_empty() {
            name.to_owned()
        } else {
            format!("{}::{name}", self.module_stack.join("::"))
        }
    }

    /// Implements `qualify type name`.
    fn qualify_type_name(&self, name: &str) -> String {
        if name.contains("::") || self.module_stack.is_empty() {
            name.to_owned()
        } else {
            self.qualified_name(name)
        }
    }

    /// Implements `is input boundary derive`.
    fn is_input_boundary_derive(&self, macro_name: &str) -> bool {
        let resolved =
            super::aliases::resolve_path_string_aliases(macro_name, &self.module_path_aliases);
        is_input_boundary_derive(&resolved) || self.boundary_derive_aliases.contains(macro_name)
    }

    /// Implements `is validate derive`.
    fn is_validate_derive(&self, macro_name: &str) -> bool {
        let resolved =
            super::aliases::resolve_path_string_aliases(macro_name, &self.module_path_aliases);
        is_validate_derive(&resolved) || self.validate_aliases.contains(macro_name)
    }
}

impl<'source> syn::visit::Visit<'source> for GardeVisitor {
    fn visit_item_use(&mut self, item: &'source syn::ItemUse) {
        super::aliases::collect_use_aliases(
            &item.tree,
            &mut self.boundary_derive_aliases,
            &mut self.deserialize_aliases,
            &mut self.validate_aliases,
            &mut self.query_as_aliases,
            &mut self.module_path_aliases,
        );
        syn::visit::visit_item_use(self, item);
    }

    fn visit_item_mod(&mut self, item: &'source syn::ItemMod) {
        if let Some((_, items)) = &item.content {
            let entered_cfg_test = attrs_contain_cfg_test(&item.attrs);
            if entered_cfg_test {
                self.cfg_test_depth += 1;
            }
            self.module_stack.push(item.ident.to_string());
            for inner in items {
                self.visit_item(inner);
            }
            let _ = self.module_stack.pop();
            if entered_cfg_test {
                self.cfg_test_depth = self.cfg_test_depth.saturating_sub(1);
            }
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_struct(&mut self, item: &'source syn::ItemStruct) {
        let macros = derive_macros(&item.attrs);
        let has_boundary = macros
            .iter()
            .any(|name| self.is_input_boundary_derive(name));
        let has_validate = macros.iter().any(|name| self.is_validate_derive(name));
        let has_non_primitive_fields = super::fields::struct_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());
        let has_context = super::fields::has_garde_context(&item.attrs);

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| self.is_input_boundary_derive(name))
                .cloned()
                .collect();
            self.derived_types.push(DerivedBoundaryType {
                line: super::fields::span_line(syn::spanned::Spanned::span(item)),
                name: name.clone(),
                boundary_kind: BoundaryKind::Struct,
                boundary_macros,
                has_validate_derive: has_validate,
                has_non_primitive_fields,
            });
            self.boundary_fields
                .extend(super::fields::collect_struct_boundary_fields(
                    item,
                    &name,
                    has_validate,
                    has_context,
                ));
        }

        let _ = self
            .type_validation_map
            .insert(name, (has_non_primitive_fields, has_validate));

        syn::visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'source syn::ItemEnum) {
        let macros = derive_macros(&item.attrs);
        let has_boundary = macros
            .iter()
            .any(|name| self.is_input_boundary_derive(name));
        let has_validate = macros.iter().any(|name| self.is_validate_derive(name));
        let has_non_primitive_fields = super::fields::enum_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());
        let has_context = super::fields::has_garde_context(&item.attrs);

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| self.is_input_boundary_derive(name))
                .cloned()
                .collect();
            self.derived_types.push(DerivedBoundaryType {
                line: super::fields::span_line(syn::spanned::Spanned::span(item)),
                name: name.clone(),
                boundary_kind: BoundaryKind::Enum,
                boundary_macros,
                has_validate_derive: has_validate,
                has_non_primitive_fields,
            });
            self.boundary_fields
                .extend(super::fields::collect_enum_boundary_fields(
                    item,
                    &name,
                    has_validate,
                    has_context,
                ));
        }

        let _ = self
            .type_validation_map
            .insert(name, (has_non_primitive_fields, has_validate));

        syn::visit::visit_item_enum(self, item);
    }

    fn visit_item_impl(&mut self, item: &'source syn::ItemImpl) {
        let Some(trait_path) = item.trait_.as_ref().map(|(_, path, _)| path) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let Some(type_name) = super::aliases::self_ty_name(&item.self_ty) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let type_name = self.qualify_type_name(&type_name);
        if super::aliases::is_deserialize_trait_path(
            trait_path,
            &self.deserialize_aliases,
            &self.module_path_aliases,
        ) {
            self.manual_deserialize_impls.push(ManualImpl {
                line: super::fields::span_line(syn::spanned::Spanned::span(item)),
                type_name: type_name.clone(),
            });
        }
        if super::aliases::is_validate_trait_path(
            trait_path,
            &self.validate_aliases,
            &self.module_path_aliases,
        ) {
            let _ = self.manual_validate_impls.insert(type_name);
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_macro(&mut self, mac: &'source syn::Macro) {
        let macro_name = super::aliases::path_to_string(&mac.path);
        let tail = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        let is_sqlx_macro =
            super::aliases::is_sqlx_query_as_macro_path(&mac.path, &self.module_path_aliases);
        let is_imported_sqlx_alias = tail
            .as_deref()
            .is_some_and(|name| self.query_as_aliases.contains(name));
        if is_sqlx_macro || is_imported_sqlx_alias {
            self.query_as_macros.push(QueryAsMacro {
                line: super::fields::span_line(syn::spanned::Spanned::span(mac)),
                macro_name,
            });
        }
        syn::visit::visit_macro(self, mac);
    }
}

/// Implements `derive macros`.
fn derive_macros(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut macros = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        if let syn::Meta::List(list) = &attr.meta {
            if let Ok(paths) = list.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) {
                macros.extend(paths.iter().map(super::aliases::path_to_string));
            }
        }
    }
    macros
}

/// Implements `is input boundary derive`.
fn is_input_boundary_derive(macro_name: &str) -> bool {
    matches!(
        macro_name.trim_start_matches(':'),
        "Deserialize"
            | "serde::Deserialize"
            | "Parser"
            | "clap::Parser"
            | "Args"
            | "clap::Args"
            | "FromRow"
            | "sqlx::FromRow"
    )
}

/// Implements `is validate derive`.
fn is_validate_derive(macro_name: &str) -> bool {
    matches!(
        macro_name.trim_start_matches(':'),
        "Validate" | "garde::Validate"
    )
}

/// Implements `attrs contain cfg test`.
fn attrs_contain_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg"))
        .any(|attr| meta_contains_test(&attr.meta))
}

/// Implements `meta contains test`.
fn meta_contains_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .map(|items| items.iter().any(meta_contains_test))
            .unwrap_or(false),
        syn::Meta::NameValue(_) => false,
    }
}
