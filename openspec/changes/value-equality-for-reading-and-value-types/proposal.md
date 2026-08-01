## Why

`suunta-contract`'s value types are inconsistent about deriving `PartialEq`/`Eq`.
`Sigil`, `Reversibility`, `Satisfaction`, `InFlightIndex`, `CoverageEffect`, and
`SurfacedFinding` already derive equality. But four **body-free** aggregate types
that bundle only already-comparable fields — `SatisfactionFinding`, `Fix`,
`CoverageFinding`, `Sounding` — do not, even though nothing about their content
resists it: they carry no `Body` by design (that is their whole point). Four
**body-carrying** value types — `Correction<Body>`, `Course<Body>`, `Bearing<Body>`,
`Residual<Body>` — also lack equality, though a conditional derive
(`impl<Body: PartialEq> PartialEq for ...`) would let consumers compare them
whenever their own payload supports it, without the core imposing any
unconditional bound.

Today a consumer that wants to assert `sounding_a == sounding_b` (or compare two
`Course<Body>` values) in their own tests cannot — they must destructure and
compare fields by hand. This is a real, additive ergonomics gap, not a design
decision: no discussion in `BACKLOG.md`, either spec, or `AGENTS.md` withholds
equality from these types deliberately.

## What Changes

- Derive `PartialEq, Eq` on the four body-free types: `SatisfactionFinding`,
  `Fix`, `CoverageFinding`, `Sounding`.
- Derive `PartialEq` (and `Eq` where the type's own fields support it) on the
  four `Body`-generic types: `Correction<Body>`, `Course<Body>`, `Bearing<Body>`,
  `Residual<Body>`. The derived bound is conditional on `Body` — the core's own
  functions gain no new bound and no operation that reads or compares `Body`.
- Purely additive: no existing method, field, or behavior changes. Non-breaking
  under semver.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `convergence-contract`: adds a requirement that these value types support
  structural equality (unconditionally for the body-free types, conditionally on
  `Body` for the body-carrying ones), without adding any bound to the core's own
  API surface.

## Impact

- Code: `crates/suunta-contract/src/lib.rs` (derive attributes only, plus new
  unit tests demonstrating the new equality).
- Specs: `openspec/specs/convergence-contract/spec.md` (new requirement).
- No dependency, runtime behavior, or public method signature changes. No
  `Body` trait bound added to any core-owned function.
