//! composes: verb-menu, interaction-resolver, response-presenter

use play::{load, Session};

const PATROL: &str = include_str!("../../../design/specimen/patrol.game");

#[test]
fn patrol_continues_identically_before_and_after_the_porter_moves() {
    let game = load(PATROL, "patrol.game").unwrap();
    let script = [
        "walk-to inner-door",
        "walk-to canteen-door",
        "pick-up tin",
        "walk-to switch",
        "walk-to corridor-door",
        "walk-to strongroom-door",
        "use-with key on cabinet",
        "pick-up bond-in-cabinet",
        "walk-to steel-door",
        "walk-to lobby-door",
        "walk-to street-doors",
    ];
    for boundary in [3, 4] {
        let mut original = Session::start(&game).unwrap();
        original.play(&script[..boundary]);
        let text = original.save().unwrap();
        assert!(text.contains("actor\tporter\t"));
        let mut restored = Session::start(&game).unwrap();
        restored.restore(&text).unwrap();
        assert_eq!(original.state, restored.state);
        assert!(restored.opening.is_empty());
        for choice in &script[boundary..] {
            assert_eq!(original.menu(), restored.menu());
            assert_eq!(original.take(choice), restored.take(choice));
            assert_eq!(original.state, restored.state);
            assert_eq!(
                original.log.last().unwrap().said,
                restored.log.last().unwrap().said
            );
        }
        assert!(restored.ended);
        assert!(restored.save().is_err());
    }
}

#[test]
fn names_survive_reordering_and_failed_loads_leave_the_session_untouched() {
    let game = load(PATROL, "patrol.game").unwrap();
    let mut original = Session::start(&game).unwrap();
    original.play(&[
        "walk-to inner-door",
        "walk-to canteen-door",
        "pick-up tin",
        "walk-to switch",
    ]);
    let text = original.save().unwrap();
    let reordered = PATROL.replace(
        "item    key         \"a small flat key\"\nitem    bond        \"the bearer bond\"",
        "item    bond        \"the bearer bond\"\nitem    key         \"a small flat key\"",
    );
    let other = load(&reordered, "reordered.game").unwrap();
    let mut restored = Session::start(&other).unwrap();
    restored.restore(&text).unwrap();
    assert_eq!(original.menu(), restored.menu());
    assert_eq!(restored.save().unwrap(), text);
    for bad in [
        text.replace("item\tkey\t1", "item\tkey\t99"),
        text.replace("actor\tporter\tcanteen", "actor\tporter\tmissing"),
        format!("{text}item\tkey\t1\n"),
        text.lines().skip(1).collect::<Vec<_>>().join("\n"),
        text.replace("flag\tkettle-on\ttrue\n", ""),
    ] {
        let before = restored.state.clone();
        assert!(restored.restore(&bad).is_err());
        assert_eq!(restored.state, before);
    }
    let changed = load(
        &PATROL.replace("Cold, and full enough.", "A different game."),
        "patrol.game",
    )
    .unwrap();
    assert!(Session::start(&changed).unwrap().restore(&text).is_err());
}

#[test]
fn restore_does_not_repeat_an_arrival() {
    let source = "room hall \"Hall\"\nroom hall\n  start\n  on-enter:\n    say \"Welcome.\"\n  hotspot door \"door\"\n    look-at:\n      say \"Closed.\"\n    walk-to:\n      ending\n";
    let game = load(source, "arrival.game").unwrap();
    let mut session = Session::start(&game).unwrap();
    let text = session.save().unwrap();
    session.restore(&text).unwrap();
    assert!(session.transcript().is_empty());
    session.take("look-at door").unwrap();
    assert!(!session.transcript().contains("Welcome"));
}
