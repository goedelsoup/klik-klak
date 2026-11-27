//! **Nothing in this corpus realizes arrival, and the reason is that the component which would
//! have has been replaced twice over.**
//!
//! `subsystem -realizes-> interaction` has been declared since genesis. Three components author
//! it. None of them runs an `on-enter`: [`interaction-resolver`] *"takes a verb, a target and an
//! optional second object"*, which an entry binding has none of, and [`path-solver`] *"reports
//! arrival"* and stops there. [`prose-edges`] found the hole by counting edges and called the
//! missing realizer the finding.
//!
//! Reading a shipped engine's arrival hook says what such a component is actually for, and it is
//! not what the corpus assumed. **Of 169 statements across Delores's 17 non-empty room `enter`
//! bodies, 8 are game events.** The rest put the room back into agreement with world state: 51
//! calls move objects, actors, cameras, walkboxes and lights; 25 set `objectState` and
//! `objectTouchable`; 81 start the twinkling, the flickering and the dripping; 4 enable trigger
//! regions.
//!
//! Both halves of that remainder are gone here by earlier decisions. The 51 and the 81 are
//! presentation, which [`effect-list-semantics`] put outside the state model. The 25 are
//! *affordance* — what the player may do with a thing — which this design does not store and
//! recomputes, because [`subsystem/verb-menu`] is *"a pure function of the target and the state,
//! recomputed whenever either moves"*. `objectTouchable(this, Bank.open ? YES : NO)` is a
//! shipped engine hand-writing what `verb_menu::in_menu` derives.
//!
//! So the realizer is not missing. **A reconciler is what an engine needs when presentation is
//! stored, and this one stores none.** What is left at arrival is the 8, and `on-enter` is
//! already the form for them.
//!
//! [`interaction-resolver`]: ../../../.yidam/corpus/subsystem/interaction-resolver.yml
//! [`path-solver`]: ../../../.yidam/corpus/subsystem/path-solver.yml
//! [`subsystem/verb-menu`]: ../../../.yidam/corpus/subsystem/verb-menu.yml
//! [`prose-edges`]: ../../../.yidam/decisions/prose-edges.yml
//! [`effect-list-semantics`]: ../../../.yidam/decisions/effect-list-semantics.yml

mod common;

use game_format::Site;
use reachability::analyse;

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "arrival.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// **The resolver's whole specification is false for this trigger, and that is why it reads as a
/// different component.**
///
/// [`interaction-resolver`] promises *"exactly one outcome: an authored response, an authored
/// refusal, or a default. Never nothing"*, and calls that *"the whole specification — a silent
/// miss is the failure players read as brokenness."* Arrival has no default (nothing stands in
/// for an unauthored room), no refusal ([`entry-binding`] argues one cannot exist, because
/// *"nobody tried anything, they walked into a room"*), and produces nothing almost always:
/// **one `on-enter` binding across fourteen rooms in four specimens.**
///
/// The promise survives with its scope written down. *Never nothing* exists because a player
/// clicked and deserves an answer. Arriving somewhere is not a click, so the guarantee was always
/// about the composed trigger and never about this one — which is the same asymmetry the format
/// records in [`Site`], where an `on-enter` binding *"is not offered to the player, it fires"*.
///
/// [`interaction-resolver`]: ../../../.yidam/corpus/subsystem/interaction-resolver.yml
/// [`entry-binding`]: ../../../.yidam/corpus/interaction/entry-binding.yml
#[test]
fn arrival_produces_nothing_in_sixteen_rooms_out_of_seventeen() {
    let (mut rooms, mut entries) = (0, 0);
    for (_, src) in common::sources() {
        let g = load(&src);
        rooms += g.rooms.len();
        entries += g
            .bindings
            .iter()
            .filter(|b| matches!(b.site, Site::OnEnter { .. }))
            .count();
    }
    assert_eq!(
        (rooms, entries),
        (20, 1),
        "a fifth game added three rooms and no second entry binding"
    );
}

/// **The two triggers are disjoint at the interface, and the format already said so.**
///
/// If one component is to answer both, the thing that must not happen is an entry binding
/// appearing in a menu: it would offer the player a choice about something that fires whether
/// they choose it or not. `verb_menu::in_menu` excludes `Site::OnEnter` outright, so the widened
/// resolver takes two inputs that cannot collide — a composed verb, target and object, or a room.
///
/// The evidence that this costs nothing is that the specimen carrying the entry binding reports
/// the same menu as if it had none.
#[test]
fn an_entry_binding_is_never_in_a_menu() {
    let g = load(&common::source("nightwatch"));
    let r = analyse(&g);
    assert_eq!(r.menu_entries, (23, 8));
    assert!(
        r.unshown.is_empty(),
        "the entry binding is authored and in no menu, and is not reported as unshown, \
         because being unshown is what it is for: {:?}",
        r.unshown
    );
}

/// **Eighty-six of a hundred and seventeen menu entries appear during play and no line in any
/// specimen makes them appear.**
///
/// This is the reconciliation, measured from the far side. A shipped engine stores affordance and
/// must therefore re-derive it on arrival — eight `objectTouchable` calls and seventeen
/// `objectState` calls in those `enter` bodies exist for no other reason. Here affordance is a
/// function of the state, so an entry becomes available the instant its guard holds and nothing
/// records that it changed.
///
/// The figure is the cost side of the same fact: **1,169 menu slots over 203 reachable states for
/// 117 distinct entries**, which is the work an engine does either once per state change or once
/// per arrival and then hopes. `default-binding` named the price of the first choice without a
/// number; this is the number.
///
/// It was 698 over 145 for 88 when this file could see four games (`.yidam/decisions/
/// unread-specimen.yml`). **The share that is showing at the start fell from 27% to 26%** — the
/// fifth game is the one where most of what you can do depends on which character you are, so
/// almost none of it is offered on arrival, and the claim this test makes got stronger by the same
/// evidence that moved every number in it.
#[test]
fn affordance_changes_eighty_six_times_and_nothing_is_authored_to_change_it() {
    let (mut entries, mut showing, mut slots, mut states) = (0, 0, 0, 0);
    for (_, src) in common::sources() {
        let r = analyse(&load(&src));
        entries += r.menu_entries.0;
        showing += r.menu_entries.1;
        slots += r.menu_slots;
        states += r.states;
    }
    assert_eq!((entries, showing, slots, states), (144, 40, 4261, 683));
    assert_eq!(entries - showing, 104);
}

/// **An entry binding fires into a state nobody rests in, which is the one thing a reconciler
/// could not be asked to do.**
///
/// `Report::entry_cycles` exists because two rooms that throw the player at each other have no
/// resting state and a search that chases it does not stop. That property belongs to the trigger
/// and not to the room: it is what *forced* means. A reconciler runs once on arrival and settles;
/// a forced binding can move the player again, and the resolver is already the component that
/// deals with a binding whose effects move somebody.
///
/// Pinned green on all four specimens, which is the assertion that the one authored entry binding
/// settles rather than the assertion that the check works — that one is `entry.rs`'s.
#[test]
fn no_specimen_arrives_somewhere_it_cannot_rest() {
    for (name, src) in common::sources() {
        let r = analyse(&load(&src));
        assert!(r.entry_cycles.is_empty(), "{name}: {:?}", r.entry_cycles);
    }
}
