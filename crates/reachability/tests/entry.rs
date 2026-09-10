//! `on-enter` — the arrival trigger, and the decidability it costs.
//!
//! A forced binding makes the state it fires from transient: the player is never left there.
//! Two rooms that each throw the player into the other therefore have no resting state at
//! all, and a search that chases them does not stop. That is the same shape as an item with
//! no declared ceiling, and it is the second primitive in this design to cost decidability.

use game_format::{parse, resolve, Site};
use reachability::analyse;

fn game(body: &str) -> game_format::Game {
    let (mut g, mut problems) = parse(body, "fixture.game");
    problems.extend(resolve(&mut g));
    let msgs: Vec<String> = problems.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "fixture does not resolve: {msgs:?}");
    g
}

const TWO_ROOMS: &str = "room    here  \"Here\"\n\
                         room    there \"There\"\n\
                         flag    seen \"seen\"\n\
                         \n\
                         room here\n\
                         \x20 start\n\
                         \x20 hotspot door \"a door\"\n\
                         \x20   walk-to:\n\
                         \x20     do    move there\n\
                         \n\
                         room there\n\
                         \x20 hotspot out \"the way out\"\n\
                         \x20   walk-to:\n\
                         \x20     ending\n";

/// An entry binding fires on *arriving*, not while standing — which is how both AGS eras
/// document the event, and is what keeps a description-only entry binding from firing
/// forever. Without that rule this fixture would be reported as a cycle.
#[test]
fn a_description_only_entry_binding_does_not_loop() {
    let src = TWO_ROOMS.replace(
        "room there\n",
        "room there\n  on-enter:\n    say   \"It is colder in here.\"\n",
    );
    let r = analyse(&game(&src));
    assert!(r.entry_cycles.is_empty(), "{:?}", r.entry_cycles);
    assert!(r.winnable);
}

/// An entry binding that writes state also fires once per arrival, so it settles.
#[test]
fn an_entry_binding_that_writes_settles() {
    let src = TWO_ROOMS.replace(
        "room there\n",
        "room there\n  on-enter:\n    do    set seen\n",
    );
    let r = analyse(&game(&src));
    assert!(r.entry_cycles.is_empty(), "{:?}", r.entry_cycles);
    assert!(r.winnable);
}

/// **The decidability cost, and the guard.** Each room throws the player into the other, so
/// nothing ever settles. The search must say so and stop, rather than run forever — the
/// lesson `item coin 0..4` taught two phases ago, arriving from a different primitive.
#[test]
fn two_rooms_that_throw_each_other_are_reported_not_chased() {
    let src = TWO_ROOMS
        .replace(
            "room there\n",
            "room there\n  on-enter:\n    do    move here\n",
        )
        .replace(
            "room here\n\x20 start\n",
            "room here\n\x20 start\n  on-enter:\n    do    move there\n",
        );
    let r = analyse(&game(&src));
    assert!(
        !r.entry_cycles.is_empty(),
        "a chain that never settles must be reported; got:\n{}",
        r.render()
    );
    assert!(r.render().contains("never settles"));
}

/// A room whose entry binding moves the player *to itself* is the degenerate case, and it is
/// the one a step budget would have got wrong: the room never changes, so the chain settles
/// immediately and there is nothing to report.
#[test]
fn an_entry_binding_that_moves_nowhere_settles() {
    let src = TWO_ROOMS.replace(
        "room there\n",
        "room there\n  on-enter:\n    do    move there\n",
    );
    let r = analyse(&game(&src));
    assert!(r.entry_cycles.is_empty(), "{:?}", r.entry_cycles);
    assert!(r.winnable);
}

/// The site exists and is its own kind. This is the assertion that fails if someone later
/// decides `on-enter` is just a verb nobody types.
#[test]
fn on_enter_is_a_site_and_not_a_verb() {
    let src = TWO_ROOMS.replace(
        "room there\n",
        "room there\n  on-enter:\n    do    set seen\n",
    );
    let g = game(&src);
    let entry = g
        .bindings
        .iter()
        .find(|b| matches!(b.site, Site::OnEnter { .. }))
        .expect("the entry binding");
    match &entry.site {
        Site::OnEnter { room } => assert_eq!(room, "there"),
        other => panic!("wrong site: {other:?}"),
    }
    assert!(entry.object.is_none());
}
