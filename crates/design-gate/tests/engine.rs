//! The gate for `engine/`.
//!
//! `directories.md` asks a new top-level directory for a gate, and *"if what it holds derives from
//! the corpus, the gate is the derivation."* `design/`'s derivation is `rests_on:` and
//! `derives_effects_from:`. This one is a tier down: **a module may not invent a subsystem.**
//!
//! A file under `engine/` that implements one declares `realizes: subsystem/<name>` in its first
//! ten lines, as markdown front matter or a `//!` doc comment, and the corpus must declare that
//! node. When it does not, the failure is deliberately ambiguous in the way the effect check is:
//! either the module is a component nothing has described, or the corpus is missing one the engine
//! has just discovered it needs. Both are findings; silence is not available.
//!
//! The second test pins the subsystems the corpus specifies and nothing implements. It is an
//! assertion rather than a report because a standing line naming four unbuilt components is
//! `inert-reports`'s INFO that everybody learns to scroll past — pinned, it says nothing until the
//! list moves, and then it says so loudly.
//!
//! `.yidam/decisions/engine-directory.yml`.

use design_gate::repo_root;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Every file under `engine/`, less the directory's contract.
fn modules(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.join("engine")];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                stack.push(p);
            } else if p != root.join("engine/README.md") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The subsystem a module says it implements, from its first ten lines.
///
/// Two spellings because two kinds of file will need it, and a convention with one spelling per
/// kind is the kind this repository has found nine of by measuring. Both are declared in
/// `engine/README.md` rather than left to be derived from examples.
fn realizes(src: &str) -> Option<String> {
    src.lines().take(10).find_map(|l| {
        let l = l.trim();
        let l = l.strip_prefix("//!").map(str::trim).unwrap_or(l);
        l.strip_prefix("realizes:")
            .map(str::trim)
            .and_then(|v| v.strip_prefix("subsystem/"))
            .map(|v| v.trim_end_matches(".yml").to_string())
    })
}

fn declared(root: &Path) -> BTreeSet<String> {
    std::fs::read_dir(root.join(".yidam/corpus/subsystem"))
        .expect("the subsystem directory")
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            (p.extension()? == "yml").then(|| p.file_stem()?.to_str().map(str::to_string))?
        })
        .collect()
}

/// **Every module names a subsystem the corpus declares.**
///
/// The whole of the derivation, and it is one rule because there is one thing an engine can get
/// structurally wrong that a compiler will not catch: build a component nothing described. The
/// corpus is where components are described, and this is the edge between them.
#[test]
fn every_module_names_a_subsystem_the_corpus_declares() {
    let root = repo_root();
    let known = declared(&root);
    let mut bad = Vec::new();
    let mut anonymous = Vec::new();
    for p in modules(&root) {
        let rel = p
            .strip_prefix(&root)
            .unwrap_or(&p)
            .to_string_lossy()
            .into_owned();
        let Ok(src) = std::fs::read_to_string(&p) else {
            continue;
        };
        match realizes(&src) {
            Some(name) if known.contains(&name) => {}
            Some(name) => bad.push(format!(
                "{rel} realizes subsystem/{name} — the corpus declares no such node. Either the \
                 component has not been described, or the corpus is missing one the engine needs"
            )),
            None => anonymous.push(rel),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n  "));
    assert!(
        anonymous.is_empty(),
        "a file under engine/ declaring no `realizes:` in its first ten lines:\n  {}",
        anonymous.join("\n  ")
    );
}

/// **Four subsystems are specified and nothing implements one.**
///
/// The engine's roadmap, derived from the corpus rather than written beside it — and pinned, so it
/// changes only when somebody builds a component or the corpus names one. Both of those should
/// come with a record; neither should arrive silently.
///
/// `response-presenter` joined the list one commit before this check existed, which is the shape
/// worth noticing: the corpus finished specifying the tier and the directory to build it in was
/// created in the same phase, rather than the tier being completed to justify the directory.
#[test]
fn the_corpus_specifies_four_components_and_the_engine_has_none() {
    let root = repo_root();
    assert_eq!(
        declared(&root).into_iter().collect::<Vec<_>>(),
        [
            "interaction-resolver",
            "path-solver",
            "response-presenter",
            "verb-menu",
        ],
        "the subsystems this corpus declares"
    );

    let built: Vec<String> = modules(&root)
        .iter()
        .filter_map(|p| realizes(&std::fs::read_to_string(p).ok()?))
        .collect();
    assert!(
        built.is_empty(),
        "a component exists now, so this pin and `engine/README.md`'s table both owe an edit: {built:?}"
    );
}

/// **The directory states its contract, and the contract says what the gate cannot see.**
///
/// `design/README.md` carries the same section for the same reason, and both were written because
/// this repository has twice overturned a conclusion by re-reading material it already held. A
/// gate's limits recorded beside the gate are the only ones anybody meets.
#[test]
fn the_contract_states_what_this_check_does_not_catch() {
    let root = repo_root();
    let readme = std::fs::read_to_string(root.join("engine/README.md")).expect("the contract");
    for phrase in [
        "It holds material that makes no claims",
        "may not invent a subsystem",
        "## What this gate does not catch",
        "It checks a name, not a component",
    ] {
        assert!(
            readme.contains(phrase),
            "engine/README.md no longer says {phrase:?}"
        );
    }
    assert!(
        readme.contains("realizes: subsystem/"),
        "and shows the declaration a module owes, in both spellings the gate reads"
    );
}
