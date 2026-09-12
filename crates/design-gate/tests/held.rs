//! **The two positions are a claim and its precondition, and the state graph does not move.**
//!
//! Seventeen files talk about duration. Two decided positions read as contradicting each other.
//!
//! [`effect-list-semantics`](../../../.yidam/decisions/effect-list-semantics.yml): *"**Instantaneous.**
//! No effect takes time and no effect yields control… **Atomic.** A binding is one edge in the
//! state graph. Nothing observes a value between two effects — not the player, not another
//! binding, not the checker."*
//!
//! [`response-presenter`](../../../.yidam/corpus/subsystem/response-presenter.yml): *"Turn a
//! response into something perceived, **and do not return until it is over**. Both halves matter
//! and only the first is obvious: a component that delivers a line and returns immediately has
//! presented nothing a player can read."*
//!
//! They do not contradict. *Instantaneous* is about the **state model**; *do not return until it
//! is over* is about the **player**; and the second is what makes the first true, because an
//! interval nothing can act during contains no state that did not exist before it. **A blocking
//! presenter is the reason a binding can stay an edge.** `.yidam/decisions/held.yml`.
//!
//! `engine/response-presenter` exists and owns *how long* — the line's length, which the node
//! already called the only lever over pacing this vocabulary offers — and decides nothing about
//! seconds, because nothing in the catalog states a reading speed.
//!
//! The one construct the first position defers duration to is still unreachable: **no
//! relationship in the whole ontology targets `cutscene`**. What changed is the reason. Duration
//! for a *line* no longer threatens the graph; duration for a *sequence* is unwritable because
//! nothing in this vocabulary can say what happens over an interval, which is a smaller hole than
//! the one that record wrote down.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every file in the corpus and the records that says the word.
fn talking(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for dir in [".yidam/corpus", ".yidam/decisions"] {
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "yml")
                    && std::fs::read_to_string(&p).is_ok_and(|s| s.contains("duration"))
                {
                    out.push(
                        p.file_stem()
                            .expect("a stem")
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
        }
    }
    out.sort();
    out
}

/// **Eighteen files, two positions, and the model is still free of time.**
///
/// The presenter owns duration and `game-format` does not: the state model has no `Wait`, no
/// `Pause`, no `Delay`, and eight effects none of which takes one. That is the half of
/// `effect-list-semantics` this phase leaves exactly where it was.
///
/// The eighteenth is `resolver-kind`, and it uses the word about a clock rather than about a
/// response: a question's duration is what `yidam due` measures and is the thing that record argues
/// means nothing for three of the nine. Two senses of one word in one repository, and this count
/// reads the string — which is worth knowing about a check whose whole subject is that the model
/// has no time in it.
#[test]
fn duration_is_the_presenters_and_the_state_model_is_free_of_it() {
    let root = repo_root();
    assert_eq!(
        talking(&root).len(),
        18,
        "files that say the word — this record is one"
    );

    let semantics = read(&root, ".yidam/decisions/effect-list-semantics.yml");
    assert!(
        flat(&semantics)
            .contains("**Instantaneous.** No effect takes time and no effect yields control"),
        "one position"
    );
    let presenter = read(&root, ".yidam/corpus/subsystem/response-presenter.yml");
    assert!(
        flat(&presenter).contains("and do not return until it is over"),
        "and the other"
    );

    // and nothing in the model has a place for it
    let model = read(&root, "crates/game-format/src/lib.rs");
    let effects = model
        .split("pub enum Effect {")
        .nth(1)
        .expect("the effect list")
        .split("\n}")
        .next()
        .expect("its end");
    let variants: Vec<&str> = effects
        .lines()
        .filter(|l| l.starts_with("    ") && !l.trim().starts_with("///") && !l.trim().is_empty())
        .collect();
    assert_eq!(variants.len(), 8, "effects, none of which takes time");
    for word in ["Wait", "Pause", "Delay", "Duration"] {
        assert!(!model.contains(word), "the model has no {word}");
    }

    // and the component that does own it invents no unit
    let presenter = read(&root, "engine/response-presenter/src/lib.rs");
    assert!(
        presenter.contains("pub fn weight(line: &str) -> usize {")
            && presenter.contains("line.chars().count()"),
        "how long is how much"
    );
    for unit in ["Duration", "millis", "secs", "sleep"] {
        assert!(!presenter.contains(unit), "and not how many {unit}");
    }
}

