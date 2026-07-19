## Why

Later sessions built the residual planner (`land-navigation-vocabulary`,
`land-residual-planner`) and widened the semantic bill of purity from three faces to
four (adding target satisfaction). But several governed docs still describe the core
as an unbuilt skeleton, and one surface still says "three faces". `openspec/specs/` is
declared shipped architecture truth, so stale status prose is a shipped-truth-honesty
defect — and it is exactly the BACKLOG-sync step the fidelity-audit cadence exists to
catch. This change syncs every surface to what actually shipped and folds the audit's
findings into the repo so the knowledge is not lost.

## What Changes

- `README.md`: the Status section states the residual planner is shipped in 0.1.0
  (not "a compiling skeleton / implemented in later changes"); the owns/supplies table
  and the sans-I/O paragraph reflect the **four** faces (identity, satisfaction,
  relevance, settlement).
- `crates/suunta-contract/src/lib.rs`: Axiom 1 lists four faces; the `# Status`
  section says the residual planner (`plan_residual`, `Bearing`, satisfaction/coverage
  findings) is landed, not deferred.
- `BACKLOG.md`: Current Baseline records the shipped planner; the semantic bill is
  four faces; the "State model" open question is resolved (functional-per-cycle,
  `plan_residual(bearing, satisfaction, coverage)` — coverage findings *about*
  in-flight, no raw `inflight` parameter); "implementation of residual planning" is
  removed from Explicitly Deferred.
- `convergence-contract` spec: the two "realized in a later change" deferral clauses
  are corrected to point at the realizing requirement; both specs' `Purpose` (TBD) are
  filled.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `convergence-contract`: the "A Course Is A Residual" and "A Course Is An Ordered
  Value Of Corrections" requirements have their deferral clauses corrected to state
  the residual computation is realized (by "The Residual Omits Only Positively-
  Certified Targets"), removing the internal contradiction with that requirement.

## Impact

- Docs and spec prose only — no code change; the DoD stays green; manifests stay
  `0.1.0`. This corrects shipped-truth honesty; it changes no behavior.
- Lifecycle discipline is unchanged (Suunta keeps its one-step sync); this change does
  not touch the sync/archive stance.
