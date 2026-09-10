//! The specimen as a fixture.
//!
//! `design/specimen/README.md` promised three cases: winnable, unwinnable by consumption,
//! and unwinnable by ordering. The first two are the committed game; the third is a
//! two-line mutation, applied here in memory so the file on disk keeps exactly one defect.
//! A fixture with two makes a checker's output ambiguous about which it found.

use reachability::{analyse, Cause};

const SPECIMEN: &str = include_str!("../../../design/specimen/lighthouse.game");

fn load(source: &str) -> game_format::Game {
    let (game, problems) = game_format::parse(source, "lighthouse.game");
    let problems: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&game))
        .map(|p| p.to_string())
        .collect();
    assert!(
        problems.is_empty(),
        "specimen does not resolve:\n{}",
        problems.join("\n")
    );
    game
}

#[test]
fn the_specimen_parses_and_resolves() {
    let game = load(SPECIMEN);
    assert_eq!(game.start_room.as_deref(), Some("quay"));
    assert!(game.bindings.iter().any(|b| b.ending));
    // `is <item>` is `inventory-item -carried-form-of-> hotspot`, and it is what makes
    // "the object leaves the room" a derived fact rather than an effect.
    assert_eq!(
        game.carried_form.get("loose-crowbar").map(|s| s.as_str()),
        Some("crowbar")
    );
    assert_eq!(
        game.carried_form.get("loose-match").map(|s| s.as_str()),
        Some("match")
    );
}

#[test]
fn the_ending_is_reachable() {
    let r = analyse(&load(SPECIMEN));
    assert!(
        r.winnable,
        "the specimen must be winnable or it is not a game"
    );
    assert!(r.states > 1);
}

#[test]
fn the_one_committed_defect_is_the_consumption_trap() {
    let r = analyse(&load(SPECIMEN));
    assert_eq!(
        r.strandings.len(),
        1,
        "the committed specimen carries exactly one defect; found:\n{}",
        r.render()
    );
    let s = &r.strandings[0];
    assert_eq!(s.cause, Cause::Consumption);
    assert_eq!(s.target, "hinge");
    assert_eq!(s.object.as_deref(), Some("oil-can"));
    assert!(r.stranded > 0);
}

/// The mutation `design/specimen/README.md` names: the door closes behind the player and
/// the oil is on the wrong side of it.
fn ordering_mutation() -> String {
    let s = SPECIMEN.replace(
        "      do    move base\n",
        "      do    move base\n      do    clear door-open\n",
    );
    let s = s.replace(
        "    walk-to:\n      do    move quay\n",
        "    walk-to:\n      when  door-open\n      do    move quay\n      else  \"It has swung shut, and there is no handle on this side.\"\n",
    );
    assert_ne!(s, SPECIMEN, "the mutation must actually apply");
    s
}

#[test]
fn the_ordering_mutation_is_detected_and_classified() {
    let r = analyse(&load(&ordering_mutation()));
    assert!(
        r.winnable,
        "the mutation must leave a winning path, or it tests nothing"
    );
    let ordering: Vec<_> = r
        .strandings
        .iter()
        .filter(|s| s.cause == Cause::Ordering)
        .collect();
    assert!(
        !ordering.is_empty(),
        "closing the door behind the player is `unwinnable-state`'s second cause and must be \
         reported as ordering; got:\n{}",
        r.render()
    );
    // and the consumption trap is still there — the two causes are distinguished, not summed
    assert!(r.strandings.iter().any(|s| s.cause == Cause::Consumption));
}

/// `world-flag/latched-flag` claims a latching flag never goes back. The declaration is
/// only worth having if something checks it.
#[test]
fn a_latch_declaration_is_held_against_the_game() {
    let clean = analyse(&load(SPECIMEN));
    assert!(
        clean.latch_violations.is_empty(),
        "{:?}",
        clean.latch_violations
    );

    let broken = SPECIMEN.replace(
        "      do    set crate-open\n",
        "      do    set crate-open\n      do    clear crate-open\n",
    );
    let r = analyse(&load(&broken));
    assert!(
        r.latch_violations.iter().any(|v| v.contains("crate-open")),
        "a cleared latch must be reported; got {:?}",
        r.latch_violations
    );
}

/// `latched-flag` argues monotonicity is what makes this computation cheap. The specimen is
/// where that stops being an argument.
#[test]
fn the_counter_only_rises() {
    let r = analyse(&load(SPECIMEN));
    assert_eq!(r.monotone_counters, vec![("lamp-prep".to_string(), true)]);
}
