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
- `11-hexarch-agent-brief.md`
- `12-code-agent-brief.md`
- `13-release-agent-brief.md`
- `14-clippy-deny-agent-brief.md`
- `15-hooks-agent-brief.md`
  - droppable per-family agent packets
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
