//! composes: interaction-resolver, response-presenter
//!
//! **Two forced lines in one turn, which no specimen has ever produced.**
//!
//! `.yidam/decisions/refused-duration.yml` counted thirty-four turns across four fixtures and found
//! none that delivered two lines, and the same count over all seven scripts is sixty-eight and
//! still none. So the order `settle_tracking` keeps, `present_all`'s loop and the rule that a
//! forced binding's clause is *selected in the state it fired from* had no case. A rule with no
//! case is not known to hold, so this file gives it one.
//!
//! The game is written here rather than added as a specimen. It is not an adventure; it is the
//! smallest arrival that fires two bindings that both speak, and both lines are guarded on a flag
//! that the first binding sets. If either line were read in the state the chain *ended* in, the
//! first would say its second clause. If either were read in the state the chain *began* in, the
//! second would.
//!
//! `.yidam/decisions/forced-order.yml`.

use std::io::Cursor;

use play::{load, Await, Session};

const CHAIN: &str = r#"
room    porch   "The Porch"
room    hall    "The Hall"
room    stair   "The Stair"

flag    lit     latched "the lamp is lit"

room porch
  start

  hotspot door "a door"
    walk-to:
      do    move hall

room hall
  on-enter:
    when  lit       "The lamp is already burning."
    otherwise       "You strike a match and the lamp takes."
    do    set lit
    do    move stair

room stair
  on-enter:
    when  lit       "By the lamp's light the stair goes up."
    otherwise       "The stair is dark."

  hotspot landing "the landing"
    walk-to:
      say   "You go up."
      ending
"#;

const FIRST: &str = "You strike a match and the lamp takes.";
const SECOND: &str = "By the lamp's light the stair goes up.";

/// **The two lines come back in the order they fired, and each is the clause selected in the
/// state its binding fired from.** The first is read before its own `set lit` and the second after
/// it, so neither of the two wrong rules could produce this pair.
#[test]
fn a_chain_of_two_speaking_bindings_says_both_in_order() {
    let game = load(CHAIN, "chain").expect("the chain reads clean");
    let mut session = Session::start(&game).expect("it starts");
    session.take("walk-to door").expect("the door is offered");

    let turn = session.log.last().expect("one turn");
    assert_eq!(turn.said, None, "the chosen binding says nothing");
    assert_eq!(turn.forced, [FIRST, SECOND]);
    assert_eq!(session.here(), "The Stair", "the chain ran to its end");

    let transcript = session.transcript();
    let (a, b) = (
        transcript.find(FIRST).expect("the first is printed"),
        transcript.find(SECOND).expect("the second is printed"),
    );
    assert!(a < b, "printed in the order fired:\n{transcript}");
}

/// **The order is presentation and not duration.** The player is held for each line in turn, and a
/// player who dismisses nothing reaches the same room with the same words logged. The world had
/// finished changing before the first line was shown, so there is no state between the two lines
/// for a player to act in, save or be shown.
#[test]
fn skipping_the_chain_reaches_the_same_state_as_reading_it() {
    let game = load(CHAIN, "chain").expect("the chain reads clean");

    let read = |presses: usize| {
        let mut session = Session::start(&game).expect("it starts");
        let mut out: Vec<u8> = Vec::new();
        let mut sink = Await::new(Cursor::new("\n".repeat(presses).into_bytes()), &mut out);
        session
            .take_with("walk-to door", &mut sink)
            .expect("the door is offered");
        let dismissals = sink.dismissals;
        (
            session.here(),
            session.log.last().unwrap().forced.clone(),
            dismissals,
        )
    };

    let (patient_room, patient_lines, held) = read(8);
    let (impatient_room, impatient_lines, none) = read(0);

    assert_eq!(held, 2, "one hold per line, and there are two");
    assert_eq!(none, 0);
    assert_eq!(patient_room, impatient_room);
    assert_eq!(patient_lines, impatient_lines);
}
