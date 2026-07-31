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
//! This crate is the initial shape: the vocabulary anchors below plus the axioms.
//! The residual-planning types and logic are defined in `openspec/specs/` and built
//! in later spec-driven changes (see `BACKLOG.md`). Durability, gating, execution,
//! and compensation of a correction are downstream consumer concerns, not this core.

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
}
