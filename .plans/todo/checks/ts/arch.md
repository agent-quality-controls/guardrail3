# TS-ARCH — TypeScript root placement and ownership checker

**Input:** discovered repo-wide TS roots, package roots, workspace roots, root-type config
**Parser:** repo discovery + JSON/config inspection
**Current code:** no cohesive family yet
**Owned root:** repo-global TypeScript placement surface

## Owns

- repo-global TS root discovery and classification
- classification of each discovered TS root as:
  - `service`
  - `extension`
  - `content`
  - `library`
  - `other`
- misplaced TS roots outside allowed architecture zones
- unambiguous architecture ownership
  - one root must not be both `hexarch` and `libarch`
  - one root must not be both `content` and `hexarch`
- illegal overlap or nesting between governed TS roots
- enablement coherence between:
  - `ts/hexarch`
  - `ts/libarch`
  - `ts/content`
  - `ts/i18n`
  - `ts/seo`

## Does not own

- service-app semantics inside a service/extension root
  - that belongs to `ts/hexarch`
- library/package semantics inside a library root
  - that belongs to `ts/libarch`
- content-model semantics inside a content root
  - that belongs to `ts/content`
- locale/message or SEO semantics inside a content/web root
  - that belongs to `ts/i18n` and `ts/seo`

## Contract direction

This is the TS analogue of Rust `rs/arch`.

It should own the repo-global questions:
- where TS roots are allowed to live
- which architecture family owns each root
- whether any root is misplaced or ambiguously owned

The architecture families should then own only their inside-the-zone semantics.
