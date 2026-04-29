# Astro media guardrails

## Goal
Add an Astro-specific media policy that stabilizes image and icon handling in static Astro content apps without putting image validation logic into the G3TS CLI.

Media means image/file assets used by the rendered site:
- favicon and browser/app icons
- default social/OG image assets
- content image references
- media helper/component usage in Astro, TS, TSX, and MDX source

Media does not include:
- sitemap generation or sitemap auditing
- robots.txt generation or auditing
- llms.txt generation or auditing
- generic Tailwind/style policy
- rendered HTML SEO validation handled by Nuasite
- full binary image quality validation, dimensions, compression, or crawl-wide image audits

## Package split

### Astro build integration
Package: `g3ts-astro-media-assets`
Location: `packages/ts/astro/media/g3ts-astro-media-assets`
Type: npm package, Astro integration, TypeScript.

Function:
- run during `astro build`
- validate that explicitly configured required assets exist in the final built output
- fail closed during build when an asset is missing
- never crawl pages
- never parse source files
- never inspect image dimensions, binary contents, compression, or metadata

Required explicit options:
- `favicon`: output-relative path, for example `/favicon.ico`
- `appIcons`: non-empty list of output-relative paths, for example `/apple-touch-icon.png`, `/icon.svg`
- `defaultSocialImage`: output-relative path, for example `/og/default.png`
- `allowSvgIcons`: boolean

Validation behavior:
- normalize every configured asset path to a root-relative URL path
- reject empty paths
- reject absolute external URLs
- reject parent traversal
- on `astro:build:done`, map configured URL paths into Astro output directory
- assert every required file exists
- if `allowSvgIcons = false`, reject configured `.svg` icon paths
- emit one actionable Astro build error listing all missing or invalid assets

Non-goals:
- no generated assets in this first version
- no image resizing
- no asset manifest copying
- no metadata extraction

### ESLint source policy plugin
Package: `g3ts-eslint-plugin-astro-media-policy`
Location: `packages/ts/g3ts-eslint-plugin-astro-media-policy`
Type: npm package, ESLint plugin, TypeScript.

Rules:

1. `astro-media-policy/no-raw-public-image-paths`
- Applies to configured Astro, TS, TSX, and MDX source lanes.
- Reports root-relative image file strings such as `/images/foo.png`, `/media/foo.webp`, `/og/default.png` when used directly in source.
- Extensions checked explicitly from options: `png`, `jpg`, `jpeg`, `webp`, `avif`, `gif`, `svg`, `ico`.
- Allows configured structural exceptions only through `allowedPublicImagePaths`.
- Allows import specifiers and package specifiers.
- Allows strings passed to configured approved helper calls.
- Does not guess arbitrary helper names. Helper names must be configured in `approvedMediaHelpers`.

2. `astro-media-policy/require-content-image-key`
- Applies to configured content image component names.
- Requires one of configured key props, for example `image`, `imageKey`, `assetKey`.
- Reports configured raw source props, for example `src`, `url`, `imageUrl`.
- Reuse the same behavior as the current i18n plugin rule, but media owns it going forward because image source shape is media policy, not language policy.

3. `astro-media-policy/no-inline-image-alt`
- Applies to configured content image component names.
- Reports any static alt string, including empty and whitespace-only strings.
- Allows dynamic alt props, because content/schema/i18n layers own the value source.
- Reuse current i18n plugin behavior, then remove i18n ownership after media is wired.

4. `astro-media-policy/require-approved-media-helper`
- Applies to route/layout/SEO source lanes configured by G3TS.
- Reports raw social image metadata values in route/layout files unless produced by approved media helper modules.
- Initial check is intentionally narrow: direct object properties named from explicit options, for example `image`, `ogImage`, `twitterImage`, `socialImage`, with static root-relative image strings.
- Does not parse generated HTML.
- Does not validate OpenGraph correctness. Nuasite owns rendered-output checks.

