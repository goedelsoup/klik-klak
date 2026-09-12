//! composes: verb-menu, interaction-resolver
//!
//! **Two games, played from the first menu to the ending, and what the transcripts show.**
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
//!
//! **The second run is here for the opposite reason: to show a thing that does not appear.**
//! `lock.game` changes which character the player is, twice, and the transcript is where you can
//! see that nothing reports it — no menu change, no inventory change, no room change on one of the
//! two. `.yidam/decisions/control-transfer.yml`.

use play::{load, Session, Stop};

const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");
const SCRIPT: &str = include_str!("../scripts/lighthouse-won.txt");
const LOCK: &str = include_str!("../../../design/specimen/lock.game");
const LOCK_SCRIPT: &str = include_str!("../scripts/lock-won.txt");

fn lines(src: &'static str) -> Vec<&'static str> {
    src.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

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

/// **Pacing, measured for the first time.**
///
/// `subsystem/response-presenter` says duration is an accident nobody decided on — *"a line's
/// length is how long the game is held… an author who wants a beat writes a longer sentence"* —
/// and it was an `[inference]` until the component existed to count it.
///
/// The client's sink returns at once, so the holding is nominal and the number is not: a winning
/// run of `lighthouse.game` asks the player to read a countable amount, and the longest single
/// hold in the run is the one line the author wrote longest. `.yidam/decisions/held.yml`.
#[test]
fn a_winning_run_holds_the_player_for_a_countable_amount() {
    let game = load(LIGHTHOUSE, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");
    assert_eq!(session.play(&script()), Stop::Ending);

    let total: usize = session.log.iter().map(|t| t.held).sum();
    let longest = session.log.iter().map(|t| t.held).max().expect("a turn");
    assert_eq!((total, longest), (PACING, LONGEST), "characters held");

    // every character held is a character an author wrote, and nothing else is
    let written: usize = session
        .log
        .iter()
        .map(|t| {
            t.said
                .as_deref()
                .map(str::chars)
                .map(Iterator::count)
                .unwrap_or(0)
                + t.forced.iter().map(|l| l.chars().count()).sum::<usize>()
        })
        .sum();
    assert_eq!(
        written, total,
        "the presenter holds for what was said and no more"
    );

    // three turns say nothing and hold for nothing, and are not silent
    let free = session.log.iter().filter(|t| t.held == 0).count();
    assert_eq!(free, 3, "a movement is a response that takes no reading");
}

/// 471 characters across thirteen turns, and the longest single hold is 74 — the keeper's answer
/// about the stair. The whole of the lighthouse, won, is about a paragraph of reading.
const PACING: usize = 471;
const LONGEST: usize = 74;

/// **The lock, won, and what a change of player looks like to somebody playing.**
///
/// It looks like nothing. `lock.game` hands control over twice in eleven turns and the only thing
/// that reports it either time is a clause an author wrote: *"and then it is her"*, *"and then it
/// is him"*. The verb menu does not change, the inventory does not change, the phrasing does not
/// change, and nothing in `verb-menu`, `interaction-resolver` or `response-presenter` has any
/// concept that it happened — because in this format it did not. A flag was written.
///
/// **The two transfers print differently, and the difference is geography rather than kind.**
/// Turn 3 crosses a room, so the transcript shows the line and then a room change, exactly as a
/// door does. Turn 10 happens inside one room and shows no room change at all — the player does
/// not move, because the character being taken is already standing there. Two spellings of one
/// act, told apart by nothing a reader of this transcript could name.
///
/// That is the specimen's finding rendered as the thing a person can judge:
/// `.yidam/decisions/control-transfer.yml`.
#[test]
fn the_lock_is_played_to_its_ending_and_says_nothing_about_changing_hands() {
    let game = load(LOCK, "lock").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");
    assert_eq!(session.here(), "The Chamber");

    assert_eq!(session.play(&lines(LOCK_SCRIPT)), Stop::Ending);
    assert_eq!(
        session.transcript(),
        "\
> pick-up locker
  You take the windlass. It is cold and it is heavier than it looks.
> use-with windlass on coping
  You send it up underarm. It lands flat, skids, and stops.
> talk-to mate-above
  You call up. She comes to the edge and looks down at you, and then it is her.
  → The Lockside
> walk-to towpath
  → The Tail
> walk-to bottom-beams
  You walk one round and then the other, and they meet without a sound.
> walk-to towpath-up
  → The Lockside
> walk-to paddle-gear
  You fit the windlass and wind, and it comes up a click at a time and stays up.
> walk-to ladder
  You put a hand down and he comes up the last two rungs on his own.
> walk-to top-beams
  You take one and he takes the other, and they come round together the way they are meant to.
> talk-to boatman
  He takes his hands out of his pockets, and then it is him.
> walk-to boat
  You step down onto the counter and take the tiller, and she goes out under the beams with the water still moving.
"
    );

    // Eleven turns, two of which change who the player is, and the only difference between a
    // transfer and a door is that one of the two transfers does not move anybody.
    assert_eq!(session.log.len(), 11, "turns taken");
    let transfers: Vec<&str> = session
        .log
        .iter()
        .filter(|t| t.said.as_deref().is_some_and(|l| l.contains("then it is")))
        .map(|t| t.picked.as_str())
        .collect();
    assert_eq!(
        transfers,
        ["talk-to mate-above", "talk-to boatman"],
        "every turn that changes who the player is, found by reading the prose — \
         which is the only place it is written down"
    );
    assert!(
        session.log.iter().all(|t| !t.silent()),
        "a silent turn: {:?}",
        session.log.iter().find(|t| t.silent())
    );
}
