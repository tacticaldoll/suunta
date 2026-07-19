# Suunta

Suunta is a thin, sans-I/O **convergence-planning** core for Rust: given a desired
`Bearing` and an observed `Fix`, it computes the residual `Course` — the
`Correction`s needed to converge — while making no semantic judgment of its own.

```text
Sounding  -> Fix ┐
Domain    -> Bearing ┤ residual -> Course (Corrections, each with a stable Sigil)
in-flight ─────────┘
```

It is for the narrow space between "I keep reconciling desired state against
observed state" and "I do not want a workflow engine to own my domain's meaning."
Suunta owns one mechanism — the residual — and outsources every semantic judgment
to the domain.

## Status (0.1.0)

This is the **initial project shape**, not a working planner yet. It ships the
contract vocabulary, the architectural axioms, the executable governance, and a
compiling crate skeleton. The residual-planning core is defined in `openspec/specs/`
and implemented in later spec-driven changes — deliberately, because the design
still has open questions (see `BACKLOG.md`).

## What Suunta owns, and what the domain supplies

Suunta owns a *mechanism* and no *meaning*. It computes *what* diverges between
intent and reality; it never decides what two things *mean*, whether one is
*relevant*, or whether an obligation is *settled*. Those are yours.

```text
The domain supplies (meaning)                Suunta owns (mechanism, no meaning)
  Sigil     a stable semantic identity         the residual computation
  coverage  which in-flight work is relevant    (Bearing vs Fix ∪ relevant in-flight)
  predicate when an obligation is settled       the Course / Correction vocabulary
                                                reversibility marking (One-Way)
```

The core decides which `Correction`s close the drift; it does not decide their
meaning, their durability, their gating, or their compensation — those are downstream
consumer concerns.

## Why sans-I/O and no semantic judgment

A pure core that reads no clock and performs no I/O cannot make a semantic judgment
either — so it outsources all three (identity, relevance, settlement) to the domain.
This is the **semantic bill of purity**: its cost is that an undetected domain
semantic error fails silently, accepted deliberately rather than patched by pulling
judgment back into the core. See `PROJECT.md` and `BACKLOG.md`.

## Domain Language

Suunta uses navigation terms as architecture, not branding — `Sounding`, `Fix`,
`Bearing`, `Course`, `Correction`, `Sigil`, `Drift`. See
[`docs/domain-language.md`](docs/domain-language.md).

## Architecture

- `PROJECT.md` — vision, positioning, non-goals.
- `openspec/specs/` — shipped requirements.
- `BACKLOG.md` — deferred decisions and open design questions.
- `AGENTS.md` — operating protocol and the Definition of Done.

## Contributing

This project uses OpenSpec and Tianheng-native governance. Start a change with:

```bash
openspec new change "your-change-name"
```

Run the full Definition of Done (see `AGENTS.md`) before committing, and read
`AGENTS.md` before making repository changes.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
