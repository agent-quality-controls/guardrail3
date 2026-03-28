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
    pub manual_validate_impls: BTreeSet<String>,
    pub type_validation_map: BTreeMap<String, (bool, bool)>,
    pub boundary_fields: Vec<BoundaryField>,
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
    boundary_fields: Vec<BoundaryField>,
    query_as_macros: Vec<QueryAsMacro>,
    module_stack: Vec<String>,
    boundary_derive_aliases: BTreeSet<String>,
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
            boundary_fields: self.boundary_fields,
            query_as_macros: self.query_as_macros,
        }
    }
}

impl<'ast> Visit<'ast> for GardeVisitor {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_aliases(
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
        let has_non_primitive_fields = struct_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());
        let has_context = has_garde_context(&item.attrs);

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| self.is_input_boundary_derive(name))
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
            self.boundary_fields.extend(collect_struct_boundary_fields(
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
        let has_non_primitive_fields = enum_has_non_primitive_fields(item);
        let name = self.qualified_name(&item.ident.to_string());
        let has_context = has_garde_context(&item.attrs);

        if has_boundary {
            let boundary_macros = macros
                .iter()
                .filter(|name| self.is_input_boundary_derive(name))
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
            self.boundary_fields.extend(collect_enum_boundary_fields(
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
        let Some(type_name) = self_ty_name(&item.self_ty) else {
            syn::visit::visit_item_impl(self, item);
            return;
        };
        let type_name = self.qualify_type_name(&type_name);
        if is_deserialize_trait_path(trait_path, &self.deserialize_aliases) {
            self.manual_deserialize_impls.push(ManualImpl {
                line: span_line(item.span()),
                type_name: type_name.clone(),
            });
        }
        if is_validate_trait_path(trait_path, &self.validate_aliases) {
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
        let is_sqlx_macro = is_sqlx_query_as_macro_path(&mac.path);
        let is_imported_sqlx_alias = tail
            .as_deref()
            .is_some_and(|name| self.query_as_aliases.contains(name));
        if is_sqlx_macro || is_imported_sqlx_alias {
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

    fn is_input_boundary_derive(&self, macro_name: &str) -> bool {
        is_input_boundary_derive(macro_name) || self.boundary_derive_aliases.contains(macro_name)
    }

    fn is_validate_derive(&self, macro_name: &str) -> bool {
        is_validate_derive(macro_name) || self.validate_aliases.contains(macro_name)
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

#[derive(Default)]
struct GardeAttrSummary {
    has_skip: bool,
    has_dive: bool,
    has_meaningful_rule: bool,
    uses_context: bool,
}

fn has_garde_context(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("garde"))
        .any(|attr| {
            let mut has_context = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("context") {
                    has_context = true;
                }
                Ok(())
            });
            has_context
        })
}

fn summarize_garde_attrs(attrs: &[syn::Attribute]) -> GardeAttrSummary {
    let mut summary = GardeAttrSummary::default();
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("garde")) {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("dive") {
                summary.has_dive = true;
            } else if meta.path.is_ident("skip") {
                summary.has_skip = true;
                summary.has_meaningful_rule = true;
            } else if !meta.path.is_ident("skip") && !meta.path.is_ident("context") {
                summary.has_meaningful_rule = true;
            }
            if meta
                .input
                .fork()
                .parse::<proc_macro2::TokenStream>()
                .ok()
                .is_some_and(token_stream_uses_ctx_variable)
            {
                summary.uses_context = true;
            }
            Ok(())
        });
    }
    summary
}

fn collect_struct_boundary_fields(
    item: &syn::ItemStruct,
    boundary_name: &str,
    boundary_has_validate_derive: bool,
    boundary_has_context: bool,
) -> Vec<BoundaryField> {
    collect_fields(
        &item.fields,
        boundary_name,
        boundary_has_validate_derive,
        boundary_has_context,
        None,
    )
}

fn collect_enum_boundary_fields(
    item: &syn::ItemEnum,
    boundary_name: &str,
    boundary_has_validate_derive: bool,
    boundary_has_context: bool,
) -> Vec<BoundaryField> {
    item.variants
        .iter()
        .flat_map(|variant| {
            collect_fields(
                &variant.fields,
                boundary_name,
                boundary_has_validate_derive,
                boundary_has_context,
                Some(variant.ident.to_string()),
            )
        })
        .collect()
}

fn collect_fields(
    fields: &syn::Fields,
    boundary_name: &str,
    boundary_has_validate_derive: bool,
    boundary_has_context: bool,
    variant_name: Option<String>,
) -> Vec<BoundaryField> {
    match fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_name = field
                    .ident
                    .as_ref()
                    .map(std::string::ToString::to_string)
                    .expect("named field");
                boundary_field(
                    field,
                    boundary_name,
                    field_name,
                    boundary_has_validate_derive,
                    boundary_has_context,
                    variant_name.as_deref(),
                )
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                boundary_field(
                    field,
                    boundary_name,
                    index.to_string(),
                    boundary_has_validate_derive,
                    boundary_has_context,
                    variant_name.as_deref(),
                )
            })
            .collect(),
        syn::Fields::Unit => Vec::new(),
    }
}

