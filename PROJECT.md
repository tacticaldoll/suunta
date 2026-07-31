# Project Contract

## Vision

Suunta is a thin, sans-I/O **convergence-planning design pattern** for Rust: given a
desired `Bearing` and the domain's certified satisfaction of each target (the `Fix`), it
computes the residual `Course` — the `Correction`s that remain to converge — by
filtering the `Bearing` to what the domain has not certified done. It observes no
reality and compares no meanings; it consumes the domain's verdicts and makes no
semantic judgment of its own.

What Suunta delivers is the *pattern*, not a batteries-included engine: it owns the
**nouns** — the residual mechanism and the navigation vocabulary — and outsources every
**verb** (judgment, execution, driving) to the domain that composes it.

It fills a narrow gap: the thinnest useful planning primitive that computes *what
remains* to close the drift between intent and reality — not by diffing reality
itself, but by filtering the `Bearing` against the domain's verdicts — without
becoming a workflow engine, a scheduler, or a durable-execution runtime. Suunta owns
the residual mechanism and outsources every semantic judgment to the domain.

## Product Positioning

Suunta is for systems that repeatedly reconcile a desired state against an observed
one and must plan the corrections:

```text
Domain certifies reality against the Bearing's targets:
  per target    -> Fix       (satisfaction verdict)  ┐
  per in-flight -> coverage   (relevance verdict)     ┤ Suunta filters the Bearing
Domain          -> Bearing    (desired targets)       ┘   -> Course (residual Corrections, each with a Sigil)
```

The core never sees raw reality or the in-flight `Correction`s themselves — only the
domain's verdicts about them. The promise is not "a planner with batteries". It is a
clean, pure place to attach:

- domain-supplied semantic identity (`Sigil`)
- domain-supplied relevance (a coverage verdict)
- domain-supplied settlement predicates
- downstream durability, gating, and compensation — consumer concerns, not Suunta

### Nouns, verbs, and inward-pointing governance

Suunta's users are **composition-driven**: they assemble freely-decoupled blocks, and
they do not drive Suunta's shape by coupling to it. The division of labour is fixed:

- Suunta owns the **nouns** — the vocabulary of exchange (`Sounding`, `Fix`, `Bearing`,
  `Course`, `Correction`, `Sigil`, `Drift`) and the residual mechanism that relates them.
- The consumer owns the **verbs** — judgment, execution, and driving the loop.

Suunta's governance teeth point **inward**: they constrain Suunta's own purity
(sans-I/O, dependency isolation, facade re-export purity), never a consumer's design.
Because the deliverable is a pattern, Suunta decides its own surface from this identity —
it does not defer that decision to a consumer that does not yet exist.

Suunta refuses **any surface that dictates a consumer's shape** — the inverse of Tower.
That refusal has more than one face, and all are rejected:

- a **trait the consumer implements** (Tower's `Service`);
- an **engine that owns the run loop** the consumer calls into (`.run()`);
- a **middleware stack** the consumer must layer into (Tower's `Layer`);
- a **readiness / backpressure contract** baked into the API (`poll_ready`).

A design pattern *drives* its consumer by offering a vocabulary and a residual mechanism
so clear they organize their thinking around it — never by owning their code shape. It
drives their thinking, not their structure; adoption is voluntary. The moment a surface
would make the consumer's structure Suunta's to dictate, it is the drift Suunta exists to
refuse.

## Core Contract

The behavior that must be protected first:

- **Thin planning core**: `suunta-contract` owns the residual mechanism — it filters
  the `Bearing` to the targets the domain's `Fix` (a satisfaction verdict per target)
  has not certified satisfied and coverage has not certified covered, surfacing what
  remains uncertain — plus the `Course`/`Correction` vocabulary. It observes no reality
  and diffs no meanings itself; it does not own execution, durability, scheduling,
  gating, or compensation.
- **No semantic judgment in the core — the semantic bill of purity**: a sans-I/O
  pure core cannot decide semantic identity, whether a target is satisfied, relevance,
  or whether an obligation is settled. These four judgments are domain-supplied; the
  core only filters the residual and records. (Satisfaction — whether reality meets a
  desired `Bearing` target — is the fourth face, surfaced when the residual computation
  was built: comparing reality against desired is a meaning comparison the pure core
  cannot make, so the domain certifies it and that per-target verdict is the `Fix`.) An
  undetected domain semantic error is
  therefore a silent failure — the deliberate cost of purity — with structural and
  idempotency defenses left to downstream consumers (see `BACKLOG.md`).
- **Sans-I/O purity**: the core exposes no `async fn`, reads no ambient clock, and
  performs no I/O. A runtime drives it and injects time at the edge.
- **Governance with teeth**: Tianheng and project specs enforce the boundaries prose
  claims — with the honest exception that "the core makes no semantic judgment" is
  not statically expressible and stays review- and structure-governed (see BACKLOG).

## Elegance

Elegance in Suunta is technical restraint:

- one owned mechanism (the residual); every semantic judgment outsourced
- precise navigation vocabulary (`Sounding`, `Fix`, `Bearing`, `Course`,
  `Correction`, `Sigil`, `Drift`)
- domain-owned judgments
- small composable interfaces
- executable governance against architectural drift

The naming universe is part of that restraint: navigation terms keep the system
from sliding back into generic workflow-engine thinking.

## Non-Goals

Suunta core is not:

- a workflow or orchestration engine
- a scheduler or job runtime
- a durable-execution backend (a `Correction`'s durability and execution are
  consumer concerns)
- a semantic-comparison engine (it never compares meanings itself)
- a rollback or compensation owner (it marks a `Correction` One-Way; it does not undo)

## References

- Canonical shipped requirements: `openspec/specs/`
- Active proposed requirements: `openspec/changes/`
- Domain language: `docs/domain-language.md`
- Deferred decisions and open design questions: `BACKLOG.md`
