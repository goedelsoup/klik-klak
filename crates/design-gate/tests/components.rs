//! **Two of the four components the corpus specifies are written, and they are private methods of
//! a struct inside the instrument.**
//!
//! `engine/` holds a contract, a gate, and no code
//! ([`engine-directory`](../../../.yidam/decisions/engine-directory.yml)), and its gate pins the
//! four subsystems nothing implements. That pin is wrong about two of them.
//!
//! `crates/reachability` declares `struct Index<'a>` — not `pub` — with **fourteen methods across
//! 544 lines**, against five public items in the whole crate. `Ix::in_menu` is
//! [`verb-menu`](../../../.yidam/corpus/subsystem/verb-menu.yml): *"given a target and a state,
//! the verbs to display and no others"*, and that node cites the function by name. `live`,
//! `offered`, `apply` and `settle` are
//! [`interaction-resolver`](../../../.yidam/corpus/subsystem/interaction-resolver.yml).
//!
//! **The corpus has been citing them from inside the instrument's test directory for four
//! phases.** `verb-menu`'s figures are `[verified]` against `crates/reachability/tests/menu.rs`,
//! and `interaction-resolver`'s arrival guarantee against `crates/reachability/tests/arrival.rs`.
//! A subsystem node whose evidence is a checker's fixture is a component that exists, in the one
//! place `.claude/CLAUDE.md` says the engine is not: *"`crates/` is not the engine… conflating
//! them is this repository's characteristic mistake."*
//!
//! **And the gate cannot admit the first module.** It asks every file under `engine/` for a
//! `realizes:` in its first ten lines, which a crate's `Cargo.toml` has nowhere to put. The rule
//! was writable while the directory was empty and is not writable once it holds a crate.
//!
//! These checks pin the defective state. They are rewritten into the rule in the same phase.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// The `impl` block's extent and the names inside it.
fn evaluator(root: &Path) -> (usize, Vec<String>, Vec<String>) {
    let src = read(root, "crates/reachability/src/lib.rs");
    let lines: Vec<&str> = src.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.starts_with("impl<'a> Index<'a> {"))
        .expect("the evaluator's impl block");
    let end = (start + 1..lines.len())
        .find(|i| lines[*i] == "}")
        .expect("its end");
    let methods = lines[start..end]
        .iter()
        .filter_map(|l| {
            l.strip_prefix("    fn ")
                .or_else(|| l.strip_prefix("    pub fn "))
        })
        .map(|l| l.split('(').next().unwrap_or(l).to_string())
        .collect();
    let public = src
        .lines()
        .filter_map(|l| {
            let rest = l
                .strip_prefix("pub struct ")
                .or_else(|| l.strip_prefix("pub enum "))
                .or_else(|| l.strip_prefix("pub fn "))?;
            Some(
                rest.split(['<', '(', ' ', '{'])
                    .next()
                    .unwrap_or(rest)
                    .trim()
                    .to_string(),
            )
        })
        .collect();
    (end - start + 1, methods, public)
}

/// **Fourteen methods, five hundred and forty-four lines, and none of it is public.**
///
/// The figure that matters is the ratio: the crate publishes a state, three report types and one
/// entry point, and keeps the entire evaluation of the language behind them.
#[test]
fn the_evaluator_is_private_and_is_most_of_the_crate() {
    let root = repo_root();
    let (lines, methods, public) = evaluator(&root);
    assert_eq!(lines, 544, "the evaluator's impl block");
    assert_eq!(
        methods,
        [
            "new",
            "settle",
            "settle_tracking",
            "initial",
            "describe",
            "holds",
            "offered",
            "in_menu",
            "observable",
            "live",
            "spends",
            "apply_without_spending",
            "can_finish",
            "apply",
        ],
        "every method of the evaluator"
    );
    assert_eq!(
        public,
        ["State", "Cause", "Stranding", "Report", "analyse"],
        "everything the crate publishes"
    );

    let src = read(&root, "crates/reachability/src/lib.rs");
    assert!(
        src.contains("\nstruct Index<'a> {") && !src.contains("\npub struct Index"),
        "and the struct itself is private"
    );
}

/// **A private method is cited by name in five committed files, and one of them is a corpus
/// node.**
///
/// `Ix::in_menu` cannot be called from outside its crate and is written into the corpus as though
/// it were a component: *"a shipped engine hand-writing `Ix::in_menu`"*. `verb-menu`'s record says
/// *"`Ix::in_menu` is those three and nothing else"*. Both subsystem nodes carry `[verified]`
/// figures against fixtures of the instrument.
///
/// A subsystem node whose evidence is a checker's fixture is a component that exists, in the one
/// place `.claude/CLAUDE.md` says the engine is not.
#[test]
fn the_corpus_names_them_and_verifies_them_against_the_instrument() {
    let root = repo_root();
    let mut citing: Vec<String> = Vec::new();
    for dir in [".yidam/corpus", ".yidam/decisions"] {
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if read(
                    &root,
                    p.strip_prefix(&root).expect("under root").to_str().unwrap(),
                )
                .contains("Ix::")
                {
                    citing.push(
                        p.strip_prefix(&root)
                            .expect("under root")
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
        }
    }
    citing.sort();
    assert_eq!(
        citing,
        [
            ".yidam/corpus/subsystem/interaction-resolver.yml",
            ".yidam/decisions/arrival-realizer.yml",
            ".yidam/decisions/two-components.yml",
            ".yidam/decisions/unread-state.yml",
            ".yidam/decisions/verb-menu.yml",
        ],
        "files naming a method that cannot be called from outside its crate"
    );

    for name in ["verb-menu", "interaction-resolver"] {
        let node = read(&root, &format!(".yidam/corpus/subsystem/{name}.yml"));
        assert!(
            node.contains("crates/reachability/tests/"),
            "{name} is verified against a fixture of the instrument"
        );
    }

    // and the gate still says nothing is built
    let gate = read(&root, "crates/design-gate/tests/engine.rs");
    assert!(
        gate.contains("fn the_corpus_specifies_four_components_and_the_engine_has_none"),
        "the pin that four are specified and none exists"
    );
    assert!(
        std::fs::read_dir(root.join("engine"))
            .expect("the engine directory")
            .flatten()
            .all(|e| e.file_name() == "README.md"),
        "and the directory holds its contract and nothing else"
    );
}

/// **The gate asks a `Cargo.toml` for a doc comment.**
///
/// `modules()` takes every file under `engine/` but the README and requires a `realizes:` in its
/// first ten lines. A crate is a manifest and its source, and a manifest has nowhere to put one —
/// so the rule that was writable over an empty directory refuses the first thing the directory is
/// for. Stated as a defect in the gate rather than worked around in the crate.
#[test]
fn the_gate_as_written_cannot_admit_a_crate() {
    let root = repo_root();
    let gate = read(&root, "crates/design-gate/tests/engine.rs");
    assert!(
        gate.contains("p != root.join(\"engine/README.md\")"),
        "every file but the contract is a module"
    );
    let modules = gate
        .split("fn modules(")
        .nth(1)
        .and_then(|t| t.split("\nfn ").next())
        .expect("the function that decides what a module is");
    assert!(
        !modules.contains("extension"),
        "and nothing in it distinguishes source from a manifest"
    );
    assert!(
        gate.contains("a file under engine/ declaring no `realizes:` in its first ten lines"),
        "so a manifest fails as an anonymous module"
    );
}
