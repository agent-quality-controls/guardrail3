# g3-garde-content-checks TODO

- The package currently owns only the root-policy slice:
  - `RS-GARDE-01`
  - `RS-GARDE-02`
  - `RS-GARDE-03`
  - `RS-GARDE-04`
  - `RS-GARDE-06`
- The app still owns missing / unparseable covering clippy warnings for `RS-GARDE-02/03/04/06`. Those rule IDs are split temporarily so the package can stay on parsed-file inputs only.
- `RS-GARDE-05`, `07`, `08`, `09`, `11`, `12`, `13`, and `14` are still app-side because their current facts are normalized AST/source facts rather than a clean parsed-file package boundary.
- `RS-GARDE-10` remains app-side and should stay the sole malformed-input sink unless the family boundary changes deliberately.
