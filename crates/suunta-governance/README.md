# suunta-governance

Executable architectural governance for the Suunta workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It
depends only on the [Tianheng](https://github.com/tacticaldoll/tianheng) composed
adopter surface to keep the workspace's architecture from drifting: normal
dependency boundaries between crates, the source-observable part of the planning
core's sans-I/O purity, workspace coverage, and the accepted constitution's
generated projection. The `SansIoPure` profile composes the clock reaction (an
inline `std::time` call ending in `now`) and the exposed-async reaction (a public
`async fn`); explicit reactions on inline `std::io`/`fs`/`net`/`process` calls
remain separate because the profile does not observe them. Macro-expanded I/O, a
clock read through a method on a value (such as `Instant::elapsed`), and a function
written to return `impl Future` are invisible to a source scan and stay
review-governed.

Suunta's active-prose presence and facade-re-exports-only reactions remain
project-specific checks in this runner. `AGENTS.suunta-law.md` is generated from
the live constitution and freshness-gated; it is context, not a second law source.

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
