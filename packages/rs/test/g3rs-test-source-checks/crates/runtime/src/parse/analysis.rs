use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::body::{analyze_function, collect_check_result_aliases, maybe_push_test_function};
use super::helpers;
use super::types::{
    CfgTestModuleInfo, IgnoreReasonInfo, ModuleInfo, ParsedTestFile, PublicValueInfo,
    PublicValueKind, UseBinding,
};

const DEFINE_RESULT_ASSERTIONS_PROOF_FUNCTIONS: &[&str] = &[
    "assert_findings",
    "assert_no_findings",
    "assert_contains",
    "assert_has_info",
    "assert_has_warn",
    "assert_has_error",
    "assert_title_count",
    "assert_message_contains",
    "assert_title_absent",
];

pub(crate) fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub(crate) fn analyze(source: &syn::File, content: &str) -> ParsedTestFile {
    let check_result_aliases = collect_check_result_aliases(source);
    let mut visitor = TestVisitor {
        out: ParsedTestFile {
            ignore_reasons: find_ignore_reasons(source, content),
            modules: Vec::new(),
            cfg_test_modules: Vec::new(),
            test_functions: Vec::new(),
            functions: Vec::new(),
            public_values: Vec::new(),
            file_value_names: BTreeSet::new(),
            file_function_names: BTreeSet::new(),
            check_result_aliases,
            file_call_paths: Vec::new(),
            imports: Vec::new(),
            macro_defined_proof_functions: BTreeSet::new(),
        },
    };
    visitor.visit_file(source);
    visitor.out
}

fn find_ignore_reasons(file: &syn::File, source: &str) -> Vec<IgnoreReasonInfo> {
    let mut visitor = IgnoreVisitor {
        lines: source.lines().collect(),
        findings: Vec::new(),
    };
    visitor.visit_file(file);
    visitor.findings
}

struct IgnoreVisitor<'s> {
    lines: Vec<&'s str>,
    findings: Vec<IgnoreReasonInfo>,
}

impl<'source> Visit<'source> for IgnoreVisitor<'_> {
    fn visit_item_fn(&mut self, item: &'source syn::ItemFn) {
        self.check_ignore_attrs(&item.attrs);
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'source syn::ImplItemFn) {
        self.check_ignore_attrs(&item.attrs);
        syn::visit::visit_impl_item_fn(self, item);
    }
}

impl IgnoreVisitor<'_> {
    fn check_ignore_attrs(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if attr.path().is_ident("ignore") {
                self.record_ignore_meta(&attr.meta, helpers::span_line(attr.span()));
                continue;
            }
            if !attr.path().is_ident("cfg_attr") {
                continue;
            }
            let syn::Meta::List(list) = &attr.meta else {
                continue;
            };
            let Ok(args) = list.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            let mut iter = args.into_iter();
            let Some(condition) = iter.next() else {
                continue;
            };
            if !cfg_meta_contains_positive_test(&condition) {
                continue;
            }
            let line = helpers::span_line(attr.span());
            for meta in iter {
                if meta.path().is_ident("ignore") {
                    self.record_ignore_meta(&meta, line);
                }
            }
        }
    }

    fn record_ignore_meta(&mut self, meta: &syn::Meta, line: usize) {
        if let Some(reason) = reason_from_ignore_meta(meta) {
            self.findings.push(IgnoreReasonInfo {
                line,
                reason: Some(reason),
            });
            return;
        }

        if !matches!(meta, syn::Meta::Path(_)) {
            return;
        }

        let idx = line.saturating_sub(1);
        if let Some(same_line) = self.lines.get(idx) {
            if let Some(reason) = extract_comment_reason(same_line) {
                self.findings.push(IgnoreReasonInfo {
                    line,
                    reason: Some(reason),
                });
                return;
            }
        }
        if idx > 0 {
            if let Some(prev_line) = self.lines.get(idx.saturating_sub(1)) {
                if let Some(reason) = extract_comment_reason(prev_line) {
                    self.findings.push(IgnoreReasonInfo {
                        line,
                        reason: Some(reason),
                    });
                    return;
                }
            }
        }

        self.findings.push(IgnoreReasonInfo { line, reason: None });
    }
}

fn reason_from_ignore_meta(meta: &syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::NameValue(name_value) => match &name_value.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(value) => Some(value.value()),
                _ => None,
            },
            _ => None,
        },
        syn::Meta::List(_) | syn::Meta::Path(_) => None,
    }
}

fn extract_comment_reason(line: &str) -> Option<String> {
    const TOKENS: [&str; 2] = ["// reason:", "//reason:"];

    TOKENS.iter().find_map(|token| {
        line.find(token)
            .map(|index| line[index + token.len()..].trim().to_owned())
    })
}

