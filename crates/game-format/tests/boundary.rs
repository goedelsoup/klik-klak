//! **The bounded predicate language is the escape language's comparison-and-logical fragment.**
//!
//! `.yidam/catalog/ags-manual.md` described the scripting language reference, for eight phases,
//! as *"where the boundary between declarative authoring and general-purpose code actually sits,
//! which is this corpus's central question"* — and listed it as unread. The corpus knew the size
//! of the room the hatch was in and nothing about what was through it.
//!
//! Read, the boundary is exact. AGS Script declares **twenty-one operators** and **nine of them
//! are this design's nine**: six comparisons and three logical connectives, identical, with the
//! arithmetic and the bitwise removed. Not a different vocabulary chosen for a different purpose
//! — the same one truncated.
//!
//! **And both systems had to answer termination.** AGS answers at runtime by guessing: a `while`
//! loop past 150,000 iterations is assumed hung and the game aborts, with `noloopcheck` shipped to
//! turn the guess off. This design answers structurally — no loops, no recursion — and
//! `ferry.game` records the one remaining infinity closed by a declared ceiling.
//!
//! `.yidam/decisions/script-boundary.yml`.

use game_format::{Cmp, Pred};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn entry() -> String {
    std::fs::read_to_string(root().join(".yidam/catalog/ags-manual.md")).expect("readable")
}

/// The operator spellings the catalog entry records, from one row of its comparison table.
fn recorded(row: &str) -> Vec<String> {
    let src = entry();
    let line = src
        .lines()
        .find(|l| l.trim_start().starts_with(&format!("| {row} |")))
        .unwrap_or_else(|| panic!("the entry no longer records a {row:?} row"));
    line.split('|')
        .nth(2)
        .expect("the AGS column")
        .split('`')
        .skip(1)
        .step_by(2)
        .map(|s| s.replace("&#124;", "|"))
        .collect()
}

/// **Every comparison this design admits is one AGS Script admits, and there are no others.**
///
/// Read off `Cmp::spelling` rather than a list, so adding a seventh comparison here fails until
/// somebody checks whether the escape language has it — which is the question that matters, since
/// a construct outside the fragment is a construct the escape cannot be a fallback for.
#[test]
fn the_comparisons_are_exactly_the_escape_languages() {
    let mine: Vec<&str> = [Cmp::Eq, Cmp::Ne, Cmp::Lt, Cmp::Le, Cmp::Gt, Cmp::Ge]
        .iter()
        .map(|c| c.spelling())
        .collect();
    let theirs = recorded("comparison");
    // `=` here is `==` there: one language assigns nothing, so it has the shorter spelling free.
    let mine_as_theirs: Vec<String> = mine
        .iter()
        .map(|s| {
            if *s == "=" {
                "==".to_string()
            } else {
                (*s).to_string()
            }
        })
        .collect();
    let (mut a, mut b) = (mine_as_theirs, theirs);
    a.sort();
    b.sort();
    assert_eq!(a, b, "the same six comparisons in both languages");
}

/// **Nine of twenty-one, and the nine are the boolean half.**
///
/// The arithmetic and the bitwise are what was left behind: five arithmetic operators, two
/// increment forms, five bitwise. `data-format/interaction-table` had already found that AGS's
/// *declarative* table has no arithmetic either — *"`Game - Set variable value` assigns and
/// nothing increments"* — so the thing this design excluded was excluded by the layer it is
/// modelled on as well, and is available only through the hatch.
#[test]
fn the_predicate_language_is_nine_of_twenty_one_operators() {
    let src = entry();
    let (mut mine, mut theirs) = (0, 0);
    for row in ["comparison", "logical", "arithmetic", "bitwise"] {
        theirs += recorded(row).len();
        let line = src
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("| {row} |")))
            .expect("the row");
        let ours = line.split('|').nth(3).expect("this design's column").trim();
        mine += match ours {
            "none" => 0,
            "the same six" => 6,
            other => other.matches('`').count() / 2,
        };
    }
    assert_eq!(
        (mine, theirs),
        (9, 21),
        "this design's operators, and AGS Script's"
    );

    // and the nine are the whole predicate vocabulary: three connectives over six comparisons
    let connectives = {
        fn count(p: &Pred) -> usize {
            match p {
                Pred::Not(q) => 1 + count(q),
                Pred::And(a, b) | Pred::Or(a, b) => 1 + count(a) + count(b),
                _ => 0,
            }
        }
        count(&Pred::Not(Box::new(Pred::And(
            Box::new(Pred::Or(
                Box::new(Pred::Flag("a".into())),
                Box::new(Pred::Flag("b".into())),
            )),
            Box::new(Pred::Flag("c".into())),
        ))))
    };
    assert_eq!(connectives, 3, "not, and, or — and nothing else combines");
}

/// **Two systems, two answers to termination, and only one of them is a guarantee.**
///
/// AGS: *"if a `while` loop runs for more than 150,000 loops, AGS will assume that the script has
/// hung and abort the game"*, with `noloopcheck` to disable the assumption per function. That is a
/// heuristic with a documented escape from the heuristic, in the language the declarative layer
/// escapes to.
///
/// Here: no loop construct exists, so the question does not arise for control flow. The one place
/// it can still arise is an unbounded item quantity, which `ferry.game` closes by declaration —
/// *"The ceiling restores it by declaration, the way a counter's range already did."*
#[test]
fn termination_is_answered_by_construction_here_and_by_a_guess_there() {
    let src = entry();
    assert!(
        src.contains("150,000 loops") && src.contains("noloopcheck"),
        "the entry records the heuristic and the keyword that disables it"
    );

    let keywords = include_str!("../src/parse.rs");
    for loop_word in ["\"while\"", "\"for\"", "\"repeat\"", "\"each\""] {
        assert!(
            !keywords.contains(loop_word),
            "the parser dispatches on {loop_word}, so evaluation no longer terminates by \
             construction and the argument in .yidam/decisions/script-boundary.yml is void"
        );
    }

    let ferry =
        std::fs::read_to_string(root().join("design/specimen/ferry.game")).expect("readable");
    assert!(
        ferry.contains("INFINITE") && ferry.contains("ceiling restores it by declaration"),
        "and the one remaining source of infinity is closed where it was found"
    );
}
