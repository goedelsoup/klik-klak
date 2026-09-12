//! **Sixteen files talk about duration and nothing models it.**
//!
//! Two decided positions, and they read as contradicting each other.
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
//! Nothing reconciles them, and the one construct the first defers duration to is unreachable:
//! **no relationship in the whole ontology targets `cutscene`**, so a binding cannot start one.
//! `.yidam/decisions/effect-list-semantics.yml` records that as *"where this decision lands a
//! defect rather than a design"* and deliberately declines to add the edge.
//!
//! Meanwhile `engine/play` prints a line and returns, which
//! [`playable`](../../../.yidam/decisions/playable.yml) states as a cost rather than hiding: the
//! slice does that component's job inline and does the first half of it.
//!
//! These checks pin the state. They are rewritten into the rule in the same phase.

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

/// **Sixteen files, two positions, and no model.**
#[test]
fn duration_is_argued_about_everywhere_and_modelled_nowhere() {
    let root = repo_root();
    assert_eq!(talking(&root).len(), 16, "files that say the word");

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

/// **Nothing presents, and the corpus declares an edge the built system does not have.**
///
/// `engine/play` prints a line and returns. And `interaction-resolver` links `depends-on` to
/// `response-presenter` in the corpus, while the crate depends on `verb-menu` and nothing else —
/// which is either an edge the engine has not needed yet or one the corpus should not have.
#[test]
fn nothing_presents_and_an_edge_is_declared_that_nothing_uses() {
    let root = repo_root();

    let engine = std::fs::read_dir(root.join("engine"))
        .expect("the engine directory")
        .flatten()
        .filter(|e| e.path().is_dir() && e.file_name() != "target")
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        !engine.contains("response-presenter"),
        "the component does not exist"
    );

    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client
            .lines()
            .take(10)
            .any(|l| l.contains("composes: verb-menu, interaction-resolver")),
        "and the client composes two components"
    );
    assert!(
        client.contains("out.push_str(&format!(\"  {line}\\n\"))"),
        "printing a line and returning, which is the first half of one job"
    );

    let node = read(&root, ".yidam/corpus/subsystem/interaction-resolver.yml");
    assert!(
        flat(&node)
            .contains("- target: ../subsystem/response-presenter.yml relationship: depends-on"),
        "the corpus declares the edge"
    );
    let manifest = read(&root, "engine/interaction-resolver/Cargo.toml");
    assert!(
        !manifest.contains("response-presenter"),
        "and nothing in the build has it"
    );
}
