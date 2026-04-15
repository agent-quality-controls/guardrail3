use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_public_struct_field_bags;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-31";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_public_struct_field_bags(input.source) {
        let has_inherent_impl = struct_has_inherent_impl(input.source, &info.struct_name);
        if input.is_shared_crate && info.all_fields_public && !has_inherent_impl {
            continue;
        }

        let severity = if info.public_field_count >= 5 {
            G3Severity::Error
        } else {
            G3Severity::Warn
        };
        let message = if input.is_shared_crate && has_inherent_impl {
            format!(
                "Shared-crate struct `{}` exposes {} named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
                info.struct_name, info.public_field_count
            )
        } else if input.is_shared_crate && !info.all_fields_public {
            format!(
                "Shared-crate struct `{}` exposes {} named `pub` fields but also hides some fields. In shared crates, either make this a plain data struct with all fields `pub`, or make the fields private and expose an API. Mixed visibility hides part of the shared data contract.",
                info.struct_name, info.public_field_count
            )
        } else {
            format!(
                "Struct `{}` exposes {} named `pub` fields. Make the fields private and expose constructors or getters instead, so callers use one API instead of reaching into raw state.",
                info.struct_name, info.public_field_count
            )
        };

        results.push(G3CheckResult::new(
            ID.to_owned(),
            severity,
            "public struct exposes named public fields".to_owned(),
            message,
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

fn struct_has_inherent_impl(source: &syn::File, struct_name: &str) -> bool {
    source.items.iter().any(|item| {
        let syn::Item::Impl(item_impl) = item else {
            return false;
        };
        if item_impl.trait_.is_some() {
            return false;
        }
        let syn::Type::Path(type_path) = item_impl.self_ty.as_ref() else {
            return false;
        };
        type_path.qself.is_none()
            && type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == struct_name)
    })
}

#[cfg(test)]
pub(super) fn check_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    check_source_with_shared(rel_path, content, is_test, false)
}

#[cfg(test)]
pub(super) fn check_source_with_shared(
    rel_path: &str,
    content: &str,
    is_test: bool,
    is_shared_crate: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let source = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let parsed = crate::support::G3RsCodeSourceFileAst {
        source_file: g3rs_code_types::G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test,
            profile_name: None,
            is_library_root: false,
        },
        source,
    };
    let input = crate::support::CodeSourceRuleInput {
        is_shared_crate,
        ..crate::support::CodeSourceRuleInput::from(&parsed)
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
