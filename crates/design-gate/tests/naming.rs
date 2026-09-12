//! **A verb has no name.**
//!
//! `crates/game-format/src/parse.rs` declares `const VERBS: [&str; 5]` and that is the whole of
//! what a verb is here: a token the parser dispatches on. `Game::labels` holds a name for every
//! other kind of thing an author declares — rooms, actors, items, hotspots, options, flags,
//! counters — and a verb is not a declaration, so it has nowhere to carry one.
//!
//! The menu prints the token. `engine/play`'s transcripts read `use-with crowbar on crate`, and
//! `verb_menu::Entry` has carried the *target's* label since
//! `.yidam/decisions/authored-text.yml` gave the parse tree labels back — **and nothing renders
//! it**. The one component that knows what a thing is called prints its canonical name.
//!
//! **A shipped game gives all fifty-seven of its verbs a display template with noun slots.**
//! `master_verb_names` sits beside `master_verb_list`, the two are checked against each other at
//! boot, and an entry reads `"Unlock $NOUN1$ with $NOUN2$"`
//! ([delores-source](../../../.yidam/catalog/delores-source.md)).
//!
//! **And this repository got that number wrong twice.**
//! [`verb-count`](../../../.yidam/decisions/verb-count.yml) corrected two corpus nodes from
//! sixty-three to fifty-seven, and `.yidam/decisions/authored-text.yml` wrote sixty-three again,
//! at `[verified]`, two phases later — citing the catalog entry that says fifty-seven. The check
//! below is the one this repository already knows how to write and had not written here.
//!
//! These checks pin the state. They are rewritten into the rule in the same phase.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// **Five verbs, no names, and a menu that prints tokens.**
#[test]
fn a_verb_is_a_token_the_parser_dispatches_on() {
    let root = repo_root();

    let parse = read(&root, "crates/game-format/src/parse.rs");
    assert!(
        parse.contains(
            r#"const VERBS: [&str; 5] = ["look-at", "use-with", "talk-to", "pick-up", "walk-to"];"#
        ),
        "the whole of what a verb is"
    );
    let model = read(&root, "crates/game-format/src/lib.rs");
    assert!(
        model.contains("pub verb: String,") && !model.contains("verb_labels"),
        "a binding carries the token and the game carries no name for it"
    );

    // every other declared thing has a label, and a verb is not a declaration
    let kinds = read(&root, "crates/game-format/src/lib.rs");
    let named = [
        "Kind::Room",
        "Kind::Actor",
        "Kind::Item",
        "Kind::Hotspot",
        "Kind::Option_",
        "Kind::Flag",
        "Kind::Counter",
        "Kind::Conversation",
    ];
    for k in named {
        assert!(kinds.contains(k), "{k} is a kind a declaration can be");
    }
    assert!(
        !kinds.contains("Kind::Verb"),
        "and a verb is not one of them"
    );
}

/// **The menu holds the label and prints the canonical name.**
///
/// `Entry` gained `label` when the parse tree stopped discarding it. `spell` — the one function
/// that turns an entry into a string — reads `verb`, `object` and `target`, and not `label`.
#[test]
fn the_one_component_that_knows_the_name_prints_the_key() {
    let root = repo_root();
    let menu = read(&root, "engine/verb-menu/src/lib.rs");
    let client = read(&root, "engine/play/src/lib.rs");
    let body = client
        .split("pub fn spell(e: &Entry<'_>) -> String {")
        .nth(1)
        .expect("the function that turns an entry into a string")
        .split("\n}")
        .next()
        .expect("its end");
    assert!(
        body.contains("e.verb") && body.contains("e.object") && body.contains("e.target"),
        "it reads the keys"
    );
    assert!(!body.contains("e.label"), "and not the name");

    assert!(
        menu.contains("pub label: Option<&'a str>,"),
        "which the entry has been carrying since `authored-text`"
    );

    // and the transcripts show it
    let won = read(&root, "engine/play/tests/transcript.rs");
    assert!(
        won.contains("> use-with crowbar on crate"),
        "a player reads the token, the second object's key, and the target's key"
    );
}

/// **Fifty-seven, and this repository has written sixty-three three times.**
///
/// The catalog entry has said 57 since it was written. Two corpus nodes said 63 at `[verified]`
/// for six phases until `verb-count` corrected them, and `authored-text` wrote it again two
/// phases ago — in a record, at `[verified]`, citing the entry that disagrees.
///
/// `figures.rs` holds every statement of a figure against the test that derives it. There is no
/// test that derives this one, which is why nothing caught the third instance.
#[test]
fn the_verb_count_is_wrong_in_a_record_again() {
    let root = repo_root();
    let catalog = read(&root, ".yidam/catalog/delores-source.md");
    assert!(
        catalog.contains("`master_verb_list` and\n`master_verb_names` hold **57 entries each**"),
        "the entry says fifty-seven"
    );

    let record = read(&root, ".yidam/decisions/authored-text.yml");
    assert!(
        record.contains("every\n  one of its 63 verbs a display string"),
        "and the record written two phases ago says sixty-three"
    );

    // nothing anywhere derives the figure, which is why nothing caught it
    let mut deriving: Vec<String> = Vec::new();
    for dir in ["crates", "engine"] {
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n == "target") {
                        continue;
                    }
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "rs")
                    && p.file_name().is_some_and(|n| n != "naming.rs")
                    && std::fs::read_to_string(&p).is_ok_and(|s| s.contains("master_verb_names"))
                {
                    deriving.push(p.display().to_string());
                }
            }
        }
    }
    assert!(deriving.is_empty(), "something checks it: {deriving:?}");
}
