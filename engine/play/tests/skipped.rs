//! composes: verb-menu, interaction-resolver, response-presenter
//!
//! **A player who reads nothing arrives where a player who reads everything arrives, and the
//! reason is an ordering rather than an argument.**
//!
//! `cutscene/scripted-sequence`'s `skippable` property names a real failure — *"a sequence that
//! writes state part-way through and is cut short leaves the world in a configuration the author
//! never wrote"* — and eliminates it rather than answering it: *"atomicity means there is no
//! part-way. That is defining the problem away."* `[inference]`.
//!
//! It is not defined away here. `Session::take_with` resolves the binding, settles the chain it
//! starts and assigns the state, and only then hands the lines to the presenter — so **every write
//! has landed before one character reaches the player.** A skip cannot be part-way through
//! anything, because by the time there is something to skip there is nothing left to do.
//!
//! Thimbleweed Park is the other arm and the corpus already holds it: that engine cannot save
//! inside a cut-scene, so it saves just *before* one and a restore puts the player back where the
//! scene began ([twp-dev-diary](../../../.yidam/catalog/twp-dev-diary.md)). This design lands in
//! the opposite place for the same reason — the state is committed before the scene is shown, not
//! after — and both are consistent. `.yidam/decisions/refused-duration.yml`.

mod common;

use play::{load, Await, Session, Stop};
use std::io::Cursor;

const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");
const SCRIPT: &str = include_str!("../scripts/lighthouse-won.txt");

/// The run that takes the ledger and walks into the hall, where the trap is sprung by an
/// `on-enter` binding the player never chose.
const INTO_THE_HALL: [&str; 5] = [
    "pick-up loose-wire",
    "walk-to window",
    "use-with wire on case",
    "pick-up ledger-in-case",
    "walk-to hall-door",
];

fn script() -> Vec<&'static str> {
    SCRIPT
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// A player with `presses` keypresses in them. Nought is one who has walked away.
fn player(presses: usize) -> Cursor<Vec<u8>> {
    Cursor::new("\n".repeat(presses).into_bytes())
}

/// Play a script with a sink that will dismiss `presses` times, and give back the session and what
/// the sink was made to do.
fn run<'a>(game: &'a game_format::Game, script: &[&str], presses: usize) -> (Session<'a>, usize) {
    let mut session = Session::start(game).expect("it starts");
    let mut out: Vec<u8> = Vec::new();
    let mut sink = Await::new(player(presses), &mut out);
    for choice in script {
        if matches!(session.take_with(choice, &mut sink), Ok(Some(_)) | Err(_)) {
            break;
        }
    }
    let dismissals = sink.dismissals;
    (session, dismissals)
}

/// **The winning run, read and skipped, and the two end in the same state.**
///
/// Ten holds against nought. Every line is still delivered — a skip is not a silence — and the
/// transcript the two produce is the same text, because the transcript is what the game said and
/// not what the player waited for.
#[test]
fn skipping_every_line_reaches_the_state_of_reading_them_all() {
    let src = common::source("lighthouse");
    let game = load(&src, "lighthouse").expect("the specimen reads clean");

    let (patient, held) = run(&game, &script(), 64);
    let (impatient, none) = run(&game, &script(), 0);

    assert_eq!((held, none), (10, 0), "one player waits and one does not");
    assert_eq!(
        patient.state, impatient.state,
        "and they are in the same state"
    );
    assert_eq!(
        patient.transcript(),
        impatient.transcript(),
        "having been told the same things"
    );
    assert_eq!(
        patient.log.len(),
        impatient.log.len(),
        "over the same number of turns"
    );
}

