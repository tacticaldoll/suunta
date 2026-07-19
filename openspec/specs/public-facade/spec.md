# public-facade Specification

## Purpose
Define the curated `suunta` facade as the workspace's single compose-level public entrypoint: a pure re-export crate over `suunta-contract` that withholds nothing (there is no advanced kernel), proves end-to-end convergence composition with a crate-root doctest and a facade integration test, and is enforced by executable governance.

## Requirements
### Requirement: Curated Public Entrypoint
Suunta SHALL provide a single facade crate `suunta` that is the curated public
entrypoint to the compose-level convergence-planning API. The facade SHALL
re-export the public items a downstream consumer needs to run a convergence loop —
build a `Bearing` of `Correction`s, supply the domain's `SatisfactionFinding`s and
`CoverageFinding`s, call `plan_residual`, and read the resulting `Residual` —
drawing them from `suunta-contract`. Because all of `suunta-contract`'s public API
is compose-level (there is no advanced kernel to withhold), the facade SHALL
re-export it in full. The facade SHALL depend only on `suunta-contract`.

#### Scenario: Facade re-exports the compose-level surface
- **WHEN** a downstream consumer depends only on `suunta`
- **THEN** it can name `Bearing`, `Correction`, `Course`, `Sigil`, `Reversibility`, `Satisfaction`, `SatisfactionFinding`, `CoverageEffect`, `CoverageFinding`, `InFlightIndex`, `SurfacedFinding`, and `Residual`, and call `plan_residual`, without depending on `suunta-contract` directly

#### Scenario: Facade depends only on the core
- **WHEN** `cargo run -p suunta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** the Tianheng constitution reports no violation, because `suunta` depends only on `suunta-contract`

### Requirement: Facade Carries No Logic
The facade SHALL be a pure re-export crate: its library SHALL contain only
re-exports, crate attributes, and documentation, and SHALL NOT define functions,
types, traits, or other behavior. This keeps the published entrypoint from
accreting convenience logic over time. This constraint SHALL be enforced by an
executable reaction, not by omission alone.

#### Scenario: A logic item in the facade fails governance
- **WHEN** the facade library defines an item other than a re-export (for example a function, struct, enum, or trait)
- **THEN** the governance reaction fails, naming the offending line

#### Scenario: The facade library composes only through re-exports
- **WHEN** the facade library is reviewed
- **THEN** every public item it offers is a re-export of an item from `suunta-contract`, and it holds no logic of its own

### Requirement: Facade Composition Doctest
The `suunta` facade SHALL carry a runnable documentation test in its crate root
that drives a convergence loop end to end through the facade — build a `Bearing`,
supply per-cycle domain satisfaction, call `plan_residual`, and halt on
`Residual::is_converged` — using only the facade's public API. This composition
proof SHALL be a doctest rather than a separate `examples/` build target, so it
runs and asserts under `cargo test` and is rendered on the published documentation.

#### Scenario: Facade composition doctest drives a converging loop
- **WHEN** the facade doctest runs a loop whose domain leaves a target unsatisfied and then certifies it `Satisfied`
- **THEN** an earlier cycle retains the target in the residual `Course` and a later cycle halts the loop via `Residual::is_converged`

#### Scenario: Facade composition doctest imports only from the facade
- **WHEN** the facade doctest is compiled
- **THEN** it references only items re-exported by `suunta`, and does not import from `suunta-contract` directly

### Requirement: Composition Is Demonstrated In Depth
The `suunta` facade SHALL carry an integration test that drives a full convergence
loop over the facade's public API across four trajectories — a target that
converges, a target left uncertain (`Unknown`), a target that never satisfies, and
an in-flight `Correction` marked conflicting — so its demonstration is earned rather
than a happy-path stub. All loop state (the cycle bound) and every disposition
(retry versus terminal, conflict handling) SHALL be held in the consumer test, not
the core. It SHALL run clean under the Definition of Done, so composability is an
enforced, non-regressing property rather than a claim.

#### Scenario: The four trajectories are exercised through the facade
- **WHEN** the integration test runs its stub domain
- **THEN** it drives a converging target, an `Unknown`-retained target, a permanently unsatisfied target, and a conflicting in-flight correction within one run, using only items re-exported by `suunta`

#### Scenario: The consumer owns the loop bound and disposition
- **WHEN** the integration test halts
- **THEN** the cycle bound and every retry-versus-terminal and conflict decision were made in the test's own loop body, and the core disposed of nothing

#### Scenario: A broken composition fails the gate
- **WHEN** the facade fails to compose or the loop does not reach its expected end state
- **THEN** running the test under the Definition of Done fails rather than passing silently
