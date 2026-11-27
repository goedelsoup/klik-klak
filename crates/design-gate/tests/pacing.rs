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
//! **Both are answered now and the answers are different sizes.** A client supplies its own
//! sink — `Session::take_with` — and `Await` holds the player until they ask for the next thing,
//! ten times in a winning run, with no clock. The rate is still unstated and nothing invents one.
//!
//! What the holding costs is the claim: under a dismissed hold a 74-character line and a
//! 3-character line are one keypress each, so *"the only lever over pacing this vocabulary
//! offers"* is a lever over a hold that **elapses**. The node names both policies now.
//! `.yidam/decisions/dismissed-hold.yml`.

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

/// **A client supplies its own perceiving, which is what the obstacle actually was.**
///
/// The marker named two absences. The second — *"no input between turns"* — was false in the file
/// it was about: `engine/play`'s interactive loop blocks on stdin every turn. What stopped the
/// client holding was `Session::take` building its own `Collect` and taking no sink, so every
/// client of this loop got the one that returns at once whatever it could have done instead.
///
/// One parameter. `take` still exists and still defaults, and nothing that called it moved.
#[test]
fn a_client_supplies_its_own_perceiving() {
    let root = repo_root();

    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client.contains("pub fn take_with(") && client.contains("sink: &mut impl Perceive,"),
        "a turn takes the client's own sink"
    );
    assert!(
        flat(&client).contains("pub fn take(&mut self, choice: &str) -> Result<Option<Stop>, Stop> { self.take_with(choice, &mut Collect::default()) }"),
        "and the old spelling is the same turn with the old sink"
    );
    assert!(
        !client.contains("let mut sink = Collect::default();"),
        "the loop no longer builds one nobody asked for"
    );

    let main = read(&root, "engine/play/src/main.rs");
    assert!(
        main.contains("Await::new(stdin.lock(), std::io::stdout())")
            && main.contains("session.take_with(&choice, &mut sink)"),
        "and the interactive loop holds the player with it"
    );
}

/// **Three sinks in the build, and exactly one of them holds.**
///
/// `Collect` and the presenter's test `Heard` return at once and hold nobody; `Await` writes the
/// line and does not come back until the stream does. So the guarantee
/// `subsystem/response-presenter` states — *"do not return until it is over"* — has a client that
/// exercises it for the first time, and the two that do not are kept because a scripted run has no
/// player to hold.
#[test]
fn exactly_one_sink_in_the_build_holds_the_player() {
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
                    if let Some(rest) = line.trim().strip_prefix("impl Perceive for ") {
                        let name = rest.trim_end_matches(" {");
                        sinks.push(name.split(['<', ' ']).next().expect("a name").to_string());
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
        "the one that returns at once"
    );
    assert!(
        flat(&client).contains("if self.line().is_some() { self.dismissals += 1; }"),
        "and the one that does not come back until the player says so"
    );
}

/// **No source states a rate, which is the half of the reason that was right.**
///
/// Thirteen sources and not one says what a character is worth in seconds. That is why
/// `engine/response-presenter` reaches for no `Duration` and invents no constant, and it is a fact
/// about the evidence rather than about terminals.
///
/// Fourteen since `maker-tracker` added `ags-tracker`, an issue tracker, which states no rate either.
/// Fifteen since `house-width` added `sierra-agi-source`, three games' logic source, which states
/// none.
#[test]
fn no_source_in_the_catalog_states_a_reading_speed() {
    let root = repo_root();
    let names = sources(&root);
    assert_eq!(names.len(), 15, "sources in the catalog");

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

/// **The pacing claim names the policy it needs.**
///
/// *"A line's length is how long the game is held"* is true of a hold that **elapses** and false of
/// one that is **dismissed** — measured: 471 characters over ten holds in a winning run, the
/// longest line twenty-four times the shortest, one keypress each.
/// `engine/play/tests/holding.rs`.
///
/// So the node states the condition rather than the claim flat, and says the thing this corpus
/// cares about: **an author writes `say` and there is one spelling**, so which hold a game gets is
/// decided once by the client and there is nothing to write.
#[test]
fn the_only_lever_over_pacing_carries_the_policy_it_depends_on() {
    let root = repo_root();
    let node = flat(&read(
        &root,
        ".yidam/corpus/subsystem/response-presenter.yml",
    ));
    assert!(
        node.contains("a line's length is how long the game is held, and that is the only lever over pacing this vocabulary offers"),
        "the claim"
    );
    for policy in ["elapses", "dismissed"] {
        assert!(
            node.contains(policy),
            "and the node names a hold that {policy}"
        );
    }
    assert!(
        node.contains("an author writes `say` and there is one spelling"),
        "and says why it is not an authoring question"
    );

    // the marker that sent this phase is retracted where it was raised, and not re-asked
    let record = flat(&read(&root, ".yidam/decisions/held.yml"));
    assert!(
        !record.contains(
            "the first client that can hold a player is the first one with a frame. `[open]`"
        ),
        "the marker still stands"
    );
    assert!(
        record.contains("dismissed-hold"),
        "and the record that answered it is named where it was asked"
    );
}
