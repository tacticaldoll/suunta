//! Executable architectural governance for the suunta workspace.
//!
//! It enforces dependency boundaries, the core's sans-I/O purity (no I/O, no ambient
//! clock, no exposed `async fn`), workspace coverage, and active-prose presence. The
//! axiom that the core makes no semantic judgment has no syntactic marker: it is not
//! statically expressible and stays review-governed, not a tooth here.

#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "suunta-contract is the isolated planning core. At this shape it depends on nothing, and must never depend on another workspace crate or a runtime framework: its residual computation is pure.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace graph it judges: it may depend only on governance-family tooling (tianheng and its guibiao coverage core), never on a workspace crate under judgment.";
const CORE_NO_IO_REASON: &str = "the sans-I/O planning core performs no I/O: no code in suunta-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.";
const AMBIENT_TIME_REASON: &str = "the planning core must read no ambient clock; time is injected at the runtime edge, never read inside suunta-contract.";
const CORE_ASYNC_REASON: &str = "the sans-I/O planning core must stay runtime-agnostic: its public API must never expose an async fn, so no runtime shape leaks into the contract.";
const FACADE_REASON: &str = "suunta is the curated published entrypoint. It may depend only on suunta-contract, never on a backend, runtime, or external framework.";
const FACADE_REEXPORT_REASON: &str =
    "the suunta facade must stay a pure re-export entrypoint and hold no logic of its own";
const FACADE_NON_REEXPORT: &str = "non-re-export item in facade library";
const PROSE_REASON: &str =
    "active prose must be present and must not reintroduce stale architecture-defining vocabulary";

/// The facade source tree the re-exports-only scan guards, relative to the workspace root.
const FACADE_SOURCE_DIR: &str = "crates/suunta/src";

const ACTIVE_PROSE_FILES: &[&str] = &[
    "AGENTS.md",
    "PROJECT.md",
    "README.md",
    "BACKLOG.md",
    "docs/development-flow.md",
    "docs/domain-language.md",
];

// No legacy vocabulary exists to guard against at this shape — Suunta is new, with no
// prior architecture to regress toward. The hook below is ready: add entries as real
// drift risks emerge (a term that means the core does what it must not). Entries must
// be phrases that never appear in legitimate prose (including non-goals), so they flag
// drift rather than false-positive on a "Suunta is not a ..." sentence.
const STALE_PHRASES: &[StalePhrase] = &[];

