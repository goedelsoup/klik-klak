//! **Every component the corpus specifies is built, says so, and names the module.**
//!
//! This was a census one commit ago and it found two things, which are the same thing read from
//! two ends.
//!
//! `subsystem/path-solver` was the one subsystem specified and not provided, and two documents
//! said why. `crates/design-gate/tests/engine.rs` said a client has nothing to walk — *"a text
//! slice has nothing to cross a room with"* — which is about a **consumer** and is true and
//! still true. The node said it was *"the only subsystem with no `[verified]` claim, and the only
//! one that never will have one… there is nothing for a check to read"*, which is about
//! **evidence** and was not. It drew that from `.yidam/decisions/drawn-boundary.yml`, whose claim
//! is about the `.game` **format** — *"no `.game` file can state one"* — and the phase directly
//! after it measured a coordinate space out of somebody else's game. A conclusion outliving the
//! argument holding it up is `.yidam/decisions/restated-guard.yml`'s shape; here it kept a
//! specified component unbuilt for six phases while the region half of it sat in
//! `crates/design-gate/tests/adjacency.rs` as private helpers to another question.
//!
//! From the other end: a component node says *"**It is built.** `engine/<name>` is this
//! component"*, and two of the three built ones did. `response-presenter` had been built for five
//! phases and its node still read *"What is still unmodelled is this component itself."*
//!
//! **So the rule is the convention, stated.** A specified component is built; a built component's
//! node says so and names the module. Both halves would have caught what was found, and the
//! second is the cheaper of the two: it is the sentence a phase forgets to write on the day it
//! finishes. `.yidam/decisions/unbuilt-solver.yml`.

use design_gate::repo_root;
use std::path::Path;

/// Every subsystem the corpus declares.
fn specified(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(root.join(".yidam/corpus/subsystem"))
        .expect("the subsystem directory")
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            (p.extension()? == "yml").then(|| p.file_stem()?.to_str().map(str::to_string))?
        })
        .collect();
    out.sort();
    out
}

