//! Composition proof: a convergence-loop consumer driven through the `suunta` facade.
//!
//! This is the first real consumer of the planning contract. It drives full
//! convergence loops over the shipped `plan_residual` using **only** the `suunta`
//! facade's public API — never `suunta-contract` directly — so it doubles as proof
//! that the facade re-exports every type a consumer needs. It forces the two
//! questions the core deliberately leaves open (see the contract crate's `BACKLOG.md`,
//! "Settlement — three layers, three homes"):
//!
//! - **Layer 1 (mechanical read):** a fully-converged cycle halts on
//!   `Residual::is_converged` — a pure structural read the core provides.
//! - **Layer 2 (disposition):** every retry-vs-terminal and conflict-handling decision
//!   is made *here, in the consumer*. The core provides no settlement trait; the
//!   `Disposition` vocabulary below is the domain's, not the contract's.
//!
//! Layer 3 (cross-cycle termination) also lives here: the loop carries its own cycle
//! bound and progress check. None of it leaks into the core.
//!
//! To have teeth, the stub domain drives four trajectories in one run: a target that
//! converges, a target stuck `Unknown`, a target that never satisfies, and an in-flight
//! correction marked `Conflicts`.

use suunta::{
    Bearing, Correction, CoverageEffect, CoverageFinding, InFlightIndex, Reversibility,
    Satisfaction, SatisfactionFinding, Sigil, SurfacedFinding,
};

/// A domain payload. The core carries it opaquely and never reads it.
#[derive(Debug, Clone)]
struct Action {
    what: &'static str,
}

/// The domain's terminal verdict for a target or in-flight correction once the loop stops
/// making progress. This is **Layer 2**: it lives in the consumer, never in the core.
#[derive(Debug, PartialEq, Eq)]
enum Disposition {
    /// The target was satisfied and left the residual.
    Converged,
    /// The target never satisfies; the domain calls it a terminal breach.
    Breached,
    /// The target's satisfaction never resolves; the domain abandons it under uncertainty.
    Abandoned,
    /// An in-flight correction conflicts with the plan; the domain holds it for resolution.
    Held,
}

const CONVERGE: &str = "goal:converge";
const STUCK_UNKNOWN: &str = "goal:stuck-unknown";
const NEVER_SAT: &str = "goal:never-satisfies";
/// The in-flight correction (by position) the domain will report as conflicting.
const CONFLICTING_INFLIGHT: InFlightIndex = InFlightIndex(0);
/// Safety bound for the loop — Layer 3, held by the consumer.
const MAX_CYCLES: usize = 8;

/// The desired end state. It is the same every cycle; what changes is reality catching up,
/// reported as satisfaction findings. Rebuilt per cycle because `plan_residual` consumes it.
fn bearing() -> Bearing<Action> {
    Bearing::new(vec![
        Correction::new(
            Sigil::new(CONVERGE),
            Reversibility::Reversible,
            Action {
                what: "steer toward the converging goal",
            },
        ),
        Correction::new(
            Sigil::new(STUCK_UNKNOWN),
            Reversibility::Reversible,
            Action {
                what: "steer toward the never-observed goal",
            },
        ),
        Correction::new(
            Sigil::new(NEVER_SAT),
            Reversibility::OneWay,
            Action {
                what: "steer toward the unreachable goal",
            },
        ),
    ])
}

/// The domain's satisfaction verdicts for a given cycle — its model of reality.
fn observe(cycle: usize) -> Vec<SatisfactionFinding> {
    vec![
        // Converges: unsatisfied until cycle 2, satisfied thereafter.
        SatisfactionFinding {
            target: Sigil::new(CONVERGE),
            satisfaction: if cycle >= 2 {
                Satisfaction::Satisfied
            } else {
                Satisfaction::Unsatisfied
            },
        },
        // Never resolves.
        SatisfactionFinding {
            target: Sigil::new(STUCK_UNKNOWN),
            satisfaction: Satisfaction::Unknown,
        },
        // Never satisfies.
        SatisfactionFinding {
            target: Sigil::new(NEVER_SAT),
            satisfaction: Satisfaction::Unsatisfied,
        },
    ]
}

/// The domain's coverage verdicts: one in-flight correction always conflicts with the plan.
fn coverage() -> Vec<CoverageFinding> {
    vec![CoverageFinding {
        inflight: CONFLICTING_INFLIGHT,
        effect: CoverageEffect::Conflicts,
    }]
}

