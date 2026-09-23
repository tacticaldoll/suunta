## Why

Several sentences about Suunta's governance claim that the gate enforces more than it
observes. The accepted boundary reasons were already narrowed to what each reaction sees:
the clock tooth sees an inline `std::time` call ending in `now`, the async tooth sees a
public `async fn`, the I/O teeth see inline `std::io`/`fs`/`net`/`process` calls, and the
dependency boundaries read normal dependencies. The quality-governance spec, the governance
crate's module doc and README, and `PROJECT.md` still say the gate enforces "no ambient
clock", "no exposed `async fn`", the whole sans-I/O purity, every dependency, and every
boundary prose claims. A reader who trusts those sentences will take a green gate as proof
of properties no source scan can see.

## What Changes

- quality-governance: the executable-constitution requirement says the gate covers the
  structural shadow of prose claims, with the rest review-governed.
- quality-governance: the dependency-boundary requirement and scenarios speak of normal
  dependencies.
- quality-governance: "Sans-I/O Purity Is Enforced" is renamed "Sans-I/O Purity Has Static
  Teeth". It keeps the purity intent and states what the teeth observe and what stays
  review-governed. Its scenarios name the observed shapes.
- The governance crate module doc and README, and the governance bullet in `PROJECT.md`, are
  narrowed the same way.

Intent is unchanged: the core still exposes no `async fn`, reads no ambient clock, and
performs no I/O. No boundary, reason, test, or product code changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-governance`: enforcement claims narrowed to what the gate observes.

## Impact

Prose only: `openspec/specs/quality-governance/spec.md`,
`crates/suunta-governance/src/main.rs` (module doc), `crates/suunta-governance/README.md`,
and `PROJECT.md`. The constitution, its projection, and the governance tests are unchanged.