fn cfg_meta_contains_positive_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::NameValue(_) => false,
        syn::Meta::List(list) if list.path.is_ident("not") => false,
        syn::Meta::List(list) => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .map(|items| items.iter().any(cfg_meta_contains_positive_test))
            .unwrap_or(false),
    }
}

struct TestVisitor {
    out: ParsedTestFile,
}

impl<'source> Visit<'source> for TestVisitor {
    fn visit_item_mod(&mut self, item: &'source syn::ItemMod) {
        let is_cfg_test = item.attrs.iter().any(helpers::is_cfg_test_attr);
        self.out.modules.push(ModuleInfo {
            line: helpers::span_line(item.span()),
            path_attr: item.attrs.iter().find_map(helpers::path_attr_value),
        });
        if is_cfg_test {
            self.out.cfg_test_modules.push(CfgTestModuleInfo {
                line: helpers::span_line(item.span()),
                name: item.ident.to_string(),
                has_body: item.content.is_some(),
                path_attr: item.attrs.iter().find_map(helpers::path_attr_value),
            });
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'source syn::ItemFn) {
        let _ = self
            .out
            .file_function_names
            .insert(item.sig.ident.to_string());
        let function = analyze_function(
            &item.attrs,
            &item.vis,
            &item.sig,
            &item.block,
            &self.out.check_result_aliases,
        );
        maybe_push_test_function(
            &item.attrs,
            &item.sig,
            &item.block,
            &function,
            &mut self.out.test_functions,
        );
        self.out.functions.push(function);
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'source syn::ImplItemFn) {
        let _ = self
            .out
            .file_function_names
            .insert(item.sig.ident.to_string());
        let function = analyze_function(
            &item.attrs,
            &item.vis,
            &item.sig,
            &item.block,
            &self.out.check_result_aliases,
        );
        maybe_push_test_function(
            &item.attrs,
            &item.sig,
            &item.block,
            &function,
            &mut self.out.test_functions,
        );
        self.out.functions.push(function);
        syn::visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_use(&mut self, item: &'source syn::ItemUse) {
        helpers::collect_use_bindings(
            &item.tree,
            &mut Vec::new(),
            helpers::span_line(item.span()),
            &mut self.out.imports,
        );
        syn::visit::visit_item_use(self, item);
    }

    fn visit_item_extern_crate(&mut self, item: &'source syn::ItemExternCrate) {
        self.out.imports.push(UseBinding {
            line: helpers::span_line(item.span()),
            path_segments: vec![item.ident.to_string()],
            local_name: item.rename.as_ref().map(|(_, ident)| ident.to_string()),
        });
        syn::visit::visit_item_extern_crate(self, item);
    }

    fn visit_item_macro(&mut self, item: &'source syn::ItemMacro) {
        if item
            .mac
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "define_rule_assertions")
        {
            self.out.macro_defined_proof_functions.extend([
                "assert_rule_quiet".to_owned(),
                "assert_rule_count".to_owned(),
                "assert_rule_files".to_owned(),
                "assert_rule_results".to_owned(),
            ]);
        } else if item
            .mac
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "define_result_assertions")
        {
            self.out.macro_defined_proof_functions.extend(
                DEFINE_RESULT_ASSERTIONS_PROOF_FUNCTIONS
                    .iter()
                    .map(|name| (*name).to_owned()),
            );
        }
        syn::visit::visit_item_macro(self, item);
    }

    fn visit_item_const(&mut self, item: &'source syn::ItemConst) {
        let _ = self.out.file_value_names.insert(item.ident.to_string());
        if matches!(item.vis, syn::Visibility::Public(_)) {
            self.out.public_values.push(PublicValueInfo {
                line: helpers::span_line(item.span()),
                name: item.ident.to_string(),
                kind: PublicValueKind::Const,
            });
        }
        syn::visit::visit_item_const(self, item);
    }

    fn visit_item_static(&mut self, item: &'source syn::ItemStatic) {
        let _ = self.out.file_value_names.insert(item.ident.to_string());
        if matches!(item.vis, syn::Visibility::Public(_)) {
            self.out.public_values.push(PublicValueInfo {
                line: helpers::span_line(item.span()),
                name: item.ident.to_string(),
                kind: PublicValueKind::Static,
            });
        }
        syn::visit::visit_item_static(self, item);
    }

    fn visit_expr_call(&mut self, expr: &'source syn::ExprCall) {
        if let Some(path) = helpers::call_path(&expr.func) {
            self.out.file_call_paths.push(path);
        }
        syn::visit::visit_expr_call(self, expr);
    }
}
