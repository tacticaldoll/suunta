# Suunta

Suunta is a thin, sans-I/O **convergence-planning** core for Rust: given a desired
`Bearing` and the domain's certified satisfaction of each target (the `Fix`), it
computes the residual `Course` — the `Correction`s that remain to converge — by
filtering the `Bearing` to what the domain has not certified done, while making no
semantic judgment of its own.

```text
Domain certifies reality against the Bearing's targets:
  per target    -> Fix       (satisfaction verdict)  ┐
  per in-flight -> coverage   (relevance verdict)     ┤ Suunta filters the Bearing
Domain          -> Bearing    (desired targets)       ┘   -> Course (residual Corrections, each with a Sigil)
```

It is for the narrow space between "I keep reconciling desired state against
observed state" and "I do not want a workflow engine to own my domain's meaning."
Suunta owns one mechanism — the residual filter — and outsources every semantic
judgment to the domain; it consumes the domain's verdicts, never reality itself.

## Status (0.1.0)

0.1.0 ships the residual planner: `plan_residual` computes the residual `Course` from
a `Bearing` and a per-cycle `Sounding` (the domain's `Fix` and coverage findings),
alongside the `Correction`/`Course`/`Sigil`/`Reversibility` vocabulary, the architectural axioms,
and the executable governance. What is still deferred — the settlement predicate, the
production-side coverage contract, and an async edge — is recorded in `BACKLOG.md`.

Depend on the curated **facade** (`suunta`) — the recommended single entrypoint that
re-exports the compose-level surface and carries a runnable convergence-loop doctest.
The isolated core (`suunta-contract`) stays available for direct use, but `suunta` is
the crate to depend on.

## What Suunta owns, and what the domain supplies

Suunta owns a *mechanism* and no *meaning*. It filters the `Bearing` to *what
remains* once the domain's verdicts have certified targets satisfied or covered; it
never decides what two things *mean*, whether one is *relevant*, or whether an
obligation is *settled*. Those are yours.

```text
The domain supplies (meaning)                 Suunta owns (mechanism, no meaning)
  Sigil        a stable semantic identity        the residual computation (plan_residual)
  satisfaction whether a Bearing target is met   the Course / Correction / Sigil vocabulary
  coverage     which in-flight work is relevant   reversibility marking (One-Way)
  predicate    when an obligation is settled      surfacing Unknown / supersession / conflict
```

The core filters the `Bearing` to the `Correction`s that remain, given the domain's
verdicts; it does not decide their meaning, their durability, their gating, or their
compensation — those are downstream consumer concerns.

## Why sans-I/O and no semantic judgment

A pure core that reads no clock and performs no I/O cannot make a semantic judgment
either — so it outsources all four (identity, satisfaction, relevance, settlement) to
the domain. This is the **semantic bill of purity**: its cost is that an undetected
domain semantic error fails silently, accepted deliberately rather than patched by
pulling judgment back into the core. See `PROJECT.md` and `BACKLOG.md`.

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
