# Domain Language

Suunta uses a navigation register as architecture, not branding. Each term names a
role in the convergence loop; prefer the canonical term over synonyms.

## The convergence loop

```text
Domain certifies reality against the Bearing's targets:
  per target    -> Fix       (normalized satisfaction effect)  ┐
  per in-flight -> coverage   (normalized planning effect)      ┤ Suunta filters the Bearing
Domain          -> Bearing    (desired targets)                 ┘   -> Course (residual Corrections)
```

- **Sounding** — one convergence cycle. In it the domain reads reality and certifies,
  per `Bearing` target, whether it is met — yielding a `Fix`. (Nautical: taking a
  depth or position measurement.) Realized as the `Sounding` type: one cycle's certified
  readings — the `Fix` and coverage findings — carrying no domain payload, fed with the
  `Bearing` to `plan_residual`.
- **Fix** — the domain's certified satisfaction of the `Bearing`'s targets: for each,
  whether reality meets it. A reading taken *against intent*, which only the domain can
  take, since comparing reality to a desired target is a meaning comparison. Not a raw
  observation — the core consumes these verdicts, never reality itself. Realized as the
  `Fix` type, aggregating the per-target `SatisfactionFinding`s.
- **Bearing** — the desired target state, supplied by the domain. What *should be*.
- **Drift** — the `Bearing` targets a `Fix` does not certify satisfied. `Correction`s
  close it; the residual `Course` retains those not already covered by a relevant
  in-flight `Correction`. Drift is a *concept*, deliberately not a type: the residual
  `Course` already embodies it, so naming a `Drift` type would be redundant ceremony
  against thinness (see `BACKLOG.md`).
- **Course** — the residual plan: the `Bearing` targets the `Fix` has not certified
  satisfied and coverage has not certified covered. The core filters the `Bearing` by
  the domain's verdicts; it does not itself diff reality.
- **Correction** — a single planned change on the `Course`. Carries a `Sigil` and a
  reversibility marking.
- **Sigil** — a domain-supplied, cross-cycle-stable semantic identity for a
  `Correction`'s target. The same intent across soundings carries the same `Sigil`;
  a genuine semantic change means a new `Sigil`. The core carries a `Sigil` but never
  interprets it — semantic identity is a domain judgment.
- **One-Way** — a `Correction` that cannot be undone. It must be marked; the core
  does not own rollback or compensation.

## Domain-supplied judgments (the semantic bill of purity)

The core makes no semantic judgment. Four judgments are the domain's obligation:

- **Semantic identity** — the `Sigil` (above).
- **Target satisfaction** — whether reality already meets a desired `Bearing` target.
  The core cannot compare reality against a desired one; the domain supplies a
  satisfaction verdict per target — this per-target verdict *is* the `Fix` — and the
  core consumes it.
- **Relevance (coverage verdict)** — which in-flight `Correction`s are relevant to a
  `Bearing` when computing the residual. The core does not compare meanings to decide
  relevance; the domain supplies the verdict.
- **Settlement predicate** — when a `Correction`'s obligation counts as concluded.
  The domain declares and evaluates it downstream; the core exposes only the
  policy-free `Residual::is_converged` structural read.

These four are one purity choice with four faces. Their cost — an undetected
domain semantic error fails silently — is accepted deliberately. See `PROJECT.md`
and `BACKLOG.md` for the rationale and the open questions.

## Judgment production and planning effects

The domain may use any internal taxonomy to make its satisfaction and relevance
judgments. Before a `Sounding` reaches Suunta, the domain normalizes that meaning
into the finite effects the residual mechanism consumes:

- `Satisfaction::{Satisfied, Unsatisfied, Unknown}` answers one target-scoped
  question. Only uniform `Satisfied` omits; uniform `Unsatisfied` retains;
  absence, `Unknown`, or mixed answers retain and surface uncertainty.
- `CoverageEffect::Covers(Sigil)` positively covers one semantic target.
- `CoverageEffect::Superseded` says the current plan supersedes the referenced
  in-flight instance.
- `CoverageEffect::Conflicts` says the referenced instance cannot coexist with
  the current plan.

Coverage effects compose independently. Absence is the identity and never
becomes positive coverage. A domain may know that an instance is disjoint, but
that production-side classification has no core contribution and therefore is
not a Suunta effect.

## Out of scope for the core

Durability of a `Correction`, gating whether it may proceed, executing it, and
compensating a One-Way `Correction` are **downstream consumer concerns**, not the
planning core. Suunta emits a `Course`; what happens to its `Correction`s afterward
is composed outside `suunta-contract`.
