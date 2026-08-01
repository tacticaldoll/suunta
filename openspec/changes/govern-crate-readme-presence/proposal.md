## Why

`suunta-governance`'s active-prose presence check only covers six root-level
documents (`AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`,
`docs/development-flow.md`, `docs/domain-language.md`). The three crate-level
READMEs (`crates/suunta/README.md`, `crates/suunta-contract/README.md`,
`crates/suunta-governance/README.md`) are shipped, published-facing prose —
`suunta` and `suunta-contract` reference their README via `readme =
"README.md"` in `Cargo.toml` and publish to crates.io — but nothing fails the
gate if one of them goes missing or becomes unreadable. This mirrors the exact
failure mode the existing check already guards against for the root docs; the
crate READMEs simply fell outside its scope when it was written.

This is a narrow, mechanical widening of an existing, already-accepted check
— not a new judgment. It does not attempt to detect *stale content* (that
stays review-governed prose, per `BACKLOG.md`'s discussion of this same
finding); it only closes the presence/readability gap for three files that
are already governed prose in spirit.

## What Changes

- Extend `suunta-governance`'s `ACTIVE_PROSE_FILES` list to include the three
  crate-level READMEs, so a missing or unreadable crate README fails the
  governance gate the same way a missing root doc already does.
- No change to the stale-phrase scan (`STALE_PHRASES` stays empty; no phrases
  are added by this change).
- No change to any runtime behavior, public API, or dependency boundary.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `quality-governance`: the "Active Prose Is Present" requirement's governed
  file set grows from six root documents to include the three crate-level
  READMEs.

## Impact

- Code: `crates/suunta-governance/src/main.rs` (`ACTIVE_PROSE_FILES` constant
  only).
- Specs: `openspec/specs/quality-governance/spec.md` ("Active Prose Is
  Present" requirement's governed-set list).
- No dependency, API, or behavioral impact outside the governance gate itself.
