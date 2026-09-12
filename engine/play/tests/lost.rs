//! composes: verb-menu, interaction-resolver
//!
//! **Four games, each walked into its own trap, and not one of them says so.**
//!
//! Every specimen carries a documented way to lose and `crates/reachability` has reported all of
//! them for phases — a line, a cause, and how many reachable states die. Nothing had ever walked
//! into one, because until `.yidam/decisions/playable.yml` nothing could play a game at all.
//!
//! `ferry.game` states the claim in a comment beside its own trap: ***"Reachable, silent, and the
//! game goes on answering normally."*** The checker proved the first word. These are the other
//! two, and they are pinned whole rather than sampled, because a transcript is the one artifact
//! here a person can judge without knowing anything about the project.
//!
//! **The patrol is the one to read if you read one.** After the mistake the strongroom opens for
//! the first time — *"The wheel turns without much noise, which is the point of the grease"* —
//! because the porter has left the corridor. The player walks into the room they were trying to
//! reach, finds the cabinet locked, goes back for the key, and is told *"He is in there with his
//! tea, and the door is not wide."* Every line reads as progress or as a temporary obstacle.
//!
//! `.yidam/decisions/walked-in.yml`.

use play::{load, Session, Stop};

struct Trap {
    name: &'static str,
    source: &'static str,
    script: &'static str,
    /// the line `crates/reachability` blames, and the turn that takes it
    blamed: usize,
    at_turn: usize,
    transcript: &'static str,
}

const TRAPS: [Trap; 4] = [
    Trap {
        name: "ferry",
        source: include_str!("../../../design/specimen/ferry.game"),
        script: include_str!("../scripts/ferry-lost.txt"),
        blamed: 64,
        at_turn: 4,
        transcript: "\
> pick-up loose-coin
  A copper coin, green at the edges.
> walk-to path
  → The Ferryman's Hut
> pick-up jar
  Three more coins, and a good deal of dust.
> walk-to door
  → The Near Bank
> use-with coin on ferryman
  He counts three of them twice, pockets them, and nods at the boat.
  → The Far Bank
> look-at gate
  Barred. Too dark to see how.
",
    },
    Trap {
        name: "lighthouse",
        source: include_str!("../../../design/specimen/lighthouse.game"),
        script: include_str!("../scripts/lighthouse-lost.txt"),
        blamed: 131,
        at_turn: 8,
        transcript: "\
> pick-up loose-crowbar
  You work it free of the rope.
> use-with crowbar on crate
  The lid comes away with a squeal of nails. A can of oil sits in the straw.
> talk-to keeper
  → keeper-talk
> option lamp
  Aye. Three nights now. I've not the hands for that stair any more.
> option wick-ask
  If you can get through the door. Key's under the coil of rope.
> option leave
  Mm.
  → The Quay
> use-with brass-key on door
  The key turns over stiffly. The door swings inward.
> walk-to door
  → The Lighthouse, Ground Floor
> use-with oil-can on hinge
  You work the oil in until it swings sweetly. The can comes away empty.
> walk-to stair
  → The Lamp Room
> pick-up loose-match
  You pocket it, carefully.
> use-with wick on lamp
  You thread the wick and trim it square.
> use-with match on lamp
  Nothing to burn. You would waste the match.
",
    },
    Trap {
        name: "nightwatch",
        source: include_str!("../../../design/specimen/nightwatch.game"),
        script: include_str!("../scripts/nightwatch-lost.txt"),
        blamed: 153,
        at_turn: 4,
        transcript: "\
> pick-up loose-wire
  You work it flat between your fingers.
> walk-to window
  → The Study
> use-with wire on case
  The wire goes in past the wafers, the lock turns over, and the lid comes up on its own weight.
> pick-up ledger-in-case
  You lift it clear. Somewhere below the floor a bell starts, and does not stop.
> walk-to hall-door
  Three steps in, a lamp comes up. The watchman takes the ledger out of your hands without hurrying, and the door at the far end is already open.
  → The Cell
> look-at door
  Locked until morning, and morning is not far.
> walk-to door
  They let you out at six, with nothing.
  → The Yard
",
    },
    Trap {
        name: "patrol",
        source: include_str!("../../../design/specimen/patrol.game"),
        script: include_str!("../scripts/patrol-lost.txt"),
        blamed: 141,
        at_turn: 3,
        transcript: "\
> walk-to inner-door
  → The Corridor
> walk-to canteen-door
  → The Canteen
> walk-to switch
  You push it down. It starts to roar, and down the corridor a newspaper is folded.
> walk-to corridor-door
  → The Corridor
> walk-to strongroom-door
  The wheel turns without much noise, which is the point of the grease.
  → The Strongroom
> look-at cabinet
  Locked, and the lock is a small flat one.
> walk-to steel-door
  → The Corridor
> walk-to canteen-door
  He is in there with his tea, and the door is not wide.
",
    },
];

