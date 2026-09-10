//! The effect list, as `.yidam/decisions/effect-list-semantics.yml` settles it.
//!
//! Sequential, total, instantaneous, atomic: one binding is one state transition, the list is
//! folded in written order, and nothing observes an intermediate value. Each test here fails
//! if some later reading quietly replaces one of those four words.

use game_format::{parse, resolve, Effect};

/// A game with one room, one flag, one counter, one item and one ending, so that a test can
/// say what it is about in the two lines that differ.
fn game_with(effects: &str) -> (game_format::Game, Vec<String>) {
    let source = format!(
        "room    hall \"The Hall\"\n\
         flag    marked \"a mark\"\n\
         counter tally 0..3 \"a tally\"\n\
         item    coin  0..4 \"a coin\"\n\
         \n\
         room hall\n\
         \x20 start\n\
         \n\
         \x20 hotspot lever \"a lever\"\n\
         \x20   use-with coin:\n{effects}\n\
         \n\
         \x20 hotspot door \"the door\"\n\
         \x20   walk-to:\n\
         \x20     ending\n"
    );
    let (mut game, mut problems) = parse(&source, "fixture.game");
    problems.extend(resolve(&mut game));
    let msgs = problems.into_iter().map(|p| p.to_string()).collect();
    (game, msgs)
}

#[test]
fn a_list_is_kept_in_written_order() {
    let (game, problems) = game_with("      do    give coin 2\n      do    spend coin 1");
    assert!(problems.is_empty(), "should resolve: {problems:?}");
    let b = game
        .bindings
        .iter()
        .find(|b| b.verb == "use-with")
        .expect("the lever");
    match (&b.effects[0], &b.effects[1]) {
        (Effect::Give(a, 2), Effect::Spend(c, 1)) => {
            assert_eq!(a, "coin");
            assert_eq!(c, "coin");
        }
        other => panic!("order not preserved: {other:?}"),
    }
}

#[test]
fn two_writes_to_one_flag_is_a_dead_write() {
    let (_, problems) = game_with("      do    set marked\n      do    clear marked");
    assert!(
        problems.iter().any(|p| p.contains("overwritten")
            && p.contains("set marked")
            && p.contains("clear marked")),
        "set-then-clear should be rejected; got {problems:?}"
    );
}

#[test]
fn two_writes_to_one_counter_is_a_dead_write() {
    let (_, problems) = game_with("      do    set tally 1\n      do    set tally 2");
    assert!(
        problems
            .iter()
            .any(|p| p.contains("overwritten") && p.contains("counter tally")),
        "two counter writes should be rejected; got {problems:?}"
    );
}

/// The distinction the check turns on, and the reason `Give`/`Spend` are exempt: they
/// accumulate. Under a rule that simply forbade naming a target twice this would be a false
/// positive, and `ferry.game`'s toll would not be writable.
#[test]
fn give_then_spend_is_a_net_change_and_not_a_dead_write() {
    let (_, problems) = game_with("      do    give coin 3\n      do    spend coin 1");
    assert!(
        !problems.iter().any(|p| p.contains("overwritten")),
        "accumulating effects must not be called dead writes; got {problems:?}"
    );
}

/// Two effects that write *different* slots are independent, and the check must not couple
/// them just because they sit in one list.
#[test]
fn different_slots_do_not_collide() {
    let (_, problems) = game_with("      do    set marked\n      do    set tally 2");
    assert!(problems.is_empty(), "should resolve: {problems:?}");
}

/// Both specimens, against the rule. This is the one that would catch a later edit writing a
/// value nobody can see.
#[test]
fn neither_specimen_contains_a_dead_write() {
    for (name, src) in [
        (
            "lighthouse",
            include_str!("../../../design/specimen/lighthouse.game"),
        ),
        ("ferry", include_str!("../../../design/specimen/ferry.game")),
    ] {
        let (mut game, mut problems) = parse(src, name);
        problems.extend(resolve(&mut game));
        let dead: Vec<String> = problems
            .iter()
            .filter(|p| p.message.contains("overwritten"))
            .map(|p| p.to_string())
            .collect();
        assert!(
            dead.is_empty(),
            "{name} has dead writes:\n{}",
            dead.join("\n")
        );
    }
}