Shared plugin options:
- `publicSourceGlobs`: explicit source lanes protected by these rules
- `mediaHelperModules`: approved helper modules for route/layout/social image construction
- `approvedMediaHelpers`: approved function names that may receive image paths
- `contentImageComponents`: component names that represent content images
- `contentImageKeyProps`: props that carry content image keys
- `bannedImageSourceProps`: props forbidden on content image components
- `bannedImageAltProps`: props forbidden as static authored strings
- `allowedPublicImagePaths`: explicit exceptions for structural site assets only
- `checkedImageExtensions`: explicit extension list
- `metadataImagePropertyNames`: explicit metadata property names to police

No defaults. Missing required options report an ESLint config error.

### G3TS Rust family packages
Family name: `astro-media`
Root: `packages/ts/astro/media`

Packages:
- `g3ts-astro-media-types`
- `g3ts-astro-media-ingestion`
- `g3ts-astro-media-config-checks`
- `g3ts-astro-media-hook-contract`

No aggregate ingestion package.
No global Astro ingestion package.
Each package has its own `guardrail3-rs.toml`, `Cargo.toml`, `clippy.toml`, `deny.toml`, `rustfmt.toml`, and `rust-toolchain.toml` following existing Astro child-family packages.

## `guardrail3-ts.toml` contract

Add `[ts.astro.media]`.

Required fields:
- `favicon`: app/output root-relative path
- `app_icons`: non-empty list of app/output root-relative paths
- `default_social_image`: app/output root-relative path
- `allow_svg_icons`: boolean
- `public_source_globs`: non-empty list of app-relative globs for Astro/TS/TSX/MDX lanes
- `media_helper_modules`: non-empty list of app-relative modules/globs
- `approved_media_helpers`: non-empty list of function names
- `content_image_components`: non-empty list of component names
- `content_image_key_props`: non-empty list of prop names
- `banned_image_source_props`: non-empty list of prop names
- `banned_image_alt_props`: non-empty list of prop names
- `allowed_public_image_paths`: list of explicitly allowed root-relative site asset paths
- `checked_image_extensions`: non-empty list of extensions
- `metadata_image_property_names`: non-empty list of property names

Path validation rules:
- TOML paths/globs are app-relative unless they are URL paths for output assets
- output asset paths must start with `/`
- app-relative paths must not start with `/`
- no path may contain `..`
- no path may be an external URL

## G3TS ingestion

`g3ts-astro-media-ingestion` reads:
- `guardrail3-ts.toml` through `guardrail3-rs-toml-parser`
- `package.json` through shared package parser
- `astro.config.*` through existing Astro config parser/support
- `eslint.config.*` through existing ESLint config parser/support

Facts produced:
- media policy surface: missing, unreadable, parse error, missing `[ts.astro.media]`, parsed snapshot
- package surface: installed dependencies/devDependencies
- Astro config media integration surface: imported/called integration packages and static options
- ESLint media surface: effective plugins/rules/options on configured public source probes

ESLint probe policy:
- derive probes from every configured `public_source_globs` extension: Astro, TS, TSX, MDX
- a delegated rule is effective only if active at `error` on every configured public probe for that extension set
- options must exactly match `[ts.astro.media]`
- plugin package namespace must resolve to `g3ts-eslint-plugin-astro-media-policy`
- `@eslint-community/eslint-comments/no-restricted-disable` must restrict `astro-media-policy/*`

## G3TS config checks

Rule IDs:

1. `g3ts-astro-media/strict-policy-configured`
- Error when `[ts.astro.media]` is missing or required fields are empty.
- Info when all required fields are present.

2. `g3ts-astro-media/policy-paths-valid`
- Error when app paths, globs, or output URL paths violate path rules.
- Info when structurally valid.

3. `g3ts-astro-media/media-assets-package-present`
- Error when `g3ts-astro-media-assets` is not in `dependencies` or `devDependencies`.

4. `g3ts-astro-media/media-assets-integration-wired`
- Error when `astro.config.*` does not import and call `g3ts-astro-media-assets` with static options matching `[ts.astro.media]`.

5. `g3ts-astro-media/media-policy-plugin-package-present`
- Error when `g3ts-eslint-plugin-astro-media-policy` is not installed.

6. `g3ts-astro-media/media-eslint-plugin-wired`
- Error when ESLint does not activate namespace `astro-media-policy` from `g3ts-eslint-plugin-astro-media-policy` on every configured media source probe.

