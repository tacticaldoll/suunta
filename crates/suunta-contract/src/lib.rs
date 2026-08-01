//! The isolated core contract for Suunta: sans-I/O convergence planning.
//!
//! Suunta computes the residual `Course` — the corrections that remain of a desired
//! `Bearing` once the domain's certified satisfaction (the `Fix`) and normalized coverage
//! effects are applied — and nothing more. It consumes domain-supplied effects, not raw
//! reality.
//!
//! # Axioms
//!
//! 1. **No semantic judgment in the core.** Semantic identity ([`Sigil`]), target
//!    satisfaction, relevance (a domain coverage verdict), and settlement predicates
//!    are domain-supplied. The core computes the residual and records; it never
//!    compares meanings — in particular it never decides whether reality meets a
//!    desired `Bearing` (that verdict is the domain's `Fix`). This is the *semantic bill
//!    of purity* (four faces of
//!    one purity choice): its cost (a silent failure on a domain semantic error) is
//!    accepted deliberately rather than patched by pulling judgment into the core.
//! 2. **Sans-I/O purity.** The core exposes no `async fn`, reads no ambient clock,
//!    and performs no I/O. A runtime drives it and injects time at the edge.
//! 3. **No dependency on other workspace crates.**
//!
//! # Status
//!
//! The residual planner is landed: [`Correction`], [`Course`], [`Sigil`],
//! [`Reversibility`], and [`plan_residual`], which computes the residual [`Course`]
//! from a `Bearing` and a per-cycle [`Sounding`] (the [`Fix`] and coverage findings).
//! Settlement, coverage-effect production, an async edge, durability, gating, execution,
//! and compensation are all settled downstream consumer concerns, not latent core work
//! (see `BACKLOG.md`).

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Correction<Body> {
    sigil: Sigil,
    reversibility: Reversibility,
    body: Body,
}

impl<Body> Correction<Body> {
    /// Plan a correction from its domain identity, reversibility, and payload.
    #[must_use]
    pub const fn new(sigil: Sigil, reversibility: Reversibility, body: Body) -> Self {
        Self {
            sigil,
            reversibility,
            body,
        }
    }

    /// The correction's domain-supplied semantic identity, compared by value.
    #[must_use]
    pub const fn sigil(&self) -> &Sigil {
        &self.sigil
    }

    /// Whether the correction can be undone.
    #[must_use]
    pub const fn reversibility(&self) -> Reversibility {
        self.reversibility
    }

    /// The opaque domain payload. The core carries it but never interprets it.
    #[must_use]
    pub const fn body(&self) -> &Body {
        &self.body
    }
}

/// The residual plan: the ordered [`Correction`]s that remain to steer toward a `Bearing`.
///
/// A `Course` is the residual *as a value* — the collection the core emits. It preserves
/// the order it is built from and never deduplicates or reorders its `Correction`s, since
/// collapsing two of them would require judging that they *mean* the same, which is not
/// the core's to decide. The computation that *forms* a `Course` as a residual is
/// performed by [`plan_residual`]; this type is only the carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course<Body> {
    corrections: Vec<Correction<Body>>,
}

impl<Body> Course<Body> {
    /// Assemble a course from corrections, preserving their order.
    #[must_use]
    pub const fn new(corrections: Vec<Correction<Body>>) -> Self {
        Self { corrections }
    }

    /// The corrections on this course, in the order supplied. The core neither
    /// deduplicates nor reorders them.
    #[must_use]
    pub fn corrections(&self) -> &[Correction<Body>] {
        &self.corrections
    }
}

/// The desired target state: the `Correction`s the domain wants to hold, each identified
/// by its [`Sigil`].
///
/// A `Bearing` is *what should be*. The residual is what remains of it once satisfied and
/// covered targets are removed. A `Bearing` target is a desired `Correction`, carried
/// opaquely; a `Bearing` is a desired target, never an observation of reality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bearing<Body> {
    targets: Vec<Correction<Body>>,
}

