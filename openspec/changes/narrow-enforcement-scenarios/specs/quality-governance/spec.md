## RENAMED Requirements

- FROM: `### Requirement: Sans-I/O Purity Is Enforced`
- TO: `### Requirement: Sans-I/O Purity Has Static Teeth`

## MODIFIED Requirements

### Requirement: Executable Constitution
Suunta SHALL enforce its architecture with an executable Tianheng constitution
(`suunta-governance`), so the structural shadow of the boundaries prose claims is
gated, not merely asserted; what a static scan cannot observe stays review-governed.
The gate SHALL use Tianheng's supported composed adopter surface and SHALL depend
directly only on `tianheng`, never on an individual governance instrument or a
workspace crate under judgment.

#### Scenario: The constitution runs clean on the workspace
- **WHEN** `cargo run -p suunta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** it reports no boundary violated for the current workspace

#### Scenario: The gate depends only on the composed governance surface
- **WHEN** `suunta-governance`'s dependencies are read
- **THEN** its only direct dependency is `tianheng`, never `guibiao`, another individual instrument, or a crate under judgment

### Requirement: Dependency Boundaries Are Enforced
The constitution SHALL restrict each crate's normal dependencies: `suunta-contract`
to no workspace or framework crate, `suunta-governance` to `tianheng` alone, and the
`suunta` facade to `suunta-contract` alone. Dev and build dependency tables are not
observed by these boundaries.

#### Scenario: An unapproved core dependency fails the gate
- **WHEN** `suunta-contract` gains a normal dependency outside its allowed set
- **THEN** the constitution reports a dependency-boundary violation

#### Scenario: An unapproved governance dependency fails the gate
- **WHEN** `suunta-governance` gains a normal dependency other than `tianheng`
- **THEN** the constitution reports a dependency-boundary violation

#### Scenario: An unapproved facade dependency fails the gate
- **WHEN** the `suunta` facade gains a normal dependency other than `suunta-contract`
- **THEN** the constitution reports a dependency-boundary violation

### Requirement: Sans-I/O Purity Has Static Teeth
`suunta-contract` SHALL call no `std::io`/`fs`/`net`/`process`, read no ambient
clock, and expose no `async fn` (including submodules). The constitution SHALL bite
the part of that purity a source scan observes: an inline call into `std::io`,
`std::fs`, `std::net`, or `std::process`; an inline `std::time` call ending in
`now`; and a public `async fn` at the crate root or in a submodule. The clock and
async-exposure reactions SHALL be declared together through Tianheng's `SansIoPure`
profile with one accepted purity reason, while the four explicit I/O source
reactions SHALL remain separately declared because the profile does not observe
them. These static teeth complement review and remain partial by nature:
macro-expanded I/O (such as `println!`), a clock read through a method on a value
(such as `Instant::elapsed`), and a public function written to return
`impl Future` are invisible to a source scan and stay review-governed.

#### Scenario: An exposed async fn in the core fails the gate
- **WHEN** `suunta-contract` exposes a public `async fn` at its root or in a submodule
- **THEN** the async-exposure boundary produced by the `SansIoPure` profile reports a violation

#### Scenario: An ambient clock read in the core fails the gate
- **WHEN** `suunta-contract` makes an inline call to a `std::time` path ending in `now`
- **THEN** the clock boundary produced by the `SansIoPure` profile reports a violation

#### Scenario: Explicit I/O remains independently guarded
- **WHEN** `suunta-contract` makes an inline call into `std::io`, `std::fs`, `std::net`, or `std::process`
- **THEN** the corresponding explicit source boundary reports a violation independently of the composed profile
