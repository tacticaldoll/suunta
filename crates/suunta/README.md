# suunta

The curated entrypoint to Suunta: a thin, sans-I/O convergence-planning core you
compose.

`suunta` is a pure re-export facade — it carries no logic of its own. It re-exports
the compose-level surface you need to run a convergence loop end to end: the
navigation vocabulary (`Bearing`, `Correction`, `Course`, `Sigil`, `Reversibility`),
the domain's verdicts (`Satisfaction`, `CoverageEffect`, and their findings), the
residual output (`Residual`, `SurfacedFinding`), and the planner `plan_residual`.
This is the recommended crate to depend on.

Suunta owns one mechanism — the residual filter — and outsources every semantic
judgment to the domain: given a desired `Bearing` and the domain's certified
satisfaction and coverage verdicts, `plan_residual` computes the residual `Course`
(the `Correction`s that remain to converge) and makes no judgment of its own.

Suunta's whole public surface is compose-level, so this facade withholds nothing;
there is no advanced kernel to reach for through
[`suunta-contract`](https://crates.io/crates/suunta-contract) directly.

Part of [Suunta](https://github.com/tacticaldoll/suunta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-MIT), at your option.