/// **The construct duration was deferred to cannot be reached.**
///
/// `effect-list-semantics` says a sequence that takes time *"is `cutscene`, and the corpus has the
/// class"* — and then records the defect in the same breath: no relationship targets it. Four
/// phases later nothing has changed.
#[test]
fn nothing_in_the_ontology_points_at_a_cutscene() {
    let root = repo_root();
    let mut pointing: Vec<String> = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let Ok(src) = std::fs::read_to_string(&p) else {
                continue;
            };
            let stem = p
                .file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned();
            let Some(links) = src.split("\nlinks:").nth(1) else {
                continue;
            };
            if links.contains("cutscene/") && !stem.contains("scripted-sequence") {
                pointing.push(stem);
            }
        }
    }
    assert!(
        pointing.is_empty(),
        "something points at a cutscene now: {pointing:?}"
    );

    let semantics = read(&root, ".yidam/decisions/effect-list-semantics.yml");
    assert!(
        flat(&semantics).contains("*No relationship in the entire ontology targets `cutscene`.*"),
        "and the record says so"
    );
}

/// **The client presents and the search does not, which is why the graph cannot move.**
///
/// Three components, and the seam is three ways: the search reads no text at all, the menu reads
/// the labels, the resolver reads the lines and hands them up, and only the presenter turns one
/// into an interval. `crates/reachability` does not depend on it and never calls it, so every
/// figure it publishes is a figure about a graph nothing in this phase touched — held by the
/// tests that pinned them before the component existed.
///
/// The corpus declares `interaction-resolver -depends-on-> response-presenter` and the built
/// resolver does not have it: the client presents, which is what that node's own responsibility
/// says — *produce* an outcome, not deliver one. Left alone rather than cut, and recorded.
#[test]
fn the_client_presents_and_the_search_does_not() {
    let root = repo_root();

    let engine = std::fs::read_dir(root.join("engine"))
        .expect("the engine directory")
        .flatten()
        .filter(|e| e.path().is_dir() && e.file_name() != "target")
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        engine.contains("response-presenter"),
        "the component exists"
    );

    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client
            .lines()
            .take(10)
            .any(|l| l.contains("composes: verb-menu, interaction-resolver, response-presenter")),
        "and the client composes three"
    );
    assert!(
        client.contains("present_all(") && client.contains("impl Perceive for Collect"),
        "everything a player is told goes through it"
    );

    // The search does not, and cannot: no dependency, and it names none of the component.
    let manifest = read(&root, "crates/reachability/Cargo.toml");
    assert!(
        !manifest.contains("response-presenter"),
        "the search does not depend on the presenter"
    );
    let search = read(&root, "crates/reachability/src/lib.rs");
    for name in ["response_presenter", "Perceive", "present_all"] {
        assert!(!search.contains(name), "and never reaches for {name}");
    }

    // And the corpus's edge is still one the build does not have.
    let node = read(&root, ".yidam/corpus/subsystem/interaction-resolver.yml");
    assert!(
        flat(&node)
            .contains("- target: ../subsystem/response-presenter.yml relationship: depends-on"),
        "the corpus declares the edge"
    );
    let resolver = read(&root, "engine/interaction-resolver/Cargo.toml");
    assert!(
        !resolver.contains("response-presenter"),
        "and the resolver does not have it — `.yidam/decisions/held.yml` leaves it alone"
    );
}
