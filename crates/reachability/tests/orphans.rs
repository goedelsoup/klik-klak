//! **Four authored lines in a hundred and five are selected in no reachable state.**
//!
//! [`failure-mode/orphaned-dialogue-branch`](../../../.yidam/corpus/failure-mode/orphaned-dialogue-branch.yml)
//! was the last mode this corpus names with nothing to detect it. Its `detectable_at` property
//! said `authoring` since genesis and nothing made it true for eleven phases; its account of why
//! shipped games carry the defect — *"they simply never see it — which is why it survives in
//! shipped games and **why nobody counts it**"* — was true of this repository too.
//!
//! Both halves were present and unjoined: the parse tree has held each response with the line it
//! was written on since `.yidam/decisions/authored-text.yml`, and the search has enumerated the
//! reachable states since genesis. A line a player can reach is a line some reachable state
//! selects.
//!
//! **The scoping in that node is on the wrong axis and this is what showed it.** It says the mode
//! *"belongs to `branching-tree` specifically… A topic list has no paths, so it cannot orphan
//! anything"*. Not one of the four is in a conversation. **A path is not what orphans content — a
//! guard is**, and the guarded response list is what this vocabulary chose in place of a tree.
//!
//! `.yidam/decisions/orphaned-lines.yml`.

mod common;

use game_format::{parse, resolve, Game};
use reachability::analyse;
use std::path::{Path, PathBuf};

fn load(source: &str, name: &str) -> Game {
    let (mut game, problems) = parse(source, name);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    let problems = resolve(&mut game);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    game
}

fn specimens() -> Vec<(String, Game)> {
    common::sources()
        .into_iter()
        .map(|(n, s)| {
            let g = load(&s, &n);
            (n, g)
        })
        .collect()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/reachability is two levels below the root")
        .to_path_buf()
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **The four, named individually rather than counted.**
///
/// A bare count is `inert-reports`'s INFO line: it moves and nobody can say why. Named with their
/// lines, this says which line stopped being unreachable and which one started, and an author who
/// repairs `ferry.game:61` is told so by a failing test rather than by a number going down.
///
/// All four stay in the specimens. Three are the evidence for the structural finding and deleting
/// them would delete it; the fourth is a genuine authoring slip and is the proof the check is not
/// vacuous. A specimen is evidence before it is an example.
///
/// **A fifth game added forty-nine response clauses and not one orphan**, which is the first
/// evidence that the four are a property of how those two games are written rather than a rate.
/// The denominator moved by half and the numerator did not move at all.
#[test]
fn four_authored_lines_in_a_hundred_and_fifty_four_are_reached_by_nobody() {
    let mut clauses = 0;
    let mut found: Vec<String> = Vec::new();
    for (name, game) in specimens() {
        clauses += game
            .bindings
            .iter()
            .map(|b| b.says.len() + b.refusals.len())
            .sum::<usize>();
        let report = analyse(&game);
        // Not one of them is a binding nobody is shown: every orphan here is at the guard.
        assert!(
            report.unshown.is_empty(),
            "{name} has a binding in no menu, which orphans its lines for a different reason"
        );
        for o in report.orphaned {
            found.push(format!("{name}: {o}"));
        }
    }
    assert_eq!(
        clauses, 194,
        "authored response clauses across the specimens"
    );
    assert_eq!(
        found,
        [
            "ferry: line 61: the line under `look-at` at line 60 — written, and reached in no \
             state a player can get to",
            "lighthouse: line 184: the refusal under `use-with wick` at line 179 — written, and \
             reached in no state a player can get to",
            "lighthouse: line 192: the refusal under `use-with oil-can` at line 186 — written, \
             and reached in no state a player can get to",
            "lighthouse: line 201: the refusal under `use-with match` at line 194 — written, and \
             reached in no state a player can get to",
        ]
    );
}

/// **Three of the four are the vocabulary's doing: a binding that spends its own second object
/// can never decline.**
///
/// `arity-cost` keys a binary entry by the object carried — which is what pays for the arity — so
/// `spend`ing that object removes the entry in the same transition that makes the refusal true.
/// The *already done* refusal is the most natural line an author writes against a one-shot
/// binding, and it is the one this vocabulary guarantees nobody reads.
///
/// Checked as a rule rather than as three line numbers. Four `use-with` bindings in
/// `lighthouse.game` spend their own object; the three that carry a refusal have its last clause
/// — the one describing the world *after* the binding has run — orphaned, every time. The clauses
/// that survive are the ones about *not yet*, which an author reaches while still holding the
/// thing.
///
/// The fourth is the same fact in its degenerate form: `use-with oil-can` on the shutter at line
/// 131 has no guard, so it never declines and has no refusal to orphan. That binding is the trap
/// `lighthouse.game` was written for.
#[test]
fn a_binding_that_spends_its_own_object_cannot_decline() {
    let game = load(&common::source("lighthouse"), "lighthouse");
    let orphaned: Vec<usize> = analyse(&game)
        .orphaned
        .iter()
        .map(|o| {
            o.trim_start_matches("line ")
                .split(':')
                .next()
                .expect("a line")
                .parse()
                .expect("a number")
        })
        .collect();

    let mut spending = 0;
    let mut refusing = 0;
    for b in &game.bindings {
        let Some(object) = &b.object else { continue };
        if !b
            .effects
            .iter()
            .any(|e| matches!(e, game_format::Effect::Spend(x, _) if x == object))
        {
            continue;
        }
        spending += 1;
        let Some(after) = b.refusals.last() else {
            // ungated: it never declines, so there is nothing to be unreachable
            assert!(b.when.is_none(), "a guarded binding with no refusal");
            continue;
        };
        refusing += 1;
        assert!(
            orphaned.contains(&after.line),
            "`{} {object}` spends it and line {} is still reachable",
            b.verb,
            after.line
        );
        // and the clause about *not yet* is reachable, because the player holds the thing
        if let Some(before) = b.refusals.first().filter(|r| r.line != after.line) {
            assert!(
                !orphaned.contains(&before.line),
                "the *not yet* clause at line {} should be reachable",
                before.line
            );
        }
    }
    assert_eq!(
        spending, 4,
        "`use-with` bindings that spend their own object"
    );
    assert_eq!(refusing, 3, "and those of them that wrote a refusal");
    assert_eq!(orphaned.len(), 3, "every one of which is dead");
}

/// **The ferry's orphan is a one-way door read as an authoring problem.**
///
/// `paid` is set by the binding that crosses, and crossing is what the specimen exists to
/// demonstrate — *"THE TRAP. The boat does not come back."* So the flag becomes true and the room
/// holding the line that reads it becomes unreachable in the same act. The specimen was written
/// to exhibit `unwinnable-state` and exhibits this too, in the same construct.
#[test]
fn the_ferrymans_line_is_true_only_where_he_cannot_be_seen() {
    let game = load(&common::source("ferry"), "ferry");
    let orphan = game
        .bindings
        .iter()
        .flat_map(|b| &b.says)
        .find(|r| r.line == 61)
        .expect("the clause");
    assert_eq!(orphan.text, "He is already unmooring.");
    assert!(orphan.when.is_some(), "guarded on `paid`");

    let crossing = game
        .bindings
        .iter()
        .find(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, game_format::Effect::SetFlag(f) if f == "paid"))
        })
        .expect("the binding that pays");
    assert!(
        crossing
            .effects
            .iter()
            .any(|e| matches!(e, game_format::Effect::Move(None, _))),
        "paying moves the player in the same effect list that sets the flag"
    );

    assert!(
        common::source("ferry").contains("THE TRAP. The boat does not come back."),
        "and the specimen says the crossing is one-way"
    );
}

