# suunta-governance

Executable architectural governance for the Suunta workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It runs
the [Tianheng](https://github.com/tacticaldoll/tianheng) family (guibiao static
boundaries + hunyi semantic reactions) to keep the workspace's architecture from
drifting: the dependency boundaries between crates, the sans-I/O purity of the
planning core (no synchronous I/O, no ambient clock, no exposed `async fn`), and
active-prose presence.

It deliberately does **not** enforce "the core makes no semantic judgment": that
axiom has no syntactic marker, so it is not statically expressible and stays
review-governed, not a tooth here.

Run it from the workspace root:

```sh
cargo run -p suunta-governance -- check --manifest-path Cargo.toml
```

Part of [Suunta](https://github.com/tacticaldoll/suunta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/suunta/blob/main/LICENSE-MIT), at your option.
