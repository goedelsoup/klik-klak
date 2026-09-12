//! **Every figure the checker publishes survives a round trip through the writer.**
//!
//! `crates/game-format/tests/round_trip.rs` compares the two trees. This compares what the
//! instrument makes of them, which is the claim anybody actually depends on: a tree that came
//! back subtly different would still be a tree, and the way to notice is to ask the thing that
//! reads it for everything it knows.
//!
//! The report is compared whole — states, transitions, winnability, strandings with their causes
//! and example states, orphaned lines, unclaimed latches, counter monotonicity — and the only
//! difference allowed is the **line numbers**, which must move because the writer emits no
//! comments and no blank lines. `.yidam/decisions/round-trip.yml`.

use game_format::{parse, print, resolve, Game};
use reachability::analyse;

const ALL: [(&str, &str); 4] = [
    ("ferry", include_str!("../../../design/specimen/ferry.game")),
    (
        "lighthouse",
        include_str!("../../../design/specimen/lighthouse.game"),
    ),
    (
        "nightwatch",
        include_str!("../../../design/specimen/nightwatch.game"),
    ),
    (
        "patrol",
        include_str!("../../../design/specimen/patrol.game"),
    ),
];

fn load(source: &str, name: &str) -> Game {
    let (mut game, problems) = parse(source, name);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    let problems = resolve(&mut game);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    game
}

/// A rendered report with every line number blanked.
fn without_lines(report: &str) -> String {
    report
        .split_whitespace()
        .map(|w| {
            // `line 61:` carries its colon into the token, and a comparison that missed it
            // would be blanking some line numbers and comparing others
            let bare = w.trim_end_matches([':', ',', '.']);
            if !bare.is_empty() && bare.chars().all(|c| c.is_ascii_digit()) {
                format!("<n>{}", &w[bare.len()..])
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// **The same report, on the same game, written out and read back.**
///
/// Not a sample of it. `render` is everything the instrument has to say, and comparing the whole
/// string is what makes this a check on the writer rather than on whichever three numbers
/// somebody thought to assert.
#[test]
fn the_checker_says_the_same_thing_about_the_game_it_wrote_out() {
    for (name, src) in ALL {
        let original = load(src, name);
        let round_tripped = load(&print(&original), name);
        let (before, after) = (
            analyse(&original).render(),
            analyse(&round_tripped).render(),
        );
        assert_ne!(before, after, "{name}: the line numbers should have moved");
        assert_eq!(
            without_lines(&before),
            without_lines(&after),
            "{name}: the checker does not recognise the game it wrote out"
        );
    }
}

/// **And the numbers that are not line numbers are identical as numbers, not as text.**
///
/// The comparison above blanks every digit, which would hide a state count changing as surely as
/// it hides a line moving. These are the figures four phases of records quote, read off the
/// struct.
#[test]
fn the_figures_records_quote_do_not_move() {
    let mut seen = Vec::new();
    for (name, src) in ALL {
        let original = load(src, name);
        let round_tripped = load(&print(&original), name);
        let (a, b) = (analyse(&original), analyse(&round_tripped));
        assert_eq!(
            (a.states, a.transitions, a.winnable, a.stranded),
            (b.states, b.transitions, b.winnable, b.stranded),
            "{name}: the graph moved"
        );
        assert_eq!(
            (
                a.strandings.len(),
                a.orphaned.len(),
                a.silent.len(),
                a.unshown.len(),
                a.unclaimed_latches.len(),
                a.observed_space
            ),
            (
                b.strandings.len(),
                b.orphaned.len(),
                b.silent.len(),
                b.unshown.len(),
                b.unclaimed_latches.len(),
                b.observed_space
            ),
            "{name}: a finding moved"
        );
        seen.push((name, a.states, a.transitions, a.strandings.len()));
    }
    assert_eq!(
        seen,
        [
            ("ferry", 20, 34, 1),
            ("lighthouse", 91, 173, 1),
            ("nightwatch", 14, 31, 2),
            ("patrol", 20, 33, 1),
        ],
        "the four specimens, as `design/specimen/README.md` states them"
    );
}
