# convergence-contract Specification

## Purpose
Define Suunta's convergence-planning contract: the navigation vocabulary; the residual `Course` that `plan_residual` computes from a `Bearing` and domain-certified satisfaction and coverage findings (omitting only positively-certified targets, surfacing uncertainty); the semantic bill of purity in four faces; stable-`Sigil` identity; opaque `Correction` payloads; One-Way marking; sans-I/O purity; and dependency isolation.
## Requirements
### Requirement: Navigation Vocabulary
Suunta SHALL name the convergence-planning roles with a fixed navigation register:
`Sounding` (one convergence cycle's certified readings), `Fix` (the domain's certified
satisfaction of the `Bearing`'s targets), `Bearing` (the domain-supplied desired state),
`Drift` (the divergence between them), `Course` (the residual plan), `Correction` (a
single planned change), and `Sigil` (a domain-supplied stable semantic identity). These
terms are architecture, not branding. `Sounding` and `Fix` SHALL be realized as types in
`suunta-contract`, not left as prose-only roles.

#### Scenario: The vocabulary is canonical
- **WHEN** documentation or the contract crate refers to a convergence-planning role
- **THEN** it uses the canonical navigation term rather than a generic synonym

#### Scenario: Sounding and Fix are realized as types
- **WHEN** `suunta-contract`'s public API is compiled
- **THEN** it exposes a `Fix` type and a `Sounding` type, so the two roles are named in the type system, not only in prose

### Requirement: A Course Is A Residual
A `Course` SHALL represent the residual needed to converge a `Fix` toward a `Bearing`:
the `Correction`s that remain once the relevant in-flight `Correction`s are accounted
for. Relevance SHALL be a domain-supplied coverage verdict, not a comparison the core
performs, and the residual SHALL NOT be a raw set union. This requirement defines the
residual *contract*; the computation that forms a `Course` is realized by the
requirement "The Residual Omits Only Positively-Certified Targets".

#### Scenario: Relevance is a domain-supplied verdict
- **WHEN** a `Course` is formed from a `Bearing`, a `Fix`, and in-flight `Correction`s
- **THEN** which in-flight `Correction`s count as relevant is taken from a domain-supplied coverage verdict, never from the core comparing meanings

### Requirement: A Course Is An Ordered Value Of Corrections
A `Course` SHALL be a public, sans-I/O value type (`Course<Body>`) that holds an
**ordered** collection of `Correction`s and preserves the order supplied by its
producer. The core SHALL NOT deduplicate, reorder, or otherwise reinterpret the
collection, since doing so would be a semantic act the core does not perform. This
requirement defines the `Course` *value*; the requirement that a `Course` is *computed*
as a residual (see "A Course Is A Residual") is realized by "The Residual Omits Only
Positively-Certified Targets".

#### Scenario: Order is preserved
- **WHEN** a `Course` is formed from a sequence of `Correction`s
- **THEN** iterating the `Course` yields those `Correction`s in the order supplied

#### Scenario: The core does not deduplicate
- **WHEN** a `Course` is formed from two `Correction`s carrying equal `Sigil`s
- **THEN** the `Course` retains both; the core collapses nothing, because equality of meaning is not the core's to decide

### Requirement: The Residual Omits Only Positively-Certified Targets
The residual `Course` SHALL contain every `Bearing` target except those a domain
finding positively certifies as `Satisfied` (reality already meets it) or
`Covered` (a relevant in-flight `Correction` handles it). Satisfaction findings
for one target SHALL compose as a mutually exclusive scalar verdict: one or more
`Satisfied` findings with no other satisfaction value SHALL omit the target; one
or more `Unsatisfied` findings with no other value SHALL retain it without an
uncertainty finding; absence, any `Unknown`, or mixed known values SHALL retain
and surface it as unknown. Coverage SHALL be existential and idempotent per
target: any `Covers(target)` finding SHALL omit that target independently of
other coverage effects. This realizes the computation deferred by "A Course Is A
Residual".

#### Scenario: A satisfied or covered target is omitted
- **WHEN** a target has only `Satisfied` satisfaction findings, or at least one `Covers(target)` coverage finding
- **THEN** the target is omitted from the residual `Course`

#### Scenario: Absence and uncertainty retain
- **WHEN** a target has no satisfaction finding, any `Unknown` finding, or mixed satisfaction values and has no `Covers(target)` finding
- **THEN** the target is retained and surfaced as `UnknownRetained`

#### Scenario: Known unsatisfied retains without uncertainty
- **WHEN** a target has one or more `Unsatisfied` findings, no other satisfaction value, and no `Covers(target)` finding
- **THEN** the target is retained without an `UnknownRetained` finding

#### Scenario: Repeated positive certification is idempotent
- **WHEN** a target receives repeated identical `Satisfied` or `Covers(target)` findings
- **THEN** its omission result is the same as for one such finding

### Requirement: Fix Is Domain-Certified Satisfaction, Not Observation
`Fix` SHALL be a named type in `suunta-contract` that aggregates
domain-certified satisfaction findings — a normalized `Satisfaction` planning
effect per referenced `Bearing` target (`SatisfactionFinding`) — and SHALL NOT be
a bare set of satisfied `Sigil`s nor a store of raw observations. The domain
SHALL own how it observes reality and produces or normalizes its judgment;
Suunta SHALL own only the finite consumption effects its residual mechanism
understands. The core SHALL NOT read observation content or compute satisfaction
itself.

#### Scenario: Satisfaction is consumed, not computed
- **WHEN** the core needs to know whether a `Bearing` target is satisfied
- **THEN** it consumes a domain-supplied normalized `Satisfaction` effect rather than comparing an observed state against the desired one

#### Scenario: Domain taxonomy remains downstream
- **WHEN** a domain has richer internal satisfaction states than Suunta consumes
- **THEN** the domain normalizes them before constructing the `Fix`, without implementing a Suunta trait or extending the core enum

#### Scenario: Fix carries effects, not observations
- **WHEN** a `Fix` is supplied to the planner
- **THEN** it carries per-target satisfaction effects, and the core reads no observation body

#### Scenario: Fix is a named aggregate
- **WHEN** a `Fix` is constructed
- **THEN** it is a distinct type wrapping the cycle's `SatisfactionFinding`s, not a bare slice or set, and the per-target entry remains a `SatisfactionFinding`

### Requirement: Uncertainty And Disposition Are Surfaced, Not Resolved
The planner SHALL surface, on its output, each target retained under uncertain
satisfaction and each in-flight `Correction` a coverage finding marks superseded
by or conflicting with the current plan. Supersession SHALL point from the
current plan to the referenced in-flight instance. Each surfaced supersession or
conflict SHALL preserve its effect category and `InFlightIndex`; no target
context SHALL be required for these instance-scoped effects. The core SHALL NOT
cancel, compensate, prioritize, or otherwise dispose of them, and SHALL name no
execution-lifecycle state.

#### Scenario: Unknown retention is an observable finding
- **WHEN** a target is retained because satisfaction is absent, `Unknown`, or mixed
- **THEN** the planner surfaces it as `UnknownRetained`, so uncertainty is visible rather than silently over-planned

#### Scenario: The current plan supersedes an in-flight instance
- **WHEN** a coverage finding marks its referenced in-flight correction `Superseded`
- **THEN** the planner surfaces `SurfacedFinding::Superseded` with that `InFlightIndex` and takes no cancelling action

#### Scenario: Conflict preserves the referenced instance
- **WHEN** a coverage finding marks its referenced in-flight correction `Conflicts`
- **THEN** the planner surfaces `SurfacedFinding::Conflicting` with that `InFlightIndex` and makes no disposition

### Requirement: Coverage Findings Are Instance-Referenced And Positively Certified
A coverage finding SHALL reference a specific in-flight `Correction` instance by
position (`InFlightIndex`), never by `Sigil`, because a `Course` does not
deduplicate and two in-flight corrections may share a `Sigil`. The domain SHALL
own judgment production and SHALL normalize it into Suunta's finite
`CoverageEffect` consumption vocabulary.

Effect scope SHALL follow mechanical contribution: `Covers(Sigil)` SHALL relate
the referenced instance to one semantic `Bearing` target;
`Superseded` and `Conflicts` SHALL describe the referenced instance relative to
the current plan as a whole. `Superseded` SHALL mean the current plan supersedes
the referenced in-flight instance.

Coverage effects SHALL compose independently. Every `Covers(target)` SHALL
contribute existential, idempotent target coverage; every `Superseded` or
`Conflicts` finding SHALL contribute its corresponding surfaced instance. The
core SHALL NOT infer precedence, mutual exclusion, or semantic contradiction
among supplied effects.

Absence SHALL be the coverage algebra's identity: the core forms no relevance
verdict, omits no target, and surfaces no instance. `Disjoint` SHALL NOT be a
Suunta `CoverageEffect`, because the residual mechanism cannot distinguish it
from absence and does not consume the domain's production-side classification.
Only a positive `Covers(target)` effect SHALL omit a target. Planner-internal
effect handling SHALL be exhaustive so a future Suunta-owned effect cannot
silently bypass residual review.

#### Scenario: Findings reference instances, not sigils
- **WHEN** two in-flight `Correction`s share a `Sigil` and the domain reports a coverage finding about one of them
- **THEN** the finding identifies that instance by its `InFlightIndex`, and the other instance is unaffected

#### Scenario: One in-flight instance covers multiple targets
- **WHEN** findings for one `InFlightIndex` contain `Covers(alpha)` and `Covers(beta)`
- **THEN** both targets are omitted without requiring pairwise supersession or conflict shapes

#### Scenario: Coverage and conflict compose independently
- **WHEN** one referenced in-flight instance has `Covers(alpha)` and `Conflicts` findings
- **THEN** `alpha` is omitted and that instance is surfaced as conflicting, so the residual does not report full convergence

#### Scenario: Coverage and supersession compose independently
- **WHEN** one referenced in-flight instance has `Covers(alpha)` and `Superseded` findings
- **THEN** `alpha` is omitted and that instance is surfaced as superseded without the core inventing precedence

#### Scenario: Absence is the identity and never positive coverage
- **WHEN** no coverage effect is supplied for an in-flight instance or target
- **THEN** the core omits no target, surfaces no instance, and infers no reason for the absence

#### Scenario: Disjointness remains production-side knowledge
- **WHEN** a domain knows an in-flight instance is disjoint from the current plan
- **THEN** it retains that classification downstream and supplies no Suunta coverage effect, because the core has no mechanical contribution for it

#### Scenario: A future effect requires planner classification
- **WHEN** Suunta adds a new `CoverageEffect` variant
- **THEN** exhaustive internal handling fails to compile until the new effect's residual and surfacing contribution is explicitly defined

### Requirement: The Planner Is Functional Per Cycle
`plan_residual` SHALL be a pure function of a single cycle's inputs — a `Bearing` and a
`Sounding` (one cycle's certified readings: a `Fix` and coverage findings) — and SHALL
hold no state across `Sounding`s. The core consumes domain-certified findings *about*
in-flight `Correction`s (coverage findings, carried by the `Sounding`), never the raw
in-flight corrections themselves; it injects no time and performs no I/O. The `Bearing`
SHALL remain a separate argument — it is the persistent reference a `Sounding` is taken
against, not part of the reading.

#### Scenario: The planner holds no cross-cycle state
- **WHEN** `plan_residual` is invoked
- **THEN** it reads only its arguments, retains nothing between invocations, and reads no ambient clock and performs no I/O

#### Scenario: The planner takes a Bearing and a Sounding
- **WHEN** `plan_residual` is called for one cycle
- **THEN** it takes the `Bearing` and that cycle's `Sounding`, and the `Sounding` supplies the `Fix` and the coverage findings

### Requirement: The Core Makes No Semantic Judgment
The planning core SHALL make no semantic judgment. Semantic identity, target
satisfaction, relevance, and whether an obligation is settled SHALL be
domain-supplied. The domain SHALL choose and normalize those judgments; Suunta
SHALL define only the mechanical consumption effects required by its residual
mechanism and SHALL apply each supplied effect without comparing meanings,
inventing precedence, or disposing of findings. This is the semantic bill of
purity: its cost — an undetected domain semantic error fails silently — SHALL be
accepted rather than patched by pulling judgment into the core.

#### Scenario: The four judgments are the domain's
- **WHEN** the core needs semantic identity, target satisfaction, relevance, or a settlement decision
- **THEN** it consumes a domain-supplied `Sigil` or normalized verdict rather than deciding meaning or requiring a judgment-production trait

#### Scenario: Owning effects does not own judgment
- **WHEN** Suunta defines how a normalized satisfaction or coverage effect changes the residual
- **THEN** that mechanical algebra does not decide whether the effect is true; the domain remains responsible for supplying it

#### Scenario: The cost is accepted, not patched
- **WHEN** a domain supplies semantically inconsistent coverage effects
- **THEN** the core applies their independent mechanical contributions without diagnosing meaning, prioritizing one, or pulling reconciliation into the core

### Requirement: Corrections Carry A Stable Sigil
Each `Correction` SHALL carry a domain-supplied `Sigil` that is stable across
`Sounding`s and changes only on a genuine semantic change. The core SHALL carry the
`Sigil` opaquely and SHALL NOT interpret it.

#### Scenario: The same intent keeps its Sigil
- **WHEN** the same intent recurs across soundings
- **THEN** it carries the same `Sigil`, and a genuine semantic change carries a new one

#### Scenario: The core does not interpret the Sigil
- **WHEN** the core handles a `Correction`
- **THEN** it treats the `Sigil` as an opaque identity, compared by value, never by meaning

### Requirement: A Correction Carries An Opaque Domain Payload
A `Correction` SHALL be a public, sans-I/O value type in `suunta-contract` that carries
its domain change payload as a type parameter (`Correction<Body>`) with **no
core-imposed trait bound on `Body`**, alongside the `Sigil` and `Reversibility` it
already carries. Because the core declares no bound on `Body`, it SHALL expose no
operation that reads or compares the payload's meaning: payload opacity is a property
the type system guarantees, not one the core merely promises. The core owns the
carrier; the domain owns the meaning.

#### Scenario: The core carries the payload without a bound
- **WHEN** `suunta-contract`'s public API is compiled
- **THEN** `Correction` is generic over a `Body` with no core-declared trait bound, and the core exposes no method that inspects or compares a `Body` value

#### Scenario: A Correction still carries identity and reversibility
- **WHEN** a `Correction` is constructed
- **THEN** it carries a domain-supplied `Sigil` and a `Reversibility` marking, and the core treats the `Sigil` by value equality and the `Body` opaquely

### Requirement: A Sounding Carries No Body
A `Sounding` SHALL bundle one convergence cycle's certified readings — the `Fix` and the
coverage findings — and SHALL carry no domain payload: it SHALL be a non-generic type that
references targets and in-flight corrections only by `Sigil`/`InFlightIndex` and verdict.
Because a `Sounding` has no `Body`, the core cannot read a payload from the readings; the
domain payload SHALL flow only from `Bearing<Body>` into `Course<Body>`. This SHALL make
body-freeness of the readings a governed invariant — a future change adding a payload to a
reading violates this requirement — rather than the emergent, unstated property it is
today (the per-target and coverage findings already carry no `Body`).

#### Scenario: A Sounding exposes no payload
- **WHEN** `suunta-contract`'s public API is compiled
- **THEN** `Sounding` is non-generic and exposes no operation returning a domain `Body`; only `Bearing`, `Course`, and `Residual` carry `Body`

#### Scenario: A Sounding bundles the cycle's readings
- **WHEN** a `Sounding` is constructed for one cycle
- **THEN** it holds that cycle's `Fix` and its coverage findings, and is fed together with the `Bearing` to `plan_residual`

### Requirement: One-Way Corrections Are Marked
A `Correction` SHALL declare its reversibility, and a One-Way `Correction` SHALL be
marked as such. The core SHALL NOT own rollback or compensation; undoing a One-Way
`Correction` is a downstream concern.

#### Scenario: One-Way is explicit
- **WHEN** a `Correction` cannot be undone
- **THEN** it is marked One-Way, and the core never silently retries or rolls it back

### Requirement: Sans-I/O Purity
The planning core SHALL be sans-I/O: it SHALL expose no `async fn`, read no ambient
clock, and perform no I/O. A runtime drives it and injects time at the edge.

#### Scenario: The core commits to no runtime shape
- **WHEN** the `suunta-contract` public API is compiled
- **THEN** it exposes no `async fn`, calls no `std::io`/`fs`/`net`/`process`, and reads no ambient clock

### Requirement: Dependency Isolation
`suunta-contract` SHALL depend on no other workspace crate, so the planning core stays
isolated and reusable.

#### Scenario: The core is isolated
- **WHEN** `suunta-contract`'s manifest is read
- **THEN** it declares no dependency on another workspace crate

### Requirement: The Residual Mechanically Reports Full Convergence
The planner output SHALL expose a pure, policy-free read reporting whether a cycle is
fully converged, defined as: the residual `Course` is empty **and** no findings are
surfaced. This read SHALL inspect no `Body`, compare no meaning, and hold no state — it
reports only the structural shape of the residual. The core SHALL NOT expose a richer
settlement classification (such as a pending/blocked/converged verdict), because deciding
whether a surfaced finding is blocking or merely pending is a disposition, which is a
domain judgment, not a mechanical read.

#### Scenario: Empty course with no surfaced findings is converged
- **WHEN** the residual `Course` is empty and no findings are surfaced
- **THEN** the convergence read is true

#### Scenario: An undisposed surfaced finding is not convergence
- **WHEN** the residual `Course` is empty but a finding is surfaced (for example a superseded or conflicting in-flight correction)
- **THEN** the convergence read is false, so a consumer cannot declare success while a finding awaits disposition

#### Scenario: A non-empty course is not convergence
- **WHEN** the residual `Course` retains one or more corrections
- **THEN** the convergence read is false regardless of the surfaced findings

#### Scenario: The read makes no semantic judgment
- **WHEN** the convergence read is computed
- **THEN** it reads only whether the course and surfaced collections are empty, inspecting no `Body` and comparing no meaning
