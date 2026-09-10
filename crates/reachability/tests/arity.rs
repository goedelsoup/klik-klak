//! Whether a binary verb earns a whole dimension.
//!
//! `verb/use-with` seeded the sharpest form of this corpus's central question: *"A vocabulary
//! without it is meaningfully smaller — not by one entry, but by a whole dimension. Whether the
//! puzzles it enables can be expressed by unary verbs over a richer world state is `[open]`, and
//! the answer decides whether 'minimal' means fewer verbs or simpler ones."*
//!
//! It is answerable mechanically, because the reduction is already implemented. `offered` gates a
//! binding's second object on `items[o] >= 1`, and `Pred::Carrying(o)` evaluates to
//! `items[o] >= 1`. The same expression, reached two ways. So moving the object into the guard is
//! not an approximation of the binary form — it is the binary form, spelled differently, and the
//! tests below hold that against every specimen.
//!
//! `.yidam/decisions/binary-dimension.yml` is the record.

use game_format::{Binding, Game, Pred};
use reachability::{analyse, Cause};

const FERRY: &str = include_str!("../../../design/specimen/ferry.game");
const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");
const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");
const PATROL: &str = include_str!("../../../design/specimen/patrol.game");
const ALL: [(&str, &str); 4] = [
    ("ferry", FERRY),
    ("lighthouse", LIGHTHOUSE),
    ("nightwatch", NIGHTWATCH),
    ("patrol", PATROL),
];

fn load(source: &str) -> Game {
    let (mut g, mut p) = game_format::parse(source, "arity.game");
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{msgs:?}");
    g
}

/// The reduction, applied to the model rather than to the text: every binding's second object
/// becomes a `carrying` conjunct on its guard. No verb is renamed and no verb is added, because
/// what is being tested is whether *arity* costs the analysis anything — not what the resulting
/// game would be called.
fn unarise(g: &mut Game) -> usize {
    let mut moved = 0;
    for b in &mut g.bindings {
        if let Some(o) = b.object.take() {
            moved += 1;
            let carrying = Pred::Carrying(o);
            b.when = Some(match b.when.take() {
                Some(q) => Pred::And(Box::new(carrying), Box::new(q)),
                None => carrying,
            });
        }
    }
    moved
}

fn binary_count(g: &Game) -> usize {
    g.bindings
        .iter()
        .filter(|b: &&Binding| b.object.is_some())
        .count()
}

/// **The answer, and it is exact rather than approximate.** Every figure the checker reports is
/// unchanged on every specimen: the same reachable set, the same transitions, the same strandings,
/// the same winnability, the same latch violations. Arity costs the state space nothing.
#[test]
fn moving_the_second_object_into_the_guard_changes_no_figure() {
    for (name, src) in ALL {
        let before = load(src);
        let n = binary_count(&before);
        assert!(n > 0, "{name} has no binary binding to reduce");

        let mut after = load(src);
        assert_eq!(unarise(&mut after), n);
        assert_eq!(binary_count(&after), 0);

        let a = analyse(&before);
        let b = analyse(&after);
        assert_eq!(
            (
                a.states,
                a.transitions,
                a.winnable,
                a.stranded,
                a.strandings.len()
            ),
            (
                b.states,
                b.transitions,
                b.winnable,
                b.stranded,
                b.strandings.len()
            ),
            "{name} analysed differently once its verbs were made unary:\n{}\n---\n{}",
            a.render(),
            b.render()
        );
        assert_eq!(a.observed_space, b.observed_space, "{name}");
        assert_eq!(a.latch_violations, b.latch_violations, "{name}");
    }
}

/// **What it costs is the report, and the specimen's committed defect is where that shows.**
/// `lighthouse.game` strands the player by pouring the oil into the wrong hinge, and the checker
/// names the object it was poured from. Reduced to a unary verb, the same states are still
/// stranded, by the same binding, for the same cause — and the report can no longer say what was
/// spent, because the object is now one conjunct of a guard rather than a field of the binding.
///
/// `verb/use-with` predicted exactly this half of the cost when it was seeded: *"every refusal has
/// to name which of the two objects is wrong."*
#[test]
fn what_it_costs_is_the_diagnostic_and_not_the_analysis() {
    let a = analyse(&load(LIGHTHOUSE));
    let mut reduced = load(LIGHTHOUSE);
    unarise(&mut reduced);
    let b = analyse(&reduced);

    assert_eq!(a.strandings.len(), 1);
    assert_eq!(b.strandings.len(), 1);
    let (x, y) = (&a.strandings[0], &b.strandings[0]);
    assert_eq!(x.cause, Cause::Consumption);
    assert_eq!(y.cause, Cause::Consumption);
    assert_eq!(x.target, y.target, "the same binding is still blamed");
    assert_eq!(x.object.as_deref(), Some("oil-can"));
    assert_eq!(
        y.object, None,
        "the reduced form cannot name what was spent, which is the whole of what it loses here"
    );
    assert!(a.render().contains("oil-can"));
    assert!(!b.render().contains(" oil-can on "));
}

/// The census the record quotes, held against the specimens.
///
/// Delores ships 203 bindings of which 26 take a second object — **12.8%** — across 16 distinct
/// binary verbs, with 18 declared binary in an engine list validated at boot and no generic
/// "use with" among them (`.yidam/catalog/delores-source.md`). These four specimens were written
/// without reference to that game and land in the same place.
#[test]
fn the_binary_share_is_about_an_eighth_in_both_bodies_of_work() {
    let total: usize = ALL.iter().map(|(_, s)| load(s).bindings.len()).sum();
    let binary: usize = ALL.iter().map(|(_, s)| binary_count(&load(s))).sum();
    assert_eq!(binary, 11);
    let share = binary as f64 / total as f64;
    assert!(
        (0.10..0.16).contains(&share),
        "binary share {share:.3} ({binary} of {total}); Delores is 0.128 and the record quotes both"
    );
}
