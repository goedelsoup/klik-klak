//! **One `depends-on` edge, and the build takes it.**
//!
//! This was a census one commit ago: `interaction-resolver` declared three and
//! `engine/interaction-resolver/Cargo.toml` took one. `depends-on` is defined in
//! `.yidam/corpus/subsystem.ont.yml` as *"another component this one requires in order to do its
//! own work"* — a claim about a built system, in a class that otherwise works hard to avoid one,
//! since a `responsibility` is *"stated without reference to any other component."*
//!
//! **The two that went failed differently and only one of them was about the engine.**
//!
//! `response-presenter` was a **misfiling**. The resolver's own responsibility is *"produce exactly
//! one outcome"*, not deliver one; the thing that needs a presenter is whatever runs the loop, and
//! `.yidam/decisions/playable.yml` put clients outside this corpus's subject. The fact had nowhere
//! to live and was written at the nearest node that would hold it.
//!
//! `path-solver` was an **imported premise**. Its node argued the coupling *"comes from the
//! vocabulary rather than from any implementation choice"* — and it does, from the vocabulary a
//! point-and-click engine normally has. `.yidam/decisions/drawn-boundary.yml` keeps only the half
//! an author types, so a `walk-to` writes `move <room>`, arrival **is** the room, and nothing waits.
//!
//! **The rule that catches the next one lives in `engine.rs`**, where the cargo→corpus direction
//! already did: a `depends-on` between two built components must be a cargo dependency. It could
//! not be written until every component named existed —
//! `.yidam/decisions/unbuilt-solver.yml` built the last — and the day it could, two of three edges
//! turned out to be wrong.
//!
//! What is held here is the other half: **the corpus says why each one went**, so a phase redrawing
//! one meets the argument rather than an empty slot. `.yidam/decisions/declared-edge.yml`.

use design_gate::repo_root;
use std::path::Path;

/// Every `depends-on` edge the corpus declares between subsystems.
fn declared(root: &Path) -> Vec<(String, String)> {
    let dir = root.join(".yidam/corpus/subsystem");
    let mut out = Vec::new();
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .expect("the subsystem directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yml"))
        .collect();
    files.sort();
    for p in files {
        let from = p
            .file_stem()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        let src = std::fs::read_to_string(&p).expect("readable");
        let Some(links) = src.split("\nlinks:").nth(1) else {
            continue;
        };
        let mut target: Option<String> = None;
        for line in links.lines() {
            let l = line.trim();
            if let Some(t) = l.strip_prefix("- target: ../subsystem/") {
                target = Some(t.trim_end_matches(".yml").to_string());
            } else if l == "relationship: depends-on" {
                if let Some(t) = target.take() {
                    out.push((from.clone(), t));
                }
            }
        }
    }
    out
}

/// Every cargo dependency from one engine crate to another.
fn wired(root: &Path) -> Vec<(String, String)> {
    let names = [
        "interaction-resolver",
        "path-solver",
        "response-presenter",
        "verb-menu",
    ];
    let mut out = Vec::new();
    for from in names {
        let Ok(src) = std::fs::read_to_string(root.join(format!("engine/{from}/Cargo.toml")))
        else {
            continue;
        };
        for to in names {
            if to != from && src.contains(&format!("\n{to}.workspace = true")) {
                out.push((from.to_string(), to.to_string()));
            }
        }
    }
    out.sort();
    out
}

/// **What the corpus declares and what the build takes are the same list.**
///
/// Pinned rather than compared alone, because a list that shrank to nothing would compare equal to
/// a build that wired nothing. One edge survived contact with a compiler out of three drawn at
/// genesis, and if a second is ever drawn it should be because somebody wrote a `use`.
#[test]
fn every_declared_dependency_is_one_the_build_takes() {
    let root = repo_root();
    assert_eq!(
        declared(&root),
        wired(&root),
        "declared against wired: `depends-on` is a claim about the built system"
    );
    assert_eq!(
        declared(&root),
        [("interaction-resolver".to_string(), "verb-menu".to_string())],
        "the one coupling between components this engine has"
    );
}

/// **Each cut edge is argued where a reader meets it, rather than leaving a slot.**
///
/// An edge removed in silence is indistinguishable from one nobody thought of — which is the
/// finding `.yidam/decisions/held.yml` recorded and could not act on, one component short of being
/// able to. Both nodes say what was claimed, why it was plausible, and what would make it true.
#[test]
fn both_cut_edges_are_argued_in_the_nodes() {
    let root = repo_root();
    let node = |rel: &str| {
        std::fs::read_to_string(root.join(".yidam/corpus/subsystem").join(rel))
            .expect("the node")
            .replace(['*', '`'], "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    let resolver = node("interaction-resolver.yml");
    assert!(
        resolver.contains("It declared three dependencies and needs one."),
        "the resolver does not say it declared more than it needs"
    );
    assert!(
        resolver.contains("not deliver one. The thing that needs a presenter is"),
        "nor why it does not depend on a presenter"
    );
    assert!(
        resolver.contains("arrival is the room, and there is nothing to wait for"),
        "nor why it does not wait on a path"
    );

    // and the two targets say it from their own side
    assert!(
        node("path-solver.yml").contains("This node said the interaction resolver depends on it"),
        "path-solver still claims a coupling it does not have"
    );
    assert!(
        node("response-presenter.yml").contains("Nothing points at this node"),
        "response-presenter does not say that nothing points at it"
    );
}

/// **The relation still means what made two of the three wrong.**
///
/// Quoted rather than paraphrased. If `depends-on` is ever widened to mean *is described in terms
/// of* or *is sequenced after*, the rule in `engine.rs` stops being a rule and the two edges cut
/// here come back by definition rather than by evidence.
#[test]
fn the_relation_is_defined_as_a_requirement() {
    let root = repo_root();
    let ont =
        std::fs::read_to_string(root.join(".yidam/corpus/subsystem.ont.yml")).expect("the class");
    assert!(
        ont.contains("Another component this one requires in order to do its own work."),
        "the relation is no longer defined as a requirement"
    );
    assert!(
        ont.contains("stated without reference to any other component"),
        "nor the class's own rule about naming one"
    );

    // and the gate carries the converse, which is what replaced the reason for not checking it
    let gate = std::fs::read_to_string(root.join("crates/design-gate/tests/engine.rs"))
        .expect("the engine gate");
    assert!(
        gate.contains("a component may not declare one it does not take"),
        "engine.rs no longer states the rule this phase made writable"
    );
    assert!(
        !gate.contains("a rule that required those to be cargo dependencies would require"),
        "and it still carries the reason that expired"
    );
}
