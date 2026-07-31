# AGENTS.md

Meta-guideline for AI coding agents working in this repository. Read this first,
then let `openspec/specs/` and active change specs be the source of durable
architecture truth.

## Suunta In One Sentence

Suunta is a thin, sans-I/O convergence-planning core: given a desired `Bearing`
and the domain's certified satisfaction of each target (the `Fix`), it computes the
residual `Course` (the `Correction`s that remain) by filtering the `Bearing` to what
the domain has not certified done, while making no semantic judgment of its own.

This repository is intentionally narrow. Suunta is not a workflow engine, a
scheduler, or a durable-execution runtime. Durability, gating, and compensation are
downstream consumer concerns, not the identity of the core.

## Architectural Axioms

Before proposing or writing code, protect these axioms:

1. **Planning core stays thin**: `suunta-contract` owns the residual computation
   and the `Course`/`Correction` vocabulary. It does not own execution, durability,
   scheduling, gating, or compensation.
2. **No semantic judgment in the core**: semantic identity (`Sigil`), target
   satisfaction (the `Fix` — whether reality meets a `Bearing` target), relevance (a
   coverage verdict), and settlement predicates are domain-supplied. The core
   computes the residual and records; it never compares meanings. This is the
   *semantic bill of purity* (four faces) — its cost (silent failure on domain
   semantic error) is accepted deliberately, not patched by pulling judgment into the
   core.
3. **Sans-I/O purity**: the core exposes no `async fn`, reads no ambient clock, and
   performs no I/O. A runtime drives it and injects time at the edge.
4. **Vocabulary is governance**: names such as `Sounding`, `Fix`, `Bearing`,
   `Course`, `Correction`, `Sigil`, and `Drift` protect the navigation worldview.

## Lineage

```text
   tianheng  +  〔sans-I/O · OpenSpec · vocabulary-as-governance · least-commitment〕
                    │  inherited discipline — provenance, not coupling
                    ▼
             ●  suunta

   siblings: ▢ ▢ ▢   intentionally blank — this repo is sibling-blind. Which
                     products compose together is a consumer app's knowledge, never
                     a component's; naming a sibling here would leak that knowledge
                     and rot when the roster changes.
   note: skeleton copied from the pacta reference implementation.
```

Suunta shares a **discipline** with its lineage, not code: its own crates, specs,
constitution, and release cadence. It does not import, track, or depend on any
sibling product.

## Document Authority

- `openspec/specs/` is shipped architecture truth.
- `openspec/changes/` contains active proposed truth until it is synced.
- `PROJECT.md` states product vision, positioning, and non-goals.
- `docs/domain-language.md` is the canonical vocabulary.
- `BACKLOG.md` records deferred decisions, open design questions, and candidate
  patterns, not mandatory phases.
- `AGENTS.md` is operating protocol for agents and contributors.

Decision provenance lives in git — the commit body and pull request that made a
change record its rationale. Forward-looking or reversed decisions are noted in
`BACKLOG.md`. There is no separate architecture-decision-record file class; the
living documents above are the single source of truth for current state, and git is
the source of truth for why it changed.

If these documents conflict, fix the conflict through an OpenSpec change before
implementing feature code.

## Adversarial Review Stance

Every change passes an adversarial review at BOTH the propose and apply phases
before it is committed. Actively challenge the design:

- **Propose phase**: Does the change make the planning core heavier than the
  residual mechanism requires? Does it smuggle a semantic judgment (identity,
  relevance, settlement) into the core instead of the domain? Does it treat a
  downstream concern (durability, gating, execution, compensation) as core identity?
- **Apply phase**: Does the implementation leak I/O, async, an ambient clock, or a
  semantic comparison into the core? Does Tianheng still bite the boundary that the
  prose claims?

Reject or redesign changes that pull Suunta toward an orchestration monolith.

## OpenSpec Workflow

This repository uses OpenSpec. The lifecycle is:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: investigate and shape intent. Do not write feature code outside a
   change.
2. **Propose**: create `proposal.md`, `design.md`, `tasks.md`, and delta specs.
   Commit as `docs(<change>): propose <summary>`.
3. **Apply**: implement against the active delta specs. Check off tasks only after
   verification. Commit coherent compiling milestones as `feat(...)` or `fix(...)`.
4. **Sync**: merge verified delta specs into `openspec/specs/` (agent-driven), then
   remove the completed change directory; its content now lives in `openspec/specs/`
   and git history. There is no archive. Commit as `docs(specs): sync <change>`.

## Commit And Integration Governance

### Branch Commits

- Use Conventional Commits: `type(scope): summary`.
- Write the subject in English, lowercase imperative mood, at no more than 72 characters.
- Use the body to record motivation, important decisions, constraints, and verification when that context exists.
- Do not append pull request or issue numbers to the subject or body.
- Development branches may contain multiple coherent commits because the pull request is squash-merged.

### Pull Requests

- Branch from `main` and open every change directly against `main`.
- Make the pull request title the intended squash commit subject.
- Give every pull request a non-empty body that explains why the change is needed, what changed, consequential decisions or tradeoffs, and verification.
- Rebase the branch onto the current `main` before final verification.
- Do not introduce a release integration branch between a change and `main`.

### Squash Merges

- Squash-merge every verified pull request into `main`.
- Make the squash commit subject exactly the approved pull request title.
- Give every squash commit a non-empty body distilled from the approved pull request body.
- Do not append a pull request number, issue number, or URL to the squash subject or body.
- Every content-changing commit on `main`, including release preparation, must come from a squash-merged pull request.
- Keep `main` releasable after every merge.

### Attribution

- Do not include AI, agent, model, tool, automation, or generation attribution in commits, pull requests, tags, changelogs, or release notes.
- A `Co-authored-by` trailer is allowed only for a real human contributor.

### Release Finalization

- Prepare release content in a pull request whose squash subject is exactly `chore(release): prepare X.Y.Z`.
- Give the release preparation squash commit a non-empty body describing scope, compatibility, metadata changes, and verification.
- Run the complete Definition of Done after that commit reaches `main`.
- Finalize with annotated tag `vX.Y.Z` on that commit, with message exactly `release: X.Y.Z`.
- Push the tag without another commit. Release branches and empty release commits are not part of the flow.

## Definition Of Done

Run these from the workspace root before checking off implementation tasks or
syncing specs. This is the single source for the gate list — `README.md` and
`docs/development-flow.md` point here rather than restating it.

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo run -p suunta-governance -- check --manifest-path Cargo.toml
```

CI runs the same gates on push and pull request, and additionally verifies the
declared MSRV builds (`cargo +1.88 build --workspace`). Rust style lives in these
checks: rustfmt formats, clippy denies warnings, rustdoc denies documentation
warnings, cargo-deny owns resolved supply-chain policy, and `suunta-governance` owns
Tianheng architecture boundaries.
