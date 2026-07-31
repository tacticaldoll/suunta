//! The isolated core contract for Suunta: sans-I/O convergence planning.
//!
//! Suunta computes the residual `Course` — the corrections needed to converge a
//! `Fix` (observed state) toward a `Bearing` (desired state) — and nothing more.
//!
//! # Axioms
//!
//! 1. **No semantic judgment in the core.** Semantic identity ([`Sigil`]), relevance
//!    (a domain coverage verdict), and settlement predicates are domain-supplied. The
//!    core computes the residual and records; it never compares meanings. This is the
//!    *semantic bill of purity*: its cost (a silent failure on a domain semantic
//!    error) is accepted deliberately rather than patched by pulling judgment into
//!    the core.
//! 2. **Sans-I/O purity.** The core exposes no `async fn`, reads no ambient clock,
//!    and performs no I/O. A runtime drives it and injects time at the edge.
//! 3. **No dependency on other workspace crates.**
//!
//! # Status
//!
//! The emit-side vocabulary — the types the core produces — is landed: [`Correction`]
//! and [`Course`], alongside [`Sigil`] and [`Reversibility`]. The residual *computation*
//! and the observe-side vocabulary (`Fix`, `Bearing`, `Sounding`, `Drift`) are defined
//! in `openspec/specs/` and built in later spec-driven changes (see `BACKLOG.md`).
//! Durability, gating, execution, and compensation of a correction are downstream
//! consumer concerns, not this core.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// A domain-supplied, cross-cycle-stable semantic identity for a correction's target.
///
/// The same intent across soundings carries the same `Sigil`; a genuine semantic
/// change means a new `Sigil`. The core carries a `Sigil` but never interprets it —
/// deciding semantic identity is a domain judgment, not the core's (see the crate
/// axioms). Construct via [`Sigil::new`] and read via [`Sigil::as_str`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sigil(String);

impl Sigil {
    /// Mint a sigil from a domain-supplied stable identity.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The opaque identity string. The core does not interpret it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Whether a correction can be undone.
///
/// A `OneWay` correction must be marked as such; the core does not own rollback or
/// compensation, which are downstream consumer concerns. `#[non_exhaustive]` because
/// this stance may gain variants as the design settles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Reversibility {
    /// The correction can be undone.
    Reversible,
    /// The correction cannot be undone; the core never silently retries or rolls it
    /// back.
    OneWay,
}

/// A single planned change on a [`Course`].
///
/// A `Correction` carries a domain-supplied [`Sigil`] (its cross-cycle-stable semantic
/// identity), a [`Reversibility`] marking, and a domain `Body` — the payload describing
/// the change itself. The core places **no trait bound on `Body`** and exposes no
/// operation that reads it: deciding what a correction *means* is a domain judgment, not
/// the core's, so payload opacity is guaranteed by the type system rather than by
/// convention (see the crate axioms). The core owns the carrier; the domain owns the
/// meaning.
///
/// Construct via [`Correction::new`]; read via [`Correction::sigil`],
/// [`Correction::reversibility`], and [`Correction::body`].
#[derive(Debug, Clone)]
pub struct Correction<Body> {
    sigil: Sigil,
    reversibility: Reversibility,
    body: Body,
}

impl<Body> Correction<Body> {
    /// Plan a correction from its domain identity, reversibility, and payload.
    #[must_use]
    pub fn new(sigil: Sigil, reversibility: Reversibility, body: Body) -> Self {
        Self {
            sigil,
            reversibility,
            body,
        }
    }

    /// The correction's domain-supplied semantic identity, compared by value.
    #[must_use]
    pub fn sigil(&self) -> &Sigil {
        &self.sigil
    }

    /// Whether the correction can be undone.
    #[must_use]
    pub fn reversibility(&self) -> Reversibility {
        self.reversibility
    }

    /// The opaque domain payload. The core carries it but never interprets it.
    #[must_use]
    pub fn body(&self) -> &Body {
        &self.body
    }
}

/// The residual plan: the ordered [`Correction`]s that steer a `Fix` toward a `Bearing`.
///
/// A `Course` is the residual *as a value* — the collection the core emits. It preserves
/// the order it is built from and never deduplicates or reorders its `Correction`s, since
/// collapsing two of them would require judging that they *mean* the same, which is not
/// the core's to decide. The computation that *forms* a `Course` as a residual is defined
/// in `openspec/specs/` and realized in a later change; this type is only the carrier.
#[derive(Debug, Clone)]
pub struct Course<Body> {
    corrections: Vec<Correction<Body>>,
}

impl<Body> Course<Body> {
    /// Assemble a course from corrections, preserving their order.
    #[must_use]
    pub fn new(corrections: Vec<Correction<Body>>) -> Self {
        Self { corrections }
    }

    /// The corrections on this course, in the order supplied. The core neither
    /// deduplicates nor reorders them.
    #[must_use]
    pub fn corrections(&self) -> &[Correction<Body>] {
        &self.corrections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sigil_carries_domain_identity_opaquely() {
        let sigil = Sigil::new("goal:rebalance-shard-7");
        assert_eq!(sigil.as_str(), "goal:rebalance-shard-7");
        // Stable identity is value equality; the core compares bytes, not meaning.
        assert_eq!(sigil, Sigil::new("goal:rebalance-shard-7"));
        assert_ne!(sigil, Sigil::new("goal:rebalance-shard-8"));
    }

    #[test]
    fn reversibility_marks_one_way() {
        assert_ne!(Reversibility::Reversible, Reversibility::OneWay);
    }

    // A stand-in for a domain-defined correction payload. The core never inspects it.
    #[derive(Debug, Clone, PartialEq)]
    struct DomainMove {
        target: u8,
    }

    #[test]
    fn correction_carries_identity_reversibility_and_opaque_body() {
        let c = Correction::new(
            Sigil::new("goal:rebalance-shard-7"),
            Reversibility::OneWay,
            DomainMove { target: 7 },
        );
        assert_eq!(c.sigil(), &Sigil::new("goal:rebalance-shard-7"));
        assert_eq!(c.reversibility(), Reversibility::OneWay);
        // The core hands the body back untouched; it makes no judgment about it.
        assert_eq!(c.body(), &DomainMove { target: 7 });
    }

    #[test]
    fn course_preserves_the_order_it_is_built_from() {
        let course = Course::new(vec![
            Correction::new(Sigil::new("a"), Reversibility::Reversible, 1u8),
            Correction::new(Sigil::new("b"), Reversibility::Reversible, 2u8),
            Correction::new(Sigil::new("c"), Reversibility::Reversible, 3u8),
        ]);
        let order: Vec<&str> = course
            .corrections()
            .iter()
            .map(|c| c.sigil().as_str())
            .collect();
        assert_eq!(order, ["a", "b", "c"]);
    }

    #[test]
    fn course_does_not_deduplicate_equal_sigils() {
        // Two corrections carry the same Sigil; the core retains both, because deciding
        // they are "the same" would be a meaning comparison it never makes.
        let course = Course::new(vec![
            Correction::new(Sigil::new("same"), Reversibility::Reversible, 1u8),
            Correction::new(Sigil::new("same"), Reversibility::OneWay, 2u8),
        ]);
        assert_eq!(course.corrections().len(), 2);
    }
}