#[derive(Debug, Clone, Copy)]
struct StalePhrase {
    phrase: &'static str,
    reason: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
struct ProseViolation {
    path: String,
    line: usize,
    phrase: &'static str,
    reason: &'static str,
}

fn constitution() -> Constitution {
    Constitution::new("suunta")
        .boundary(
            CrateBoundary::crate_("suunta-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("suunta-governance")
                .restrict_dependencies_to(["tianheng", "guibiao"])
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("suunta")
                .restrict_dependencies_to(["suunta-contract"])
                .because(FACADE_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("suunta-contract")
                .module("crate")
                .must_not_call_inline("std::time")
                .ending_with(["now"])
                .because(AMBIENT_TIME_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("suunta-contract")
                .module("crate")
                .must_not_call_inline("std::io")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("suunta-contract")
                .module("crate")
                .must_not_call_inline("std::fs")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("suunta-contract")
                .module("crate")
                .must_not_call_inline("std::net")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("suunta-contract")
                .module("crate")
                .must_not_call_inline("std::process")
                .because(CORE_NO_IO_REASON),
        )
        .async_exposure_boundary(
            AsyncExposureBoundary::in_crate("suunta-contract")
                .module("crate")
                .must_not_expose_async_fn()
                .including_submodules()
                .because(CORE_ASYNC_REASON),
        )
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();

    if should_check_prose(&args) {
        let manifest = manifest_path_from_args(&args);
        let root = manifest
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        if let Err(violations) = check_active_prose(&root) {
            eprintln!("suunta prose governance failed: {PROSE_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}` - {}",
                    violation.path, violation.line, violation.phrase, violation.reason
                );
            }
            return ExitCode::from(1);
        }

        if let Err(violations) = check_facade_reexports_only(&root) {
            eprintln!("suunta facade governance failed: {FACADE_REEXPORT_REASON}");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}`",
                    violation.path, violation.line, violation.marker
                );
            }
            return ExitCode::from(1);
        }
    }

    tianheng::run(&constitution(), args)
}

fn should_check_prose(args: &[String]) -> bool {
    args.iter().skip(1).any(|arg| arg == "check")
}

fn manifest_path_from_args(args: &[String]) -> PathBuf {
    for index in 0..args.len() {
        if args[index] == "--manifest-path"
            && let Some(path) = args.get(index + 1)
        {
            return PathBuf::from(path);
        }

        if let Some(path) = args[index].strip_prefix("--manifest-path=") {
            return PathBuf::from(path);
        }
    }

    PathBuf::from("Cargo.toml")
}

fn check_active_prose(root: &Path) -> Result<(), Vec<ProseViolation>> {
    let mut violations = Vec::new();

    for relative in ACTIVE_PROSE_FILES {
        let path = root.join(relative);
        let Ok(content) = fs::read_to_string(&path) else {
            // A canonical governed file that cannot be read must fail the gate, not be
            // silently skipped — otherwise a governed doc that vanishes grants a free
            // pass. Fail loudly, naming the file.
            violations.push(ProseViolation {
                path: String::from(*relative),
                line: 0,
                phrase: "<unreadable>",
                reason: "a governed active-prose file must be present and readable",
            });
            continue;
        };

        violations.extend(check_prose_content(relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn check_prose_content(path: &str, content: &str) -> Vec<ProseViolation> {
    let mut violations = Vec::new();

    for (index, line) in content.lines().enumerate() {
        for rule in STALE_PHRASES {
            if line.contains(rule.phrase) {
                violations.push(ProseViolation {
                    path: path.to_owned(),
                    line: index + 1,
                    phrase: rule.phrase,
                    reason: rule.reason,
                });
            }
        }
    }

    violations
}

#[derive(Debug, PartialEq, Eq)]
struct SourceViolation {
    path: String,
    line: usize,
    marker: &'static str,
}

fn check_facade_reexports_only(root: &Path) -> Result<(), Vec<SourceViolation>> {
    let mut violations = Vec::new();
    let files = collect_rs_files(&root.join(FACADE_SOURCE_DIR));

    // No facade source found at all (missing or empty source tree) is a vacuous
    // pass — mirror the coverage check's non-vacuous guard and fail. Keyed on files
    // *found*, not files *read*, so a present-but-unreadable file reports as
    // unreadable below rather than as an empty tree.
    if files.is_empty() {
        violations.push(SourceViolation {
            path: FACADE_SOURCE_DIR.to_owned(),
            line: 0,
            marker: "no facade source files found",
        });
    }

    for file in files {
        let relative = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .to_string_lossy()
            .into_owned();
        let Ok(content) = fs::read_to_string(&file) else {
            // An unreadable facade source file must fail the gate, not be skipped —
            // a file the scan cannot read cannot be certified re-exports-only.
            violations.push(SourceViolation {
                path: relative,
                line: 0,
                marker: "unreadable facade source",
            });
            continue;
        };
        violations.extend(check_facade_content(&relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// A brace-depth-aware line scan: at brace depth zero, the facade library may hold
/// only re-exports, `use` imports, attributes, and comments. Any other item
/// (a `fn`, `struct`, `impl`, `const`, ...) is logic the facade must not carry. It
/// is deliberately a line scan, not a parser: `suunta-governance` may depend only on
/// `tianheng`, so it cannot pull in `syn`. A logic item co-located on a `pub use`
/// line (`pub use X; pub const Y = 1;`) escapes this line heuristic, but the DoD
/// `cargo fmt --all --check` gate splits it onto its own line, where this scan then
/// catches it.
fn check_facade_content(path: &str, content: &str) -> Vec<SourceViolation> {
    let mut violations = Vec::new();
    let mut depth: i32 = 0;

    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let is_comment = trimmed.starts_with("//");

        // A line inside a multi-line `pub use { ... }` block is a re-export
        // continuation; only judge lines that start a fresh item at depth zero.
        if depth == 0
            && !trimmed.is_empty()
            && !is_comment
            && !trimmed.starts_with('#')
            && !trimmed.starts_with("pub use ")
            && !trimmed.starts_with("use ")
        {
            violations.push(SourceViolation {
                path: path.to_owned(),
                line: index + 1,
                marker: FACADE_NON_REEXPORT,
            });
        }

        // Track brace depth off code lines only, so a brace inside a doc comment
        // does not desynchronize the scan.
        if !is_comment {
            depth += line.matches('{').count() as i32;
            depth -= line.matches('}').count() as i32;
            if depth < 0 {
                depth = 0;
            }
        }
    }

    violations
}

fn collect_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_rs_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_workspace_satisfies_constitution() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");

        assert_eq!(
            check(constitution().static_boundaries(), &manifest),
            Outcome::Clean
        );
    }

    #[test]
    fn unapproved_core_dependency_is_rejected() {
        let workspace = TempWorkspace::new("suunta-governance-forbidden-dependency");
        workspace.write_package("tokio", "");
        workspace.write_package(
            "suunta-contract",
            r#"
[dependencies]
tokio = { path = "../tokio" }
"#,
        );
        workspace.write_package("suunta-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&[
            "suunta",
            "suunta-contract",
            "suunta-governance",
            "tokio",
        ]);

        let outcome = check(
            constitution().static_boundaries(),
            &workspace.path.join("Cargo.toml"),
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected an unapproved dependency violation, got {outcome:?}");
        };
        assert!(report.violations.iter().any(|violation| {
            let id = violation.id();
            id.target == "suunta-contract"
                && id.rule == "restrict dependencies to"
                && id.finding == "tokio"
        }));
    }

    #[test]
    fn unapproved_facade_dependency_is_rejected() {
        // The facade boundary must bite: a `suunta` that depends on anything other
        // than `suunta-contract` fails the gate.
        let workspace = TempWorkspace::new("suunta-governance-facade-dependency");
        workspace.write_package("tokio", "");
        workspace.write_package("suunta-contract", "");
        workspace.write_package("suunta-governance", "");
        workspace.write_package(
            "suunta",
            r#"
[dependencies]
suunta-contract = { path = "../suunta-contract" }
tokio = { path = "../tokio" }
"#,
        );
        workspace.write_root_manifest_members(&[
            "suunta",
            "suunta-contract",
            "suunta-governance",
            "tokio",
        ]);

        let outcome = check(
            constitution().static_boundaries(),
            &workspace.path.join("Cargo.toml"),
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected an unapproved facade dependency violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "suunta"
                    && id.rule == "restrict dependencies to"
                    && id.finding == "tokio"
            }),
            "expected the facade dependency boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_io_call_is_rejected() {
        // Prove the sans-I/O tooth bites, not just the async one: a std::fs call in
        // the core must fire the no-I/O ModuleBoundary. In this single-crate fixture no
        // other boundary can fire, so any violation is this tooth.
        let workspace = TempWorkspace::new("suunta-governance-core-io-leak");
        workspace.write_package_with_source(
            "suunta-contract",
            "",
            "pub fn leak() -> bool {\n    std::fs::metadata(\"x\").is_ok()\n}\n",
        );
        workspace.write_package("suunta-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&["suunta", "suunta-contract", "suunta-governance"]);

        let outcome = check(
            constitution().static_boundaries(),
            &workspace.path.join("Cargo.toml"),
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a no-I/O violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "std::fs" && id.rule == "inline symbol path confined to module"
            }),
            "expected the core no-I/O boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_ambient_clock_call_is_rejected() {
        // Prove the ambient-clock tooth bites (the `.ending_with(["now"])` matcher), a
        // distinct mechanism from the plain no-I/O paths. In this single-crate fixture no
        // other boundary can fire, so any violation is this tooth.
        let workspace = TempWorkspace::new("suunta-governance-core-clock-leak");
        workspace.write_package_with_source(
            "suunta-contract",
            "",
            "pub fn leak() -> std::time::SystemTime {\n    std::time::SystemTime::now()\n}\n",
        );
        workspace.write_package("suunta-governance", "");
        workspace.write_facade();
        workspace.write_root_manifest_members(&["suunta", "suunta-contract", "suunta-governance"]);

        let outcome = check(
            constitution().static_boundaries(),
            &workspace.path.join("Cargo.toml"),
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected an ambient-clock violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "std::time" && id.rule == "inline symbol path confined to module"
            }),
            "expected the core ambient-clock boundary to fire: {report:?}"
        );
    }

    #[test]
    fn current_active_prose_satisfies_governance() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_active_prose(&root), Ok(()));
    }

    #[test]
    fn current_facade_is_reexports_only() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_facade_reexports_only(&root), Ok(()));
    }

    #[test]
    fn facade_reexports_and_comments_are_allowed() {
        let content = "\
//! Facade docs.
#![forbid(unsafe_code)]

pub use suunta_contract::{Bearing, Correction};
pub use suunta_contract::{
    Residual,
    Sigil,
};
";
        assert!(check_facade_content("lib.rs", content).is_empty());
    }

    #[test]
    fn facade_logic_item_is_rejected() {
        assert_eq!(
            check_facade_content("lib.rs", "pub fn helper() {}\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
        // A struct declaration inside the facade is logic, not a re-export.
        assert_eq!(
            check_facade_content("lib.rs", "struct Sneaky;\n"),
            vec![SourceViolation {
                path: "lib.rs".to_owned(),
                line: 1,
                marker: FACADE_NON_REEXPORT,
            }]
        );
    }

    #[test]
    fn empty_facade_source_tree_fails_loudly() {
        // A root with no facade source tree scans zero files; the non-vacuous guard
        // must convert that into a failure rather than an empty (clean) pass.
        let workspace = TempWorkspace::new("suunta-governance-empty-facade");

        let Err(violations) = check_facade_reexports_only(&workspace.path) else {
            panic!("a root with no facade source must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.marker == "no facade source files found"),
            "expected a no-facade-source violation: {violations:?}"
        );
    }

    #[test]
    fn missing_active_prose_file_fails_loudly() {
        // A root with none of the canonical governed prose files must fail the gate,
        // not pass vacuously by skipping every unreadable file.
        let workspace = TempWorkspace::new("suunta-governance-missing-prose");

        let Err(violations) = check_active_prose(&workspace.path) else {
            panic!("a root missing every governed prose file must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.phrase == "<unreadable>"),
            "expected an unreadable-file violation naming a governed file: {violations:?}"
        );
    }

    #[test]
    fn every_workspace_crate_is_covered() {
        // Tianheng coverage is advisory and never fails CI, so assert completeness here
        // through the native projection. `check_and_cover` takes the static (guibiao)
        // constitution and reads real `cargo metadata`.
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");
        let (_outcome, coverage) =
            guibiao::check_and_cover(constitution().static_boundaries(), &manifest);
        let coverage = coverage.expect("workspace metadata should be readable in-repo");
        assert!(
            coverage.total > 0,
            "coverage read no crates — the gate would pass vacuously"
        );
        assert!(
            coverage.uncovered.is_empty(),
            "every workspace crate must have a dependency boundary; ungoverned: {:?}",
            coverage.uncovered
        );
    }

    #[test]
    fn core_async_exposure_reaction_fires() {
        let outcome = semantic_reaction_outcome(
            "suunta-governance-core-async-leak",
            "pub async fn leak() {}\n",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a core async-exposure violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.target == "crate" && id.rule == "must not expose async fn"
            }),
            "expected the core async-exposure boundary to fire: {report:?}"
        );
    }

    #[test]
    fn core_async_in_submodule_is_rejected() {
        // Prove `.including_submodules()` actually recurses: an async fn nested in a
        // submodule must fire too, not just one at the crate root.
        let outcome = semantic_reaction_outcome(
            "suunta-governance-core-async-submodule-leak",
            "pub mod inner {\n    pub async fn leak() {}\n}\n",
        );

        let Outcome::Violations(report) = outcome else {
            panic!("expected a submodule async-exposure violation, got {outcome:?}");
        };
        assert!(
            report.violations.iter().any(|violation| {
                let id = violation.id();
                id.rule == "must not expose async fn"
            }),
            "expected the async-exposure boundary to fire inside a submodule: {report:?}"
        );
    }

    #[test]
    fn semantic_reactions_stay_clean_without_a_leak() {
        // Precision: the same crate shape without the leak must be clean, so the firing
        // test proves a reacting boundary, not one that always fires.
        let outcome =
            semantic_reaction_outcome("suunta-governance-semantic-clean", "pub fn plan() {}\n");

        assert_eq!(
            outcome,
            Outcome::Clean,
            "a core with no async exposure must raise no semantic violation"
        );
    }

    /// Build a minimal one-crate workspace (`suunta-contract`, the semantic boundary's
    /// target) and run the semantic bundle against it. The crate is always present: a
    /// missing target makes `check_all` return `Outcome::ConstitutionError`, not a
    /// silent skip, so a firing fixture differs from a clean one only in the leak.
    fn semantic_reaction_outcome(name: &str, contract_source: &str) -> Outcome {
        let workspace = TempWorkspace::new(name);
        workspace.write_package_with_source("suunta-contract", "", contract_source);
        workspace.write_root_manifest_members(&["suunta-contract"]);

        tianheng::check_all(
            constitution().semantic_boundaries(),
            &workspace.path.join("Cargo.toml"),
        )
    }

    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("stale temporary workspace should be removable");
            }
            fs::create_dir_all(&path).expect("temporary workspace should be creatable");
            Self { path }
        }

        fn write_root_manifest_members(&self, members: &[&str]) {
            let entries = members
                .iter()
                .map(|member| format!("    \"{member}\","))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(
                self.path.join("Cargo.toml"),
                format!(
                    r#"
[workspace]
resolver = "2"
members = [
{entries}
]
"#
                ),
            )
            .expect("workspace manifest should be writable");
        }

        fn write_package(&self, name: &str, dependencies: &str) {
            self.write_package_with_source(name, dependencies, "");
        }

        /// Write a `suunta` facade package depending only on `suunta-contract`, so the
        /// facade `CrateBoundary` has a real target in a fixture workspace.
        fn write_facade(&self) {
            self.write_package(
                "suunta",
                "[dependencies]\nsuunta-contract = { path = \"../suunta-contract\" }\n",
            );
        }

        fn write_package_with_source(&self, name: &str, dependencies: &str, source: &str) {
            let package = self.path.join(name);
            fs::create_dir_all(package.join("src")).expect("package source dir should be writable");
            fs::write(
                package.join("Cargo.toml"),
                format!(
                    r#"
[package]
name = "{name}"
version = "0.1.0"
edition = "2024"
{dependencies}
"#
                ),
            )
            .expect("package manifest should be writable");
            fs::write(package.join("src/lib.rs"), source)
                .expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
