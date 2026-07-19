# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-07-17

An identity-and-governance release. No change to the public API surface: the same items
are exported and every behaviour is unchanged. The work sharpens Suunta's stated identity
and makes one completeness invariant structural.

### Changed

- **The facade re-exports the core surface by glob** (`pub use suunta_contract::*`), so
  "the facade withholds nothing" is enforced by the compiler in both directions — a new
  core item appears automatically and none can be silently dropped or left behind. The
  exported surface is identical to before.

### Documentation

- Reclaimed the vision: Suunta is a convergence-planning **design pattern** for
  composition-driven users; it owns the nouns and outsources every verb. Retired
  "wait for the first real consumer to force the shape" as a disposition mode.
- Added the **governance/conformance model** and its four-gate test for when a judgment
  may become a Tianheng tooth, plus **manifestation** as a third disposition alongside
  noun and verb.
- Generalized the anti-Tower stance to refusing **any surface that dictates the
  consumer's shape** (trait, run-loop engine, middleware stack, readiness contract).
- Recorded `Drift` as a concept, not a type — the residual `Course` already embodies it.

## [0.1.0] - 2026-07-14

The initial release: the residual planner, the navigation vocabulary (realized as
types), the curated `suunta` facade, and executable Tianheng governance.

### Added

- **The residual planner** (`suunta-contract`): `plan_residual` computes the residual
  `Course` — the `Correction`s that steer an observed state toward a desired `Bearing` —
  from a `Bearing` and a per-cycle `Sounding` (the domain's `Fix` — satisfaction verdicts
  — and coverage findings). Only positively-certified targets are omitted; absence,
  `Unknown`, and `Unsatisfied` are retained (false-negative-safe), and uncertainty,
  supersession, and conflict are surfaced, never disposed.
- **Navigation vocabulary**: `Sigil`, `Correction`, `Course`, `Bearing`, `Reversibility`,
  `Satisfaction`, `SatisfactionFinding`, `Fix`, `CoverageEffect`, `CoverageFinding`,
  `InFlightIndex`, `Sounding`, `SurfacedFinding`, and `Residual`. A `Sounding` is one
  cycle's certified readings (`Fix` + coverage) and carries no domain payload.
- **`Residual::is_converged`**: a pure, policy-free structural read reporting full
  convergence (the `Course` is empty and no findings are surfaced).
- **Curated facade** (`suunta`): the recommended single entrypoint — a pure re-export of
  the compose-level surface, carrying no logic of its own. Its crate-root doctest drives a
  converging loop end-to-end through the public API; `crates/suunta/tests/convergence_loop.rs`
  drives the four-trajectory (converge / `Unknown` / never-satisfies / conflicting in-flight)
  demonstration. This retires the former `suunta-contract` `examples/converge.rs`: the
  composition proof now lives on the facade, off the core crate.
- **Executable governance** (`suunta-governance`): dependency-isolation, sans-I/O purity
  (no I/O, no ambient clock, no exposed `async fn`), a facade dependency boundary and a
  re-exports-only source tooth, workspace coverage, and active-prose boundaries, each with a
  firing test.

### Design

- **The semantic bill of purity** (four faces): semantic identity, target satisfaction,
  relevance, and settlement are domain-supplied; the core computes the residual and never
  compares meanings. Settlement decomposes into a mechanical read (the core's
  `is_converged`), a single-cycle disposition (the domain's), and cross-cycle termination
  (a driver's, deferred). See `BACKLOG.md`.

[0.1.1]: https://github.com/tacticaldoll/suunta/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/suunta/releases/tag/v0.1.0