impl<Body> Bearing<Body> {
    /// A bearing over the domain's desired `Correction`s, in the order supplied.
    #[must_use]
    pub const fn new(targets: Vec<Correction<Body>>) -> Self {
        Self { targets }
    }

    /// The desired targets, in the order supplied.
    #[must_use]
    pub fn targets(&self) -> &[Correction<Body>] {
        &self.targets
    }

    /// Consume the bearing, yielding its desired targets.
    #[must_use]
    pub fn into_targets(self) -> Vec<Correction<Body>> {
        self.targets
    }
}

/// A normalized planning effect for the domain's judgment about whether a `Bearing`
/// target is already met by reality.
///
/// The core cannot decide this — comparing an observed state against a desired one is a
/// semantic judgment (the *fourth face* of the semantic bill of purity) — so the domain
/// supplies the judgment and normalizes it to one of the finite effects the residual
/// mechanism consumes. The domain's production taxonomy remains unrestricted downstream;
/// this enum belongs to Suunta and defines only mechanical residual effects.
/// `#[non_exhaustive]` permits Suunta to evolve that consumption vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Satisfaction {
    /// The target is already met; the residual may omit it.
    Satisfied,
    /// The target is not met; the residual retains it.
    Unsatisfied,
    /// Whether the target is met is not known; the residual retains it, conservatively.
    Unknown,
}

/// A satisfaction finding: the domain's normalized [`Satisfaction`] effect for one
/// `Bearing` target.
///
/// This is the `Fix` reaching the core — a domain-certified effect per target, never a
/// raw observation. Satisfaction is judged and normalized upstream; the core only applies
/// its mechanical contribution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfactionFinding {
    /// The `Bearing` target this verdict is about, by its `Sigil`.
    pub target: Sigil,
    /// Whether the target is met.
    pub satisfaction: Satisfaction,
}

/// The `Fix`: one sounding's certified satisfaction of the `Bearing`'s targets — the
/// aggregate of per-target normalized [`SatisfactionFinding`]s.
///
/// A reading taken *against intent*, which only the domain can take (comparing reality
/// to a desired target is a meaning comparison the core cannot make); the core consumes
/// it and never computes it. A `Fix` is **body-free** — it references targets only by
/// [`Sigil`] and carries no domain payload. Construct via [`Fix::new`]; read via
/// [`Fix::findings`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    findings: Vec<SatisfactionFinding>,
}

impl Fix {
    /// Assemble a fix from one cycle's per-target satisfaction findings.
    #[must_use]
    pub const fn new(findings: Vec<SatisfactionFinding>) -> Self {
        Self { findings }
    }

    /// The per-target satisfaction findings, in the order supplied.
    #[must_use]
    pub fn findings(&self) -> &[SatisfactionFinding] {
        &self.findings
    }
}

/// A reference to a specific in-flight `Correction` instance, by position in the caller's
/// in-flight slice.
///
/// Instances are referenced by position, not by `Sigil`, because a `Course` does not
/// deduplicate: two in-flight `Correction`s may share a `Sigil`. The core treats the index
/// as an opaque coordinate assigned by the domain and read by the consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InFlightIndex(pub usize);

/// A normalized planning effect for the domain's judgment about an in-flight
/// `Correction`'s relation to the current plan.
///
/// The domain owns how it produces and normalizes its relation judgment. Suunta owns only
/// these finite effects because each changes the residual or its surfaced findings.
/// `Covers` is target-scoped; `Superseded` and `Conflicts` describe the referenced
/// in-flight instance relative to the current plan as a whole. `#[non_exhaustive]`
/// permits Suunta to evolve its consumption vocabulary; it does not make the enum
/// downstream-extensible.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CoverageEffect {
    /// The in-flight correction covers the named `Bearing` target; the residual may omit
    /// that target.
    Covers(Sigil),
    /// The current plan makes the referenced in-flight correction obsolete; surfaced,
    /// never disposed.
    Superseded,
    /// The in-flight correction cannot safely coexist with the plan; surfaced, never disposed.
    Conflicts,
}

