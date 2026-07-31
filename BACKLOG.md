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

- **Residual model.** A `Course` is `Bearing` diffed against `Fix` unioned with the
  *relevant* in-flight `Correction`s — not a raw union, and not a semantic comparison
  the core performs. Relevance is a domain-supplied coverage verdict.
- **The semantic bill of purity.** A sans-I/O pure core cannot make semantic
  judgments, so it outsources four to the domain: **semantic identity** (`Sigil`),
  **target satisfaction** (whether reality meets a `Bearing` target), **relevance**
  (coverage verdict), and **settlement predicate**. These are one purity choice with
  four faces — satisfaction was added when the planner landed, specifically so the core
  never compares an observed `Fix` against a desired `Bearing` itself. The cost — an
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
  structure-governed. Whether to add **structural contradiction detection** (e.g.
  same `Sigil` with a drifted fingerprint, or same fingerprint with split `Sigil`s)
  to turn a silent domain error into an observable alarm — and whether that detection
  belongs in this core at all or in a downstream consumer that reconciles content
  identity — is open. Leaning: keep the core pure; detection is a downstream concern.
- **Async variant.** Deferred until a real driver forces it; the sans-I/O core is
  agnostic to sync/async at the edge.

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
