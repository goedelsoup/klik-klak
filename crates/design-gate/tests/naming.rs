//! **A verb has a name, and it is the component's rather than the author's.**
//!
//! `crates/game-format/src/parse.rs` declares `const VERBS: [&str; 5]` and that is the whole of
//! what a verb is here: a token the parser dispatches on. `Game::labels` holds a name for every
//! other kind of thing an author declares — rooms, actors, items, hotspots, options, flags,
//! counters — and a verb is not a declaration, so it has nowhere to carry one.
//!
//! The menu printed the token. `verb_menu::Entry` had carried the *target's* label since
//! `.yidam/decisions/authored-text.yml` gave the parse tree labels back and nothing rendered it,
//! so the one component that knew what a thing was called printed its canonical name.
//!
//! Six templates in `engine/verb-menu` now — `Use $2 with $1` and five others, one of them for
//! `option`, which is not a verb — and **a menu entry has two spellings**: a phrase a player reads
//! and a key a script writes. `.yidam/decisions/verb-names.yml`.
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
//! below is the one `figures.rs` already is, applied to a figure about somebody else's source for
//! the first time.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// **Five verbs the language fixes, six templates the component owns.**
///
/// The `.game` surface is unchanged — a verb is still a token the parser dispatches on, and there
/// is still no `Kind::Verb` — because a name for something an author cannot change is not
/// something an author writes.
#[test]
fn a_verb_is_a_token_and_its_name_belongs_to_the_menu() {
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

    let menu = read(&root, "engine/verb-menu/src/lib.rs");
    assert!(
        menu.contains("const NAMES: [(&str, &str); 6] = ["),
        "the names are the component's"
    );
    for (verb, template) in [
        ("look-at", "Look at $1"),
        ("pick-up", "Pick up $1"),
        ("talk-to", "Talk to $1"),
        ("walk-to", "Walk to $1"),
        ("use-with", "Use $2 with $1"),
        ("option", "$1"),
    ] {
        assert!(
            menu.contains(&format!("(\"{verb}\", \"{template}\")")),
            "{verb} reads {template}"
        );
    }
}

/// **Two spellings, and they are different strings.**
///
/// `Entry::phrase` is what a player reads; `play::spell` is the key a script writes and a
/// transcript records. A shipped game has had them apart from the beginning — `master_verb_list`
/// holds ids in menu order and `master_verb_names` holds what is shown, checked against each other
/// at boot because the two can drift.
#[test]
fn an_entry_has_a_phrase_and_a_key() {
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
        "the key reads the keys"
    );
    assert!(!body.contains("e.label"), "and none of the names");

    assert!(
        menu.contains("pub fn phrase(&self) -> String {")
            && menu.contains("self.label.unwrap_or(self.target)")
            && menu.contains("self.object_label.or(self.object)"),
        "and the phrase reads the names, falling back to the keys so a menu is never blank"
    );

    // The transcripts keep the key. One that moved every time somebody reworded a template would
    // pin prose rather than behaviour.
    let won = read(&root, "engine/play/tests/transcript.rs");
    assert!(
        won.contains("> use-with crowbar on crate"),
        "a transcript records what was picked, by key"
    );
    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client.contains("pub fn reads(&self)") && client.contains("e.phrase()"),
        "and the interactive menu reads the phrase"
    );
}

/// **Fifty-seven, everywhere, and this is the test that had been missing.**
///
/// The catalog entry has said 57 since it was written. Two corpus nodes said 63 at `[verified]`
/// for six phases until `verb-count` corrected them, and `authored-text` wrote it again two phases
/// later — in a record, at `[verified]`, citing the entry that disagrees.
///
/// `figures.rs` holds every statement of a figure against the test that derives it, and there was
/// no test deriving this one. **What it can hold is the corpus against the catalog entry**, which
/// is the widest it goes: a figure about a catalogued artifact can only be derived from the entry,
/// and nothing holds an entry against the file it describes.
#[test]
fn every_statement_of_delores_verb_count_says_fifty_seven() {
    let root = repo_root();
    let catalog = read(&root, ".yidam/catalog/delores-source.md");
    assert!(
        catalog.contains("`master_verb_list` and\n`master_verb_names` hold **57 entries each**"),
        "the entry says fifty-seven"
    );

    // Every place in the corpus and the records that states how many verbs that game has.
    let mut wrong: Vec<String> = Vec::new();
    for dir in [".yidam/corpus", ".yidam/decisions"] {
        let mut stack = vec![root.join(dir)];
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
                let flat = src.split_whitespace().collect::<Vec<_>>().join(" ");
                for bad in ["63 verbs", "sixty-three verbs", "its 63 "] {
                    // used versus discussed for the thirteenth time: three records quote the wrong
                    // figure in order to correct it, and a correction that cannot name what it
                    // corrects is not one
                    if flat.contains(bad)
                        && !flat.contains("never right")
                        && !flat.contains("wrong")
                    {
                        wrong.push(format!("{}: {bad}", p.display()));
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a statement of somebody else's verb count that the catalog entry contradicts:\n  {}",
        wrong.join("\n  ")
    );
}
