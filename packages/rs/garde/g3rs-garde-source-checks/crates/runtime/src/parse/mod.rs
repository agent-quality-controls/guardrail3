mod aliases;
mod fields;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoundaryKind {
    Struct,
    Enum,
}

#[derive(Debug, Clone)]
pub(crate) struct DerivedBoundaryType {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) boundary_kind: BoundaryKind,
    pub(crate) boundary_macros: Vec<String>,
    pub(crate) has_validate_derive: bool,
    pub(crate) has_non_primitive_fields: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ManualImpl {
    pub(crate) line: usize,
    pub(crate) type_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct QueryAsMacro {
    pub(crate) line: usize,
    pub(crate) macro_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct BoundaryField {
    pub(crate) line: usize,
    pub(crate) boundary_name: String,
    pub(crate) field_name: String,
    pub(crate) field_type: String,
    pub(crate) candidate_type_names: Vec<String>,
    pub(crate) boundary_has_validate_derive: bool,
    pub(crate) boundary_has_context: bool,
    pub(crate) requires_field_validation: bool,
    pub(crate) has_garde_skip: bool,
    pub(crate) has_garde_dive: bool,
    pub(crate) has_meaningful_garde_rule: bool,
    pub(crate) uses_context: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ParsedGardeFile {
    pub(crate) derived_types: Vec<DerivedBoundaryType>,
    pub(crate) manual_deserialize_impls: Vec<ManualImpl>,
    pub(crate) manual_validate_impls: std::collections::BTreeSet<String>,
    pub(crate) type_validation_map: std::collections::BTreeMap<String, (bool, bool)>,
    pub(crate) boundary_fields: Vec<BoundaryField>,
    pub(crate) query_as_macros: Vec<QueryAsMacro>,
}

pub(crate) fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub(crate) fn analyze(source: &syn::File) -> ParsedGardeFile {
    let mut visitor = GardeVisitor::default();
    syn::visit::Visit::visit_file(&mut visitor, source);
    visitor.finish()
}

#[derive(Default)]
struct GardeVisitor {
    derived_types: Vec<DerivedBoundaryType>,
    manual_deserialize_impls: Vec<ManualImpl>,
    manual_validate_impls: std::collections::BTreeSet<String>,
    type_validation_map: std::collections::BTreeMap<String, (bool, bool)>,
    boundary_fields: Vec<BoundaryField>,
    query_as_macros: Vec<QueryAsMacro>,
    module_stack: Vec<String>,
    boundary_derive_aliases: std::collections::BTreeSet<String>,
    deserialize_aliases: std::collections::BTreeSet<String>,
    validate_aliases: std::collections::BTreeSet<String>,
    query_as_aliases: std::collections::BTreeSet<String>,
    module_path_aliases: std::collections::BTreeMap<String, String>,
    cfg_test_depth: usize,
}

impl GardeVisitor {
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

    fn qualified_name(&self, name: &str) -> String {
        if self.module_stack.is_empty() {
            name.to_owned()
        } else {
            format!("{}::{name}", self.module_stack.join("::"))
        }
    }

    fn qualify_type_name(&self, name: &str) -> String {
        if name.contains("::") || self.module_stack.is_empty() {
            name.to_owned()
        } else {
            self.qualified_name(name)
        }
    }

    fn is_input_boundary_derive(&self, macro_name: &str) -> bool {
        let resolved = aliases::resolve_path_string_aliases(macro_name, &self.module_path_aliases);
        is_input_boundary_derive(&resolved) || self.boundary_derive_aliases.contains(macro_name)
    }

    fn is_validate_derive(&self, macro_name: &str) -> bool {
        let resolved = aliases::resolve_path_string_aliases(macro_name, &self.module_path_aliases);
        is_validate_derive(&resolved) || self.validate_aliases.contains(macro_name)
    }
}

impl<'source> syn::visit::Visit<'source> for GardeVisitor {
    fn visit_item_use(&mut self, item: &'source syn::ItemUse) {
        aliases::collect_use_aliases(
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
        let has_boundary = macros.iter().any(|name| self.is_input_boundary_derive(name));
        let has_validate = macros.iter().any(|name| self.is_validate_derive(name));
        let has_non_primitive_fields = fields::struct_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());
        let has_context = fields::has_garde_context(&item.attrs);

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| self.is_input_boundary_derive(name))
                .cloned()
                .collect();
            self.derived_types.push(DerivedBoundaryType {
                line: fields::span_line(syn::spanned::Spanned::span(item)),
                name: name.clone(),
                boundary_kind: BoundaryKind::Struct,
                boundary_macros,
                has_validate_derive: has_validate,
                has_non_primitive_fields,
            });
            self.boundary_fields
                .extend(fields::collect_struct_boundary_fields(
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
        let has_boundary = macros.iter().any(|name| self.is_input_boundary_derive(name));
        let has_validate = macros.iter().any(|name| self.is_validate_derive(name));
        let has_non_primitive_fields = fields::enum_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());
        let has_context = fields::has_garde_context(&item.attrs);

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| self.is_input_boundary_derive(name))
                .cloned()
                .collect();
            self.derived_types.push(DerivedBoundaryType {
                line: fields::span_line(syn::spanned::Spanned::span(item)),
                name: name.clone(),
                boundary_kind: BoundaryKind::Enum,
                boundary_macros,
                has_validate_derive: has_validate,
                has_non_primitive_fields,
            });
            self.boundary_fields
                .extend(fields::collect_enum_boundary_fields(
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
        let Some(type_name) = aliases::self_ty_name(&item.self_ty) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let type_name = self.qualify_type_name(&type_name);
        if aliases::is_deserialize_trait_path(
            trait_path,
            &self.deserialize_aliases,
            &self.module_path_aliases,
        ) {
            self.manual_deserialize_impls.push(ManualImpl {
                line: fields::span_line(syn::spanned::Spanned::span(item)),
                type_name: type_name.clone(),
            });
        }
        if aliases::is_validate_trait_path(
            trait_path,
            &self.validate_aliases,
            &self.module_path_aliases,
        ) {
            let _ = self.manual_validate_impls.insert(type_name);
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_macro(&mut self, mac: &'source syn::Macro) {
        let macro_name = aliases::path_to_string(&mac.path);
        let tail = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        let is_sqlx_macro =
            aliases::is_sqlx_query_as_macro_path(&mac.path, &self.module_path_aliases);
        let is_imported_sqlx_alias = tail
            .as_deref()
            .is_some_and(|name| self.query_as_aliases.contains(name));
        if is_sqlx_macro || is_imported_sqlx_alias {
            self.query_as_macros.push(QueryAsMacro {
                line: fields::span_line(syn::spanned::Spanned::span(mac)),
                macro_name,
            });
        }
        syn::visit::visit_macro(self, mac);
    }
}

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
                macros.extend(paths.iter().map(aliases::path_to_string));
            }
        }
    }
    macros
}

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

fn is_validate_derive(macro_name: &str) -> bool {
    matches!(
        macro_name.trim_start_matches(':'),
        "Validate" | "garde::Validate"
    )
}

fn attrs_contain_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg"))
        .any(|attr| meta_contains_test(&attr.meta))
}

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
