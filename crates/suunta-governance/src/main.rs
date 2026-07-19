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
const PROSE_REASON: &str =
    "active prose must be present and must not reintroduce stale architecture-defining vocabulary";

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
        workspace.write_root_manifest_members(&["suunta-contract", "suunta-governance", "tokio"]);

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
        workspace.write_root_manifest_members(&["suunta-contract", "suunta-governance"]);

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
    fn current_active_prose_satisfies_governance() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_active_prose(&root), Ok(()));
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
