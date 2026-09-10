//! The fourth specimen, for one question: does a non-player actor's position need to be state?
//!
//! `cutscene/scripted-sequence` said it did not know, from a specimen too small to tell. This
//! game is that case written in the vocabulary, and these tests pin the three things the answer
//! turns on: that the position is readable, that reading it is what makes it cost anything, and
//! that a site whose actor has walked off is not a site.

use reachability::{analyse, Cause};

const SPECIMEN: &str = include_str!("../../../design/specimen/patrol.game");

fn load(source: &str) -> game_format::Game {
    let (mut game, problems) = game_format::parse(source, "patrol.game");
    let problems: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
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
fn the_specimen_parses_and_places_the_porter() {
    let game = load(SPECIMEN);
    assert_eq!(game.start_room.as_deref(), Some("lobby"));
    // `holds porter` is `actor -occupies-> room` at rest, and it is the value `move ... to`
    // writes over. Without it the read below has nothing to compare against.
    assert_eq!(
        game.actor_start.get("porter").map(|s| s.as_str()),
        Some("corridor")
    );
}

#[test]
fn the_ending_is_reachable() {
    let r = analyse(&load(SPECIMEN));
    assert!(
        r.winnable,
        "the specimen must be winnable or it is not a game"
    );
}

/// The committed defect: boil the kettle before taking the key and the porter takes his tea in
/// the only room the key is in. The gate that closes is a person.
#[test]
fn the_one_committed_defect_is_a_person_standing_in_a_doorway() {
    let r = analyse(&load(SPECIMEN));
    assert_eq!(
        r.strandings.len(),
        1,
        "the committed specimen carries exactly one defect; found:\n{}",
        r.render()
    );
    let s = &r.strandings[0];
    assert_eq!(s.cause, Cause::Ordering);
    // Blamed on *leaving* the canteen rather than on boiling the kettle, and that is right:
    // while you are still in there with the porter you can take the key. Nothing is lost until
    // you walk out.
    assert_eq!(s.target, "canteen.corridor-door");
    assert!(r.stranded > 0);
}

/// The report names where the porter is, because a stranding a reader cannot see the cause of
/// is a stranding they will argue with.
#[test]
fn a_stranded_state_says_where_the_actor_is() {
    let r = analyse(&load(SPECIMEN));
    assert!(
        r.render().contains("porter in canteen"),
        "the report must place the actor:\n{}",
        r.render()
    );
}

/// The porter's `look-at` is written in the corridor and he does not stay there. An actor site
/// says where he may be *addressed*; `holds` and `move ... to` say where he *is*; and the site
/// is live only when they agree.
#[test]
fn an_actor_who_has_walked_off_cannot_be_addressed() {
    let r = analyse(&load(SPECIMEN));
    assert_eq!((r.states, r.transitions), (20, 33), "{}", r.render());
}

/// The fact the count above rests on, isolated. A site on an actor is live only where the actor
/// is, so a binding that needs the cat stroked becomes unreachable once the cat has been sent
/// away — and the checker sees that as a stranding rather than as a game that parses.
#[test]
fn an_actor_site_follows_the_actor() {
    let sends = analyse(&load(
        &STROKED.replace("__MOVE__", "      do move cat to yard\n"),
    ));
    let does_not = analyse(&load(&STROKED.replace("__MOVE__", "")));
    assert!(does_not.strandings.is_empty(), "{}", does_not.render());
    assert!(
        !sends.strandings.is_empty(),
        "sending the cat away takes its bindings with it, and that must strand:\n{}",
        sends.render()
    );
    assert!(
        sends.winnable,
        "not ringing the bell is still a way through"
    );
}

const STROKED: &str = "\
room    hall  \"a hall\"
room    yard  \"a yard\"
actor   cat   \"a cat\"
flag    purring \"the cat is purring\"

room hall
  start
  holds cat

  actor cat
    look-at:
      when not purring
      say  \"You stroke it once and it starts up.\"
      do   set purring
      else \"It is already going.\"

  hotspot bell \"a bell pull\"
    walk-to:
      say  \"It rings somewhere below.\"
__MOVE__
  hotspot door \"a door\"
    walk-to:
      do move yard

room yard
  hotspot gate \"a gate\"
    walk-to:
      when purring
      ending
      else \"Not yet.\"

  hotspot back \"the way back\"
    walk-to:
      do move hall
";

// ── the cost, which is the reason the design is shaped this way ─────────────────────────────

/// An effect nothing can read is not state. `move <actor> to <room>` on an actor no predicate
/// asks about splits no states, so a game pays for actor positions exactly when it reads them —
/// which is what keeps a primitive that could multiply the space by rooms^n free by default.
/// **The primitive is free until it is used.** An actor nobody moves and nobody asks about is
/// absent from `State` altogether, so declaring one — as all three specimens written before this
/// existed do — costs no state and no search. The measurable form is the observed space: without
/// a move there is no rooms-wide factor for the cat in it.
#[test]
fn an_actor_nobody_moves_is_not_in_the_state_at_all() {
    let still = analyse(&load(&STROKED.replace("__MOVE__", "")));
    let moved = analyse(&load(
        &STROKED.replace("__MOVE__", "      do move cat to yard\n"),
    ));
    assert_eq!(
        moved.observed_space,
        still.observed_space * 2,
        "one tracked actor over two rooms doubles the space, and no tracked actor leaves it \
         alone: {} vs {}",
        still.observed_space,
        moved.observed_space
    );
}

/// **And what costs is the move, not the question** — which is not where this was expected to
/// land. `offered` gates an actor's site on where the actor is, so the site is itself a read:
/// an actor who is moved has to be followed whether or not any predicate names them, or their
/// bindings would stay live in a room they have left.
///
/// So "an effect nothing can read is not state" survives with the reading corrected. There is
/// no such thing as moving an actor nobody reads, because the sites read them.
#[test]
fn the_move_is_what_costs_because_a_site_is_a_read() {
    let still = analyse(&load(&STROKED.replace("__MOVE__", "")));
    let moved = analyse(&load(
        &STROKED.replace("__MOVE__", "      do move cat to yard\n"),
    ));
    assert!(
        moved.states > still.states,
        "moving the cat must widen the state even though no predicate names it: {} vs {}",
        still.states,
        moved.states
    );
}

/// A question with no initial answer is a defect, and it is the one check the pair of primitives
/// needs that neither has alone.
#[test]
fn asking_about_an_actor_no_room_holds_is_reported() {
    let source = STROKED
        .replace("__MOVE__", "")
        .replace("  holds cat\n", "")
        .replace(
            "      when purring\n",
            "      when purring and cat in yard\n",
        );
    let (mut game, problems) = game_format::parse(&source, "unheld.game");
    let all: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
        .map(|p| p.to_string())
        .collect();
    assert!(
        all.iter().any(|p| p.contains("no room `holds cat`")),
        "an unplaced actor that something asks about must be reported; got {all:?}"
    );
}
