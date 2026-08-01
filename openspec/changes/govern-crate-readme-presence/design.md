## Context

`suunta-governance`'s `check_active_prose` function already walks a fixed list of
repo-relative paths (`ACTIVE_PROSE_FILES`) and fails the gate if any is missing or
unreadable, plus scans each for `STALE_PHRASES` (currently empty). This change adds
three more paths to that list. The mechanism, its failure behavior, and its
stale-phrase scan are unchanged; only the governed file set grows.

This document is intentionally minimal: the change is a one-constant edit with no
cross-cutting surface, no new dependency, and no ambiguity to resolve before coding.

## Goals / Non-Goals

**Goals:**
- Close the presence/readability gap for the three crate-level READMEs, mirroring
  the guarantee the six root docs already have.

**Non-Goals:**
- Detecting *stale content* in any governed file (version markers, superseded
  disposition language). That failure mode was identified separately and stays
  review-governed prose per `BACKLOG.md` — it does not pass the four-gate test in
  `AGENTS.md` for becoming a Tianheng tooth (not a faithful, non-gameable proxy; a
  version-number check would also be an unstable, per-release moving target).
- Adding any `STALE_PHRASES` entries.
- Changing dependency boundaries, sans-I/O checks, or the facade re-export scan.

## Decisions

- **Extend `ACTIVE_PROSE_FILES`, not a new check.** The existing function already
  does exactly what's needed (presence + readability); a new mechanism would
  duplicate it. Reusing it also means the new paths get the stale-phrase scan for
  free if entries are ever added there.
- **No new `STALE_PHRASES` entries.** The specific stale wording found in
  `crates/suunta-contract/README.md` (fixed separately, outside this change) is not
  a safe global ban — similar phrasing can appear in legitimate prose elsewhere
  (e.g. `BACKLOG.md` discusses "deferred" as a concept). Banning it here would risk
  false positives rather than a faithful signal.

## Risks / Trade-offs

- [Risk] A future crate README rename or relocation silently drops out of coverage
  until someone updates `ACTIVE_PROSE_FILES` by hand. → Mitigation: none needed
  beyond review; this is the same trade-off the six existing root docs already
  accept, and crate READMEs change path even less often than those.
- [Risk] This only catches a file vanishing, not its content going stale — someone
  could read "governed" and assume more coverage than exists. → Mitigation: the
  proposal and this design both state the narrower scope explicitly; no wording in
  the shipped spec should imply content freshness is checked.
