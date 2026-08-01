## ADDED Requirements

### Requirement: Value Types Support Structural Equality

Suunta SHALL derive `PartialEq`/`Eq` for the body-free reading types (`Fix`, `Sounding`, `SatisfactionFinding`, `CoverageFinding`) and SHALL derive `PartialEq`/`Eq` conditionally on `Body` for the `Body`-generic value types (`Correction<Body>`, `Course<Body>`, `Bearing<Body>`, `Residual<Body>`).

Equality on the body-free types compares their fields structurally by value. Equality on the `Body`-generic types is conditional — `PartialEq` requires `Body: PartialEq` and `Eq` requires `Body: Eq` — so a consumer whose payload supports equality can compare these values directly. The core SHALL NOT impose any unconditional trait bound on `Body` as a result, and SHALL NOT author a comparison of the payload's meaning: the derived equality is a structural, compiler-generated field comparison, the same class as the `Sigil` value comparison the core already performs.

#### Scenario: Body-free reading types compare structurally
- **WHEN** two `Sounding`, `Fix`, `SatisfactionFinding`, or `CoverageFinding`
  values are compared
- **THEN** they are equal exactly when their fields are equal by value

#### Scenario: Body-carrying types compare only when Body does
- **WHEN** a consumer's `Body` implements `PartialEq`
- **THEN** `Correction<Body>`, `Course<Body>`, `Bearing<Body>`, and
  `Residual<Body>` values with that `Body` can be compared by value, structurally

#### Scenario: No unconditional bound is added to the core
- **WHEN** `suunta-contract`'s public API is compiled
- **THEN** no core-owned function (`Correction::new`, `.sigil()`, `.body()`,
  `plan_residual`, and so on) requires `Body: PartialEq` or `Body: Eq`; the
  conditional derive only activates equality for consumers whose own `Body`
  already supports it
