## Why

This repository was stamped from the `rust-openspec-starter` template with empty
placeholders. It needs its initial project shape: the vision, the navigation domain
language, the architectural axioms, executable governance, and a compiling crate
skeleton — enough that the residual-planning core can be built from the repo alone in
later spec-driven changes, without losing the design knowledge that motivated it.

The core implementation is deliberately **not** included: the design still has open
questions (the coverage-verdict and settlement-predicate shapes, the unenforceable
purity invariant — see `BACKLOG.md`), and freezing them into code now would violate
least-commitment. This change establishes the shape and the contract; implementation
follows spec-driven.

## What Changes

- Replace the starter placeholders: `PROJECT.md` (vision, positioning, non-goals),
  `AGENTS.md` (axioms, ritual, single-sourced Definition of Done, Lineage),
  `README.md`, and `docs/domain-language.md` (the navigation vocabulary).
- Choose the crate layout and add two compiling crates: `suunta-contract` (the
  isolated planning core, with the `Sigil` and `Reversibility` anchors and the
  sans-I/O axioms) and `suunta-governance` (the Tianheng constitution, unpublished).
- Add supply-chain policy (`deny.toml`) and CI (`.github/workflows/ci.yml`), making
  the Rust Definition of Done runnable.
- Record the design decisions and open questions in `BACKLOG.md`.
- Drop the starter's `docs/adr/` and `openspec/changes/archive/` — this project uses
  git-as-provenance and a no-archive sync (inherited discipline).

## Capabilities

### New Capabilities

- `convergence-contract`: the core planning contract — the navigation vocabulary, the
  residual definition, the semantic bill of purity (no semantic judgment in the
  core), stable-`Sigil` identity, One-Way marking, sans-I/O purity, and dependency
  isolation.
- `quality-governance`: the executable Tianheng constitution and the Definition of
  Done — the dependency boundaries, the sans-I/O teeth, workspace coverage,
  active-prose presence, and the honest acknowledgement that "no semantic judgment"
  is not statically enforceable.

### Modified Capabilities

- (none) — this is the first project-specific change; there is no prior spec.

## Impact

- New crates `suunta-contract` and `suunta-governance`; new `deny.toml`, CI, and the
  filled prose/spec surface. No published API yet — the core types beyond the anchors
  are specified in this change's delta specs (promoted to `openspec/specs/` at sync)
  and built later.
- Manifests at `0.1.0`; nothing is published.
