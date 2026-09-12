//! composes: verb-menu, interaction-resolver
//!
//! **A game, played from the first menu to the ending, and what the transcript shows.**
//!
//! Five component tests passed before this one existed and the components had never been asked in
//! turn. This is the first thing in the repository that plays: it settles the start, shows a menu,
//! takes a choice, resolves it, prints what the game says, and does it again.
//!
//! **It found something on its first run.** Three turns of the winning run said nothing —
//! `talk-to keeper`, `walk-to door`, `walk-to stair` — and calling those silent misses would have
//! been wrong. In a point-and-click the movement *is* the response.
//! `subsystem/interaction-resolver`'s *"never nothing"* is a promise about the **outcome**, not
//! about the line, and nothing could tell the two apart until something printed a transcript.
//! `.yidam/decisions/playable.yml`.

use play::{load, Session, Stop};

const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");
const SCRIPT: &str = include_str!("../scripts/lighthouse-won.txt");

fn script() -> Vec<&'static str> {
    SCRIPT
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// **The lighthouse, won.**
///
/// Pinned whole rather than sampled. A transcript is the one artifact in this repository that a
/// person can read and judge without knowing anything about it, and a check that sampled it would
/// be checking the parts somebody thought to look at.
#[test]
fn the_lighthouse_is_played_to_its_ending() {
    let game = load(LIGHTHOUSE, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");
    assert_eq!(session.here(), "The Quay");

    assert_eq!(session.play(&script()), Stop::Ending);
    assert_eq!(
        session.transcript(),
        "\
> pick-up loose-crowbar
  You work it free of the rope.
> use-with crowbar on crate
  The lid comes away with a squeal of nails. A can of oil sits in the straw.
> talk-to keeper
  → Talking to the lighthouse keeper
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
> walk-to stair
  → The Lamp Room
> pick-up loose-match
  You pocket it, carefully.
> use-with wick on lamp
  You thread the wick and trim it square.
> use-with oil-can on lamp
  You fill the reservoir. The wick darkens as it drinks.
> use-with match on lamp
  The wick catches, gutters, and stands up bright. The bay comes back.
"
    );
}

/// **Not one turn of it is silent, and three of them said nothing.**
///
/// The distinction the slice exists to have found. A turn is silent when nothing was said *and*
/// nothing the player can see changed; three turns here said nothing and moved the player, which
/// is a response in a form this vocabulary has no word for.
#[test]
fn nothing_in_the_run_is_silent_and_three_turns_say_nothing() {
    let game = load(LIGHTHOUSE, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");
    assert_eq!(session.play(&script()), Stop::Ending);

    assert_eq!(session.log.len(), 13, "turns taken");
    assert!(
        session.log.iter().all(|t| !t.silent()),
        "a silent turn: {:?}",
        session.log.iter().find(|t| t.silent())
    );
    let wordless: Vec<&str> = session
        .log
        .iter()
        .filter(|t| t.said.is_none())
        .map(|t| t.picked.as_str())
        .collect();
    assert_eq!(
        wordless,
        ["talk-to keeper", "walk-to door", "walk-to stair"],
        "turns whose whole response is that the world moved"
    );
    assert!(
        session.log.iter().all(|t| t.fired),
        "and every one of them fired — nothing in a winning run is refused"
    );
}

/// **The menu is the whole of what can be picked.**
///
/// `subsystem/verb-menu`'s negative half, from the client's side: a choice the menu did not offer
/// is not resolved, it stops the session. That is why `interaction-resolver` can promise *never
/// nothing* with no default to fall through to — it is never asked about an unauthored pair.
#[test]
fn a_choice_the_menu_does_not_offer_is_not_resolved() {
    let game = load(LIGHTHOUSE, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");

    let menu = session.menu();
    assert_eq!(
        menu,
        [
            "look-at loose-crowbar",
            "pick-up loose-crowbar",
            "look-at crate",
            "look-at door",
            "walk-to door",
            "look-at keeper",
            "talk-to keeper",
        ],
        "the quay, before anything has happened"
    );
    assert!(
        !menu.iter().any(|e| e.contains("use-with")),
        "and no binary verb, because nothing is carried"
    );

    assert_eq!(
        session.play(&["use-with crowbar on crate"]),
        Stop::NoSuchChoice("use-with crowbar on crate".into()),
        "the entry the crate gains only once the crowbar is held"
    );
    assert!(session.log.is_empty(), "and no turn was taken");
}
