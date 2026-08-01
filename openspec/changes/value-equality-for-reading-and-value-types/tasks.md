## 1. Implementation

- [ ] 1.1 Derive `PartialEq, Eq` on `SatisfactionFinding`, `Fix`,
      `CoverageFinding`, and `Sounding` in `crates/suunta-contract/src/lib.rs`.
- [ ] 1.2 Derive `PartialEq, Eq` on `Correction<Body>`, `Course<Body>`,
      `Bearing<Body>`, and `Residual<Body>`.
- [ ] 1.3 Add unit tests demonstrating equality for at least one body-free type
      and one `Body`-generic type (equal case and unequal case).
- [ ] 1.4 Run the full Definition of Done (`AGENTS.md`) and confirm it passes.

## 2. Sync

- [ ] 2.1 Merge the delta spec's ADDED requirement into
      `openspec/specs/convergence-contract/spec.md`.
- [ ] 2.2 Remove the `value-equality-for-reading-and-value-types` change
      directory.
