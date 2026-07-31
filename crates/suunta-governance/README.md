# suunta-governance

Executable architectural governance for the Suunta workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It
depends only on the [Tianheng](https://github.com/tacticaldoll/tianheng) composed
adopter surface to keep the workspace's architecture from drifting: dependency
boundaries between crates, the planning core's sans-I/O purity, workspace
coverage, and the accepted constitution's generated projection. The `SansIoPure`
profile composes the ambient-clock and exposed-async reactions; explicit
`std::io`/`fs`/`net`/`process` reactions remain separate because the profile does
not observe them.

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
