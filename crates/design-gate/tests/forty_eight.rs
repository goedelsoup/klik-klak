//! **The corpus believes it holds a forty-eight-entry table and holds seven rows of it.**
//!
//! `.yidam/catalog/ags-source.md` records what was read from AGS's own source and says of the
//! action table: *"all forty-eight entries. Recorded in full in
//! `data-format/interaction-table.yml`."* The node records the **seven** that carry `AFLG_COND`,
//! names about nine more in passing, and has no table.
//!
//! The reading was right and the place it was written down does not exist. That matters beyond
//! bookkeeping: the Interaction Editor is the only shipped declarative interaction layer this
//! corpus has read at its own scope, `interaction-table` has already compared its **predicate**
//! half to this design item for item — *"the gap is real and it is exactly one thing:
//! composition"* — and the **effect** half was never compared, because the effects were not
//! written anywhere to compare.
//!
//! What `effect-half` settled instead rests on one game: Delores, 125 write sites, 120 of them
//! onto eight effects. This table is a second population from a different engine and a different
//! decade, and it is the one that can say whether *"in every engine the effect list and the
//! presentation list are the same list"* is about engines or about that engine.

use design_gate::repo_root;
use std::path::Path;

fn catalog(root: &Path) -> String {
    std::fs::read_to_string(root.join(".yidam/catalog/ags-source.md")).expect("readable")
}

fn node(root: &Path) -> String {
    std::fs::read_to_string(root.join(".yidam/corpus/data-format/interaction-table.yml"))
        .expect("readable")
}

/// Rows of a markdown table whose first cell is an integer — the shape the table is recorded in.
fn numbered_rows(src: &str) -> Vec<usize> {
    src.lines()
        .filter_map(|l| {
            let t = l.trim();
            let n = t.strip_prefix('|')?.split('|').next()?.trim();
            n.parse().ok()
        })
        .collect()
}

/// **The catalog delegates the table to a node that does not carry it.**
///
/// Pinned as a census. The entry's sentence is precise about what was read and wrong about where
/// it went, which is the hardest kind of claim to catch: nothing about it is vague.
#[test]
fn the_table_is_not_where_the_catalog_says_it_is() {
    let root = repo_root();
    let entry = catalog(&root);
    assert!(
        entry.contains("Recorded in\n  full in `data-format/interaction-table.yml`")
            || entry.contains("Recorded in\n  full in"),
        "the entry no longer delegates the table, which is the census this pins"
    );
    assert_eq!(
        numbered_rows(&node(&root)).len(),
        7,
        "rows of the action table the node carries, against the forty-eight it is said to hold"
    );
    assert_eq!(
        numbered_rows(&entry).len(),
        0,
        "and the entry carries none either, so the table is in neither place"
    );
}

/// **Seven of the forty-eight are predicates, and the other forty-one have never been counted.**
///
/// `interaction-table` compared the predicate half in full and concluded that AGS had predicates
/// over world state and no way to combine them. The forty-one that *do* something were left as a
/// number in a sentence.
#[test]
fn only_the_predicate_half_has_been_read_against_this_design() {
    let root = repo_root();
    let src = node(&root);
    assert!(
        src.contains("NUM_ACTION_TYPES` is **48**"),
        "the node states the size of the table"
    );
    assert!(
        src.contains("Seven carry it"),
        "and enumerates the seven predicates"
    );
    // The effect half: named nowhere, by any of the words that would name it.
    for absent in [
        "presentation*",
        "forty-one",
        "41 of the",
        "effect half of the table",
    ] {
        assert!(
            !src.contains(absent),
            "the node already partitions the other forty-one: {absent:?}"
        );
    }
}

/// **`effect-half` rests on one game, and says so.**
///
/// *"A shipped adventure game's entire state-writing vocabulary is ten operations, 120 of its 125
/// write sites map onto the existing eight."* One game, one language, one decade. The claim it
/// makes is about **every** engine — that the effect list and the presentation list are the same
/// list — and a claim of that shape wants a second population.
#[test]
fn the_effect_vocabulary_was_settled_on_a_single_game() {
    let root = repo_root();
    let record =
        std::fs::read_to_string(root.join(".yidam/decisions/effect-half.yml")).expect("readable");
    assert!(
        record.contains("125") && record.contains("in every engine"),
        "the record's figure and the scope of its claim"
    );
    assert!(
        !record.contains("actiontype") && !record.contains("NUM_ACTION_TYPES"),
        "and it does not reach for the other table, which was two documents away"
    );
}
