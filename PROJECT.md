# Project Contract

## Vision

Suunta is a thin, sans-I/O convergence-planning core for Rust: given a desired
`Bearing` and an observed `Fix`, it computes the residual `Course` — the
`Correction`s needed to converge — while making no semantic judgment of its own.

It fills a narrow gap: the thinnest useful planning primitive that decides *what
must change* to close the drift between intent and reality, without becoming a
workflow engine, a scheduler, or a durable-execution runtime. Suunta owns the
residual mechanism and outsources every semantic judgment to the domain.

## Product Positioning

Suunta is for systems that repeatedly reconcile a desired state against an observed
one and must plan the corrections:

```text
Sounding  -> Fix ┐
Domain    -> Bearing ┤ residual -> Course (Corrections, each with a stable Sigil)
in-flight ─────────┘
```

The promise is not "a planner with batteries". It is a clean, pure place to attach:

- domain-supplied semantic identity (`Sigil`)
- domain-supplied relevance (a coverage verdict)
- domain-supplied settlement predicates
- downstream durability, gating, and compensation — consumer concerns, not Suunta

## Core Contract

The behavior that must be protected first:

- **Thin planning core**: `suunta-contract` owns the residual computation
  (`Bearing` vs `Fix` unioned with the relevant in-flight `Correction`s) and the
  `Course`/`Correction` vocabulary. It does not own execution, durability,
  scheduling, gating, or compensation.
- **No semantic judgment in the core — the semantic bill of purity**: a sans-I/O
  pure core cannot decide semantic identity, relevance, or whether an obligation is
  settled. These three judgments are domain-supplied; the core only computes the
  residual and records. An undetected domain semantic error is therefore a silent
  failure — the deliberate cost of purity — with structural and idempotency defenses
  left to downstream consumers (see `BACKLOG.md`).
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