fn boundary_field(
    field: &syn::Field,
    boundary_name: &str,
    field_name: String,
    boundary_has_validate_derive: bool,
    boundary_has_context: bool,
    variant_name: Option<&str>,
) -> BoundaryField {
    let summary = summarize_garde_attrs(&field.attrs);
    let qualified_field_name = variant_name
        .map(|variant| format!("{variant}::{field_name}"))
        .unwrap_or(field_name);
    BoundaryField {
        line: span_line(field.span()),
        boundary_name: boundary_name.to_owned(),
        field_name: qualified_field_name,
        field_type: type_to_string(&field.ty),
        candidate_type_names: collect_candidate_type_names(&field.ty),
        boundary_has_validate_derive,
        boundary_has_context,
        requires_field_validation: type_requires_field_validation(&field.ty),
        has_garde_skip: summary.has_skip,
        has_garde_dive: summary.has_dive,
        has_meaningful_garde_rule: summary.has_meaningful_rule,
        uses_context: summary.uses_context,
    }
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

fn type_requires_field_validation(ty: &syn::Type) -> bool {
    !type_is_primitive_safe(ty) && !type_is_unvalidatable(ty)
}

fn type_is_primitive_safe(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Array(array) => type_is_primitive_safe(&array.elem),
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

fn type_is_unvalidatable(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            let Some(last) = type_path.path.segments.last() else {
                return false;
            };
            let ident = last.ident.to_string();
            if matches!(
                ident.as_str(),
                "BTreeMap"
                    | "HashMap"
                    | "IndexMap"
                    | "DashMap"
                    | "BTreeSet"
                    | "HashSet"
                    | "IndexSet"
            ) {
                return true;
            }
            if ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                    if args.args.len() == 1 {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return type_is_unvalidatable(inner);
                        }
                    }
                }
            }
            false
        }
        syn::Type::Reference(_) | syn::Type::TraitObject(_) => true,
        _ => false,
    }
}

fn collect_candidate_type_names(ty: &syn::Type) -> Vec<String> {
    let mut out = Vec::new();
    collect_candidate_type_names_inner(ty, &mut out);
    out
}

fn collect_candidate_type_names_inner(ty: &syn::Type, out: &mut Vec<String>) {
    match ty {
        syn::Type::Array(array) => collect_candidate_type_names_inner(&array.elem, out),
        syn::Type::Path(type_path) => {
            let Some(last) = type_path.path.segments.last() else {
                return;
            };
            let ident = last.ident.to_string();
            if matches!(
                ident.as_str(),
                "Option"
                    | "Vec"
                    | "VecDeque"
                    | "HashMap"
                    | "BTreeMap"
                    | "HashSet"
                    | "BTreeSet"
                    | "Box"
                    | "Rc"
                    | "Arc"
            ) {
                if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner) = arg {
                            collect_candidate_type_names_inner(inner, out);
                        }
                    }
                }
            } else {
                out.push(path_to_string(&type_path.path));
            }
        }
        syn::Type::Reference(reference) => collect_candidate_type_names_inner(&reference.elem, out),
        syn::Type::Paren(inner) => collect_candidate_type_names_inner(&inner.elem, out),
        syn::Type::Group(inner) => collect_candidate_type_names_inner(&inner.elem, out),
        syn::Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                collect_candidate_type_names_inner(elem, out);
            }
        }
        _ => {}
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
    boundary_derive_aliases: &mut BTreeSet<String>,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
) {
    collect_use_aliases_with_prefix(
        tree,
        &[],
        boundary_derive_aliases,
        deserialize_aliases,
        validate_aliases,
        query_as_aliases,
    );
}

