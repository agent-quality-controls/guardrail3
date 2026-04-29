# g3ts-eslint-plugin-astro-i18n-policy

Astro i18n source-policy rules for G3TS-managed content apps.

The package is an ESLint plugin, not a CLI. G3TS enforces that this plugin is installed and active on the configured Astro public source lanes.

## Rules

- `astro-i18n-policy/no-unlocalized-internal-hrefs`
- `astro-i18n-policy/no-inline-image-alt`
- `astro-i18n-policy/require-content-image-key`

Every rule requires explicit options. Missing required options reports an ESLint config error.

`no-unlocalized-internal-hrefs` checks JSX `href` and `to` attributes by default. Function-call link checks are explicit: list call names in `checkedInternalLinkHelpers`. Calls listed in `approvedInternalLinkHelpers` are trusted localized helpers and are not reported.
