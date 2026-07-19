## MODIFIED Requirements

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
