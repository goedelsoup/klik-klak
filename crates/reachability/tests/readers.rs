//! **Every slot of `State` is read by something**, checked rather than applied by hand.
//!
//! [`unread-state`] left the rule applied in five places and nothing to say a sixth obeyed it:
//! *"A general form needs the state vector to be data rather than a struct."* It does not. A
//! struct can be destructured without `..`, and then a sixth field is a compile error in this file
//! until someone writes down how it varies. That is the whole of the generality wanted — the check
//! cannot be skipped by adding a component, which is what "nothing checks that a sixth obeys it"
//! asked for.
//!
//! The check itself is a perturbation. Hold every slot at its initial value, change one, and ask
//! whether anything the engine consults can tell: whether any binding becomes live or dead
//! ([`verb_menu::live`]), or any atom of any predicate changes truth ([`World::holds`]). A slot
//! that no change can make visible is not state. Run over every specimen, against the projection
//! the checker builds — `World::new`, not `with_inventory`, which widens `items` on purpose for a
//! reader that is a person looking at a screen.
//!
//! [`unread-state`]: ../../../.yidam/decisions/unread-state.yml

mod common;

use game_format::{Game, Pred};
use game_format::{State, World};
use verb_menu::live;

fn load(src: &str) -> Game {
    let (mut g, mut p) = game_format::parse(src, "readers.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// The leaves of a predicate. Read one at a time so a conjunct the rest of the formula masks in
/// the initial state still counts as read — whether it is ever unmasked is reachability's
/// question, not this one.
fn atoms<'p>(q: &'p Pred, into: &mut Vec<&'p Pred>) {
    match q {
        Pred::Not(a) => atoms(a, into),
        Pred::And(a, b) | Pred::Or(a, b) => {
            atoms(a, into);
            atoms(b, into);
        }
        _ => into.push(q),
    }
}

/// Everything the engine can see of a state.
fn observe(w: &World, s: &State) -> Vec<bool> {
    let mut seen = Vec::new();
    for b in &w.game.bindings {
        seen.push(live(w, b, s));
        let mut a = Vec::new();
        for q in b.when.iter().chain(b.guards()) {
            atoms(q, &mut a);
        }
        seen.extend(a.into_iter().map(|q| w.holds(q, s)));
    }
    seen
}

type Change = Box<dyn Fn(&mut State)>;

/// One named slot and every other value it may take.
fn slots(w: &World) -> Vec<(String, Vec<Change>)> {
    // No `..`. A component added to `State` stops this compiling until it is listed here, with
    // the values it can take — which is the check refusing to be bypassed.
    let State {
        room: _,
        mode: _,
        items,
        taken,
        flags,
        counters,
        chosen,
        actors,
    } = w.initial();

    let rooms = w.rooms.len();
    let mut out: Vec<(String, Vec<Change>)> = Vec::new();

    // a one-room game has a position with one value, which is no variable at all
    if rooms > 1 {
        out.push((
            "room".into(),
            (0..rooms)
                .map(|r| Box::new(move |s: &mut State| s.room = r) as Change)
                .collect(),
        ));
    }
    if !w.convs.is_empty() {
        out.push((
            "mode".into(),
            (0..w.convs.len())
                .map(|m| Box::new(move |s: &mut State| s.mode = Some(m)) as Change)
                .collect(),
        ));
    }
    for i in 0..items.len() {
        out.push((
            format!("item {}", w.items[i]),
            (0..=w.caps[i])
                .map(|n| Box::new(move |s: &mut State| s.items[i] = n) as Change)
                .collect(),
        ));
    }
    for i in 0..taken.len() {
        out.push((
            format!("taken {}", w.takeable[i]),
            vec![Box::new(move |s: &mut State| s.taken[i] = !s.taken[i])],
        ));
    }
    for i in 0..flags.len() {
        out.push((
            format!("flag {}", w.flags[i]),
            vec![Box::new(move |s: &mut State| s.flags[i] = !s.flags[i])],
        ));
    }
    for i in 0..counters.len() {
        let c = w
            .game
            .counters
            .iter()
            .find(|c| c.name == w.counters[i])
            .expect("declared counter");
        out.push((
            format!("counter {}", w.counters[i]),
            (c.lo..=c.hi)
                .map(|n| Box::new(move |s: &mut State| s.counters[i] = n) as Change)
                .collect(),
        ));
    }
    for i in 0..chosen.len() {
        out.push((
            format!("chosen {}", w.options[i]),
            vec![Box::new(move |s: &mut State| s.chosen[i] = !s.chosen[i])],
        ));
    }
    for i in 0..actors.len() {
        out.push((
            format!("actor {}", w.actors[i]),
            (0..rooms)
                .map(|r| Box::new(move |s: &mut State| s.actors[i] = r) as Change)
                .collect(),
        ));
    }
    out
}

/// The states a slot is varied against: standing in each room, and inside each conversation.
fn contexts(w: &World) -> Vec<State> {
    let base = w.initial();
    let mut out = Vec::new();
    for r in 0..w.rooms.len() {
        out.push(State {
            room: r,
            ..base.clone()
        });
    }
    for m in 0..w.convs.len() {
        out.push(State {
            mode: Some(m),
            ..base.clone()
        });
    }
    out
}

/// The slots nothing can tell apart.
fn blind(w: &World) -> Vec<String> {
    let ctx = contexts(w);
    let seen: Vec<Vec<bool>> = ctx.iter().map(|s| observe(w, s)).collect();
    slots(w)
        .into_iter()
        .filter(|(_, changes)| {
            !ctx.iter().zip(&seen).any(|(s, before)| {
                changes.iter().any(|change| {
                    let mut t = s.clone();
                    change(&mut t);
                    observe(w, &t) != *before
                })
            })
        })
        .map(|(name, _)| name)
        .collect()
}

/// **Every specimen's state is read, slot by slot.** The property `unread-state` wanted and had
/// only as five constructions each done right.
#[test]
fn every_slot_of_every_specimen_is_seen_by_something() {
    for (name, src) in common::sources() {
        let g = load(&src);
        let w = World::new(&g);
        assert!(!slots(&w).is_empty(), "{name}: nothing to check");
        assert_eq!(blind(&w), Vec::<String>::new(), "{name}");
    }
}

/// **The check would have caught the defects it is written after.** Each vector is put back the
/// way it was before it was narrowed — built from declarations rather than from reads — on a game
/// that declares one of each thing and reads none of them, and the check names exactly the slots
/// the narrowing removed.
#[test]
fn the_old_enumerations_are_caught() {
    const G: &str = "room cell \"The Cell\"\nitem stone \"a loose stone\"\n\
                     flag rung \"the bell has rung\"\ncounter turns 0..2 \"turns of the key\"\n\n\
                     room cell\n  start\n\
                     \x20 hotspot rubble \"rubble\"\n    is stone\n    pick-up:\n      do    give stone\n\
                     \x20 hotspot bell \"a bell\"\n    walk-to:\n      do    set rung\n      do    set turns 1\n\
                     \x20 hotspot out \"out\"\n    walk-to:\n      ending\n";
    let g = load(G);

    // narrowed, there is nothing unread left to find
    let w = World::new(&g);
    assert!(w.items.is_empty() && w.flags.is_empty() && w.counters.is_empty());
    assert_eq!(blind(&w), Vec::<String>::new());

    // the enumerations `Index::new` used to make
    let mut old = World::new(&g);
    old.flags = g.flags.iter().map(|f| f.name.clone()).collect();
    old.counters = g.counters.iter().map(|c| c.name.clone()).collect();
    let old = old.with_inventory();
    assert_eq!(
        blind(&old),
        ["item stone", "flag rung", "counter turns"],
        "each widening is a slot nothing reads"
    );
}

/// **`taken` is read by the site it removes**, and so is a slot that disappears with its site.
/// The vector is the one narrowing `observed-state` did first, and it is narrowed by construction
/// rather than by a search for reads: a takeable hotspot is a site, and `live` reads its bit.
#[test]
fn an_acquisition_is_read_by_the_hotspot_it_empties() {
    let g = load(
        "room cell \"The Cell\"\nitem key \"a bent key\"\n\n\
         room cell\n  start\n\
         \x20 hotspot ledge \"a ledge\"\n    is key\n    pick-up:\n      do    give key\n\
         \x20 hotspot out \"out\"\n    walk-to:\n      ending\n",
    );
    let w = World::new(&g);
    assert_eq!(w.takeable, ["cell.ledge"]);
    assert_eq!(blind(&w), Vec::<String>::new());
}
