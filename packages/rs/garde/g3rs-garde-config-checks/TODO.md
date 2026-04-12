# g3rs-garde-config-checks TODO

- The package currently owns only the root-policy slice:
  - `RS-GARDE-CONFIG-01`
  - `RS-GARDE-CONFIG-02`
  - `RS-GARDE-CONFIG-03`
  - `RS-GARDE-CONFIG-04`
  - `RS-GARDE-CONFIG-05`
- The app still owns missing / unparseable covering clippy warnings for `RS-GARDE-CONFIG-02/03/04/06`. Those rule IDs are split temporarily so the package can stay on parsed-file inputs only.
- `RS-GARDE-SOURCE-01`, `07`, `08`, `09`, `11`, `12`, `13`, and `14` are still app-side because their current facts are normalized source facts rather than a clean parsed-file package boundary.
- `RS-GARDE-SOURCE-10` remains app-side and should stay the sole malformed-input sink unless the family boundary changes deliberately.