fn collect_use_aliases_with_prefix(
    tree: &syn::UseTree,
    prefix: &[String],
    boundary_derive_aliases: &mut BTreeSet<String>,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            let mut next_prefix = prefix.to_vec();
            next_prefix.push(path.ident.to_string());
            collect_use_aliases_with_prefix(
                &path.tree,
                &next_prefix,
                boundary_derive_aliases,
                deserialize_aliases,
                validate_aliases,
                query_as_aliases,
            );
        }
        syn::UseTree::Name(name) => {
            let ident = name.ident.to_string();
            register_canonical_alias(
                prefix,
                &ident,
                &ident,
                boundary_derive_aliases,
                deserialize_aliases,
                validate_aliases,
                query_as_aliases,
            );
        }
        syn::UseTree::Rename(rename) => {
            let target = rename.ident.to_string();
            let alias = rename.rename.to_string();
            register_canonical_alias(
                prefix,
                &target,
                &alias,
                boundary_derive_aliases,
                deserialize_aliases,
                validate_aliases,
                query_as_aliases,
            );
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_aliases_with_prefix(
                    item,
                    prefix,
                    boundary_derive_aliases,
                    deserialize_aliases,
                    validate_aliases,
                    query_as_aliases,
                );
            }
        }
        _ => {}
    }
}

fn register_canonical_alias(
    prefix: &[String],
    target: &str,
    alias: &str,
    boundary_derive_aliases: &mut BTreeSet<String>,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
) {
    let path = qualified_use_target(prefix, target);
    match path.as_str() {
        "serde::Deserialize" => {
            let _ = boundary_derive_aliases.insert(alias.to_owned());
            let _ = deserialize_aliases.insert(alias.to_owned());
        }
        "clap::Parser" | "clap::Args" | "sqlx::FromRow" => {
            let _ = boundary_derive_aliases.insert(alias.to_owned());
        }
        "garde::Validate" => {
            let _ = validate_aliases.insert(alias.to_owned());
        }
        "sqlx::query_as" | "sqlx::query_as_unchecked" => {
            let _ = query_as_aliases.insert(alias.to_owned());
        }
        _ => {}
    }
}

fn qualified_use_target(prefix: &[String], target: &str) -> String {
    if prefix.is_empty() {
        target.to_owned()
    } else {
        format!("{}::{target}", prefix.join("::"))
    }
}

fn is_deserialize_trait_path(path: &syn::Path, aliases: &BTreeSet<String>) -> bool {
    is_canonical_or_aliased_path(path, aliases, "serde::Deserialize")
}

fn is_validate_trait_path(path: &syn::Path, aliases: &BTreeSet<String>) -> bool {
    is_canonical_or_aliased_path(path, aliases, "garde::Validate")
}

fn is_canonical_or_aliased_path(
    path: &syn::Path,
    aliases: &BTreeSet<String>,
    canonical: &str,
) -> bool {
    let rendered = path_to_string(path);
    if rendered.trim_start_matches(':') == canonical {
        return true;
    }
    path.segments.len() == 1
        && path
            .segments
            .last()
            .is_some_and(|segment| aliases.contains(&segment.ident.to_string()))
}

fn is_sqlx_query_as_macro_path(path: &syn::Path) -> bool {
    matches!(
        path_to_string(path).trim_start_matches(':'),
        "sqlx::query_as" | "sqlx::query_as_unchecked"
    )
}

fn path_to_string(path: &syn::Path) -> String {
    path.to_token_stream().to_string().replace(' ', "")
}

fn token_stream_uses_ctx_variable(stream: proc_macro2::TokenStream) -> bool {
    let tokens: Vec<_> = stream.into_iter().collect();
    for (index, token) in tokens.iter().enumerate() {
        match token {
            proc_macro2::TokenTree::Ident(found) if found == "ctx" => {
                if !ctx_token_is_path_segment(&tokens, index) {
                    return true;
                }
            }
            proc_macro2::TokenTree::Group(group)
                if token_stream_uses_ctx_variable(group.stream()) =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn ctx_token_is_path_segment(tokens: &[proc_macro2::TokenTree], index: usize) -> bool {
    preceding_double_colon(tokens, index)
        || following_double_colon(tokens, index)
        || matches!(
            tokens.get(index.saturating_sub(1)),
            Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '.'
        )
}

fn preceding_double_colon(tokens: &[proc_macro2::TokenTree], index: usize) -> bool {
    matches!(
        (index.checked_sub(2).and_then(|i| tokens.get(i)), index.checked_sub(1).and_then(|i| tokens.get(i))),
        (
            Some(proc_macro2::TokenTree::Punct(first)),
            Some(proc_macro2::TokenTree::Punct(second))
        ) if first.as_char() == ':' && second.as_char() == ':'
    )
}

fn following_double_colon(tokens: &[proc_macro2::TokenTree], index: usize) -> bool {
    matches!(
        (tokens.get(index + 1), tokens.get(index + 2)),
        (
            Some(proc_macro2::TokenTree::Punct(first)),
            Some(proc_macro2::TokenTree::Punct(second))
        ) if first.as_char() == ':' && second.as_char() == ':'
    )
}

fn type_to_string(ty: &syn::Type) -> String {
    ty.to_token_stream().to_string().replace(' ', "")
}

fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}
