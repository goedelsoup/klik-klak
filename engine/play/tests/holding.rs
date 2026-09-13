//! composes: verb-menu, interaction-resolver, response-presenter
//!
//! **A terminal holds a player, and the hold it performs is not the one the author was told they
//! controlled.**
//!
//! `.yidam/decisions/held.yml` closed with *"a terminal with no clock and no input between turns
//! has nowhere to put an interval… the first client that can hold a player is the first one with a
//! frame."* This file is the refutation and the correction at once: the run below holds a real
//! player a real number of times, on a terminal, with no clock — and every hold is the same size
//! whatever the author wrote. `.yidam/decisions/dismissed-hold.yml`.

mod common;

use play::{load, Await, Session, Stop};
use std::io::Cursor;

const SCRIPT: &str = include_str!("../scripts/lighthouse-won.txt");

fn script() -> Vec<&'static str> {
    SCRIPT
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// A player who reads everything and asks for the next thing: one keypress per hold.
fn player(presses: usize) -> Cursor<Vec<u8>> {
    Cursor::new("\n".repeat(presses).into_bytes())
}

/// **The winning run, held.**
///
/// Ten lines are delivered and the player is stopped ten times, on a terminal,
/// with nothing that could be called a clock anywhere in the build. Whatever a frame is for, it is
/// not this.
#[test]
fn a_terminal_holds_the_player_with_no_clock_and_no_invented_rate() {
    let src = common::source("lighthouse");
    let game = load(&src, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");

    let mut out: Vec<u8> = Vec::new();
    let mut sink = Await::new(player(64), &mut out);
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
    let dismissals = sink.dismissals;
    assert_eq!(stop, Stop::Ending, "the lighthouse is still won");

    let lines: usize = session
        .log
        .iter()
        .map(|t| usize::from(t.said.is_some()) + t.forced.len())
        .sum();
    assert_eq!(
        (dismissals, lines),
        (DISMISSALS, DISMISSALS),
        "one hold per line"
    );

    // and the component reached for no unit to do it with
    let presenter = include_str!("../../response-presenter/src/lib.rs");
    for unit in ["Duration", "millis", "secs", "sleep"] {
        assert!(
            !presenter.contains(unit),
            "the presenter reached for {unit}"
        );
    }
    // Prose stripped first, because this client's own header now names `std::thread::sleep` — to
    // say it was always available and was never what was missing.
    let client: String = include_str!("../src/lib.rs")
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    for unit in ["Duration", "sleep"] {
        assert!(!client.contains(unit), "the client reached for {unit}");
    }
}

/// **Every hold is the same size and the lines are not, which is the lever gone.**
///
/// `subsystem/response-presenter` says *"a line's length is how long the game is held, and that is
/// the only lever over pacing this vocabulary offers… an author who wants a beat writes a longer
/// sentence."* The longest line in this run and the shortest cost the player exactly one keypress
/// each. The weight is still reported and nothing can act on it.
///
/// The claim is true of a hold that **elapses** and false of one that is **dismissed**, and
/// nothing in this vocabulary lets an author say which they are writing for.
#[test]
fn the_authors_longest_sentence_and_their_shortest_cost_the_same() {
    let src = common::source("lighthouse");
    let game = load(&src, "lighthouse").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");

    let mut out: Vec<u8> = Vec::new();
    let mut sink = Await::new(player(64), &mut out);
    for choice in script() {
        if matches!(session.take_with(choice, &mut sink), Ok(Some(_)) | Err(_)) {
            break;
        }
    }
    let dismissals = sink.dismissals;

    let mut lengths: Vec<usize> = session
        .log
        .iter()
        .flat_map(|t| {
            t.said
                .iter()
                .map(|s| s.as_str())
                .chain(t.forced.iter().map(String::as_str))
                .map(|l| l.chars().count())
                .collect::<Vec<_>>()
        })
        .collect();
    lengths.sort_unstable();

    let held: usize = session.log.iter().map(|t| t.held).sum();
    assert_eq!(
        held,
        lengths.iter().sum::<usize>(),
        "the weight is unchanged"
    );
    assert_eq!(
        (
            *lengths.first().expect("a line"),
            *lengths.last().expect("a line")
        ),
        (SHORTEST, LONGEST),
        "what the author wrote, at its two ends"
    );
    assert_eq!(
        dismissals,
        lengths.len(),
        "and what it cost the player, at both ends and everywhere between"
    );
    assert_eq!(
        (held / dismissals, LONGEST / SHORTEST),
        (MEAN, SPREAD),
        "the lever the author was told they had, and what survives the hold"
    );
}

/// Ten lines in the winning run of `lighthouse.game`, and the same number of holds.
const DISMISSALS: usize = 10;
/// The two ends of what the author wrote, in characters: *"Aye."* and the keeper's answer about
/// the stair.
const SHORTEST: usize = 3;
const LONGEST: usize = 74;
/// The mean line, and the spread a hold that elapsed would have honoured. **471 characters over
/// ten holds, and a hold is a hold**: the author wrote one line twenty-four times the length of
/// another and the player pays the same for both.
const MEAN: usize = 47;
const SPREAD: usize = 24;
