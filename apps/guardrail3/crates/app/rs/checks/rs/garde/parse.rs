use std::collections::{BTreeMap, BTreeSet};

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;

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

#[derive(Debug, Clone, Default)]
pub struct ParsedGardeFile {
    pub derived_types: Vec<DerivedBoundaryType>,
    pub manual_deserialize_impls: Vec<ManualImpl>,
    pub manual_validate_impls: BTreeSet<String>,
    pub type_validation_map: BTreeMap<String, (bool, bool)>,
    pub query_as_macros: Vec<QueryAsMacro>,
}

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn analyze(ast: &syn::File) -> ParsedGardeFile {
    let mut visitor = GardeVisitor::default();
    visitor.visit_file(ast);
    visitor.finish()
}

#[derive(Default)]
struct GardeVisitor {
    derived_types: Vec<DerivedBoundaryType>,
    manual_deserialize_impls: Vec<ManualImpl>,
    manual_validate_impls: BTreeSet<String>,
    type_validation_map: BTreeMap<String, (bool, bool)>,
    query_as_macros: Vec<QueryAsMacro>,
    module_stack: Vec<String>,
    deserialize_aliases: BTreeSet<String>,
    validate_aliases: BTreeSet<String>,
    query_as_aliases: BTreeSet<String>,
}

impl GardeVisitor {
    fn finish(self) -> ParsedGardeFile {
        ParsedGardeFile {
            derived_types: self.derived_types,
            manual_deserialize_impls: self.manual_deserialize_impls,
            manual_validate_impls: self.manual_validate_impls,
            type_validation_map: self.type_validation_map,
            query_as_macros: self.query_as_macros,
        }
    }
}

impl<'ast> Visit<'ast> for GardeVisitor {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_aliases(
            &item.tree,
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
        let has_boundary = macros.iter().any(|name| is_input_boundary_derive(name));
        let has_validate = macros.iter().any(|name| is_validate_derive(name));
        let has_non_primitive_fields = struct_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| is_input_boundary_derive(name))
                .cloned()
                .collect();
            self.derived_types.push(DerivedBoundaryType {
                line: span_line(item.span()),
                name: name.clone(),
                boundary_kind: BoundaryKind::Struct,
                boundary_macros,
                has_validate_derive: has_validate,
                has_non_primitive_fields,
            });
        }

        let _ = self
            .type_validation_map
            .insert(name, (has_non_primitive_fields, has_validate));

        syn::visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        let macros = derive_macros(&item.attrs);
        let has_boundary = macros.iter().any(|name| is_input_boundary_derive(name));
        let has_validate = macros.iter().any(|name| is_validate_derive(name));
        let has_non_primitive_fields = enum_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| is_input_boundary_derive(name))
                .cloned()
                .collect();
            self.derived_types.push(DerivedBoundaryType {
                line: span_line(item.span()),
                name: name.clone(),
                boundary_kind: BoundaryKind::Enum,
                boundary_macros,
                has_validate_derive: has_validate,
                has_non_primitive_fields,
            });
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
        let Some(type_name) = self_ty_name(&item.self_ty) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let type_name = self.qualify_type_name(&type_name);
        let trait_name = trait_path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        if trait_name
            .as_deref()
            .is_some_and(|name| name == "Deserialize" || self.deserialize_aliases.contains(name))
        {
            self.manual_deserialize_impls.push(ManualImpl {
                line: span_line(item.span()),
                type_name: type_name.clone(),
            });
        }
        if trait_name
            .as_deref()
            .is_some_and(|name| name == "Validate" || self.validate_aliases.contains(name))
        {
            let _ = self.manual_validate_impls.insert(type_name);
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let macro_name = path_to_string(&mac.path);
        let tail = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        if tail.as_deref().is_some_and(|name| {
            name == "query_as"
                || name == "query_as_unchecked"
                || self.query_as_aliases.contains(name)
        }) {
            self.query_as_macros.push(QueryAsMacro {
                line: span_line(mac.span()),
                macro_name,
            });
        }
        syn::visit::visit_macro(self, mac);
    }
}

impl GardeVisitor {
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
                macros.extend(paths.iter().map(path_to_string));
            }
        }
    }
    macros
}

fn is_input_boundary_derive(macro_name: &str) -> bool {
    ["Deserialize", "Parser", "Args", "FromRow"]
        .iter()
        .any(|name| macro_name == *name || macro_name.ends_with(&format!("::{name}")))
}

fn is_validate_derive(macro_name: &str) -> bool {
    macro_name == "Validate" || macro_name.ends_with("::Validate")
}

fn struct_has_non_primitive_fields(item: &syn::ItemStruct) -> bool {
    match &item.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .any(|field| type_needs_validation(&field.ty)),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .any(|field| type_needs_validation(&field.ty)),
        syn::Fields::Unit => false,
    }
}

fn enum_has_non_primitive_fields(item: &syn::ItemEnum) -> bool {
    item.variants.iter().any(|variant| match &variant.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .any(|field| type_needs_validation(&field.ty)),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .any(|field| type_needs_validation(&field.ty)),
        syn::Fields::Unit => false,
    })
}

fn type_needs_validation(ty: &syn::Type) -> bool {
    !type_is_primitive_safe(ty)
}

fn type_is_primitive_safe(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            let Some(last) = type_path.path.segments.last() else {
                return false;
            };
            let ident = last.ident.to_string();
            if matches!(
                ident.as_str(),
                "bool"
                    | "char"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "f32"
                    | "f64"
            ) {
                return true;
            }
            if ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                    if args.args.len() == 1 {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return type_is_primitive_safe(inner);
                        }
                    }
                }
            }
            false
        }
        syn::Type::Tuple(tuple) => tuple.elems.iter().all(type_is_primitive_safe),
        _ => false,
    }
}

fn self_ty_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(type_path) => Some(
            type_path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
        ),
        _ => None,
    }
}

fn collect_use_aliases(
    tree: &syn::UseTree,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Path(path) => collect_use_aliases(
            &path.tree,
            deserialize_aliases,
            validate_aliases,
            query_as_aliases,
        ),
        syn::UseTree::Name(name) => {
            let ident = name.ident.to_string();
            match ident.as_str() {
                "Deserialize" => {
                    let _ = deserialize_aliases.insert("Deserialize".to_owned());
                }
                "Validate" => {
                    let _ = validate_aliases.insert("Validate".to_owned());
                }
                "query_as" | "query_as_unchecked" => {
                    let _ = query_as_aliases.insert(ident);
                }
                _ => {}
            }
        }
        syn::UseTree::Rename(rename) => {
            let target = rename.ident.to_string();
            let alias = rename.rename.to_string();
            match target.as_str() {
                "Deserialize" => {
                    let _ = deserialize_aliases.insert(alias);
                }
                "Validate" => {
                    let _ = validate_aliases.insert(alias);
                }
                "query_as" | "query_as_unchecked" => {
                    let _ = query_as_aliases.insert(alias);
                }
                _ => {}
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_aliases(
                    item,
                    deserialize_aliases,
                    validate_aliases,
                    query_as_aliases,
                );
            }
        }
        _ => {}
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.to_token_stream().to_string().replace(' ', "")
}

fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}
