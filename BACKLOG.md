# Backlog & Deferred Decisions

This file records deferred decisions, open design questions, and candidate patterns.
It is not a phase roadmap and creates no implementation commitments. Shipped truth
lives in `openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- The **initial project shape** is established: vision and non-goals (`PROJECT.md`),
  operating protocol and axioms (`AGENTS.md`), navigation domain language
  (`docs/domain-language.md`), executable governance (`suunta-governance`), and the
  crate layout (`suunta-contract` + `suunta-governance`) — since grown, per the next
  entry, into the shipped planner.
- The residual planner is **shipped**: `plan_residual` computes the residual `Course`
  from a `Bearing` and domain-supplied satisfaction and coverage findings, alongside
  the `Correction`/`Course`/`Sigil`/`Reversibility` vocabulary. Its contract lives in
  `openspec/specs/` (`convergence-contract`). Remaining open design questions are below.

## Workspace Composition

The workspace stays thin. It owns the planning contract (`suunta-contract`) and its
governance gate (`suunta-governance`, unpublished). Durability, gating, execution,
and compensation of a `Correction` are **downstream consumer concerns** and live
outside this workspace. Adding a workspace crate requires a justified Tianheng
boundary or the coverage gate fails.

## Design Decisions (this shape)

- **Residual model.** A `Course` is the `Bearing` filtered to the targets the domain's
  `Fix` (a satisfaction verdict per target) has not certified satisfied and coverage has
  not certified covered — not a diff the core performs over reality, and not a semantic
  comparison. Both satisfaction and relevance are domain-supplied verdicts.
- **The semantic bill of purity.** A sans-I/O pure core cannot make semantic
  judgments, so it outsources four to the domain: **semantic identity** (`Sigil`),
  **target satisfaction** (whether reality meets a `Bearing` target), **relevance**
  (coverage verdict), and **settlement predicate**. These are one purity choice with
  four faces — satisfaction was added when the planner landed, specifically so the core
  never compares reality against a desired `Bearing` itself. The cost — an
  undetected domain semantic error fails silently — is accepted deliberately, not
  patched by pulling judgment into the core.
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

- **Coverage-verdict shape.** How the domain supplies relevance. **Update (shipped):**
  the *consumption* side is now realized in the planner and specs (`The Residual Omits
  Only Positively-Certified Targets`, `Fix Is Domain-Certified Satisfaction, Not
  Observation`, `Uncertainty And Disposition Are Surfaced, Not Resolved`); only the
  *production* side remains open.
  **Clarified boundary:** split the *consumption* contract (realized) from the
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
    directionality, pairwise vs. aggregate — are NOT frozen. **Update (normative):** the
    instance-reference law and the positive-certification / false-negative-bound law are
    now requirements in `convergence-contract` ("Coverage Findings Are Instance-Referenced
    And Positively Certified"); only the *production* side below remains open.
  - *Production (waits for the consumer):* how the domain computes, keys, indexes,
    batches, or caches findings; any `may_overlap`/candidate mechanism; any
    user-implemented judgment trait. Never frozen ahead of a real consumer.
  - **Who cancels a `Superseded` One-Way?** No one in the core. Supersession is a
    *cycle-scoped relation* (a function of this `Bearing`/`Fix`/context), surfaced as a
    finding on the planning output — never a mutable lifecycle status on `Correction`.
    The core does not name execution-lifecycle states (`Claim` etc. are downstream/sibling
    vocabulary), does not cancel, and does not erase realized effects; pre/post-execution
    disposition is consumer policy.
- **Settlement — three layers, three homes** (was: "settlement-predicate shape").
  "Is this converged?" is not one question, and asking it as one is why it stalled. It
  decomposes into three layers, each with a different owner. Keeping them separate is
  the point; a single settlement *predicate* would fuse a mechanical read, a domain
  judgment, and a runtime concern into one surface that cannot honestly live in the
  core.
  - *Layer 1 — mechanical single-cycle read (consumption). **Resolved.*** "Is the
    residual `Course` empty and are the surfaced findings clear?" The core exposes one
    pure, policy-free read — `Residual::is_converged` (true iff the `Course` is empty
    **and** no findings are surfaced) — and nothing richer. The three-way
    `Settlement { Converged | Pending | Blocked }` classifier once sketched here is
    **rejected**: deciding whether a `Conflicting` or `UnknownRetained` finding is
    *blocking* vs. *pending* is a disposition, not a mechanical read, so it belongs to
    Layer 2, not the core. `is_converged` earns its place by closing a real silent
    failure — a consumer checking only an empty `Course` would declare success while an
    undisposed surfaced finding still lingers.
  - *Layer 2 — single-cycle disposition (domain production). **Resolved.*** "Is a
    retained `Unsatisfied` target a transient miss (keep correcting) or a permanent
    breach (terminal)? Does a surfaced `Conflicting`/`Superseded` finding block
    settlement?" This is a semantic judgment — the fourth face of the semantic bill — so
    it stays the domain's, evaluated in the consumer's loop body. The core provides **no
    settlement trait**; the composition example demonstrates the domain making the
    retry / breach / abandon / hold calls itself, in its own vocabulary, while the core
    stays a pure per-cycle planner.
  - *Layer 3 — cross-cycle termination (runtime/driver). **Open, deferred.*** "No
    progress for N cycles → stalled", "attempt another round?". This needs
    cross-`Sounding` state, which the core refuses (functional-per-cycle). It is
    inherently a runtime/driver concern, deferred until a real driver forces it — the
    same trigger as the async variant below. The composition example holds its own loop
    bound and progress check in the consumer, never in the core, which is exactly where
    Layer 3 lives today.
  - **How it was settled.** A minimal convergence-loop consumer
    (`crates/suunta-contract/examples/converge.rs`) forced Layers 1 and 2 by driving four
    trajectories in one run — a target that converges, one stuck `Unknown`, one
    permanently `Unsatisfied`, and an in-flight `Conflicts`. Building it confirmed
    `is_converged` is the whole of Layer 1 and that disposition is domain loop-body
    policy (Layer 2). A self-authored example is a strawman guarded by the adversarial
    review at propose and apply; the example self-asserts each trajectory's outcome so a
    regression fails the gate rather than passing quietly.
- **The unenforceable purity invariant.** "The core makes no semantic judgment" is
  not statically expressible — semantic comparison has no syntactic marker, so
  Tianheng cannot bite it the way it bites no-I/O or no-async. It stays review- and
  structure-governed.
  **Resolved (detection):** the core performs **no** structural contradiction detection.
  Comparing a `Sigil` against a content fingerprint to spot a drifted or split identity is
  itself a comparison the pure core does not make; it belongs to a downstream
  identity-reconciliation consumer, never here.
  **Open, deferred (carrying):** whether the core *carries* a domain-supplied content
  `Fingerprint` so a downstream detector can make that comparison. Determined shape, if
  landed:
  - Identity has two faces — `Sigil` (stable across meaning-preserving change) and a
    content `Fingerprint` (the current content's identity); both domain-supplied.
  - It is an **opaque value** on the `Correction`, a peer of `Sigil` — **not a trait or a
    `Body` bound.** A trait/bound would make the core call a method on the payload,
    breaching "the core exposes no operation that reads a `Body`"; a value never touches
    it. The core carries it and does **not even compare it** (comparison is the downstream
    detector's), so it is *more* opaque than `Sigil`, which the core at least compares by
    value for the residual filter.
  - Obligation form follows what the core does with it: a *carried* value is a field with
    its law stated in the spec **when landed**; only a behaviour the core *calls* would be
    a trait — and the core calls none here.
  - **Least-commitment:** landing waits for the first real consumer that reconciles content
    identity to force the shape; recorded now only as a determined candidate. Framed
    generally (any identity-reconciliation consumer); if it cannot stand as a general
    identity contract, the fingerprint rides inside the opaque `Body` and the core carries
    nothing new.
- **Async variant.** Deferred until a real driver forces it; the sans-I/O core is
  agnostic to sync/async at the edge.
- **The driver/core seam — the shape is resolved, realization deferred.** Explored
  (see git history) by tracing what actually crosses the core boundary each cycle in
  the facade's convergence-loop test. When you try to draw a "driver contract", it
  collapses into two disjoint halves:
  - *Already owned, and purely nouns.* The `Bearing` (desired targets, carrying the
    domain payloads) is the persistent *reference* the domain sounds against. One
    cycle's readings are a `Sounding` — the `Fix` (per-target satisfaction verdicts)
    plus the coverage verdicts — fed with the `Bearing` to `plan_residual`, which
    returns a `Residual` (the `Course` plus surfaced findings) carrying the Layer-1
    `is_converged` read. The seam is `plan_residual(&Bearing, &Sounding) -> Residual`:
    two input nouns, one output noun, one pure function. The core owns 100% of this,
    statelessly and functional-per-cycle; there is no third thing waiting to be named.
    (An earlier sketch wrongly put the `Bearing` *inside* the `Sounding`. Per
    `docs/domain-language.md` a `Sounding` is what one cycle reads and certifies
    *against* the `Bearing`, so the `Bearing` is the reference, never part of the
    reading.)
  - *A purity bonus falls out.* A `Sounding` (Fix + coverage) references targets and
    in-flight only by `Sigil`/index plus verdict — it carries **no `Body`**. The domain
    payload therefore threads `Bearing<Body> -> Course<Body>` and is never visible to
    the readings, so "the core consumes verdicts, not reality" becomes a property the
    type system can back, not only prose.
  - *Must never become a contract.* The loop, single-cycle disposition (Layer 2, the
    domain's), and cross-cycle termination (Layer 3, the driver's — needs
    cross-`Sounding` state the core refuses) have no data the core can own. Giving any
    of them a *trait the user implements* rebuilds a Tower-style `Service` — a
    framework dictating the consumer's shape — which is the identity Suunta is defined
    against. The seam stays **values the driver supplies to a pure function**, never a
    behaviour it must implement.
  - *Why the collapse is the answer.* The core already owns the whole honest seam and
    the driver is correctly empty, which is precisely why Layer 3 and the async variant
    wait for a *real* driver rather than a speculative contract. Positioning (candidate
    for `PROJECT.md`, not yet captured there): the framework owns the **nouns** (the
    vocabulary of exchange), the consumer owns the **verbs** (judgment, execution,
    driving); Suunta's governance teeth point **inward** (they constrain its own
    purity), never **outward** at the consumer's design — the inverse of Tower.
  - *One open sub-decision (undecided), in three grades.* All are naming-only — no new
    capability, no cross-cycle state, no judgment — and fall inside the consumption
    envelope this file already permits to freeze provisionally. `Sounding` and `Fix`
    are defined in `docs/domain-language.md` but unbuilt (a vocabulary/code gap
    **vocabulary-as-governance** argues to close):
    - **B1a — name `Fix` only.** Wrap the per-target satisfaction verdicts as a `Fix`;
      keep `plan_residual(bearing, &fix, &coverage)`. Strongest fit, least churn.
    - **B1b — name `Fix` + collect a body-free `Sounding {fix, coverage}`.**
      `plan_residual(bearing, &sounding)`. Fuller vocabulary realization, and welds the
      verdicts-not-reality axiom into a type (the purity bonus above). Cost: accept
      "`Sounding` = one cycle's certified readings" as its meaning.
    - **B1c — `Sounding {bearing, fix, coverage}`.** *Rejected*: conflates the
      reference with the reading.
    Leaning **B1b** for the type-backed axiom; **B1a** is the clean fallback. Against
    both stands **least-commitment**: keep the current three-argument signature until a
    real driver shows the naming earns its keep.

## Recorded Reconsiderations

Inherited discipline first, then this project's own resolved design decisions.


- **No architecture-decision-record files.** Decision provenance lives in git commit
  bodies and pull requests; reconsiderations live here; the living docs are the
  single source of truth for current state. The starter's `docs/adr/` was removed on
  birth.
- **No OpenSpec change archive.** Sync promotes delta specs into `openspec/specs/`
  and removes the change directory; git retains the deliberation. `openspec archive`
  recreates `openspec/changes/archive/` — remove it after each sync.
- **Definition of Done is single-sourced in `AGENTS.md`.** `README.md` and
  `docs/development-flow.md` point to it rather than restating a divergent subset.
- **State model — resolved: functional per cycle.** `plan_residual(bearing,
  satisfaction, coverage)` is a pure function of one cycle's inputs and holds no
  cross-`Sounding` state. It consumes domain-certified coverage findings *about*
  in-flight `Correction`s, not the raw in-flight set — so there is no `inflight`
  parameter (an earlier leaning had `plan_residual(bearing, fix, inflight, findings)`;
  the shipped shape replaced `fix` with satisfaction findings and dropped raw
  `inflight`). Settled by the residual-planner landing, not left open.

## Explicitly Deferred

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
