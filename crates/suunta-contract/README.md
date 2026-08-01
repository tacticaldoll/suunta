# suunta-contract

The isolated core contract for Suunta: sans-I/O convergence planning.

`suunta-contract` computes the residual `Course` — the `Correction`s that remain of a
desired `Bearing` once the domain's verdicts have certified targets done — and makes no
semantic judgment of its own:
identity (`Sigil`), target satisfaction, relevance, and settlement are all
domain-supplied. It exposes no `async fn`, reads no ambient clock, and performs no
I/O; a runtime drives it.

The residual planner is shipped — `plan_residual` takes a `Bearing` and a per-cycle
`Sounding` (the `Fix` and coverage findings) — alongside the
`Correction`/`Course`/`Sigil`/`Reversibility` vocabulary and the body-free `Fix`/`Sounding`
reading types. Settlement decomposes into three layers: the core exposes only the
policy-free `Residual::is_converged` (Layer 1); per-target disposition (Layer 2) and
cross-cycle termination (Layer 3) are the domain's and driver's downstream verbs. The
coverage-production contract and an async edge are likewise placed downstream, not core
work awaiting a consumer — see `BACKLOG.md`.

Part of [Suunta](https://github.com/tacticaldoll/suunta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-MIT), at your option.