7. `g3ts-astro-media/no-raw-public-image-paths-rule`
- Error when `astro-media-policy/no-raw-public-image-paths` is not `error` with options matching `[ts.astro.media]`.

8. `g3ts-astro-media/require-content-image-key-rule`
- Error when `astro-media-policy/require-content-image-key` is not `error` with options matching `[ts.astro.media]`.

9. `g3ts-astro-media/no-inline-image-alt-rule`
- Error when `astro-media-policy/no-inline-image-alt` is not `error` with options matching `[ts.astro.media]`.

10. `g3ts-astro-media/require-approved-media-helper-rule`
- Error when `astro-media-policy/require-approved-media-helper` is not `error` with options matching `[ts.astro.media]`.

11. `g3ts-astro-media/protected-media-rule-disables-restricted`
- Error when `@eslint-community/eslint-comments/no-restricted-disable` does not restrict `astro-media-policy/*` on configured media source probes.

12. `g3ts-astro-media/media-build-validation-runs`
- Error when the app `validate` script does not run `astro build` through the standard script parser facts.
- This is required because the Astro media asset integration only runs during build.

## Hook contract

`g3ts-astro-media-hook-contract` publishes `hook_contract()` with no input parameters.

Triggers:
- `guardrail3-ts.toml`
- `astro.config.*`
- `eslint.config.*`
- `src/**/*.{astro,ts,tsx}`
- `content/**/*.mdx`
- configured source lanes cannot be dynamic here, so contract uses conservative Astro media defaults

Package, lockfile, and Syncpack trigger routing is owned by Astro setup because setup owns required package pins. Public asset changes are not added as a media hook trigger in this slice because the current hook router does not route `public/**/*`; adding that contract would create a hook failure without making the media checks more reliable. The build-time media integration still fails `validate` when configured assets are missing.

Required commands:
- `G3TsValidatePath`
- `AppValidateScript`

Critical commands:
- no package-manager binary hardcode unless hook source checks support package-manager alternatives cleanly

## Syncpack/package policy

Add required pins after packages are published:
- `g3ts-astro-media-assets`: `0.1.0`
- `g3ts-eslint-plugin-astro-media-policy`: `0.1.2`

Do not add generic image tooling pins unless a package is actually required by the media contract.

Forbidden packages:
- no media-specific bans yet unless implementation finds obsolete package names being replaced

## Migration from i18n ownership

Current i18n plugin owns:
- `no-inline-image-alt`
- `require-content-image-key`

Media should own those long-term because they define image source/alt sourcing shape, not language routing.

Migration steps:
1. Implement media plugin with equivalent stricter rules.
2. G3TS media requires media rules.
3. G3TS i18n stops requiring image component rules in a separate commit.
4. Keep i18n focused on locale prefixes, unlocalized hrefs, raw date/number formatting, and public-copy detection.

No backwards compatibility requirement.

## Implementation order

1. Build and publish `g3ts-astro-media-assets`.
2. Build and publish `g3ts-eslint-plugin-astro-media-policy`.
3. Extend `guardrail3-rs-toml-parser` with optional `[ts.astro.media]` type.
4. Add `g3ts-astro-media-types`.
5. Add `g3ts-astro-media-ingestion`.
6. Add `g3ts-astro-media-config-checks` with sidecar tests per rule or tightly grouped test cases only where existing TS family pattern requires it.
7. Add `g3ts-astro-media-hook-contract`.
8. Wire media family into `apps/guardrail3-ts` structure runner and supported family order.
9. Update Astro setup Syncpack required pins.
10. Install local `g3ts`.
11. Run against landing and verify it reports missing media setup.
12. Send adversarial review against this plan and implementation before reporting complete.

## Verification

Required before commit:
- `npm test` in `g3ts-astro-media-assets`
- `npm test` in `g3ts-eslint-plugin-astro-media-policy`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- package-level cargo tests for every new media Rust package
- `g3rs validate --path` for every new media Rust package and `apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- `git diff --check`

Expected landing signal after implementation, before landing is fixed:
- missing `[ts.astro.media]`
- missing `g3ts-astro-media-assets`
- missing `g3ts-eslint-plugin-astro-media-policy`
- missing Astro media integration wiring
- missing media ESLint rule wiring