/// A coverage finding: one normalized [`CoverageEffect`] for an in-flight instance.
///
/// This is the *relevant in-flight* reaching the core — a domain-certified effect, never
/// a raw in-flight `Correction`. Production and richer relation taxonomies remain
/// downstream; the core never inspects the in-flight corrections themselves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageFinding {
    /// The in-flight instance this verdict is about.
    pub inflight: InFlightIndex,
    /// The instance's relation to the current plan.
    pub effect: CoverageEffect,
}

/// A `Sounding`: one convergence cycle's certified readings — the [`Fix`] and the
/// normalized coverage effects the domain supplied this cycle.
///
/// It carries **no** domain `Body`: it is a non-generic type that references targets and
/// in-flight `Correction`s only by [`Sigil`]/[`InFlightIndex`] and verdict, so the core
/// cannot read a payload from the readings. The `Bearing` is the reference a sounding is
/// taken *against*, so it is not part of the `Sounding`; the domain payload flows only
/// from `Bearing<Body>` into [`Course<Body>`](Course). Construct via [`Sounding::new`];
/// read via [`Sounding::fix`] and [`Sounding::coverage`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sounding {
    fix: Fix,
    coverage: Vec<CoverageFinding>,
}

impl Sounding {
    /// Assemble one cycle's readings from its `Fix` and its coverage findings.
    #[must_use]
    pub const fn new(fix: Fix, coverage: Vec<CoverageFinding>) -> Self {
        Self { fix, coverage }
    }

    /// This cycle's certified satisfaction of the `Bearing`'s targets.
    #[must_use]
    pub const fn fix(&self) -> &Fix {
        &self.fix
    }

    /// This cycle's normalized coverage effects for in-flight `Correction`s.
    #[must_use]
    pub fn coverage(&self) -> &[CoverageFinding] {
        &self.coverage
    }
}

/// A finding the core surfaces on its output without disposing of it.
///
/// The core never cancels, compensates, or names an execution-lifecycle state; these make
/// uncertainty and in-flight disposition observable to the consumer, whose policy disposes.
/// `#[non_exhaustive]` because the surfaced set may grow.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SurfacedFinding {
    /// A `Bearing` target retained only because its satisfaction was `Unknown` or absent —
    /// the observable alarm for a possible silent domain error.
    UnknownRetained(Sigil),
    /// An in-flight correction a coverage finding marked superseded by the current plan.
    Superseded(InFlightIndex),
    /// An in-flight correction a coverage finding marked in conflict with the current plan.
    Conflicting(InFlightIndex),
}

/// The output of [`plan_residual`]: the residual `Course` plus the surfaced findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Residual<Body> {
    /// The residual plan: `Bearing` targets neither satisfied nor covered.
    pub course: Course<Body>,
    /// Findings surfaced for the consumer to dispose of (uncertainty, supersession, conflict).
    pub surfaced: Vec<SurfacedFinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FoldedSatisfaction {
    Satisfied,
    Unsatisfied,
    Uncertain,
}

fn fold_satisfaction(findings: &[SatisfactionFinding], target: &Sigil) -> FoldedSatisfaction {
    let mut saw_satisfied = false;
    let mut saw_unsatisfied = false;
    let mut saw_unknown = false;

    for finding in findings.iter().filter(|finding| &finding.target == target) {
        match finding.satisfaction {
            Satisfaction::Satisfied => saw_satisfied = true,
            Satisfaction::Unsatisfied => saw_unsatisfied = true,
            Satisfaction::Unknown => saw_unknown = true,
        }
    }

    match (saw_satisfied, saw_unsatisfied, saw_unknown) {
        (true, false, false) => FoldedSatisfaction::Satisfied,
        (false, true, false) => FoldedSatisfaction::Unsatisfied,
        _ => FoldedSatisfaction::Uncertain,
    }
}

