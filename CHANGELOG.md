# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_0.1.0 is being prepared; it has not yet been published to crates.io._

### Added

- **The residual planner** (`suunta-contract`): `plan_residual` computes the residual
  `Course` — the `Correction`s that steer an observed state toward a desired `Bearing` —
  from domain-supplied satisfaction and coverage findings. Only positively-certified
  targets are omitted; absence, `Unknown`, and `Unsatisfied` are retained
  (false-negative-safe), and uncertainty, supersession, and conflict are surfaced, never
  disposed.
- **Navigation vocabulary**: `Sigil`, `Correction`, `Course`, `Bearing`, `Reversibility`,
  `Satisfaction`, `SatisfactionFinding`, `CoverageEffect`, `CoverageFinding`,
  `InFlightIndex`, `SurfacedFinding`, and `Residual`.
- **`Residual::is_converged`**: a pure, policy-free structural read reporting full
  convergence (the `Course` is empty and no findings are surfaced).
- **Composition example** (`examples/converge.rs`): a convergence-loop consumer that drives
  the planner end-to-end over the public API, demonstrating the fulfilled and
  domain-disposition halt paths.
- **Executable governance** (`suunta-governance`): dependency-isolation, sans-I/O purity
  (no I/O, no ambient clock, no exposed `async fn`), workspace coverage, and active-prose
  boundaries, each with a firing test.

### Design

- **The semantic bill of purity** (four faces): semantic identity, target satisfaction,
  relevance, and settlement are domain-supplied; the core computes the residual and never
  compares meanings. Settlement decomposes into a mechanical read (the core's
  `is_converged`), a single-cycle disposition (the domain's), and cross-cycle termination
  (a driver's, deferred). See `BACKLOG.md`.
