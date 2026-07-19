# Domain Language

Suunta uses a navigation register as architecture, not branding. Each term names a
role in the convergence loop; prefer the canonical term over synonyms.

## The convergence loop

```text
Sounding  -> Fix ┐
Domain    -> Bearing ┤ residual -> Course (Corrections)
in-flight ─────────┘
```

- **Sounding** — one observation cycle. A sounding measures the world and yields a
  `Fix`. (Nautical: taking a depth or position measurement.)
- **Fix** — the determined current state, derived from a sounding. What *is*.
- **Bearing** — the desired target state, supplied by the domain. What *should be*.
- **Drift** — the divergence between `Bearing` and `Fix` that `Correction`s close.
- **Course** — the residual plan: the `Correction`s that steer from `Fix` toward
  `Bearing`, computed as `Bearing` diffed against `Fix` unioned with the *relevant*
  in-flight `Correction`s.
- **Correction** — a single planned change on the `Course`. Carries a `Sigil` and a
  reversibility marking.
- **Sigil** — a domain-supplied, cross-cycle-stable semantic identity for a
  `Correction`'s target. The same intent across soundings carries the same `Sigil`;
  a genuine semantic change means a new `Sigil`. The core carries a `Sigil` but never
  interprets it — semantic identity is a domain judgment.
- **One-Way** — a `Correction` that cannot be undone. It must be marked; the core
  does not own rollback or compensation.

## Domain-supplied judgments (the semantic bill of purity)

The core makes no semantic judgment. Three judgments are the domain's obligation:

- **Semantic identity** — the `Sigil` (above).
- **Relevance (coverage verdict)** — which in-flight `Correction`s are relevant to a
  `Bearing` when computing the residual. The core does not compare meanings to decide
  relevance; the domain supplies the verdict.
- **Settlement predicate** — when a `Correction`'s obligation counts as concluded.
  The domain declares it; the core (or a downstream evaluator) only evaluates it,
  never invents it.

These three are one purity choice with three faces. Their cost — an undetected
domain semantic error fails silently — is accepted deliberately. See `PROJECT.md`
and `BACKLOG.md` for the rationale and the open questions.

## Out of scope for the core

Durability of a `Correction`, gating whether it may proceed, executing it, and
compensating a One-Way `Correction` are **downstream consumer concerns**, not the
planning core. Suunta emits a `Course`; what happens to its `Correction`s afterward
is composed outside `suunta-contract`.
