## Context

Suunta inherits a discipline from its Tianheng lineage — sans-I/O purity, OpenSpec,
vocabulary-as-governance, least-commitment — and shares no code with it. It is
stamped from the `pacta` reference implementation's skeleton and then made its own:
copy the skeleton, replace the domain, rewrite the worldview, and stand alone. After
this change the repository owns its architecture independently.

## Goals / Non-Goals

**Goals:**
- A self-sufficient initial shape: vision, vocabulary, axioms, governance, and a
  compiling skeleton, so the core can be built from the repo alone.
- Preserve the design knowledge (decisions + open questions) in `BACKLOG.md` and the
  specs, without freezing open design into code.

**Non-Goals:**
- The residual-planning implementation (deferred, spec-driven).
- Naming any sibling product — this repo is sibling-blind. Downstream durability,
  gating, execution, and compensation are generic "consumer concerns", never named
  siblings.
- A published API; version bump; async variant.

## Decisions

- **Scaffold-first, implementation later.** The specs describe the *established*
  contract (vocabulary, axioms, the semantic bill, sans-I/O purity, dependency
  isolation) — all true now — and the residual *definition*, but not the residual
  *algorithm*, which is an open design (`BACKLOG.md`). This keeps `openspec/specs/` an
  honest record of shipped truth rather than a promise of unbuilt behavior.
- **Two crates.** `suunta-contract` (the pure core) and `suunta-governance` (the
  gate, `publish = false`). No facade crate — there is one core, not a re-export
  surface. The gate depends only on `tianheng`/`guibiao`, never on the graph it judges.
- **The semantic bill of purity is the central design commitment.** A sans-I/O pure
  core cannot make a semantic judgment, so identity (`Sigil`), relevance (coverage
  verdict), and settlement (predicate) are domain-supplied. The core computes the
  residual and records; it never compares meanings.
- **The unenforceable invariant is acknowledged, not faked.** "The core makes no
  semantic judgment" has no syntactic marker, so Tianheng cannot bite it the way it
  bites no-I/O or no-async. `quality-governance` states this honestly and keeps it
  review- and structure-governed; whether to add downstream structural contradiction
  detection is an open question, leaning "not in this pure core."
- **Governance mirrors the reference discipline** minus what does not apply: crate
  dependency boundaries, sans-I/O teeth (no I/O, no ambient clock, no async
  exposure across the whole core), coverage, and active-prose presence. No facade
  re-exports check (no facade) and no kernel-no-serde check (no serde yet).
- **Inherited hygiene from birth.** No ADRs (git-as-provenance), no OpenSpec archive
  (sync removes the change dir), single-sourced Definition of Done in `AGENTS.md`,
  crate-local READMEs with absolute LICENSE URLs.

## Risks / Trade-offs

- **Specs describe some not-yet-built contract (the residual definition).** →
  Mitigated by specing only definitional/constraint requirements that hold now (what
  a Course *is*, the purity constraints), not algorithmic behavior; the algorithm's
  requirements are added by the change that builds it.
- **The purity invariant that matters most is unenforceable.** → Acknowledged in the
  spec rather than hidden; this is the honest ceiling of the semantic bill.
