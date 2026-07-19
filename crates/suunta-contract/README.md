# suunta-contract

The isolated core contract for Suunta: sans-I/O convergence planning.

`suunta-contract` computes the residual `Course` — the `Correction`s that remain of a
desired `Bearing` once the domain's verdicts have certified targets done — and makes no
semantic judgment of its own:
identity (`Sigil`), target satisfaction, relevance, and settlement are all
domain-supplied. It exposes no `async fn`, reads no ambient clock, and performs no
I/O; a runtime drives it.

0.1.0 ships the residual planner — `plan_residual` takes a `Bearing` and a per-cycle
`Sounding` (the `Fix` and coverage findings) — alongside the
`Correction`/`Course`/`Sigil`/`Reversibility` vocabulary and the body-free `Fix`/`Sounding`
reading types. The settlement predicate, the production-side coverage contract, and an
async edge remain deferred.

Part of [Suunta](https://github.com/tacticaldoll/suunta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-MIT), at your option.