/// **The nearest thing to a cutscene this vocabulary has is complete before its first word.**
///
/// `walked-in` names it: a chain of forced bindings is the one construct where ordering inside a
/// single transition is observable, and `nightwatch.game`'s trap is one — the player walks into
/// the hall, an `on-enter` binding takes the ledger and moves them to a cell, and the 143-character
/// line that says so is the longest response any author here has written.
///
/// Walk away before it is said and the ledger is still gone and the cell door is still shut. That
/// is `skippable` answered rather than eliminated.
#[test]
fn the_trap_is_sprung_before_the_player_is_told_it_sprang() {
    let game = load(NIGHTWATCH, "nightwatch").expect("the specimen reads clean");

    let (told, held) = run(&game, &INTO_THE_HALL, 64);
    let (untold, none) = run(&game, &INTO_THE_HALL, 0);

    assert_eq!((held, none), (4, 0), "the lines one player waited through");
    assert_eq!(told.here(), "The Cell", "the trap fired");
    assert_eq!(
        untold.here(),
        told.here(),
        "and it fired for the one who was not waiting"
    );
    assert_eq!(told.state, untold.state, "with the same writes landed");

    let last = untold.log.last().expect("a turn");
    assert_eq!(
        last.forced.len(),
        1,
        "one forced line, which the player did not choose and could not have"
    );
    assert_eq!(
        last.said, None,
        "and the binding they did choose says nothing at all"
    );
}

/// **A turn says one thing at most, in every run anybody has written here.**
///
/// Thirty-four turns across four fixtures, and **not one delivers two lines**: no turn has both a
/// chosen binding's response and a forced one, and the longest chain an arrival starts is a single
/// binding. So the nearest thing to a sequence this vocabulary has is one sentence long, which is
/// the sharpest form the collapse in `cutscene/scripted-sequence` has ever been put in.
///
/// **Two pieces of machinery are unexercised by that, and both were built for a reason.**
/// `response_presenter::present_all` presents *several in order*; `walked-in` made
/// `settle_tracking` select each binding *"in the state it fired from"* because a chain fires each
/// from what the last one left. Neither claim has ever had two lines to be right about. That is
/// `.yidam/decisions/or-earns-it.yml`'s shape a fourth time — a guarantee nothing exercises —
/// except that here it is found before anything was built on top of it.
#[test]
fn no_turn_in_any_fixture_delivers_two_lines() {
    let (mut longest, mut turns, mut both, mut forced) = (0, 0, 0, 0);
    for (name, path) in [
        ("lighthouse", "lighthouse-won.txt"),
        ("nightwatch", "nightwatch-lost.txt"),
        ("patrol", "patrol-lost.txt"),
        ("ferry", "ferry-lost.txt"),
    ] {
        let src = common::source(name);
        let game = load(&src, name).expect("the specimen reads clean");
        let text = std::fs::read_to_string(common::root().join("engine/play/scripts").join(path))
            .expect("the script");
        let lines: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        let (session, _) = run(&game, &lines, 0);
        for t in &session.log {
            longest = longest.max(usize::from(t.said.is_some()) + t.forced.len());
            turns += 1;
            if t.said.is_some() && !t.forced.is_empty() {
                both += 1;
            }
            forced = forced.max(t.forced.len());
        }
    }
    assert_eq!(
        (longest, turns, both, forced),
        (LONGEST, TURNS, 0, 1),
        "lines in one turn, turns played, turns with both kinds, longest forced chain"
    );
}

/// The most lines one turn delivers, and the turns played across the fixtures.
const LONGEST: usize = 1;
const TURNS: usize = 34;

/// A run that stops is still a run: this pins that nothing above ended early by accident.
#[test]
fn the_winning_run_still_wins_with_nobody_reading_it() {
    let src = common::source("lighthouse");
    let game = load(&src, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");
    let mut out: Vec<u8> = Vec::new();
    let mut sink = Await::new(player(0), &mut out);
    let mut stop = Stop::OutOfInput;
    for choice in script() {
        match session.take_with(choice, &mut sink) {
            Ok(Some(s)) | Err(s) => {
                stop = s;
                break;
            }
            Ok(None) => {}
        }
    }
    assert_eq!(stop, Stop::Ending, "the lighthouse is won unread");
}
