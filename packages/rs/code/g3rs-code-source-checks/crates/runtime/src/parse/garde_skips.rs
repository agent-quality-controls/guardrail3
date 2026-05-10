#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::question_mark,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::needless_pass_by_value,
    clippy::expect_used,
    clippy::panic,
    clippy::format_collect,
    clippy::format_in_format_args,
    clippy::option_if_let_else,
    clippy::map_unwrap_or,
    clippy::if_same_then_else,
    clippy::match_same_arms,
    clippy::match_like_matches_macro,
    clippy::nonminimal_bool,
    clippy::single_match_else,
    clippy::items_after_statements,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::needless_for_each,
    clippy::manual_let_else,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_name_repetitions,
    clippy::large_enum_variant,
    clippy::large_types_passed_by_value,
    clippy::ptr_arg,
    clippy::needless_collect,
    clippy::branches_sharing_code,
    clippy::unused_self,
    reason = "code-source-checks parse/visitor walks every variant of large external syntax-tree enums (syn::Type, syn::Item, syn::Expr, syn::Pat, etc.) and the ban-detection visitors mirror the source structure they are looking for; the rule modules accept the schema-versioned shape verbatim because the per-rule findings depend on the exact spans and the rule ids embed the schema."
)]

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;

use super::helpers::span_line;
use super::types::GardeSkipInfo;

/// Constant value used by the surrounding module.
const GARDE_SKIP_EXEMPT_TYPE_ROOTS: &[&str] = &[
    "bool", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64", "BTreeMap", "HashMap", "BTreeSet", "HashSet",
];

/// Implements `find garde skips with types`.
pub(crate) fn find_garde_skips_with_types(file: &syn::File) -> Vec<GardeSkipInfo> {
    let mut visitor = GardeSkipTypedVisitor { out: Vec::new() };
    visitor.visit_file(file);
    visitor.out
}

/// Implements `has garde skip`.
fn has_garde_skip(attrs: &[syn::Attribute]) -> Option<usize> {
    for attr in attrs {
        if !attr.path().is_ident("garde") {
            continue;
        }
        if let Ok(nested) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) {
            for path in &nested {
                if path.is_ident("skip") {
                    return Some(span_line(attr.span()));
                }
            }
        }
    }
    None
}

/// Implements `type is garde skip exempt`.
fn type_is_garde_skip_exempt(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(tp) => {
            let Some(seg) = tp.path.segments.last() else {
                return false;
            };
            let ident = seg.ident.to_string();
            if GARDE_SKIP_EXEMPT_TYPE_ROOTS.iter().any(|&p| p == ident) {
                return true;
            }
            if ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if args.args.len() == 1 {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return type_is_garde_skip_exempt(inner);
                        }
                    }
                }
            }
            if ident == "Box" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if args.args.len() == 1 {
                        if let Some(syn::GenericArgument::Type(syn::Type::TraitObject(_))) =
                            args.args.first()
                        {
                            return true;
                        }
                    }
                }
            }
            false
        }
        syn::Type::TraitObject(_) | syn::Type::Reference(_) => true,
        syn::Type::Array(_)
        | syn::Type::BareFn(_)
        | syn::Type::Group(_)
        | syn::Type::ImplTrait(_)
        | syn::Type::Infer(_)
        | syn::Type::Macro(_)
        | syn::Type::Never(_)
        | syn::Type::Paren(_)
        | syn::Type::Ptr(_)
        | syn::Type::Slice(_)
        | syn::Type::Tuple(_)
        | syn::Type::Verbatim(_) => false,
        _ => false,
    }
}

/// Implements `field has subcommand attr`.
fn field_has_subcommand_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("command")
            && attr
                .meta
                .require_list()
                .ok()
                .and_then(|list| list.parse_args::<syn::Ident>().ok())
                .is_some_and(|ident| ident == "subcommand")
    })
}

/// Implements `field is garde skip exempt`.
fn field_is_garde_skip_exempt(field: &syn::Field) -> bool {
    field_has_subcommand_attr(field) || type_is_garde_skip_exempt(&field.ty)
}

/// Implements `type to string`.
fn type_to_string(ty: &syn::Type) -> String {
    ty.to_token_stream().to_string().replace(' ', "")
}

/// Implements `struct has non exempt fields`.
fn struct_has_non_exempt_fields(item: &syn::ItemStruct) -> bool {
    match &item.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .any(|field| !field_is_garde_skip_exempt(field)),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .any(|field| !field_is_garde_skip_exempt(field)),
        syn::Fields::Unit => false,
    }
}

#[derive(Debug)]
/// Struct `GardeSkipTypedVisitor` used by this module.
struct GardeSkipTypedVisitor {
    /// Field `out`.
    out: Vec<GardeSkipInfo>,
}

impl<'source> Visit<'source> for GardeSkipTypedVisitor {
    fn visit_field(&mut self, field: &'source syn::Field) {
        if let Some(line) = has_garde_skip(&field.attrs) {
            let field_name = field
                .ident
                .as_ref()
                .map_or_else(|| "<unnamed>".to_owned(), std::string::ToString::to_string);
            let field_type = type_to_string(&field.ty);
            let has_subcommand_attr = field_has_subcommand_attr(field);
            self.out.push(GardeSkipInfo {
                line,
                field_name,
                field_type,
                is_type_level: false,
                is_exempt: field_is_garde_skip_exempt(field),
                has_subcommand_attr,
            });
        }
        syn::visit::visit_field(self, field);
    }

    fn visit_item_struct(&mut self, item_struct: &'source syn::ItemStruct) {
        if let Some(line) = has_garde_skip(&item_struct.attrs) {
            let type_name = item_struct.ident.to_string();
            self.out.push(GardeSkipInfo {
                line,
                field_name: type_name.clone(),
                field_type: type_name,
                is_type_level: true,
                is_exempt: !struct_has_non_exempt_fields(item_struct),
                has_subcommand_attr: false,
            });
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