impl<Body> Residual<Body> {
    /// Whether this cycle is fully converged: the `Course` is empty **and** no findings
    /// are surfaced.
    ///
    /// This is a pure, policy-free structural read — it inspects only whether the two
    /// collections are empty, never a `Body`, and compares no meaning. It reports *full*
    /// convergence: nothing left to plan and nothing flagged for disposition. A consumer
    /// that checked only for an empty `Course` would declare success while an undisposed
    /// surfaced finding (a superseded or conflicting in-flight `Correction`) still awaits
    /// the domain's attention; requiring `surfaced` to be empty too closes that gap.
    ///
    /// Deciding what to do when this is `false` — whether a surfaced finding is blocking or
    /// merely pending, whether to keep correcting or treat a target as terminal — is a
    /// disposition, which is the domain's judgment, not a read the core provides.
    #[must_use]
    pub fn is_converged(&self) -> bool {
        self.course.corrections().is_empty() && self.surfaced.is_empty()
    }
}

/// Compute the residual for one convergence cycle.
///
/// The residual is the `Bearing`'s targets minus those a domain finding positively
/// certifies as `Satisfied` or `Covered`. A target that is unsatisfied, uncovered, or of
/// `Unknown`/absent satisfaction is **retained** — only positive certification omits, so
/// absence and uncertainty never drop a target (the conservative, false-negative-safe
/// rule). A target retained under `Unknown`/absence is surfaced, as are in-flight
/// corrections a coverage finding marks superseded or conflicting; the core disposes of
/// nothing.
///
/// The cycle's readings arrive as a [`Sounding`] — the [`Fix`] (per-target
/// [`SatisfactionFinding`]s) and the relevant in-flight as [`CoverageFinding`]s — never
/// raw observations or raw in-flight corrections, which is why there is no in-flight
/// parameter. The `Bearing` stays a separate argument: it is the reference the `Sounding`
/// is taken against. The function is pure and functional-per-cycle: it reads only its
/// arguments, holds no state across `Sounding`s, reads no clock, and performs no I/O. It
/// compares `Sigil`s by value only and never inspects a `Body` (the `Sounding` has none).
#[must_use]
pub fn plan_residual<Body>(bearing: Bearing<Body>, sounding: &Sounding) -> Residual<Body> {
    let satisfaction = sounding.fix().findings();
    let coverage = sounding.coverage();

    let mut surfaced = Vec::new();
    let mut covered = Vec::new();

    // Every normalized coverage effect contributes independently. This match is
    // deliberately exhaustive: a future effect must define its mechanical contribution.
    for finding in coverage {
        match &finding.effect {
            CoverageEffect::Covers(target) => covered.push(target),
            CoverageEffect::Superseded => {
                surfaced.push(SurfacedFinding::Superseded(finding.inflight));
            }
            CoverageEffect::Conflicts => {
                surfaced.push(SurfacedFinding::Conflicting(finding.inflight));
            }
        }
    }

    let mut retained = Vec::new();
    for target in bearing.into_targets() {
        let sig = target.sigil();
        let folded = fold_satisfaction(satisfaction, sig);
        let is_covered = covered.contains(&sig);
        if folded == FoldedSatisfaction::Satisfied || is_covered {
            continue;
        }
        if folded == FoldedSatisfaction::Uncertain {
            surfaced.push(SurfacedFinding::UnknownRetained(sig.clone()));
        }
        retained.push(target);
    }

    Residual {
        course: Course::new(retained),
        surfaced,
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

    #[test]
    fn body_free_reading_types_compare_structurally() {
        let a = Sounding::new(
            Fix::new(vec![sat("a", Satisfaction::Satisfied)]),
            vec![CoverageFinding {
                inflight: InFlightIndex(0),
                effect: CoverageEffect::Covers(Sigil::new("b")),
            }],
        );
        let same = Sounding::new(
            Fix::new(vec![sat("a", Satisfaction::Satisfied)]),
            vec![CoverageFinding {
                inflight: InFlightIndex(0),
                effect: CoverageEffect::Covers(Sigil::new("b")),
            }],
        );
        let different = Sounding::new(Fix::new(vec![sat("a", Satisfaction::Unsatisfied)]), vec![]);

        assert_eq!(a, same, "equal fields make equal Soundings");
        assert_ne!(a, different, "a differing Fix makes an unequal Sounding");
    }

    #[test]
    fn body_carrying_types_compare_structurally_when_body_does() {
        let a = Course::new(vec![corr("x", 1u8)]);
        let same = Course::new(vec![corr("x", 1u8)]);
        let different_body = Course::new(vec![corr("x", 2u8)]);
        let different_sigil = Course::new(vec![corr("y", 1u8)]);

        assert_eq!(
            a, same,
            "equal Sigil, Reversibility, and Body make equal Corrections"
        );
        assert_ne!(
            a, different_body,
            "a differing Body makes an unequal Course"
        );
        assert_ne!(
            a, different_sigil,
            "a differing Sigil makes an unequal Course"
        );
    }

    // --- residual planner (the test helpers below are the non-shipped fixture consumer:
    // they only *supply* pre-computed findings; they invent no judgment) ---

    fn corr(sig: &str, body: u8) -> Correction<u8> {
        Correction::new(Sigil::new(sig), Reversibility::Reversible, body)
    }

    fn sat(sig: &str, satisfaction: Satisfaction) -> SatisfactionFinding {
        SatisfactionFinding {
            target: Sigil::new(sig),
            satisfaction,
        }
    }

    fn sounding(fix: Vec<SatisfactionFinding>, coverage: Vec<CoverageFinding>) -> Sounding {
        Sounding::new(Fix::new(fix), coverage)
    }

    fn retained_sigils(residual: &Residual<u8>) -> Vec<&str> {
        residual
            .course
            .corrections()
            .iter()
            .map(|c| c.sigil().as_str())
            .collect()
    }

    #[test]
    fn satisfied_or_covered_targets_are_omitted() {
        let bearing = Bearing::new(vec![corr("a", 1), corr("b", 2), corr("c", 3)]);
        let satisfaction = vec![sat("a", Satisfaction::Satisfied)];
        let coverage = vec![CoverageFinding {
            inflight: InFlightIndex(0),
            effect: CoverageEffect::Covers(Sigil::new("b")),
        }];
        let residual = plan_residual(bearing, &sounding(satisfaction, coverage));
        // a satisfied, b covered, c neither -> only c remains.
        assert_eq!(retained_sigils(&residual), ["c"]);
    }

    #[test]
    fn absence_unknown_and_unsatisfied_all_retain() {
        let bearing = Bearing::new(vec![
            corr("absent", 1),
            corr("unknown", 2),
            corr("unsat", 3),
        ]);
        let satisfaction = vec![
            sat("unknown", Satisfaction::Unknown),
            sat("unsat", Satisfaction::Unsatisfied),
        ];
        let residual = plan_residual(bearing, &sounding(satisfaction, vec![]));
        // Only positive certification omits; absence and uncertainty never do.
        assert_eq!(residual.course.corrections().len(), 3);
    }

    #[test]
    fn only_uncertain_retention_is_surfaced() {
        let bearing = Bearing::new(vec![
            corr("absent", 1),
            corr("unknown", 2),
            corr("unsat", 3),
        ]);
        let satisfaction = vec![
            sat("unknown", Satisfaction::Unknown),
            sat("unsat", Satisfaction::Unsatisfied),
        ];
        let residual = plan_residual(bearing, &sounding(satisfaction, vec![]));
        let alarms: Vec<&str> = residual
            .surfaced
            .iter()
            .filter_map(|f| match f {
                SurfacedFinding::UnknownRetained(s) => Some(s.as_str()),
                _ => None,
            })
            .collect();
        assert!(alarms.contains(&"absent"), "absence is uncertainty");
        assert!(alarms.contains(&"unknown"), "unknown is uncertainty");
        assert!(
            !alarms.contains(&"unsat"),
            "known-unsatisfied is not an alarm"
        );
    }

    #[test]
    fn supersession_and_conflict_surface_referenced_instances() {
        let bearing: Bearing<u8> = Bearing::new(vec![]);
        let coverage = vec![
            CoverageFinding {
                inflight: InFlightIndex(2),
                effect: CoverageEffect::Superseded,
            },
            CoverageFinding {
                inflight: InFlightIndex(5),
                effect: CoverageEffect::Conflicts,
            },
        ];
        let residual = plan_residual(bearing, &sounding(vec![], coverage));
        assert!(
            residual
                .surfaced
                .contains(&SurfacedFinding::Superseded(InFlightIndex(2)))
        );
        assert!(
            residual
                .surfaced
                .contains(&SurfacedFinding::Conflicting(InFlightIndex(5)))
        );
        assert_eq!(residual.surfaced.len(), 2);
    }

    #[test]
    fn satisfaction_effects_fold_conservatively_and_idempotently() {
        struct Case {
            name: &'static str,
            findings: Vec<SatisfactionFinding>,
            retained: bool,
            surfaced_unknown: bool,
        }

        let cases = [
            Case {
                name: "repeated satisfied",
                findings: vec![
                    sat("a", Satisfaction::Satisfied),
                    sat("a", Satisfaction::Satisfied),
                ],
                retained: false,
                surfaced_unknown: false,
            },
            Case {
                name: "repeated unsatisfied",
                findings: vec![
                    sat("a", Satisfaction::Unsatisfied),
                    sat("a", Satisfaction::Unsatisfied),
                ],
                retained: true,
                surfaced_unknown: false,
            },
            Case {
                name: "absent",
                findings: vec![],
                retained: true,
                surfaced_unknown: true,
            },
            Case {
                name: "unknown",
                findings: vec![sat("a", Satisfaction::Unknown)],
                retained: true,
                surfaced_unknown: true,
            },
            Case {
                name: "mixed known values",
                findings: vec![
                    sat("a", Satisfaction::Satisfied),
                    sat("a", Satisfaction::Unsatisfied),
                ],
                retained: true,
                surfaced_unknown: true,
            },
            Case {
                name: "known plus unknown",
                findings: vec![
                    sat("a", Satisfaction::Unsatisfied),
                    sat("a", Satisfaction::Unknown),
                ],
                retained: true,
                surfaced_unknown: true,
            },
        ];

        for case in cases {
            let residual = plan_residual(
                Bearing::new(vec![corr("a", 1)]),
                &sounding(case.findings, vec![]),
            );
            assert_eq!(
                !residual.course.corrections().is_empty(),
                case.retained,
                "{}",
                case.name
            );
            assert_eq!(
                residual
                    .surfaced
                    .contains(&SurfacedFinding::UnknownRetained(Sigil::new("a"))),
                case.surfaced_unknown,
                "{}",
                case.name
            );
        }
    }

    #[test]
    fn coverage_is_existential_and_idempotent_per_target() {
        let bearing = Bearing::new(vec![corr("a", 1), corr("b", 2), corr("c", 3)]);
        let coverage = vec![
            CoverageFinding {
                inflight: InFlightIndex(0),
                effect: CoverageEffect::Covers(Sigil::new("a")),
            },
            CoverageFinding {
                inflight: InFlightIndex(0),
                effect: CoverageEffect::Covers(Sigil::new("b")),
            },
            CoverageFinding {
                inflight: InFlightIndex(0),
                effect: CoverageEffect::Covers(Sigil::new("a")),
            },
            CoverageFinding {
                inflight: InFlightIndex(1),
                effect: CoverageEffect::Covers(Sigil::new("b")),
            },
        ];
        let satisfaction = vec![
            sat("a", Satisfaction::Unsatisfied),
            sat("b", Satisfaction::Unsatisfied),
            sat("c", Satisfaction::Unsatisfied),
        ];
        let residual = plan_residual(bearing, &sounding(satisfaction, coverage));

        assert_eq!(retained_sigils(&residual), ["c"]);
        assert!(residual.surfaced.is_empty());
    }

    #[test]
    fn conflict_and_supersession_each_compose_independently_with_coverage() {
        let referenced = InFlightIndex(7);
        let cases = [
            (
                CoverageEffect::Conflicts,
                SurfacedFinding::Conflicting(referenced),
            ),
            (
                CoverageEffect::Superseded,
                SurfacedFinding::Superseded(referenced),
            ),
        ];

        for (effect, expected) in cases {
            let residual = plan_residual(
                Bearing::new(vec![corr("a", 1)]),
                &sounding(
                    vec![sat("a", Satisfaction::Unsatisfied)],
                    vec![
                        CoverageFinding {
                            inflight: referenced,
                            effect: CoverageEffect::Covers(Sigil::new("a")),
                        },
                        CoverageFinding {
                            inflight: referenced,
                            effect,
                        },
                    ],
                ),
            );

            assert!(residual.course.corrections().is_empty());
            assert!(residual.surfaced.contains(&expected));
            assert!(!residual.is_converged());
        }
    }

    #[test]
    fn absent_coverage_is_the_identity_not_a_disjointness_inference() {
        let bearing = Bearing::new(vec![corr("a", 1)]);
        let residual = plan_residual(
            bearing,
            &sounding(vec![sat("a", Satisfaction::Unsatisfied)], vec![]),
        );

        assert_eq!(retained_sigils(&residual), ["a"]);
        assert!(residual.surfaced.is_empty());
    }

    #[test]
    fn equal_sigil_targets_are_not_deduplicated() {
        let bearing = Bearing::new(vec![corr("same", 1), corr("same", 2)]);
        let residual = plan_residual(bearing, &sounding(vec![], vec![]));
        assert_eq!(residual.course.corrections().len(), 2);
    }

    #[test]
    fn contradictory_satisfaction_retains_conservatively() {
        let bearing = Bearing::new(vec![corr("x", 1)]);
        let satisfaction = vec![
            sat("x", Satisfaction::Satisfied),
            sat("x", Satisfaction::Unsatisfied),
        ];
        let residual = plan_residual(bearing, &sounding(satisfaction, vec![]));
        // A contradiction is not unambiguous certification, so the target is retained.
        assert_eq!(residual.course.corrections().len(), 1);
    }

    #[test]
    fn is_converged_when_course_and_surfaced_are_both_empty() {
        // Every target satisfied, nothing surfaced -> fully converged.
        let bearing = Bearing::new(vec![corr("a", 1)]);
        let residual = plan_residual(
            bearing,
            &sounding(vec![sat("a", Satisfaction::Satisfied)], vec![]),
        );
        assert!(residual.course.corrections().is_empty());
        assert!(residual.surfaced.is_empty());
        assert!(residual.is_converged());
    }

    #[test]
    fn empty_course_with_a_surfaced_finding_is_not_converged() {
        // No targets remain, but a conflicting in-flight correction is surfaced and
        // undisposed: not convergence. This is the silent-failure is_converged guards.
        let bearing: Bearing<u8> = Bearing::new(vec![]);
        let coverage = vec![CoverageFinding {
            inflight: InFlightIndex(0),
            effect: CoverageEffect::Conflicts,
        }];
        let residual = plan_residual(bearing, &sounding(vec![], coverage));
        assert!(residual.course.corrections().is_empty());
        assert!(!residual.surfaced.is_empty());
        assert!(!residual.is_converged());
    }

    #[test]
    fn non_empty_course_is_not_converged() {
        // A retained target means work remains, regardless of surfaced findings.
        let bearing = Bearing::new(vec![corr("a", 1)]);
        let residual = plan_residual(
            bearing,
            &sounding(vec![sat("a", Satisfaction::Unsatisfied)], vec![]),
        );
        assert!(!residual.course.corrections().is_empty());
        assert!(!residual.is_converged());
    }

    #[test]
    fn is_converged_needs_no_capability_from_body() {
        // An opaque Body with no trait beyond what `Correction::new` requires (none):
        // `is_converged` still works, so it reads only collection emptiness, never the payload.
        #[derive(Debug, Clone)]
        struct OpaqueBody;
        let bearing = Bearing::new(vec![Correction::new(
            Sigil::new("a"),
            Reversibility::Reversible,
            OpaqueBody,
        )]);
        let residual = plan_residual(
            bearing,
            &sounding(vec![sat("a", Satisfaction::Satisfied)], vec![]),
        );
        assert!(residual.is_converged());
    }
}
