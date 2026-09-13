//! **The component the engine says it cannot build has been half-written twice, in the instrument,
//! by phases that were answering something else.**
//!
//! `subsystem/path-solver` is the one subsystem this corpus specifies and the engine does not
//! provide. Two documents say why and they are not saying the same thing:
//!
//! - `crates/design-gate/tests/engine.rs` — *"it will not be until something has a path to walk —
//!   a text slice has nothing to cross a room with."* That is about a **client**, and it is true.
//! - `.yidam/corpus/subsystem/path-solver.yml` — *"the only subsystem with no `[verified]` claim,
//!   and the only one that never will have one… there is nothing for a check to read."* That is
//!   about **evidence**, and this file is the census that says it is not.
//!
//! `.yidam/decisions/drawn-boundary.yml` is what the second rests on: *"nothing can measure a
//! claim about a walk cycle or a coordinate space, because no `.game` file can state one."* The
//! premise is about the **format** and it holds. The conclusion drawn from it is about **every
//! possible check**, and the phase immediately after it measured a coordinate space —
//! `.yidam/decisions/adjacency.yml`, over 84 polygons out of somebody else's game, two rooms of
//! which are transcribed in `crates/design-gate/tests/adjacency.rs` and executed.
//!
//! That is the shape `.yidam/decisions/restated-guard.yml` named one phase ago: a conclusion that
//! outlived the argument holding it up. Here it is load-bearing — it is why a specified component
//! was never attempted.
//!
//! **And the attempt was already made.** `adjacency.rs` defines `edges`, `adjacent`, `components`
//! and `without`: portal extraction, the adjacency graph, a flood fill over it, and
//! state-dependent region removal. That is the region half of a path solver, written as private
//! helpers to a test about something else. `.yidam/decisions/two-components.yml` is the record of
//! this happening the first time, when `verb-menu` and `interaction-resolver` turned out to be
//! private methods of `crates/reachability` — *"what that phase built was not the components; it
//! was the place they say they are."*
//!
//! **The second defect is the same convention read from the other end.** A component node says
//! *"**It is built.** `engine/<name>` is this component"* — two of the three built ones do.
//! `response-presenter` was built and its node still reads *"What is still unmodelled is this
//! component itself."* A convention with a following and no statement is what this repository has
//! now found nine times; what is new is a follower that says the opposite of the truth.

use design_gate::repo_root;
use std::path::Path;

/// Every subsystem the corpus declares, and the module under `engine/` that says it is that
/// component.
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

/// **Three of four, and the fourth is the one with the geometry.**
#[test]
fn one_specified_component_is_not_built() {
    let root = repo_root();
    assert_eq!(
        built(&root),
        ["interaction-resolver", "response-presenter", "verb-menu"],
        "the components that exist"
    );
}

/// **The node says no check will ever read it, and one already did.**
///
/// Both halves are quoted rather than paraphrased, because the whole census is that two sentences
/// in this repository disagree and nobody had put them next to each other.
#[test]
fn the_unbuilt_component_declares_itself_unverifiable() {
    let root = repo_root();
    let node = std::fs::read_to_string(root.join(".yidam/corpus/subsystem/path-solver.yml"))
        .expect("the node");
    let flat = node
        .replace(['*', '`'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        flat.contains(
            "the only subsystem with no [verified] claim, and the only one that never will have one"
        ),
        "path-solver.yml no longer makes the claim this census is about"
    );
    assert!(
        flat.contains("there is nothing for a check to read"),
        "nor the reason it gives for it"
    );

    // And the measurement that was run one phase after the premise was decided.
    let adjacency =
        std::fs::read_to_string(root.join(".yidam/decisions/adjacency.yml")).expect("the record");
    assert!(
        adjacency.contains("290") && adjacency.contains("66"),
        "adjacency.yml no longer carries the coordinate-space measurement"
    );
}

/// **The component's core is written, in the instrument, as private helpers to another question.**
///
/// Four functions, and between them they are everything a path solver does with regions: turn a
/// ring into portals, decide adjacency from them, walk the graph, and drop a region because the
/// world state hid it. What is missing is the part that takes **points** — locating a click, and
/// reporting that it reached nothing.
#[test]
fn the_instrument_privately_implements_the_region_half_of_it() {
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
        !src.contains("path_solver") && !src.contains("path-solver`"),
        "and it does not know that is what they are"
    );
}

/// **A component node says it is built and names the module — and the newest one says the
/// opposite.**
#[test]
fn one_built_component_still_says_it_is_unmodelled() {
    let root = repo_root();
    let dir = root.join(".yidam/corpus/subsystem");
    let says: Vec<String> = ["interaction-resolver", "response-presenter", "verb-menu"]
        .into_iter()
        .filter(|n| {
            std::fs::read_to_string(dir.join(format!("{n}.yml")))
                .expect("the node")
                .contains(&format!("**It is built.** `engine/{n}`"))
        })
        .map(str::to_string)
        .collect();
    assert_eq!(
        says,
        ["interaction-resolver", "verb-menu"],
        "built components whose node says so and names the module"
    );

    let presenter = std::fs::read_to_string(dir.join("response-presenter.yml")).expect("the node");
    assert!(
        presenter.contains("What is still unmodelled is this component itself."),
        "response-presenter.yml no longer carries the sentence that is false"
    );
    assert!(
        !presenter.contains("engine/"),
        "and it still does not name the module that realizes it"
    );
}
