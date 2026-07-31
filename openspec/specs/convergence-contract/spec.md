# convergence-contract Specification

## Purpose
Define Suunta's convergence-planning contract: the navigation vocabulary; the residual `Course` that `plan_residual` computes from a `Bearing` and domain-certified satisfaction and coverage findings (omitting only positively-certified targets, surfacing uncertainty); the semantic bill of purity in four faces; stable-`Sigil` identity; opaque `Correction` payloads; One-Way marking; sans-I/O purity; and dependency isolation.
## Requirements
### Requirement: Navigation Vocabulary
Suunta SHALL name the convergence-planning roles with a fixed navigation register:
`Sounding` (one observation cycle), `Fix` (the observed current state), `Bearing`
(the domain-supplied desired state), `Drift` (the divergence between them), `Course`
(the residual plan), `Correction` (a single planned change), and `Sigil` (a
domain-supplied stable semantic identity). These terms are architecture, not branding.

#### Scenario: The vocabulary is canonical
- **WHEN** documentation or the contract crate refers to a convergence-planning role
- **THEN** it uses the canonical navigation term rather than a generic synonym

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
The residual `Course` SHALL contain every `Bearing` target except those a domain finding
positively certifies as `Satisfied` (reality already meets it) or `Covered` (a relevant
in-flight `Correction` handles it). A target that is `Unsatisfied`, uncovered, `Unknown`,
or has no finding SHALL be retained. Absence and uncertainty SHALL NOT omit a target — only
positive certification omits. This realizes the computation deferred by "A Course Is A
Residual".

#### Scenario: A satisfied or covered target is omitted
- **WHEN** a `Bearing` target has a `Satisfied` satisfaction finding, or a `Covered` coverage finding naming a relevant in-flight `Correction`
- **THEN** the target is omitted from the residual `Course`

#### Scenario: Absence and uncertainty retain
- **WHEN** a `Bearing` target has no finding, or an `Unsatisfied` or `Unknown` finding
- **THEN** the target is retained in the residual `Course`

### Requirement: Fix Is Domain-Certified Satisfaction, Not Observation
`Fix` SHALL be expressed as domain-certified satisfaction findings — a `Satisfaction`
verdict per referenced `Bearing` target — and SHALL NOT be a bare set of satisfied
`Sigil`s nor a store of raw observations. Whether a target is satisfied is a domain
judgment the core consumes; the core SHALL NOT read observation content or compute
satisfaction itself.

#### Scenario: Satisfaction is consumed, not computed
- **WHEN** the core needs to know whether a `Bearing` target is satisfied
- **THEN** it consumes a domain-supplied `Satisfaction` verdict rather than comparing an observed state against the desired one

#### Scenario: Fix carries verdicts, not observations
- **WHEN** a `Fix` is supplied to the planner
- **THEN** it carries per-target satisfaction verdicts, and the core reads no observation body

### Requirement: Uncertainty And Disposition Are Surfaced, Not Resolved
The planner SHALL surface, on its output, each target retained under `Unknown` and each
in-flight `Correction` a coverage finding marks superseded or conflicting. The core SHALL
NOT cancel, compensate, or otherwise dispose of them, and SHALL name no execution-lifecycle
state; disposition is a downstream/consumer concern.

#### Scenario: Unknown retention is an observable finding
- **WHEN** a target is retained only because its satisfaction is `Unknown`
- **THEN** the planner surfaces it as an `Unknown`-retained finding, so the uncertainty is visible rather than silently over-planned

#### Scenario: Supersession and conflict are surfaced, not disposed
- **WHEN** a coverage finding marks an in-flight `Correction` superseded or conflicting
- **THEN** the planner surfaces it on the output and takes no cancelling or compensating action

### Requirement: The Planner Is Functional Per Cycle
`plan_residual` SHALL be a pure function of a single cycle's inputs — a `Bearing`,
satisfaction findings, and coverage findings — and SHALL hold no state across `Sounding`s.
The core consumes domain-certified findings *about* in-flight `Correction`s (coverage
findings), never the raw in-flight corrections themselves; it injects no time and performs
no I/O.

#### Scenario: The planner holds no cross-cycle state
- **WHEN** `plan_residual` is invoked
- **THEN** it reads only its arguments, retains nothing between invocations, and reads no ambient clock and performs no I/O

### Requirement: The Core Makes No Semantic Judgment
The planning core SHALL make no semantic judgment. Semantic identity, **target
satisfaction**, relevance, and whether an obligation is settled SHALL be domain-supplied —
as a `Sigil`, a **satisfaction verdict**, a coverage verdict, and a settlement predicate
respectively. The core's role SHALL be limited to computing the residual and recording; it
SHALL NOT compare meanings — in particular it cannot decide whether reality meets a desired
`Bearing` target, which is why satisfaction is domain-supplied and that per-target verdict
is the `Fix`. This is the semantic bill of purity — now **four faces** of one purity
choice: its cost — an undetected domain semantic error fails silently — SHALL be accepted
rather than patched by pulling judgment into the core.

#### Scenario: The four judgments are the domain's
- **WHEN** the core needs semantic identity, target satisfaction, relevance, or a settlement decision
- **THEN** it consumes a domain-supplied `Sigil`, satisfaction verdict, coverage verdict, or settlement predicate rather than deciding it

#### Scenario: The cost is accepted, not patched
- **WHEN** a domain supplies an incorrect semantic judgment
- **THEN** the core does not prevent the resulting failure by making the judgment itself; it stays pure, and any defense is a downstream concern

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

