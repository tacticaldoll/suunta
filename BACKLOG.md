# Backlog & Deferred Decisions

This file records deferred decisions, open design questions, and candidate patterns.
It is not a phase roadmap and creates no implementation commitments. Shipped truth
lives in `openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- The **initial project shape** is established: vision and non-goals (`PROJECT.md`),
  operating protocol and axioms (`AGENTS.md`), navigation domain language
  (`docs/domain-language.md`), executable governance (`suunta-governance`), and a
  compiling crate skeleton (`suunta-contract`).
- The residual-planning **core is not yet implemented**. Its intended contract lives
  in `openspec/specs/`; implementation follows in later spec-driven changes,
  deliberately, because the design still has open questions (below).

## Workspace Composition

The workspace stays thin. It owns the planning contract (`suunta-contract`) and its
governance gate (`suunta-governance`, unpublished). Durability, gating, execution,
and compensation of a `Correction` are **downstream consumer concerns** and live
outside this workspace. Adding a workspace crate requires a justified Tianheng
boundary or the coverage gate fails.

## Design Decisions (this shape)

- **Residual model.** A `Course` is `Bearing` diffed against `Fix` unioned with the
  *relevant* in-flight `Correction`s — not a raw union, and not a semantic comparison
  the core performs. Relevance is a domain-supplied coverage verdict.
- **The semantic bill of purity.** A sans-I/O pure core cannot make semantic
  judgments, so it outsources three to the domain: **semantic identity** (`Sigil`),
  **relevance** (coverage verdict), and **settlement predicate**. These are one
  purity choice with three faces. The cost — an undetected domain semantic error
  fails silently — is accepted deliberately, not patched by pulling judgment into the
  core.
- **Sigil stability is a domain contract.** Each `Correction` carries a
  domain-supplied `Sigil` that is stable across soundings and changes only on a
  genuine semantic change. The core carries it opaquely and never interprets it.
- **Reversibility.** A One-Way `Correction` must be marked. The core does not own
  rollback or compensation.
- **Core mechanism, edge policy.** The core computes *what* diverges; the disposition
  of a failed or breached `Correction` (retry, replan, retreat, escalate) is
  edge/consumer policy, not core behavior. Compensation of a One-Way `Correction` is
  a downstream concern.
- **Governance mostly mirrors the reference discipline** (crate-dependency boundaries,
  sans-I/O purity teeth, workspace coverage, active-prose governance) — with one
  honest exception: see the open question on the unenforceable purity invariant.

## Open Design Questions

These are recorded so the repo can drive its own development; none is decided here.
Discipline: keep judgment domain-supplied; the core only computes or evaluates,
never compares meanings. Do not freeze a user-**implemented** judgment-**production**
trait ahead of its first real consumer — but the planner MAY provisionally expose the
minimum **consumption** envelope residual mechanics require (the planning effects the
core acts on, the structural laws findings obey, calibration fixtures), because that
envelope is produced by the domain and consumed by the core, not implemented by the
user. Consumption may freeze provisionally; production waits for the consumer.

- **Coverage-verdict shape.** How the domain supplies relevance. **Clarified boundary:**
  split the *consumption* contract (freezable now — mechanism-determined) from the
  *production* contract (waits for the first consumer).
  - *Consumption (provisionally freezable):* the core consumes domain-produced findings
    as opaque values and acts on a fixed set of **planning effects** it must mechanically
    distinguish — (a) an in-flight `Correction` already covers the residual → suppress a
    duplicate; (b) the current plan supersedes an in-flight one → surface a supersession
    finding; (c) an in-flight one cannot safely coexist → surface a conflict, never
    silently plan; (d) the domain positively certifies two are mutually ignorable →
    exclude from coverage. A finding references a specific in-flight **instance** (not a
    `Sigil`, since a `Course` does not dedup); **any in-flight instance with no finding is
    `unknown` — retained, never treated as ignorable.** `Independent`/ignorable is a
    positively-certified verdict, never derived from absence (the seam's false-negative
    bound). The class *names/taxonomy* — whether exactly four, supersession
    directionality, pairwise vs. aggregate — are NOT frozen.
  - *Production (waits for the consumer):* how the domain computes, keys, indexes,
    batches, or caches findings; any `may_overlap`/candidate mechanism; any
    user-implemented judgment trait. Never frozen ahead of a real consumer.
  - **Who cancels a `Superseded` One-Way?** No one in the core. Supersession is a
    *cycle-scoped relation* (a function of this `Bearing`/`Fix`/context), surfaced as a
    finding on the planning output — never a mutable lifecycle status on `Correction`.
    The core does not name execution-lifecycle states (`Claim` etc. are downstream/sibling
    vocabulary), does not cancel, and does not erase realized effects; pre/post-execution
    disposition is consumer policy.
- **Settlement-predicate shape.** Likely a small closed enum (e.g. `Fulfilled` /
  `Breached` / `Terminal`), declared by the domain and only evaluated by the core or
  a downstream evaluator — must not become a DSL. Not yet designed; co-design it with
  its first consumer.
- **The unenforceable purity invariant.** "The core makes no semantic judgment" is
  not statically expressible — semantic comparison has no syntactic marker, so
  Tianheng cannot bite it the way it bites no-I/O or no-async. It stays review- and
  structure-governed. Whether to add **structural contradiction detection** (e.g.
  same `Sigil` with a drifted fingerprint, or same fingerprint with split `Sigil`s)
  to turn a silent domain error into an observable alarm — and whether that detection
  belongs in this core at all or in a downstream idempotency-adjudication consumer —
  is open. Leaning: keep the core pure; detection is a downstream concern.
- **State model.** Whether the residual core is purely functional per cycle or must
  track in-flight `Correction`s across soundings, and if the latter, where that state
  lives without pulling I/O into the core. **Leaning:** functional-per-cycle — a
  `plan_residual(bearing, fix, inflight, findings)` shape takes the in-flight set as a
  caller-supplied input each cycle (state injected at the edge, like time), so the core
  holds no cross-sounding state. To be settled by the residual-mechanism change that
  lands that signature, not decided here.
- **Async variant.** Deferred until a real driver forces it; the sans-I/O core is
  agnostic to sync/async at the edge.

## Recorded Reconsiderations (inherited discipline)

- **No architecture-decision-record files.** Decision provenance lives in git commit
  bodies and pull requests; reconsiderations live here; the living docs are the
  single source of truth for current state. The starter's `docs/adr/` was removed on
  birth.
- **No OpenSpec change archive.** Sync promotes delta specs into `openspec/specs/`
  and removes the change directory; git retains the deliberation. `openspec archive`
  recreates `openspec/changes/archive/` — remove it after each sync.
- **Definition of Done is single-sourced in `AGENTS.md`.** `README.md` and
  `docs/development-flow.md` point to it rather than restating a divergent subset.

## Explicitly Deferred

- Implementation of residual planning (spec-driven, later changes).
- Any semantic comparison inside the core (never — it is the domain's).
- Durable execution, gating, and compensation of `Correction`s (downstream).
- An async core variant (until a driver forces it).

## Prioritization

Prefer changes that preserve thinness and strengthen governance:

1. Protect the planning core and the navigation vocabulary.
2. Keep semantic judgment domain-supplied.
3. Add behavior only as a governed pattern on a named surface.
4. Reject downstream concerns (durability, gating, execution, compensation) leaking
   into the core.
