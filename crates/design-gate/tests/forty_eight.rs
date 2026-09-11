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

/// **The table is recorded where the source is, and the node is not carrying a copy.**
///
/// The entry delegated its own contents to `data-format/interaction-table.yml` for six phases and
/// that node carried the seven predicates. The delegation is reversed rather than honoured: a
/// catalog entry is the record of what a source contains, a node is the argument drawn from it,
/// and that node is already the largest in this corpus.
#[test]
fn the_table_is_recorded_where_the_source_is() {
    let root = repo_root();
    let rows = numbered_rows(&catalog(&root));
    assert_eq!(rows.len(), 48, "every entry of `NUM_ACTION_TYPES`");
    assert_eq!(
        rows,
        (0..48).collect::<Vec<_>>(),
        "in table order, because the index is the on-disk encoding"
    );
    assert_eq!(
        numbered_rows(&node(&root)).len(),
        7,
        "the node keeps the predicates it compared and does not carry the table twice"
    );
}

/// **Every effect the table maps onto is an effect the format declares.**
///
/// The wire. The last column of the table is this corpus's reading of AGS's vocabulary in terms of
/// its own, so it is a claim about two files at once: if `Effect` loses a variant or renames one,
/// a row of a 1999 action table stops naming anything and this fails.
///
/// The mapping is total in one direction and not the other. Fifteen actions write world state and
/// all fifteen map; `leave` is the one effect here with no action to map from, because AGS's
/// dialogs end themselves.
#[test]
fn every_effect_the_table_names_is_one_the_format_has() {
    let root = repo_root();
    let src =
        std::fs::read_to_string(root.join("crates/game-format/src/lib.rs")).expect("readable");
    let declared: Vec<String> = src
        .split("pub enum Effect {")
        .nth(1)
        .and_then(|s| s.split('}').next())
        .expect("the effect enum")
        .lines()
        .filter_map(|l| {
            let v = l.trim().split(['(', '{', ',']).next()?.trim();
            (!v.is_empty() && v.starts_with(char::is_uppercase)).then(|| {
                // `SetFlag` is written `set-flag` where an author writes it
                let mut out = String::new();
                for (i, c) in v.char_indices() {
                    if c.is_uppercase() && i > 0 {
                        out.push('-');
                    }
                    out.push(c.to_ascii_lowercase());
                }
                out
            })
        })
        .collect();
    assert_eq!(declared.len(), 8, "the effect vocabulary");

    let mut named: Vec<String> = Vec::new();
    for line in catalog(&root).lines() {
        let t = line.trim();
        if t.strip_prefix('|')
            .and_then(|r| r.split('|').next()?.trim().parse::<usize>().ok())
            .is_none()
        {
            continue;
        }
        let Some(cell) = t.trim_end_matches('|').rsplit('|').next() else {
            continue;
        };
        for e in cell.split(',') {
            let e = e.trim().trim_matches('`').trim();
            if e.is_empty() || e.starts_with('*') {
                continue;
            }
            named.push(e.to_string());
        }
    }
    let mut unknown: Vec<&String> = named.iter().filter(|e| !declared.contains(e)).collect();
    unknown.dedup();
    assert!(
        unknown.is_empty(),
        "the table maps an action onto something the format does not have: {unknown:?}"
    );

    named.sort();
    named.dedup();
    let mut missing: Vec<&String> = declared.iter().filter(|d| !named.contains(d)).collect();
    missing.sort();
    assert_eq!(
        missing,
        [&"leave".to_string()],
        "the effects with no action to map from — AGS enters a conversation and never leaves one"
    );
}

/// **The partition covers the table exactly once.**
///
/// Four kinds and no row in two of them: seven predicates, three that act on the command list,
/// twenty-three that write no world state, fifteen that do. The middle number is the finding —
/// **61% of a shipped authoring vocabulary is presentation and timing** — and it is only a finding
/// if nothing is double-counted.
#[test]
fn the_partition_covers_the_table_exactly_once() {
    let root = repo_root();
    let entry = catalog(&root);
    let (mut predicate, mut control, mut presentation, mut effect) = (0, 0, 0, 0);
    for line in entry.lines() {
        let t = line.trim();
        if t.strip_prefix('|')
            .and_then(|r| r.split('|').next()?.trim().parse::<usize>().ok())
            .is_none()
        {
            continue;
        }
        let cell = t
            .trim_end_matches('|')
            .rsplit('|')
            .next()
            .unwrap_or("")
            .trim();
        match cell {
            "*predicate*" => predicate += 1,
            "*control*" => control += 1,
            "*presentation*" => presentation += 1,
            _ => effect += 1,
        }
    }
    assert_eq!(
        (predicate, control, presentation, effect),
        (7, 3, 23, 15),
        "predicates, command-list control, presentation or timing, and writes to world state"
    );
    assert_eq!(predicate + control + presentation + effect, 48);
    assert_eq!(
        presentation * 100 / (presentation + effect),
        60,
        "the share of the real actions that write nothing a predicate could read"
    );
}

/// **The effect vocabulary now rests on two populations, and they agree.**
///
/// `effect-half` settled eight effects on Delores — ten operations, 120 of 125 write sites — and
/// made a claim about *every* engine on the evidence of one. This table is the second, from a
/// different engine, decade and authoring model, and it says the same thing: most of what a
/// shipped authoring surface offers does not write world state, and what does fits in eight.
#[test]
fn the_effect_vocabulary_rests_on_two_populations() {
    let root = repo_root();
    let settled =
        std::fs::read_to_string(root.join(".yidam/decisions/effect-half.yml")).expect("readable");
    assert!(
        settled.contains("125") && settled.contains("in every engine"),
        "the record's figure and the scope of its claim"
    );
    let second =
        std::fs::read_to_string(root.join(".yidam/decisions/forty-eight.yml")).expect("readable");
    assert!(
        second.contains("effect-half") && second.contains("second population"),
        "and the record that supplies the second says which claim it is under"
    );
}
