# TS-CSS — CSS checker

**Input:** stylelint config, root/package `package.json`
**Parser:** structured config parse when possible
**Current code:** `app/ts/validate/stylelint_check.rs`, CSS package portion of `package_deps.rs`
**Owned root:** TS package/app roots whose product contract includes authored CSS, with nearest stylelint config root for local config resolution

## Owns

- CSS stylelint config existence and parseability
- stylelint package presence
- required stylelint plugin/config package presence
- CSS accessibility plugin presence
- required CSS accessibility rule set
- documented exception surfaces for supported CSS architecture exceptions
- content/frontend profile gating for CSS-only tool surfaces

## Does not own

- general `ESLint` content plugins
- locale/message completeness
  - that belongs to `ts/i18n`
- route/content/SEO semantics
  - that belongs to `ts/seo` or `ts/content`

## Contract direction

This family is named for the policy surface, not the tool brand.

Today it is enforced through `stylelint`, but the family owns CSS quality and CSS accessibility.