/// A clean consumer: every target satisfies and nothing is surfaced, so the loop halts on
/// `Residual::is_converged` — the fulfilled path, where the Layer 1 read is load-bearing.
#[test]
fn clean_domain_halts_via_is_converged() {
    let bearing = || {
        Bearing::new(vec![
            Correction::new(
                Sigil::new("goal:alpha"),
                Reversibility::Reversible,
                Action {
                    what: "steer alpha",
                },
            ),
            Correction::new(
                Sigil::new("goal:beta"),
                Reversibility::Reversible,
                Action { what: "steer beta" },
            ),
        ])
    };
    let observe_clean = |cycle: usize| {
        let verdict = if cycle >= 1 {
            Satisfaction::Satisfied
        } else {
            Satisfaction::Unsatisfied
        };
        vec![
            SatisfactionFinding {
                target: Sigil::new("goal:alpha"),
                satisfaction: verdict,
            },
            SatisfactionFinding {
                target: Sigil::new("goal:beta"),
                satisfaction: verdict,
            },
        ]
    };

    let mut fulfilled = false;
    for cycle in 0..MAX_CYCLES {
        let residual = suunta::plan_residual(bearing(), &observe_clean(cycle), &[]);
        if residual.is_converged() {
            fulfilled = true;
            break;
        }
    }
    assert!(
        fulfilled,
        "a fully-satisfied domain with nothing surfaced must halt via is_converged"
    );
}

/// The four-trajectory consumer: a converging target, one stuck `Unknown`, one permanently
/// `Unsatisfied`, and a conflicting in-flight correction. This domain can never fully
/// converge, so it halts by domain disposition (Layer 2), not by `is_converged`.
#[test]
fn four_trajectories_halt_by_disposition() {
    let mut cycles_run = 0usize;
    let mut fulfilled = false;
    let mut dispositions: Vec<(String, Disposition)> = Vec::new();

    for cycle in 0..MAX_CYCLES {
        cycles_run += 1;
        let residual = suunta::plan_residual(bearing(), &observe(cycle), &coverage());

        // Layer 1: the one mechanical read the core provides.
        if residual.is_converged() {
            fulfilled = true;
            break;
        }

        // Layer 2 (disposition), in the consumer's loop body. The converging goal is the
        // only movable target; once it has left the residual, no further observation will
        // change the permanent ones, so the domain stops correcting and disposes terminally.
        let converge_still_open = residual
            .course
            .corrections()
            .iter()
            .any(|c| c.sigil().as_str() == CONVERGE);
        if converge_still_open {
            // Keep correcting: retry next cycle. This is the retry path, owned by the domain.
            continue;
        }

        // Progress exhausted: classify what remains. The core disposes of nothing.
        // The converging goal has already left the residual — the domain records it fulfilled.
        dispositions.push((CONVERGE.to_owned(), Disposition::Converged));
        for retained in residual.course.corrections() {
            // The domain reads its own payload here; the core never does.
            let action = retained.body();
            let disp = match retained.sigil().as_str() {
                NEVER_SAT => Disposition::Breached,
                STUCK_UNKNOWN => Disposition::Abandoned,
                other => panic!("unexpected retained target: {other}"),
            };
            let _ = action.what;
            dispositions.push((retained.sigil().as_str().to_owned(), disp));
        }
        for finding in &residual.surfaced {
            // SurfacedFinding is #[non_exhaustive]; a conflict is held, the rest are the
            // uncertainty alarms already reflected by the retained targets above.
            match finding {
                SurfacedFinding::Conflicting(idx) => {
                    dispositions.push((format!("in-flight#{}", idx.0), Disposition::Held));
                }
                SurfacedFinding::UnknownRetained(_) | SurfacedFinding::Superseded(_) => {}
                _ => {}
            }
        }
        break;
    }

    // The retry path was exercised: the converging goal took more than one cycle.
    assert!(
        cycles_run >= 3,
        "expected the retry path across >=3 cycles, ran {cycles_run}"
    );
    // The loop halted by domain disposition, not by full convergence — the permanent
    // trajectories can never all clear.
    assert!(
        !fulfilled,
        "this domain can never fully converge; halt must be by disposition"
    );

    let find = |who: &str| dispositions.iter().find(|(w, _)| w == who).map(|(_, d)| d);
    assert_eq!(
        find(NEVER_SAT),
        Some(&Disposition::Breached),
        "the never-satisfying target must be a terminal breach"
    );
    assert_eq!(
        find(STUCK_UNKNOWN),
        Some(&Disposition::Abandoned),
        "the stuck-Unknown target must be abandoned under uncertainty"
    );
    assert_eq!(
        find(&format!("in-flight#{}", CONFLICTING_INFLIGHT.0)),
        Some(&Disposition::Held),
        "the conflicting in-flight correction must be held"
    );
    // The converging goal converged: recorded fulfilled, having left the residual.
    assert_eq!(
        find(CONVERGE),
        Some(&Disposition::Converged),
        "the converging goal should have converged and left the residual"
    );
}
