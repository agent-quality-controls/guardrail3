use g3rs_arch_types::types::G3RsArchSourceFile;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use syn::spanned::Spanned;

const ID: &str = "RS-ARCH-SOURCE-09";

pub(crate) fn check_file(file: &G3RsArchSourceFile, results: &mut Vec<G3CheckResult>) {
    let Ok(ast) = syn::parse_file(
        file.content
            .strip_prefix('\u{feff}')
            .unwrap_or(&file.content),
    ) else {
        return;
    };

    for item in &ast.items {
        let syn::Item::Mod(module) = item else {
            continue;
        };

        for attr in &module.attrs {
            if !attr.path().is_ident("path") {
                continue;
            }

            let path_value = extract_path_value(attr);
            if path_value
                .as_deref()
                .is_some_and(|path| is_test_sidecar_exempt(&file.rel_path, module, path))
            {
                continue;
            }
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "#[path] attribute forbidden".to_owned(),
                format!(
                    "`#[path = \"{}\"]` on `mod {}` bypasses the module facade. Use standard module resolution with mod.rs instead. Every module directory must have a mod.rs that serves as its facade.",
                    path_value.as_deref().unwrap_or("..."),
                    module.ident
                ),
                Some(file.rel_path.clone()),
                Some(attr.span().start().line),
            ));
        }
    }
}

fn extract_path_value(attr: &syn::Attribute) -> Option<String> {
    if let syn::Meta::NameValue(name_value) = &attr.meta {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(value),
            ..
        }) = &name_value.value
        {
            return Some(value.value());
        }
    }
    None
}

fn is_test_sidecar_exempt(
    file_rel_path: &str,
    module: &syn::ItemMod,
    path_value: &str,
) -> bool {
    let Some(expected_module_name) = owned_sidecar_module_name(file_rel_path) else {
        return false;
    };
    if module.ident != expected_module_name {
        return false;
    }
    if path_value != format!("{expected_module_name}/mod.rs") {
        return false;
    }
    module.attrs.iter().any(attr_is_cfg_test)
}

fn owned_sidecar_module_name(file_rel_path: &str) -> Option<String> {
    let file_name = file_rel_path.rsplit('/').next()?;
    let stem = file_name.strip_suffix(".rs")?;
    if stem == "mod" || stem.is_empty() {
        return None;
    }
    Some(format!("{stem}_tests"))
}

fn attr_is_cfg_test(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return false;
    };
    let Ok(meta) = list.parse_args::<syn::Meta>() else {
        return false;
    };
    cfg_meta_is_test(&meta, true)
}

fn cfg_meta_is_test(meta: &syn::Meta, positive: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => positive && path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") || list.path.is_ident("any") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| items.iter().any(|item| cfg_meta_is_test(item, positive))),
        syn::Meta::List(list) if list.path.is_ident("not") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| items.iter().any(|item| cfg_meta_is_test(item, !positive))),
        _ => false,
    }
}

#[cfg(test)]
#[path = "rs_arch_09_no_path_attr_tests/mod.rs"]
// reason: keep rule tests in the owned x_tests sidecar directory.
mod rs_arch_09_no_path_attr_tests;
