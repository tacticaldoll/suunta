## MODIFIED Requirements

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
