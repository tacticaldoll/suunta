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