/// **The mode is counted, and the corpus no longer scopes it to a construct this design refused.**
///
/// The node keeps the original sentence and corrects it in place, which is this repository's rule
/// for a claim that was reasonable and wrong: the reader meets the argument and the refutation
/// together rather than only the conclusion.
#[test]
fn the_corpus_says_a_guard_orphans_and_not_only_a_path() {
    let root = repo_root();
    let node = std::fs::read_to_string(
        root.join(".yidam/corpus/failure-mode/orphaned-dialogue-branch.yml"),
    )
    .expect("the node");
    let node = flat(&node);
    assert!(
        node.contains(
            "It belongs to `branching-tree` specifically rather than to conversations generally. \
             A topic list has no paths, so it cannot orphan anything; a tree can."
        ),
        "the original claim is kept"
    );
    assert!(
        node.contains("A path is not what orphans content — a guard is."),
        "and corrected"
    );
    assert!(
        node.contains("not one of them is in a conversation"),
        "with what corrected it"
    );
    assert!(
        node.contains("the binding cannot be asked again, so it cannot decline."),
        "and the structural half"
    );
    assert!(
        node.contains("Authoring — and it is detected."),
        "`detectable_at` is true now and says so"
    );
}

/// **A clause is named by the resolver, because the caller counting them cannot name it.**
///
/// `select` returned `&str`. `clause` is the selection and `select` is `clause` plus `.text`, so
/// there is still one definition of first-match-wins and one evaluator —
/// `.yidam/decisions/two-components.yml`.
///
/// `Fired.says` carries the clause for the same reason, and that is what makes the census correct
/// rather than merely possible: a forced binding's line is read **between** two resting states.
/// Collected over the resting states alone, every `on-enter` line in four specimens reads as
/// unreachable.
#[test]
fn the_census_sees_a_line_said_between_two_resting_states() {
    let game = load(&common::source("nightwatch"), "nightwatch");
    let forced: Vec<usize> = game
        .bindings
        .iter()
        .filter(|b| matches!(b.site, game_format::Site::OnEnter { .. }))
        .flat_map(|b| &b.says)
        .map(|r| r.line)
        .collect();
    assert!(!forced.is_empty(), "the specimen has forced lines");

    let orphaned = analyse(&game).orphaned;
    assert!(
        orphaned.is_empty(),
        "and none of them is orphaned: {orphaned:?}"
    );

    // the 143-character watchman line `walked-in` found the engine dropping is one of them
    let longest = game
        .bindings
        .iter()
        .flat_map(|b| &b.says)
        .max_by_key(|r| r.text.chars().count())
        .expect("a line");
    assert_eq!(longest.line, 89);
    assert!(forced.contains(&longest.line), "and it is forced");
}
