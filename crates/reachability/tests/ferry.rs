//! The second specimen: a tally, and the two things a second one is for.
//!
//! It exists to answer whether `count item` earns its place, and it carries an **ordering**
//! defect where the lighthouse carries a consumption one — two specimens, the two causes
//! `failure-mode/unwinnable-state` names, one each.

use game_format::{Cmp, Pred};
use reachability::{analyse, Cause};

const FERRY: &str = include_str!("../../../design/specimen/ferry.game");

fn load(source: &str) -> game_format::Game {
    let (mut game, problems) = game_format::parse(source, "ferry.game");
    let problems: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
        .map(|p| p.to_string())
        .collect();
    assert!(
        problems.is_empty(),
        "ferry does not resolve:\n{}",
        problems.join("\n")
    );
    game
}

#[test]
fn the_toll_is_a_count_and_the_coin_is_stackable() {
    let game = load(FERRY);
    assert_eq!(game.item_cap.get("coin").copied(), Some(4));
    assert_eq!(game.item_cap.get("lantern").copied(), Some(1));
    let toll = game
        .bindings
        .iter()
        .find(|b| b.object.as_deref() == Some("coin"))
        .expect("the toll");
    match toll.when.as_ref().expect("the toll is gated") {
        Pred::Count(item, Cmp::Ge, 3) => assert_eq!(item, "coin"),
        other => panic!("the toll should be a count: {other:?}"),
    }
}

#[test]
fn the_ending_is_reachable() {
    let r = analyse(&load(FERRY));
    assert!(r.winnable);
    assert!(r.overflows.is_empty(), "{:?}", r.overflows);
}

/// The lighthouse's committed defect is consumption. This one's is ordering, and the two are
/// told apart by a counterfactual rather than by which effects the binding carries — the toll
/// spends three coins *and* crosses a river, and the coins are not why the player is stranded.
#[test]
fn the_one_committed_defect_is_an_ordering_trap() {
    let r = analyse(&load(FERRY));
    assert_eq!(
        r.strandings.len(),
        1,
        "exactly one defect; got:\n{}",
        r.render()
    );
    let s = &r.strandings[0];
    assert_eq!(
        s.cause,
        Cause::Ordering,
        "the toll spends coins and is not a consumption trap"
    );
    assert_eq!(s.target, "ferryman");
}

/// **The state space closes only because the coin declares a ceiling.**
///
/// Clearing the flag that guards the jar makes it an unbounded source: it hands out three
/// coins every time it is used, forever. Loops and recursion were excluded from this language
/// so that *evaluation* terminates; this is *analysis* terminating, which is a different
/// property, and quantity is the one thing that breaks it. Found by running exactly this
/// mutation and watching the search fail to stop.
#[test]
fn an_unbounded_source_is_reported_rather_than_chased() {
    // The mutation is an *omission*, and it was not always. It used to append
    // `do clear jar-emptied` beside the `set`, which `effect-list-semantics` made
    // inexpressible: two writes to one flag in one list is a dead write, since nothing reads
    // the value in between. Forgetting to latch the source is the realistic mistake anyway.
    let broken = FERRY.replace("      do    set jar-emptied\n", "");
    assert_ne!(broken, FERRY);
    let r = analyse(&load(&broken));
    assert!(
        !r.overflows.is_empty(),
        "an unbounded supply must be reported; got:\n{}",
        r.render()
    );
    assert!(r.overflows.iter().any(|o| o.contains("coin")));
}

/// The collapse `world-flag/latched-flag` argues for, as a number rather than an argument.
#[test]
fn the_reachable_set_is_a_fraction_of_the_space() {
    let r = analyse(&load(FERRY));
    assert!(r.observed_space > r.states as u128);
}
