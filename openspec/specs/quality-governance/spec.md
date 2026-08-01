# quality-governance Specification

## Purpose
Define Suunta's executable governance: the Tianheng constitution's dependency boundaries and sans-I/O teeth, the facade's re-export purity, workspace coverage, active-prose presence, the honest limit that "no semantic judgment" is not statically enforceable, and the single-sourced Definition of Done.
## Requirements
### Requirement: Executable Constitution
Suunta SHALL enforce its architecture with an executable Tianheng constitution
(`suunta-governance`), so the boundaries prose claims are gated, not merely
asserted. The gate SHALL use Tianheng's supported composed adopter surface and
SHALL depend directly only on `tianheng`, never on an individual governance
instrument or a workspace crate under judgment.

#### Scenario: The constitution runs clean on the workspace
- **WHEN** `cargo run -p suunta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** it reports no boundary violated for the current workspace

#### Scenario: The gate depends only on the composed governance surface
- **WHEN** `suunta-governance`'s dependencies are read
- **THEN** its only direct dependency is `tianheng`, never `guibiao`, another individual instrument, or a crate under judgment

### Requirement: Dependency Boundaries Are Enforced
The constitution SHALL restrict each crate's dependencies: `suunta-contract` to no
workspace or framework crate, `suunta-governance` to `tianheng` alone, and the
`suunta` facade to `suunta-contract` alone.

#### Scenario: An unapproved core dependency fails the gate
- **WHEN** `suunta-contract` gains a dependency outside its allowed set
- **THEN** the constitution reports a dependency-boundary violation

#### Scenario: An unapproved governance dependency fails the gate
- **WHEN** `suunta-governance` gains a direct dependency other than `tianheng`
- **THEN** the constitution reports a dependency-boundary violation

#### Scenario: An unapproved facade dependency fails the gate
- **WHEN** the `suunta` facade gains a dependency other than `suunta-contract`
- **THEN** the constitution reports a dependency-boundary violation

### Requirement: Sans-I/O Purity Is Enforced
The constitution SHALL bite the core's sans-I/O purity:
`suunta-contract` SHALL call no `std::io`/`fs`/`net`/`process`, read no ambient
clock, and expose no `async fn` (including submodules). The ambient-clock and
async-exposure reactions SHALL be declared together through Tianheng's
`SansIoPure` profile with one accepted purity reason, while the four explicit I/O
source reactions SHALL remain separately declared because the profile does not
observe them. These static teeth complement review and remain partial by nature
(for example, macro-expanded I/O is invisible to a source scan).

#### Scenario: An exposed async fn in the core fails the gate
- **WHEN** `suunta-contract` exposes an `async fn`
- **THEN** the async-exposure boundary produced by the `SansIoPure` profile reports a violation

#### Scenario: An ambient clock read in the core fails the gate
- **WHEN** `suunta-contract` calls a `std::time` path ending in `now`
- **THEN** the clock boundary produced by the `SansIoPure` profile reports a violation

#### Scenario: Explicit I/O remains independently guarded
- **WHEN** `suunta-contract` calls `std::io`, `std::fs`, `std::net`, or `std::process`
- **THEN** the corresponding explicit source boundary reports a violation independently of the composed profile

### Requirement: Accepted Law Has A Fresh Human Projection
Suunta SHALL check in a human- and agent-readable Markdown projection generated
from the accepted Rust constitution. The Rust declaration SHALL remain the
authority, the projection SHALL identify itself as generated and
non-authoritative, and a test SHALL fail when the checked-in projection is
missing, unreadable, or stale.

#### Scenario: A stale projection fails the gate
- **WHEN** the accepted constitution changes without regenerating the checked-in Markdown projection
- **THEN** the projection-freshness test fails and names the regeneration command

#### Scenario: The projection adds no independent law
- **WHEN** a contributor reads the checked-in projection
- **THEN** its preamble identifies the Rust constitution as authority and directs the contributor not to edit the projection by hand

### Requirement: The Facade Is A Pure Re-Export Surface
The constitution SHALL enforce that the `suunta` facade library holds only
re-exports, crate attributes, and documentation, so the curated entrypoint cannot
accrete logic. The check SHALL scan the facade source tree and SHALL NOT pass
vacuously: a facade source tree that is missing or unreadable SHALL fail the gate
rather than scan zero files and pass. Because `suunta-governance` may depend only on
governance-family tooling, the scan SHALL be a brace-depth line heuristic rather
than a full parser; the Definition of Done's `cargo fmt --all --check` backstops the
one gap where a logic item is co-located on a re-export line.

#### Scenario: A logic item in the facade fails the gate
- **WHEN** the facade library defines an item other than a re-export at brace-depth zero
- **THEN** the re-exports-only scan reports a violation naming the file and line

#### Scenario: A missing facade source tree fails loudly
- **WHEN** the re-exports-only scan finds no facade source files
- **THEN** it fails the gate rather than passing on an empty scan

#### Scenario: A clean facade passes
- **WHEN** the facade library contains only re-exports, attributes, and comments
- **THEN** the re-exports-only scan reports no violation

### Requirement: Workspace Coverage
Every workspace crate SHALL be covered by a dependency boundary, so no crate is
silently ungoverned.

#### Scenario: Coverage is complete and non-vacuous
- **WHEN** coverage is computed from `cargo metadata`
- **THEN** the crate count is greater than zero and no crate is uncovered

### Requirement: Active Prose Is Present
The governed active-prose files SHALL be present and readable, and a governed doc that
vanishes SHALL fail the gate rather than pass vacuously. The governed set is
`AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`, `docs/development-flow.md`,
`docs/domain-language.md`, `crates/suunta/README.md`,
`crates/suunta-contract/README.md`, and `crates/suunta-governance/README.md`.

#### Scenario: A missing governed doc fails loudly
- **WHEN** the prose check runs against a root missing a governed file
- **THEN** it fails the gate, naming the unreadable file

#### Scenario: A missing crate readme fails loudly
- **WHEN** a governed crate-level README (`crates/suunta/README.md`,
  `crates/suunta-contract/README.md`, or `crates/suunta-governance/README.md`) is
  missing or unreadable
- **THEN** the prose check fails the gate, naming the unreadable file, the same way
  a missing root-level governed doc does

### Requirement: The No-Semantic-Judgment Invariant Is Not Statically Enforced
The constitution SHALL NOT claim to statically enforce "the core makes no semantic
judgment": semantic comparison has no syntactic marker, so it is not expressible as a
static boundary. It SHALL remain review- and structure-governed, and this honest limit
SHALL be recorded rather than papered over.

#### Scenario: The limit is acknowledged
- **WHEN** the governance surface describes what it enforces
- **THEN** it states that the no-semantic-judgment axiom is review-governed, not a Tianheng tooth

### Requirement: Definition Of Done Is Single-Sourced
`AGENTS.md` SHALL state the complete Definition of Done, and other active prose
(`README.md`, `docs/development-flow.md`) SHALL point to it rather than restate a
divergent subset.

#### Scenario: The Definition of Done is stated once
- **WHEN** the Definition of Done is documented
- **THEN** `AGENTS.md` holds the complete gate list and other docs point to it
