## ADDED Requirements

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
residual *contract*; the computation that forms a `Course` is realized in a later
spec-driven change (see `BACKLOG.md`), so at this shape the constraint binds the
design rather than describing a shipped algorithm.

#### Scenario: Relevance is a domain-supplied verdict
- **WHEN** a `Course` is formed from a `Bearing`, a `Fix`, and in-flight `Correction`s
- **THEN** which in-flight `Correction`s count as relevant is taken from a domain-supplied coverage verdict, never from the core comparing meanings

### Requirement: The Core Makes No Semantic Judgment
The planning core SHALL make no semantic judgment. Semantic identity, relevance, and
whether an obligation is settled SHALL be domain-supplied — as a `Sigil`, a coverage
verdict, and a settlement predicate respectively. The core's role SHALL be limited to
computing the residual and recording; it SHALL NOT compare meanings. This is the
semantic bill of purity: its
cost — an undetected domain semantic error fails silently — SHALL be accepted rather
than patched by pulling judgment into the core.

#### Scenario: The three judgments are the domain's
- **WHEN** the core needs semantic identity, relevance, or a settlement decision
- **THEN** it consumes a domain-supplied `Sigil`, coverage verdict, or settlement predicate rather than deciding it

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
