# g3rs-code-ast-ingestion TODO

- implement profile resolution from Cargo target ownership
  - emit `profile_name = Some("library" | "binary")`
  - emit `is_library_root` on `G3RsSourceFile`
  - follow `.plans/todo/checks/2026-04-09-code-ast-profile-resolution.md`
- add more package-level parity smoke tests as more `code` AST rules move over
- decide whether fixture filtering should become shared utility instead of local duplication
