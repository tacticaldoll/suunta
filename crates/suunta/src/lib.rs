//! Suunta: a thin, sans-I/O convergence-planning core you compose.
//!
//! This crate is the curated public entrypoint. It re-exports the compose-level
//! API of the Suunta workspace so a consumer can depend on one crate:
//!
//! - the navigation vocabulary — [`Bearing`], [`Correction`], [`Course`],
//!   [`Sigil`], [`Reversibility`];
//! - the domain's verdicts — [`Satisfaction`], [`SatisfactionFinding`], [`Fix`],
//!   [`CoverageEffect`], [`CoverageFinding`], [`InFlightIndex`];
//! - one cycle's readings — [`Sounding`];
//! - the residual output — [`Residual`], [`SurfacedFinding`] — and the planner
//!   [`plan_residual`].
//!
//! It carries no logic of its own: every item here is a re-export. Suunta's whole
//! public surface is compose-level, so the facade withholds nothing — there is no
//! advanced kernel to reach through [`suunta_contract`] directly.
//!
//! # The contract
//!
//! Suunta owns a *mechanism* and no *meaning*. [`plan_residual`] filters a
//! [`Bearing`] to the [`Correction`]s that remain once the domain's verdicts have
//! certified targets satisfied or covered; it decides no semantic identity, no
//! relevance, and no settlement. Those are the domain's — see the crate-level docs
//! of [`suunta_contract`] for the full axioms and the semantic bill of purity.
//!
//! # Composing a convergence loop
//!
//! A convergence loop wired entirely through this entrypoint: a target starts
//! unsatisfied and is retained in the residual `Course`, then the domain certifies
//! it `Satisfied` and [`Residual::is_converged`] halts the loop. The loop bound and
//! the decision to stop are the consumer's; the core disposes of nothing (run
//! `cargo test` to see it execute):
//!
//! ```
//! use suunta::{
//!     Bearing, Correction, Fix, Reversibility, Satisfaction, SatisfactionFinding, Sigil,
//!     Sounding,
//! };
//!
//! // The desired end state: one reversible correction toward a goal. Rebuilt each
//! // cycle because `plan_residual` consumes the `Bearing`.
//! let bearing = || {
//!     Bearing::new(vec![Correction::new(
//!         Sigil::new("goal:trim-sail"),
//!         Reversibility::Reversible,
//!         "trim the sail",
//!     )])
//! };
//!
//! // The domain's model of reality: unsatisfied until cycle 1, satisfied after.
//! let observe = |cycle: usize| {
//!     vec![SatisfactionFinding {
//!         target: Sigil::new("goal:trim-sail"),
//!         satisfaction: if cycle >= 1 {
//!             Satisfaction::Satisfied
//!         } else {
//!             Satisfaction::Unsatisfied
//!         },
//!     }]
//! };
//!
//! let mut converged_at = None;
//! for cycle in 0..4 {
//!     // One cycle's readings: this Fix, no coverage. A Sounding carries no payload.
//!     let sounding = Sounding::new(Fix::new(observe(cycle)), vec![]);
//!     let residual = suunta::plan_residual(bearing(), &sounding);
//!     if cycle == 0 {
//!         // Not yet satisfied: the target is retained in the residual Course.
//!         assert_eq!(residual.course.corrections().len(), 1);
//!         assert!(!residual.is_converged());
//!     }
//!     if residual.is_converged() {
//!         converged_at = Some(cycle);
//!         break;
//!     }
//! }
//! // Reality caught up at cycle 1: nothing remains to plan, so the loop halts.
//! assert_eq!(converged_at, Some(1));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use suunta_contract::{
    Bearing, Correction, Course, CoverageEffect, CoverageFinding, Fix, InFlightIndex, Residual,
    Reversibility, Satisfaction, SatisfactionFinding, Sigil, Sounding, SurfacedFinding,
    plan_residual,
};
