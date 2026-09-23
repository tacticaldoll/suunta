# Suunta Tianheng Law Projection

This file is generated from `constitution()` in `crates/suunta-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p suunta-governance law_projection_is_fresh`.

# Constitution: suunta

## Static boundaries

### `suunta-contract` (crate)

> suunta-contract is the isolated planning core. At this shape it has no normal dependencies, and must never take one on another workspace crate or a runtime framework: its residual computation is pure.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `suunta-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: its normal dependencies are Tianheng's composed adopter surface alone, never an individual governance instrument or a workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `suunta` (crate)

> suunta is the curated published entrypoint. Its normal dependencies are suunta-contract alone, never a backend, runtime, or external framework.

- **rule**: restrict dependencies to (only: suunta-contract)
- **kind**: crate · **severity**: enforce

### `suunta-contract::crate` (module)

> suunta-contract is the sans-I/O planning core: it makes no inline `std::time` `now` call and exposes no public `async fn`; time and asynchronous driving live at the runtime edge. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, is invisible to a source scan, as is a public function written to return `impl Future`), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `suunta-contract::crate` (module)

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `suunta-contract::crate` (module)

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `suunta-contract::crate` (module)

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

### `suunta-contract::crate` (module)

> the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: suunta-contract

## Async-exposure boundaries

### `suunta-contract::crate` (semantic)

> suunta-contract is the sans-I/O planning core: it makes no inline `std::time` `now` call and exposes no public `async fn`; time and asynchronous driving live at the runtime edge. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, is invisible to a source scan, as is a public function written to return `impl Future`), so this tooth complements review rather than replacing it.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: suunta-contract
