## 1. Spec deltas

- [x] 1.1 `convergence-contract` delta (ADDED): vocabulary, residual definition, semantic bill, stable Sigil, One-Way marking, sans-I/O purity, dependency isolation
- [x] 1.2 `quality-governance` delta (ADDED): executable constitution, sans-I/O teeth, coverage, active-prose presence, the unenforceable-invariant acknowledgement, single-sourced DoD
- [x] 1.3 `openspec validate establish-suunta-shape --strict` passes

## 2. Workspace & crates

- [x] 2.1 Root `Cargo.toml`: workspace with `suunta-contract` + `suunta-governance`, `workspace.package`, `workspace.dependencies`
- [x] 2.2 `suunta-contract`: `Sigil` + `Reversibility` anchors and the sans-I/O axioms in the crate doc; compiles with `forbid(unsafe_code)` + `warn(missing_docs)`
- [x] 2.3 `suunta-governance`: the Tianheng constitution (`publish = false`)
- [x] 2.4 `deny.toml` + `.github/workflows/ci.yml` (7-gate DoD + MSRV)
- [x] 2.5 Drop the starter's `docs/adr/` and `openspec/changes/archive/`

## 3. Prose

- [x] 3.1 `PROJECT.md` (vision, positioning, non-goals)
- [x] 3.2 `AGENTS.md` (axioms, ritual, single-sourced DoD, Lineage stamp)
- [x] 3.3 `README.md` and `docs/development-flow.md` (point to `AGENTS.md` for the DoD)
- [x] 3.4 `docs/domain-language.md` (navigation vocabulary + semantic bill)
- [x] 3.5 `BACKLOG.md` (design decisions + open questions)
- [x] 3.6 Crate-local READMEs with absolute LICENSE URLs

## 4. Governance boundaries

- [x] 4.1 Dependency boundaries: `suunta-contract` (none), `suunta-governance` (`tianheng`/`guibiao`)
- [x] 4.2 Sans-I/O teeth on `suunta-contract`: no `std::io/fs/net/process`, no ambient clock, no exposed `async fn` (incl. submodules)
- [x] 4.3 Coverage (every crate covered) and active-prose presence, both proven by test

## 5. Verify (Definition of Done)

- [x] 5.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 5.2 `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 5.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.4 `cargo deny check`
- [x] 5.5 `cargo run -p suunta-governance -- check --manifest-path Cargo.toml` (constitution + active prose)
