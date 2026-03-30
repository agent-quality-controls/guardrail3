use quote::ToTokens;
use syn::spanned::Spanned;

use super::aliases::{path_to_string, token_stream_uses_ctx_variable};
use super::BoundaryField;

#[derive(Default)]
struct GardeAttrSummary {
    has_skip: bool,
    has_dive: bool,
    has_meaningful_rule: bool,
    uses_context: bool,
}

pub(super) fn has_garde_context(attrs: &[syn::Attribute]) -> bool {
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

pub(super) fn collect_struct_boundary_fields(
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

pub(super) fn collect_enum_boundary_fields(
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

pub(super) fn struct_has_non_primitive_fields(item: &syn::ItemStruct) -> bool {
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

pub(super) fn enum_has_non_primitive_fields(item: &syn::ItemEnum) -> bool {
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

pub(super) fn type_requires_field_validation(ty: &syn::Type) -> bool {
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

pub(super) fn collect_candidate_type_names(ty: &syn::Type) -> Vec<String> {
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

pub(super) fn type_to_string(ty: &syn::Type) -> String {
    ty.to_token_stream().to_string().replace(' ', "")
}

pub(super) fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}
