# Domain Language

Suunta uses a navigation register as architecture, not branding. Each term names a
role in the convergence loop; prefer the canonical term over synonyms.

## The convergence loop

```text
Domain certifies reality against the Bearing's targets:
  per target    -> Fix       (satisfaction verdict)  ┐
  per in-flight -> coverage   (relevance verdict)     ┤ Suunta filters the Bearing
Domain          -> Bearing    (desired targets)       ┘   -> Course (residual Corrections)
```

- **Sounding** — one convergence cycle. In it the domain reads reality and certifies,
  per `Bearing` target, whether it is met — yielding a `Fix`. (Nautical: taking a
  depth or position measurement.)
- **Fix** — the domain's certified satisfaction of the `Bearing`'s targets: for each,
  whether reality meets it. A reading taken *against intent*, which only the domain can
  take, since comparing reality to a desired target is a meaning comparison. Not a raw
  observation — the core consumes these verdicts, never reality itself.
- **Bearing** — the desired target state, supplied by the domain. What *should be*.
- **Drift** — the `Bearing` targets a `Fix` does not certify satisfied. `Correction`s
  close it; the residual `Course` retains those not already covered by a relevant
  in-flight `Correction`.
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
  The domain declares it; the core (or a downstream evaluator) only evaluates it,
  never invents it.

These four are one purity choice with four faces. Their cost — an undetected
domain semantic error fails silently — is accepted deliberately. See `PROJECT.md`
and `BACKLOG.md` for the rationale and the open questions.

## Out of scope for the core

Durability of a `Correction`, gating whether it may proceed, executing it, and
compensating a One-Way `Correction` are **downstream consumer concerns**, not the
planning core. Suunta emits a `Course`; what happens to its `Correction`s afterward
is composed outside `suunta-contract`.