/// Every subsystem some module under `engine/` says it is.
fn built(root: &Path) -> Vec<String> {
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
            } else if p.extension().is_some_and(|x| x == "rs") {
                let Ok(src) = std::fs::read_to_string(&p) else {
                    continue;
                };
                if let Some(name) = src.lines().take(10).find_map(|l| {
                    l.trim()
                        .strip_prefix("//!")?
                        .trim()
                        .strip_prefix("realizes:")?
                        .trim()
                        .strip_prefix("subsystem/")
                        .map(|n| n.trim_end_matches(".yml").to_string())
                }) {
                    out.push(name);
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// **Nothing is specified and missing.**
///
/// `engine.rs` pins the same list and this says the thing that list is *for*: the two are equal.
/// A component the corpus describes and the engine does not provide is either work nobody has
/// done or a component the corpus should not have declared, and both are findings that belong in
/// a record rather than in a dash in a table.
#[test]
fn every_specified_component_is_built() {
    let root = repo_root();
    assert_eq!(
        built(&root),
        specified(&root),
        "a subsystem the corpus declares and no module realizes"
    );
}

/// **Every component node says it is built and names the module.**
///
/// The convention had a following of two and no statement, which is the shape this repository has
/// now found nine times — `.yidam/decisions/unread-list.yml`, `.yidam/decisions/resolver-kind.yml`.
/// What was new was a follower left saying the **opposite** of the truth: a node describing a
/// component that had existed for five phases as the thing still missing. A convention that is
/// only ever copied is not checked, and an uncopied one decays in the one direction nobody reads.
#[test]
fn every_component_node_says_it_is_built_and_names_the_module() {
    let root = repo_root();
    let dir = root.join(".yidam/corpus/subsystem");
    let mut silent = Vec::new();
    for name in specified(&root) {
        let src = std::fs::read_to_string(dir.join(format!("{name}.yml"))).expect("the node");
        if !src.contains(&format!("**It is built.** `engine/{name}`")) {
            silent.push(name);
        }
    }
    assert!(
        silent.is_empty(),
        "a component node that does not say it is built and name the module: {}",
        silent.join(", ")
    );
}

/// **No node claims a check cannot read it while a check reads it.**
///
/// The narrow form of the defect, held on the one node that had it. `path-solver` now carries
/// `[verified]` claims and cites the test that makes them, and the sentence denying that was ever
/// possible is gone.
///
/// The premise underneath is untouched and the check says so: a `.game` file still cannot state a
/// polygon, which is why this component's evidence is a shipped game's rather than
/// `design/specimen/`'s — the true and permanent version of what the old sentence was reaching
/// for.
#[test]
fn the_component_with_borrowed_evidence_no_longer_denies_it_has_any() {
    let root = repo_root();
    let node = std::fs::read_to_string(root.join(".yidam/corpus/subsystem/path-solver.yml"))
        .expect("the node");
    let flat = node
        .replace(['*', '`'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    // The sentence is still in the node, once, inside the paragraph that withdraws it. A
    // retraction quotes what it retracts and a check that forbade the words outright would forbid
    // saying what changed — which is `.yidam/decisions/marker-count.yml`'s distinction between a
    // live claim and a quotation of a dead one, in prose rather than in a token.
    let claim = "the only one that never will have one";
    assert_eq!(
        flat.matches(claim).count(),
        1,
        "path-solver.yml states the withdrawn claim more than once"
    );
    let withdrawn = flat
        .find("For six phases this node said something stronger and wrong")
        .expect("the node does not say that it withdrew anything");
    assert!(
        flat.find(claim).expect("present") > withdrawn,
        "path-solver.yml still asserts that no check will ever read it before withdrawing it"
    );
    assert!(
        node.contains("[verified]") && node.contains("engine/path-solver/tests/delores.rs"),
        "and it does not carry a verified claim citing the check that makes one"
    );
    assert!(
        flat.contains("only component whose evidence is borrowed"),
        "nor the claim that replaced it"
    );

    // The premise is unchanged and load-bearing, which is the half that was always right.
    let boundary = std::fs::read_to_string(root.join(".yidam/decisions/drawn-boundary.yml"))
        .expect("the record");
    let premise = boundary
        .replace(['*', '`'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        premise.contains("this design has no geometry"),
        "drawn-boundary no longer states the premise this rests on"
    );
}

/// **The instrument's private copy of the component declares itself one.**
///
/// `adjacency.rs` defines `edges`, `adjacent`, `components` and `without` — the region half of a
/// path solver, written before the component and kept, because the instrument may not depend on
/// the engine and the engine must not depend on the instrument. What is checked is not that the
/// duplication is gone but that it is **said**: an undeclared second implementation is the defect
/// `.yidam/decisions/two-components.yml` found twice by accident.
///
/// The two are held equal by arithmetic rather than by a shared function — the engine's test reads
/// these rings and re-derives the pair counts pinned here — and that is checked from the other
/// side, in `engine/path-solver/tests/delores.rs`.
#[test]
fn the_instruments_copy_of_the_component_says_that_is_what_it_is() {
    let root = repo_root();
    let src = std::fs::read_to_string(root.join("crates/design-gate/tests/adjacency.rs"))
        .expect("the measurement");
    let defined: Vec<&str> = ["edges", "adjacent", "components", "without"]
        .into_iter()
        .filter(|f| src.contains(&format!("fn {f}(")))
        .collect();
    assert_eq!(
        defined,
        ["edges", "adjacent", "components", "without"],
        "the four helpers that are the component"
    );
    assert!(
        src.contains("the region half of `engine/path-solver`"),
        "and the file does not say that is what they are"
    );

    let engine = std::fs::read_to_string(root.join("engine/path-solver/tests/delores.rs"))
        .expect("the component's test");
    assert!(
        engine.contains("crates/design-gate/tests/adjacency.rs"),
        "the engine's test no longer reads the transcription and must be holding a copy"
    );
}
