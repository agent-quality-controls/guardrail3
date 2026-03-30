mod aliases;
mod fields;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryKind {
    Struct,
    Enum,
}

#[derive(Debug, Clone)]
pub struct DerivedBoundaryType {
    pub line: usize,
    pub name: String,
    pub boundary_kind: BoundaryKind,
    pub boundary_macros: Vec<String>,
    pub has_validate_derive: bool,
    pub has_non_primitive_fields: bool,
}

#[derive(Debug, Clone)]
pub struct ManualImpl {
    pub line: usize,
    pub type_name: String,
}

#[derive(Debug, Clone)]
pub struct QueryAsMacro {
    pub line: usize,
    pub macro_name: String,
}

#[derive(Debug, Clone)]
pub struct BoundaryField {
    pub line: usize,
    pub boundary_name: String,
    pub field_name: String,
    pub field_type: String,
    pub candidate_type_names: Vec<String>,
    pub boundary_has_validate_derive: bool,
    pub boundary_has_context: bool,
    pub requires_field_validation: bool,
    pub has_garde_skip: bool,
    pub has_garde_dive: bool,
    pub has_meaningful_garde_rule: bool,
    pub uses_context: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedGardeFile {
    pub derived_types: Vec<DerivedBoundaryType>,
    pub manual_deserialize_impls: Vec<ManualImpl>,
    pub manual_validate_impls: std::collections::BTreeSet<String>,
    pub type_validation_map: std::collections::BTreeMap<String, (bool, bool)>,
    pub boundary_fields: Vec<BoundaryField>,
    pub query_as_macros: Vec<QueryAsMacro>,
}

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn analyze(ast: &syn::File) -> ParsedGardeFile {
    let mut visitor = GardeVisitor::default();
    syn::visit::Visit::visit_file(&mut visitor, ast);
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
        is_input_boundary_derive(macro_name) || self.boundary_derive_aliases.contains(macro_name)
    }

    fn is_validate_derive(&self, macro_name: &str) -> bool {
        is_validate_derive(macro_name) || self.validate_aliases.contains(macro_name)
    }
}

impl<'ast> syn::visit::Visit<'ast> for GardeVisitor {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        aliases::collect_use_aliases(
            &item.tree,
            &mut self.boundary_derive_aliases,
            &mut self.deserialize_aliases,
            &mut self.validate_aliases,
            &mut self.query_as_aliases,
        );
        syn::visit::visit_item_use(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if let Some((_, items)) = &item.content {
            self.module_stack.push(item.ident.to_string());
            for inner in items {
                self.visit_item(inner);
            }
            let _ = self.module_stack.pop();
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        let macros = derive_macros(&item.attrs);
        let has_boundary = macros
            .iter()
            .any(|name| self.is_input_boundary_derive(name));
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
            self.boundary_fields.extend(fields::collect_struct_boundary_fields(
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

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        let macros = derive_macros(&item.attrs);
        let has_boundary = macros
            .iter()
            .any(|name| self.is_input_boundary_derive(name));
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
            self.boundary_fields.extend(fields::collect_enum_boundary_fields(
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

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let Some(trait_path) = item.trait_.as_ref().map(|(_, path, _)| path) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let Some(type_name) = aliases::self_ty_name(&item.self_ty) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let type_name = self.qualify_type_name(&type_name);
        if aliases::is_deserialize_trait_path(trait_path, &self.deserialize_aliases) {
            self.manual_deserialize_impls.push(ManualImpl {
                line: fields::span_line(syn::spanned::Spanned::span(item)),
                type_name: type_name.clone(),
            });
        }
        if aliases::is_validate_trait_path(trait_path, &self.validate_aliases) {
            let _ = self.manual_validate_impls.insert(type_name);
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let macro_name = aliases::path_to_string(&mac.path);
        let tail = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        let is_sqlx_macro = aliases::is_sqlx_query_as_macro_path(&mac.path);
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
