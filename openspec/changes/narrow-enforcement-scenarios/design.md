## Context

The accepted boundary reasons were narrowed to what each reaction observes, per the
Tianheng 0.6.1 observation perimeter. The prose around the gate was not. This change
aligns that prose with the reasons.

## Goals / Non-Goals

**Goals:**

- Each sentence that says what the gate, a tooth, or a check enforces or fails on names
  only shapes the reaction observes.
- The unobserved remainder is named as review-governed, not dropped.

**Non-Goals:**

- Changing intent prose about what the core does ("reads no ambient clock"). Judgment lives
  in prose and may be broader than its tooth. The convergence-contract "Sans-I/O Purity"
  requirement and the axiom lists in `AGENTS.md`, `PROJECT.md`, and the crate READMEs stay.
- Changing any boundary, reason, test, or the generated law projection.

## Decisions

- **Keep the intent sentence and add the enforcement scope.** The sans-I/O requirement keeps
  "`suunta-contract` SHALL call no `std::io`/`fs`/`net`/`process`, read no ambient clock, and
  expose no `async fn`" as intent. A separate sentence says which shapes the constitution
  bites. Deleting the intent to make the sentence true was rejected.
- **Rename the requirement.** "Is Enforced" in the title is itself the overclaim. "Has
  Static Teeth" matches the family's tooth vocabulary. No other file references the old
  name.
- **The facade re-exports-only requirement stays.** It already names the brace-depth line
  heuristic and the rustfmt backstop for its one gap.
- **No new scenario for the unobserved shapes.** No test pins "the gate stays silent on a
  receiver-method clock read", so the requirement text records the limit instead of a
  scenario nothing checks.

## Risks / Trade-offs

- [Prose and reasons can drift apart again] → The wording follows the accepted reasons
  closely, so a later reason amendment shows which sentences to revisit.
