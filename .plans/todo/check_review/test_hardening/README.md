# Parallel Rust Test Hardening

This folder is the execution plan for the next hardening phase.

It is intentionally parallelizable:
- one shared contract file
- one file per family lane

The purpose is not to “add more tests”. The purpose is to make every rule hard to bypass by attacking it from many angles in the same way.

## Files

- `00-shared-test-story.md`
  - universal attack model
  - exact assertion standard
  - golden-fixture mutation philosophy
- `../../checks/2026-03-25-rust-layered-test-architecture.md`
  - layer split between rule, facts/discovery, family integration, and runtime/product tests
- `11-hexarch-agent-brief.md`
- `12-code-agent-brief.md`
- `13-release-agent-brief.md`
- `14-clippy-deny-agent-brief.md`
- `15-hooks-agent-brief.md`
  - combined migration lane for shared + Rust hooks plus remaining routing/parity debt
- `19-garde-agent-brief.md`
- `20-cargo-agent-brief.md`
- `21-deps-agent-brief.md`
- `22-hooks-shared-agent-brief.md`
- `23-hooks-rs-agent-brief.md`
- `24-fmt-agent-brief.md`
- `25-toolchain-agent-brief.md`
- `26-test-agent-brief.md`
- `34-test-family-rewrite-agent-brief.md`
  - current droppable packet for rewriting `rs/test` to the accepted family README contract
- `35-arch-family-rewrite-agent-brief.md`
  - current droppable packet for rewriting `rs/arch` to the accepted family README contract
- `27-libarch-agent-brief.md`
- `28-rust-validation-cutover-agent-brief.md`
- `29-arch-agent-brief.md`
  - droppable family/combined-lane agent packets
- `37-code-remaining-agent-brief.md`
  - current remaining-work packet for closing the planned `rs/code` backlog
- `38-deps-remaining-agent-brief.md`
  - current remaining-work packet for landing `RS-DEPS-12`
- `39-hooks-rs-runtime-agent-brief.md`
  - current operational cutover packet for wiring `hooks-rs` into the live runtime
- `40-test-remaining-agent-brief.md`
  - current closure packet for finishing `rs/test`
- `41-libarch-implementation-agent-brief.md`
  - current implementation packet for the not-yet-live `rs/libarch` family
- `31-hexarch-layered-test-map.md`
  - concrete rule/facts/integration split for `rs/hexarch`
- `32-hexarch-01-06-layered-migration-checklist.md`
  - concrete execution checklist for migrating `RS-HEXARCH-01..06` to the corrected layered model
- `33-hexarch-layered-test-architecture-note.md`
  - lightweight future-architecture note for any later agent touching `rs/hexarch` tests or family crate split
- `16-hexarch-execution-plan.md`
  - exhaustive step-by-step implementation order for the full `rs/hexarch` hardening pass
- `17-hooks-execution-plan.md`
  - exhaustive step-by-step implementation order for the full hook migration and hardening pass
- `18-hooks-coverage-matrix.md`
  - old-to-new hook rule mapping, routing impacts, and canonical migrated module location
- `01-hexarch.md`
  - highest-risk structural family
- `02-code.md`
  - source-rule bypasses and parser/suppression attacks
- `03-release.md`
  - workflow, publishability, and inherited-edge attacks
- `04-clippy-and-deny.md`
  - generator/checker parity and config-policy attacks
- `05-hooks.md`
  - new-architecture hook migration plus executable-command semantics

## Working rule

One test represents one attack vector.

That test mutates the golden fixture everywhere that attack vector should matter:
- all relevant Rust roots
- all nested roots
- all matching files/configs

Then the assertions must prove:
- exact owned hits
- exact owned non-hits
- exact rule and severity

This is deliberately broader than classic “mutate one file and see one error” testing.

There is no “small rule” exception here. Every rule should be attacked under the same model.
