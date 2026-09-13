//! **One component declares three dependencies and needs one, and the other two were
//! unfalsifiable until last phase.**
//!
//! `subsystem -depends-on-> subsystem` is defined in `.yidam/corpus/subsystem.ont.yml` as *"another
//! component this one requires in order to do its own work"* — a claim about the built system, in a
//! class whose `responsibility` property insists on *"what this component is answerable for, stated
//! without reference to any other component."* The one edge that couples two components is the one
//! the corpus drew three times.
//!
//! `interaction-resolver` declares `depends-on` to `path-solver`, `verb-menu` and
//! `response-presenter`, and `engine/interaction-resolver/Cargo.toml` takes `verb-menu`.
//!
//! **The gate says in writing why it does not check the other direction.**
//! `crates/design-gate/tests/engine.rs`: *"One direction only. The resolver also declares
//! `depends-on` `path-solver` and `response-presenter`, and a rule that required those to be cargo
//! dependencies would require building them."* That was true and is a dated reason: a rule cannot
//! ask a component to depend on something that does not exist.
//!
//! **`.yidam/decisions/unbuilt-solver.yml` built the last of them.** All four components the corpus
//! specifies exist, so the converse is askable for the first time, and two edges that could not be
//! wrong are now either right or not.
//!
//! `.yidam/decisions/held.yml` left a marker on one of the two: *"an edge the build does not need
//! is exactly as visible as one it does — which is to say invisible — and what this record can say
//! is that somebody looked once."* This is the second look, and it is a different question because
//! the thing it asks about is built.

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

/// **Three declared, one wired.**
#[test]
fn the_resolver_declares_three_dependencies_and_the_build_takes_one() {
    let root = repo_root();
    assert_eq!(
        declared(&root),
        [
            (
                "interaction-resolver".to_string(),
                "path-solver".to_string()
            ),
            ("interaction-resolver".to_string(), "verb-menu".to_string()),
            (
                "interaction-resolver".to_string(),
                "response-presenter".to_string()
            ),
        ],
        "every depends-on the corpus declares between components"
    );
    assert_eq!(
        wired(&root),
        [("interaction-resolver".to_string(), "verb-menu".to_string())],
        "and every one the build takes"
    );
}

/// **The reason the converse was never checked has expired, and it expired last phase.**
///
/// A rule cannot ask a component to depend on one that does not exist. All four exist now —
/// `.yidam/decisions/unbuilt-solver.yml` — so the two unwired edges have stopped being
/// unfalsifiable and have started being either true or false.
#[test]
fn both_unwired_targets_are_built_now() {
    let root = repo_root();
    let unwired: Vec<String> = {
        let w = wired(&root);
        declared(&root)
            .into_iter()
            .filter(|e| !w.contains(e))
            .map(|(_, to)| to)
            .collect()
    };
    assert_eq!(
        unwired,
        ["path-solver", "response-presenter"],
        "declared and not wired"
    );

    for name in &unwired {
        assert!(
            root.join(format!("engine/{name}/src/lib.rs")).exists(),
            "{name} is not built, and an edge to an unbuilt component cannot be wrong yet"
        );
    }

    // and the gate still carries the dated reason, in the test that acts on it
    let gate = std::fs::read_to_string(root.join("crates/design-gate/tests/engine.rs"))
        .expect("the engine gate");
    let flat = gate.replace("//!", "").replace("///", "");
    let flat = flat.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains(
            "a rule that required those to be cargo dependencies would require building them"
        ),
        "engine.rs no longer carries the reason this census is about"
    );
}

/// **What the relation means, and what the class says about naming another component at all.**
///
/// Both quoted rather than paraphrased, because the census is that two edges do not satisfy the
/// first and the class was written to discourage the second.
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

    // and the resolver's own responsibility says it produces rather than delivers
    let node =
        std::fs::read_to_string(root.join(".yidam/corpus/subsystem/interaction-resolver.yml"))
            .expect("the node");
    let flat = node.replace(['*', '`'], "");
    let flat = flat.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("produce exactly one outcome"),
        "the resolver's responsibility no longer says what it is answerable for"
    );
}
