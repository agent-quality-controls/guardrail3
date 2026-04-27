## Summary

Fixed the semantic rule ID migration artifacts after normalized old-vs-new output comparison showed the active runtime code was migrated but the migration map was incomplete. The regenerated map now covers old active definitions from the parent commit and excludes retired TODO-only entries.

## Decisions

- Regenerated `rule-id-map.toml`, inventories, and ID lists from old active definitions against current semantic definitions.
- Kept retired README/TODO-only IDs out of the active migration map because they do not correspond to emitted current rules.
- Recorded `TS-ASTRO-MDX-CONFIG-20` as mapping to `g3ts-astro-mdx/mdx-eslint-lane-wired` for output normalization. The new package-presence rule is a split assertion that has no unique old ID because the old implementation reused one ID for two assertions.

## Key Files

- `.plans/rule-id-migration/rule-id-map.toml`
- `.plans/rule-id-migration/generated-map.tsv`
- `.plans/rule-id-migration/rs-inventory.md`
- `.plans/rule-id-migration/ts-inventory.md`

## Verification

- Parsed `.plans/rule-id-migration/rule-id-map.toml` with Python `tomllib`; 355 active entries.
- Grep found no unresolved placeholders or retired TODO-only IDs in `.plans/rule-id-migration`.
- Normalized old-vs-new output comparison passed exactly for:
  - landing Astro validation: 41 findings, exit `0` vs `0`
  - `apps/guardrail3-rs` G3RS validation: 500 findings, exit `0` vs `0`
  - `apps/guardrail3-ts` G3RS validation: 496 findings, exit `0` vs `0`

## Next Steps

- Use the parity script pattern again for future global rule ID migrations before committing the first pass.
