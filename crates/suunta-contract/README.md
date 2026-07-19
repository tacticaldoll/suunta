# suunta-contract

The isolated core contract for Suunta: sans-I/O convergence planning.

`suunta-contract` computes the residual `Course` — the `Correction`s needed to
converge an observed `Fix` toward a desired `Bearing` — and makes no semantic
judgment of its own: identity (`Sigil`), relevance, and settlement are all
domain-supplied. It exposes no `async fn`, reads no ambient clock, and performs no
I/O; a runtime drives it.

This is the initial shape — the vocabulary anchors and axioms are in place; the
residual-planning types and logic follow in later spec-driven changes.

Part of [Suunta](https://github.com/tacticaldoll/suunta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-MIT), at your option.
