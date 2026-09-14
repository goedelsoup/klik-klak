//! composes: verb-menu, interaction-resolver, response-presenter

use play::{load, Session};

const GAME: &str = r#"room hall "Hall"
room yard "Yard"
counter unused 7..9 "unused"
counter used 0..2 "used"
room hall
  start
  on-enter:
    say "Arrived in hall."
  hotspot door "door"
    look-at:
      say "A door."
    walk-to:
      do move yard
    talk-to:
      do move hall
  hotspot exit "exit"
    walk-to:
      when used = 0
      ending
      else "Blocked."
room yard
  on-enter:
    say "Arrived in yard."
    do move hall
"#;

#[test]
fn startup_and_real_arrivals_speak_but_stationary_actions_do_not_reenter() {
    let game = load(GAME, "boundary.game").unwrap();
    let mut s = Session::start(&game).unwrap();
    assert_eq!(s.opening, ["Arrived in hall."]);
    assert_eq!(
        s.state.counters,
        [0],
        "unread declarations cannot shift values"
    );
    s.take("look-at door").unwrap();
    s.take("talk-to door").unwrap();
    assert!(s.log.iter().all(|t| t.forced.is_empty()));
    s.take("walk-to door").unwrap();
    assert_eq!(s.log[2].forced, ["Arrived in yard.", "Arrived in hall."]);
    assert!(s
        .transcript()
        .starts_with("  Arrived in hall.\n> look-at door"));
    assert_eq!(s.take("walk-to exit").unwrap(), Some(play::Stop::Ending));
}

#[test]
fn an_entry_cycle_is_reported_and_does_not_commit_a_partial_turn() {
    let source = GAME.replace(
        "  on-enter:\n    say \"Arrived in hall.\"",
        "  on-enter:\n    say \"Arrived in hall.\"\n    do move yard",
    );
    let game = load(&source, "cycle.game").unwrap();
    assert!(matches!(
        Session::start(&game),
        Err(play::Stop::EntryCycle(_))
    ));
}
