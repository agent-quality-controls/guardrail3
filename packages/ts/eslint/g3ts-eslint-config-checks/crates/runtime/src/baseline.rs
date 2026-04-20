pub(crate) const THRESHOLD_RULES: &[(&str, i64, &[&str])] = &[
    ("max-lines", 400, &["max"]),
    ("max-lines-per-function", 100, &["max"]),
    ("complexity", 25, &["max"]),
];

pub(crate) const REQUIRED_THRESHOLD_PRESENCE_RULES: &[&str] = &["no-restricted-imports"];
pub(crate) const REQUIRED_TS_SOURCE_PLUGINS: &[&str] = &["unicorn", "regexp", "sonarjs"];

pub(crate) const CORE_BASELINE_RULES: &[&str] = &[
    "@typescript-eslint/no-floating-promises",
    "eqeqeq",
    "no-restricted-globals",
    "import-x/no-cycle",
    "import-x/max-dependencies",
    "@typescript-eslint/explicit-function-return-type",
    "@typescript-eslint/strict-boolean-expressions",
];

pub(crate) const TYPE_SAFETY_RULES: &[&str] = &[
    "@typescript-eslint/no-misused-promises",
    "@typescript-eslint/await-thenable",
    "@typescript-eslint/consistent-type-imports",
    "@typescript-eslint/no-non-null-assertion",
    "@typescript-eslint/switch-exhaustiveness-check",
    "@typescript-eslint/no-unused-vars",
    "@typescript-eslint/require-await",
    "no-param-reassign",
    "@typescript-eslint/no-unsafe-assignment",
    "@typescript-eslint/no-unsafe-member-access",
    "@typescript-eslint/no-unsafe-call",
    "@typescript-eslint/no-unsafe-return",
    "@typescript-eslint/no-unsafe-argument",
];

pub(crate) const HYGIENE_RULES: &[&str] = &[
    "@typescript-eslint/explicit-module-boundary-types",
    "@typescript-eslint/promise-function-async",
    "@typescript-eslint/consistent-type-exports",
    "@typescript-eslint/consistent-type-definitions",
    "@typescript-eslint/no-unnecessary-condition",
    "@typescript-eslint/prefer-nullish-coalescing",
    "@typescript-eslint/prefer-optional-chain",
    "@typescript-eslint/no-deprecated",
    "@typescript-eslint/restrict-template-expressions",
    "no-throw-literal",
    "no-empty",
];

pub(crate) const UNICORN_RULES: &[&str] = &[
    "unicorn/no-keyword-prefix",
    "unicorn/no-unused-properties",
    "unicorn/require-post-message-target-origin",
    "unicorn/no-anonymous-default-export",
];

pub(crate) const REGEXP_RULES: &[&str] = &[
    "regexp/require-unicode-regexp",
    "regexp/require-unicode-sets-regexp",
    "regexp/prefer-named-capture-group",
    "regexp/prefer-named-backreference",
    "regexp/prefer-result-array-groups",
    "regexp/no-misleading-capturing-group",
];

pub(crate) const SONARJS_RULES: &[&str] = &[
    "sonarjs/cognitive-complexity",
    "sonarjs/no-identical-functions",
    "sonarjs/no-all-duplicated-branches",
    "sonarjs/no-duplicated-branches",
    "sonarjs/no-collapsible-if",
    "sonarjs/no-identical-conditions",
    "sonarjs/no-identical-expressions",
    "sonarjs/no-inverted-boolean-check",
    "sonarjs/no-redundant-boolean",
    "sonarjs/prefer-single-boolean-return",
    "sonarjs/no-gratuitous-expressions",
    "sonarjs/no-invariant-returns",
    "sonarjs/no-collection-size-mischeck",
    "sonarjs/no-empty-collection",
    "sonarjs/no-element-overwrite",
    "sonarjs/no-unused-collection",
    "sonarjs/no-use-of-empty-return-value",
    "sonarjs/no-nested-switch",
    "sonarjs/no-nested-template-literals",
    "sonarjs/no-redundant-jump",
    "sonarjs/expression-complexity",
    "sonarjs/no-async-constructor",
    "sonarjs/no-hook-setter-in-body",
    "sonarjs/no-useless-react-setstate",
];
