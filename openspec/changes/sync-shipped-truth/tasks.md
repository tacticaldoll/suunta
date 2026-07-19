## 1. Spec delta

- [x] 1.1 `convergence-contract` delta (MODIFIED): correct the deferral clauses in "A Course Is A Residual" and "A Course Is An Ordered Value Of Corrections" to point at the realizing requirement
- [x] 1.2 `openspec validate sync-shipped-truth --strict` passes

## 2. Prose synced to shipped truth

- [x] 2.1 `README.md`: Status states the residual planner is shipped in 0.1.0; owns/supplies table + sans-I/O paragraph reflect the four faces (identity, satisfaction, relevance, settlement)
- [x] 2.2 `crates/suunta-contract/src/lib.rs`: Axiom 1 → four faces; `# Status` → residual planner (`plan_residual`, `Bearing`, satisfaction/coverage) landed
- [x] 2.3 `BACKLOG.md`: Current Baseline records the shipped planner; semantic bill → four faces; State-model question resolved (functional-per-cycle, `plan_residual(bearing, satisfaction, coverage)`); remove "implementation of residual planning" from Explicitly Deferred
- [x] 2.4 `crates/suunta-contract/README.md` (the crates.io page): four faces + planner shipped, not "three faces / logic follows in later changes"
- [x] 2.5 `crates/suunta-contract/src/lib.rs` `Course` struct doc: computation is performed by `plan_residual`, not "realized in a later change"

## 3. Spec Purpose

- [x] 3.1 Fill the `Purpose` (currently `TBD`) in `convergence-contract` and `quality-governance` specs at sync

## 4. Verify (Definition of Done)

- [x] 4.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 4.2 `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 4.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` (the corrected crate-doc renders clean)
- [x] 4.4 `cargo deny check`
- [x] 4.5 `cargo run -p suunta-governance -- check --manifest-path Cargo.toml`
