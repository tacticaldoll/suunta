# quality-governance Specification

## Purpose
TBD - created by archiving change establish-suunta-shape. Update Purpose after archive.
## Requirements
### Requirement: Executable Constitution
Suunta SHALL enforce its architecture with an executable Tianheng constitution
(`suunta-governance`), so the boundaries prose claims are gated, not merely asserted.
The gate SHALL depend only on governance-family tooling, never on the workspace graph
it judges.

#### Scenario: The constitution runs clean on the workspace
- **WHEN** `cargo run -p suunta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** it reports no boundary violated for the current workspace

#### Scenario: The gate is independent of the graph it judges
- **WHEN** `suunta-governance`'s dependencies are read
- **THEN** they are limited to `tianheng` and `guibiao`, never a crate under judgment

### Requirement: Dependency Boundaries Are Enforced
The constitution SHALL restrict each crate's dependencies: `suunta-contract` to no
workspace or framework crate, and `suunta-governance` to `tianheng` and `guibiao`.

#### Scenario: An unapproved core dependency fails the gate
- **WHEN** `suunta-contract` gains a dependency outside its allowed set
- **THEN** the constitution reports a dependency-boundary violation

### Requirement: Sans-I/O Purity Is Enforced
The constitution SHALL bite the core's sans-I/O purity: `suunta-contract` SHALL call
no `std::io`/`fs`/`net`/`process`, read no ambient clock, and expose no `async fn`
(including submodules). This static tooth complements review and is partial by nature
(macro-expanded I/O is invisible to a source scan).

#### Scenario: An exposed async fn in the core fails the gate
- **WHEN** `suunta-contract` exposes an `async fn`
- **THEN** the async-exposure boundary reports a violation

### Requirement: Workspace Coverage
Every workspace crate SHALL be covered by a dependency boundary, so no crate is
silently ungoverned.

#### Scenario: Coverage is complete and non-vacuous
- **WHEN** coverage is computed from `cargo metadata`
- **THEN** the crate count is greater than zero and no crate is uncovered

### Requirement: Active Prose Is Present
The governed active-prose files SHALL be present and readable, and a governed doc that
vanishes SHALL fail the gate rather than pass vacuously. The governed set is
`AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`, `docs/development-flow.md`, and
`docs/domain-language.md`.

#### Scenario: A missing governed doc fails loudly
- **WHEN** the prose check runs against a root missing a governed file
- **THEN** it fails the gate, naming the unreadable file

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

