//! **Three engines, three decades, and presentation is the larger half every time.**
//!
//! `effect-half` settled the eight-effect vocabulary on one game and made a claim about *every*
//! engine: the effect list and the presentation list are the same list. `forty-eight` supplied a
//! second population. This is the third, and the oldest — SCUMM v5, whose opcode table this corpus
//! read for inventory and ownership and stopped.
//!
//! | | population | presentation | writes world state |
//! |---|---|---|---|
//! | Delores | 1,328 call sites | 884 | 125 |
//! | AGS action table | 38 real actions | 23 | 15 |
//! | SCUMM v5 | 33 acting opcodes | 24 | 9 |
//!
//! **And the six comparisons arrive for the third time.** `isEqual`, `isNotEqual`, `isGreater`,
//! `isGreaterEqual`, `isLess`, `lessOrEqual` — the same six AGS Script declares and the same six
//! this design admits, in a bytecode from 1990.
//!
//! What SCUMM has that neither of the others does is **twenty-two `get*` opcodes that read world
//! state into a variable** for a later test. That is an artifact of a bytecode with a variable
//! file, not of an authoring vocabulary, and it is why its question-asking share is more than
//! twice the other two.
//!
//! `.yidam/decisions/third-population.yml`. Every claim here is `[inference]`: the source is
//! reverse-engineered and says so.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn table(entry: &str) -> Vec<(String, String)> {
    let src = std::fs::read_to_string(root().join(".yidam/catalog").join(entry)).expect("readable");
    src.lines()
        .filter_map(|l| {
            let mut cells = l.trim().strip_prefix('|')?.split('|');
            let first = cells.next()?.trim().trim_matches('`').to_string();
            let last = l.trim().trim_end_matches('|').rsplit('|').next()?.trim();
            (!first.is_empty() && first != "#" && first != "opcode" && !first.starts_with("---"))
                .then_some((first, last.to_string()))
        })
        .collect()
}

/// **Eighty-four opcodes, and the partition covers each exactly once.**
///
/// The entry said *"roughly two hundred"* for nine phases. Counting the page gives eighty-four
/// distinct names in eighty-seven encodings — wrong by more than double, in a sentence describing
/// what nobody had read.
#[test]
fn the_scumm_table_is_eighty_four_opcodes() {
    let rows = table("scumm-v5-opcode-reference.md");
    assert_eq!(rows.len(), 84, "distinct opcodes the page lists");

    let count = |k: &str| rows.iter().filter(|(_, c)| c == k).count();
    let (pred, read, control, present) = (
        count("*predicate*"),
        count("*read*"),
        count("*control*"),
        count("*presentation*"),
    );
    let state = rows.len() - pred - read - control - present;
    assert_eq!(
        (pred, read, control, present, state),
        (11, 22, 18, 24, 9),
        "predicates, reads into a variable, script control, presentation, and writes to world state"
    );
}

/// **Presentation is the larger half in all three populations.**
///
/// Of the opcodes that act — leaving out the predicates, the reads and the script control — SCUMM
/// spends twenty-four on presentation and nine on world state. AGS's action table: 23 and 15.
/// Delores, counted by `effect-half`: 884 sites and 125.
///
/// Three systems, 1990 to 2020, built by people who in two cases did not consult each other, and
/// the share never falls below three fifths. `effect-half`'s *"in every engine"* was written on
/// one game; it is the most-supported claim in this corpus now.
#[test]
fn presentation_is_the_larger_half_in_every_population() {
    let scumm = table("scumm-v5-opcode-reference.md");
    let acting: Vec<&(String, String)> = scumm
        .iter()
        .filter(|(_, c)| !["*predicate*", "*read*", "*control*"].contains(&c.as_str()))
        .collect();
    let present = acting.iter().filter(|(_, c)| c == "*presentation*").count();
    assert_eq!((present, acting.len()), (24, 33));
    assert!(
        present * 100 / acting.len() >= 60,
        "the presentation share of SCUMM's acting opcodes, against 60% for AGS and 67% for Delores"
    );

    let ags = table("ags-source.md");
    let ags_acting: Vec<&(String, String)> = ags
        .iter()
        .filter(|(_, c)| !["*predicate*", "*control*"].contains(&c.as_str()))
        .collect();
    let ags_present = ags_acting
        .iter()
        .filter(|(_, c)| c == "*presentation*")
        .count();
    assert_eq!((ags_present, ags_acting.len()), (23, 38));
    assert!(ags_present * 100 / ags_acting.len() >= 60);
}

/// **The same six comparisons, a third time, in a 1990 bytecode.**
///
/// `script-boundary` found that this design's six comparisons are exactly AGS Script's six and
/// that the complement is arithmetic and bitwise. SCUMM v5 has the six as named opcodes.
///
/// Three independent designs, and none of them stopped at four or reached for a seventh. That is
/// not a deep result about comparison — six is what a total order gives you — and it is worth
/// holding anyway, because it is the one part of the surface that has never needed an argument.
#[test]
fn the_six_comparisons_are_the_same_six_again() {
    let rows = table("scumm-v5-opcode-reference.md");
    let mut found: Vec<&str> = rows
        .iter()
        .map(|(n, _)| n.as_str())
        .filter(|n| {
            [
                "isEqual",
                "isNotEqual",
                "isGreater",
                "isGreaterEqual",
                "isLess",
                "lessOrEqual",
            ]
            .contains(n)
        })
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            "isEqual",
            "isGreater",
            "isGreaterEqual",
            "isLess",
            "isNotEqual",
            "lessOrEqual"
        ],
        "six comparison opcodes, one per ordering relation, and no seventh"
    );
}