fn choices(script: &str) -> Vec<&str> {
    script
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// **The four transcripts, whole.**
#[test]
fn every_specimen_is_walked_into_its_trap() {
    for t in &TRAPS {
        let game = load(t.source, t.name).expect("the specimen reads clean");
        let mut session = Session::start(&game).expect("it starts");
        assert_eq!(
            session.play(&choices(t.script)),
            Stop::OutOfInput,
            "{}: a losing run runs out of script rather than reaching an ending",
            t.name
        );
        assert_eq!(session.transcript(), t.transcript, "{}", t.name);
    }
}

/// **The game goes on answering normally**, which is the half of the claim only a transcript can
/// show.
///
/// The turn that loses the game fires and is answered like any other. Every turn after it is
/// answered the same way. The menu never empties. Nothing stops.
///
/// **Three of the four say something a player reads as progress and the fourth says nothing at
/// all.** `patrol.game` ends on `walk-to corridor-door`, a bare movement with no line — so the
/// strongest case is not a reward misread but a turn with no text in it whatever.
#[test]
fn the_game_goes_on_answering_after_the_turn_that_ends_it() {
    for t in &TRAPS {
        let game = load(t.source, t.name).expect("the specimen reads clean");
        let mut session = Session::start(&game).expect("it starts");
        session.play(&choices(t.script));

        let fatal = &session.log[t.at_turn];
        assert_eq!(
            fatal.line, t.blamed,
            "{}: the turn the checker blames",
            t.name
        );
        assert!(fatal.fired, "{}: and it fires", t.name);
        assert!(
            !fatal.silent(),
            "{}: and is answered like any other",
            t.name
        );
        assert_eq!(
            fatal.said.is_some() || !fatal.forced.is_empty(),
            t.name != "patrol",
            "{}: three of four say a line and the patrol says nothing at all",
            t.name
        );

        let after = &session.log[t.at_turn + 1..];
        assert!(!after.is_empty(), "{}: the game goes on", t.name);
        assert!(
            after.iter().all(|x| !x.silent()),
            "{}: and every turn of it is answered",
            t.name
        );
        assert!(
            !session.menu().is_empty(),
            "{}: with something still to click at the end",
            t.name
        );
    }
}

/// **The checker knows, the engine does not, and the two meet only here.**
///
/// `reachability` is a **dev-dependency** of this crate and never a dependency.
/// `.yidam/decisions/two-components.yml` runs the arrow instrument-to-engine; this is the one
/// place it points back, and a test is the only place it can. An engine that could ask whether
/// the game is still winnable would be a different design, and
/// `.yidam/decisions/walked-in.yml` leaves that open rather than taking it.
#[test]
fn the_checker_blames_the_turn_the_player_took() {
    for t in &TRAPS {
        let game = load(t.source, t.name).expect("the specimen reads clean");
        let report = reachability::analyse(&game);
        assert!(
            report.winnable,
            "{}: which is what makes a trap one",
            t.name
        );

        let lines: Vec<usize> = report.strandings.iter().map(|s| s.line).collect();
        assert!(
            lines.contains(&t.blamed),
            "{}: the checker blames {lines:?} and the script takes {}",
            t.name,
            t.blamed
        );

        let mut session = Session::start(&game).expect("it starts");
        session.play(&choices(t.script));
        assert_eq!(session.log[t.at_turn].line, t.blamed, "{}", t.name);
    }
}
