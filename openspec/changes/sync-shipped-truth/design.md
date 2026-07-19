## Context

An adversarial fidelity audit found Suunta's *code* faithful to the founding intent
(the residual planner compares `Sigil` by value only, stays sans-I/O, teeth fire, DoD
green) but its *status prose* stale: README, the crate module doc, and BACKLOG still
call the planner unbuilt, and BACKLOG still says the semantic bill has three faces
where every other surface says four. The specs also carry two "realized in a later
change" clauses that now contradict the requirement that realized them.

## Goals / Non-Goals

**Goals:**
- Make every governed surface tell the shipped truth: the residual planner is built.
- Preserve this session's design knowledge in the repo (the three→four-faces
  widening, the functional-per-cycle signature, the coverage consumption/production
  split) so it is not lost with the session.

**Non-Goals:**
- No code change — the implementation is faithful; only the docs lag.
- No lifecycle-discipline change — Suunta keeps its one-step sync (the sibling's
  two-step evolution is that repo's self-governed choice, not imported here).

## Decisions

- **Sync prose to reality, not the reverse.** The planner shipped; the docs are
  corrected to match. Only genuinely-deferred items (settlement predicate, async,
  production-side coverage contract) remain future.
- **Four faces everywhere.** The semantic bill is identity + satisfaction + relevance
  + settlement. Satisfaction was added precisely so the core never compares an
  observed `Fix` to a desired `Bearing` — a faithful widening that keeps meaning out
  of the core, recorded consistently.
- **Resolve the State-model question as shipped.** Functional-per-cycle,
  `plan_residual(bearing, satisfaction, coverage)` — the core consumes coverage
  findings *about* in-flight `Correction`s, never raw in-flight, so there is no
  `inflight` parameter. Moved from open question to a resolved reconsideration.
- **Correct the spec deferral clauses.** The two clauses saying the computation is
  "realized in a later change" now point at "The Residual Omits Only Positively-
  Certified Targets", which realized it — removing the shipped-truth contradiction.
  Purpose TBDs on both specs are filled at sync.

## Risks / Trade-offs

- **The prose gate cannot catch this class of drift** (it checks presence, not
  freshness — `STALE_PHRASES` is empty). → This sync is review-driven; the residual
  guard is the fidelity-audit cadence, not an executable tooth. Noted, not papered
  over.
