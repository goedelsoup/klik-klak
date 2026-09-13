//! **A line's length is a lever over pacing under a policy nobody has chosen, and the reason
//! given for not choosing was a device rather than a rate.**
//!
//! [`held`](../../../.yidam/decisions/held.yml) closed with a marker: *"`play` implements
//! `Perceive` by printing and returning, so it holds the player for nothing at all… a terminal
//! with no clock and no input between turns has nowhere to put an interval… **the first client
//! that can hold a player is the first one with a frame.**"*
//!
//! That reason names two absences and the repository contradicts one of them in the file the
//! marker is about: `engine/play`'s interactive loop blocks on stdin every turn. A terminal has
//! input. What it does not have is input *during* a response — and that is not a property of
//! terminals, it is `Session::take` constructing its own sink so that no caller can supply one.
//!
//! The half that is true is the other half, and it is about a **rate** rather than a device.
//! `weight` is characters and no source in the catalog says what a character is worth in seconds.
//!
//! This file is the census. It pins the state as it was found.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every `.md` in the catalog, by name.
fn sources(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(root.join(".yidam/catalog"))
        .expect("the catalog")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".md") && n != "README.md")
        .collect();
    out.sort();
    out
}

/// **The obstacle named is a device, and the obstacle is a parameter.**
///
/// The marker gives two reasons a terminal cannot hold a player. The second is *"no input between
/// turns"*, and `engine/play`'s interactive loop reads a line from stdin on every one of them —
/// so the client the marker is about does exactly the thing the marker says it cannot.
///
/// What actually stops it is smaller and is in this repository rather than in terminals:
/// `Session::take` builds a `Collect` and never lets a caller pass one in. Every client of this
/// loop gets the sink that returns at once, whatever it could have done instead.
#[test]
fn the_reason_the_client_cannot_hold_is_not_the_reason_given() {
    let root = repo_root();

    let record = flat(&read(&root, ".yidam/decisions/held.yml"));
    assert!(
        record.contains(
            "a terminal with no clock and no input between turns has nowhere to put an interval"
        ),
        "the marker's reason"
    );
    assert!(
        record.contains("the first client that can hold a player is the first one with a frame"),
        "and its conclusion"
    );

    // and the client reads input every turn
    let main = read(&root, "engine/play/src/main.rs");
    assert!(
        main.contains("std::io::stdin().read_line(&mut line)"),
        "the interactive loop blocks on stdin, which is input between turns"
    );

    // what stops it is one hardcoded line and one missing parameter
    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client.contains("let mut sink = Collect::default();"),
        "the loop builds its own sink"
    );
    assert!(
        client.contains("pub fn take(&mut self, choice: &str) -> Result<Option<Stop>, Stop> {"),
        "and takes none, so no client can choose how it holds"
    );
}

/// **Two sinks exist in the whole build and both return at once.**
///
/// So the lever `subsystem/response-presenter` calls *"the only lever over pacing this vocabulary
/// offers"* is inert in everything that has ever run here: the component reports the weight, and
/// the two things that could act on it discard it.
#[test]
fn every_sink_in_the_build_returns_immediately() {
    let root = repo_root();
    let mut sinks: Vec<String> = Vec::new();
    let mut stack = vec![root.join("engine")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n != "target") {
                    stack.push(p);
                }
                continue;
            }
            if p.extension().is_some_and(|x| x == "rs") {
                let src = std::fs::read_to_string(&p).expect("readable");
                for line in src.lines() {
                    if let Some(name) = line.trim().strip_prefix("impl Perceive for ") {
                        sinks.push(name.trim_end_matches(" {").to_string());
                    }
                }
            }
        }
    }
    sinks.sort();
    assert_eq!(sinks, ["Collect", "Heard"], "every sink in the build");

    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        flat(&client)
            .contains("fn deliver(&mut self, line: &str) { self.0.push(line.to_string()); }"),
        "the client's sink pushes and returns"
    );
}

/// **No source states a rate, which is the half of the reason that was right.**
///
/// Eleven sources and not one says what a character is worth in seconds. That is why
/// `engine/response-presenter` reaches for no `Duration` and invents no constant, and it is a fact
/// about the evidence rather than about terminals.
#[test]
fn no_source_in_the_catalog_states_a_reading_speed() {
    let root = repo_root();
    let names = sources(&root);
    assert_eq!(names.len(), 11, "sources in the catalog");

    let mut stating: Vec<&String> = Vec::new();
    for name in &names {
        let src = read(&root, &format!(".yidam/catalog/{name}")).to_lowercase();
        if [
            "words per minute",
            "characters per second",
            "reading speed",
            "reading rate",
        ]
        .iter()
        .any(|w| src.contains(w))
        {
            stating.push(name);
        }
    }
    assert!(stating.is_empty(), "a source states a rate: {stating:?}");
}

/// **The pacing claim is written without the condition it needs.**
///
/// *"A line's length is how long the game is held, and that is the only lever over pacing this
/// vocabulary offers"* — stated flat. It is true of a hold that **elapses** and false of one that
/// is **dismissed**: under dismissal a player takes as long as they take and the author's longest
/// sentence buys nothing.
///
/// Nobody has chosen between the two, and nothing in the corpus names them as two.
#[test]
fn the_only_lever_over_pacing_is_asserted_without_a_policy() {
    let root = repo_root();
    let node = flat(&read(
        &root,
        ".yidam/corpus/subsystem/response-presenter.yml",
    ));
    assert!(
        node.contains("a line's length is how long the game is held, and that is the only lever over pacing this vocabulary offers"),
        "the claim, unconditional"
    );
    assert!(
        node.contains("An author who wants a beat writes a longer sentence"),
        "and what it is taken to mean for an author"
    );
    assert!(
        !node.contains("dismiss") && !node.contains("elapse"),
        "and the node names neither policy"
    );
}
