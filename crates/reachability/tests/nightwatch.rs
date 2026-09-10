//! The third specimen, for one question: does `cutscene` earn its place?
//!
//! `cutscene.ont.yml` seeded the class while doubting it and named the reduction to argue
//! against — "an interaction whose response is long and whose input is locked". This game
//! wants a sequence twice, in two shapes, so that the argument has something to be about.
//!
//! One shape reduced to a binding on the first reading. The other did not, and what failed
//! was the *trigger* — which is now `on-enter`, and this file is what changed when it arrived.

use game_format::{Effect, Site};
use reachability::{analyse, Cause};

const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");

fn load(source: &str) -> game_format::Game {
    let (mut game, problems) = game_format::parse(source, "nightwatch.game");
    let problems: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
        .map(|p| p.to_string())
        .collect();
    assert!(
        problems.is_empty(),
        "nightwatch does not resolve:\n{}",
        problems.join("\n")
    );
    game
}

#[test]
fn the_specimen_parses_and_is_winnable() {
    let game = load(NIGHTWATCH);
    assert_eq!(game.start_room.as_deref(), Some("yard"));
    let r = analyse(&game);
    assert!(
        r.winnable,
        "a specimen that cannot be finished is not a game"
    );
    assert!(r.entry_cycles.is_empty(), "{:?}", r.entry_cycles);
}

/// **The reduction, asserted.** `study.case` is a whole scripted beat — the wire goes in past
/// the wafers, the lock turns over, the lid comes up on its own weight — and it is one binding
/// with one effect. Everything that makes it a *scene* is in the `say`, and `say` is
/// presentation: `effect-list-semantics` puts it outside the state model and `Binding` has no
/// field for it.
///
/// So "the response is long" is not a fact about the vocabulary, and the first half of
/// `cutscene`'s reduction goes through.
#[test]
fn a_click_triggered_sequence_is_one_binding() {
    let game = load(NIGHTWATCH);
    let beat = game
        .bindings
        .iter()
        .find(|b| b.verb == "use-with" && b.object.as_deref() == Some("wire"))
        .expect("the case");
    assert_eq!(beat.effects.len(), 1, "a scripted beat, and one write");
    assert!(beat.when.is_some(), "gated, so it plays once");
}

/// **What the arrival trigger bought, structurally.** The sequence used to be written once per
/// door into `hall` — four bindings for one intent, and drifting one copy changed the game
/// while every finding stayed identical. It is one binding now, at a site no verb can name,
/// and there is nothing left to keep in step.
#[test]
fn the_sequence_is_one_binding_at_one_site() {
    let game = load(NIGHTWATCH);
    let entries: Vec<_> = game
        .bindings
        .iter()
        .filter(|b| matches!(b.site, Site::OnEnter { .. }))
        .collect();
    assert_eq!(entries.len(), 1, "one sequence, one home");
    match &entries[0].site {
        Site::OnEnter { room } => assert_eq!(room, "hall"),
        other => panic!("wrong site: {other:?}"),
    }

    // and nothing else in the game spends the ledger — the copies are gone rather than moved
    let spenders: Vec<usize> = game
        .bindings
        .iter()
        .filter(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, Effect::Spend(x, _) if x == "ledger"))
        })
        .map(|b| b.line)
        .collect();
    assert_eq!(spenders.len(), 1, "one binding takes the ledger");
}

/// **The primitive is faithful to the game it replaced.** The two-copy version reported 14
/// reachable states and 31 transitions. So does this one. A rewrite that changed the numbers
/// would have changed the game rather than only how it is written down.
#[test]
fn the_rewrite_did_not_change_the_game() {
    let r = analyse(&load(NIGHTWATCH));
    assert_eq!(r.states, 14);
    assert_eq!(r.transitions, 31);
}

/// **Two reports remain, and they are no longer two copies of anything.** There are two doors
/// into the hall, so there are two ways for a player to make the mistake — and a report naming
/// both is describing the game rather than repeating itself. What changed is that neither
/// names the sequence: the blame lands on the door the player chose.
#[test]
fn the_blame_lands_on_the_chosen_binding_not_the_forced_one() {
    let game = load(NIGHTWATCH);
    let entry_line = game
        .bindings
        .iter()
        .find(|b| matches!(b.site, Site::OnEnter { .. }))
        .expect("the entry binding")
        .line;

    let r = analyse(&game);
    assert_eq!(r.strandings.len(), 2, "two doors, two choices");
    assert!(r.strandings.iter().all(|s| s.verb == "walk-to"));
    assert!(
        r.strandings.iter().all(|s| s.line != entry_line),
        "a forced binding is nothing the player chose, so it cannot be the blame"
    );
    let targets: Vec<&str> = r.strandings.iter().map(|s| s.target.as_str()).collect();
    assert!(targets.contains(&"yard.hall-door"));
    assert!(targets.contains(&"study.hall-door"));
}

/// **And the cause survived the change, which it did not do for free.** The classifier reads a
/// binding's effect list to decide what it spent, and `walk-to` spends nothing — the watchman
/// does, on the other side of the door. Following the forced chain is what keeps this
/// `Consumption` rather than the `Ordering` it briefly reported.
#[test]
fn the_cause_follows_the_forced_chain() {
    let r = analyse(&load(NIGHTWATCH));
    assert!(r.strandings.iter().all(|s| s.cause == Cause::Consumption));
    assert!(r.strandings.iter().all(|s| s.detail.contains("ledger")));
}

#[test]
fn the_reachable_set_is_a_fraction_of_the_space() {
    let r = analyse(&load(NIGHTWATCH));
    assert!(r.observed_space > r.states as u128);
}
