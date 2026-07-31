# Suunta Tianheng Law Projection

This file is generated from `constitution()` in `crates/suunta-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p suunta-governance law_projection_is_fresh`.

# Constitution: suunta

## Static boundaries

### `suunta-contract`

> suunta-contract is the isolated planning core. At this shape it depends on nothing, and must never depend on another workspace crate or a runtime framework: its residual computation is pure.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `suunta-governance`

> the governance gate must stay independent of the workspace graph it judges: it may depend only on Tianheng's composed adopter surface, never on an individual governance instrument or a workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `suunta`

> suunta is the curated published entrypoint. It may depend only on suunta-contract, never on a backend, runtime, or external framework.

- **rule**: restrict dependencies to (only: suunta-contract)
- **kind**: crate · **severity**: enforce

### `crate`

> suunta-contract is the sans-I/O planning core: it reads no ambient clock and exposes no async function; time and asynchronous driving live at the runtime edge.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `crate`

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `crate`

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `crate`

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `crate`

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

## Async-exposure boundaries

### `crate`

> suunta-contract is the sans-I/O planning core: it reads no ambient clock and exposes no async function; time and asynchronous driving live at the runtime edge.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: suunta-contract
