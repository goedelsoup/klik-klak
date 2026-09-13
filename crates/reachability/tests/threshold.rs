//! **A sequence that begins because a counter reached a value** — the last leg
//! [`cutscene/scripted-sequence`] stands on.
//!
//! Two of the three triggers that class claimed have been housed or refused. *Entering a room* is
//! a position event and became `on-enter`. *Binding resolution* needed an edge into a class no
//! edge reaches. The third has sat `[open]` for four phases with the question stated as: *"whether
//! that is a genuine gap or the same position trigger wearing a number."*
//!
//! It is neither, and the reason is a decision this corpus made before the question was asked. A
//! threshold trigger needs an **occasion** — a moment at which somebody looks to see whether the
//! value has been reached. AGS has one and it is a game cycle, forty times a second; the condition
//! that goes in it is a scripted `if`. [`effect-list-semantics`] refused duration here, so there
//! are no cycles and nothing runs between bindings. **The only occasions are the bindings that
//! write the value**, and those are enumerable.
//!
//! So the trigger is expressible by appending the sequence to every binding that crosses the
//! threshold, and what that costs is one copy per writer.
//! `.yidam/decisions/threshold-trigger.yml`.
//!
//! [`cutscene/scripted-sequence`]: ../../../.yidam/corpus/cutscene/scripted-sequence.yml
//! [`effect-list-semantics`]: ../../../.yidam/decisions/effect-list-semantics.yml

mod common;

use game_format::Effect;
use reachability::analyse;
use std::collections::BTreeMap;

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "threshold.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// **Every state value about the world is written in exactly one place, and the one that is not
/// about the world is written at six.**
///
/// This test asserted *"every state value in the corpus is written in exactly one place"* over
/// twelve writes in four games, and `.yidam/decisions/threshold-trigger.yml` declined the trigger
/// on it: *"a threshold trigger buys nothing where the threshold has a single door."* It could not
/// see the fifth game for eleven commits (`.yidam/decisions/unread-specimen.yml`).
///
/// Eighteen writes now, and **two of them have more than one writer: `set ashore` and
/// `clear ashore`**, at lines 88, 104, 206 and 122, 195, 232 of `lock.game`. Sixteen of eighteen
/// still have a single door.
///
/// **It is the same exception for the third time in this file's neighbourhood.** `ashore` is the
/// flag `lock.game` spends on recording which character the player is driving, because
/// `.yidam/decisions/control-transfer.yml` found the format has no site for a control switch. It
/// is also the only flag anything clears (`latch.rs`), and the only atom in a five-way conjunction
/// (`conditions.rs`). Every generalisation this corpus had made about flags — latched, single-
/// writer, never joined — holds for flags that record something about the world and fails for the
/// one that records who you are.
///
/// **The decision survives and its reason changes.** A trigger buys nothing here not because every
/// value has one door, but because the value with six doors has no sequence to append: the three
/// sets and the three clears are the same event from two sides, and what an author wants at each
/// is the sentence, which they wrote.
#[test]
fn only_the_flag_that_is_not_about_the_world_has_more_than_one_writer() {
    let mut writers: BTreeMap<String, usize> = BTreeMap::new();
    for (n, src) in common::sources() {
        for b in &load(&src).bindings {
            for e in &b.effects {
                let k = match e {
                    Effect::SetFlag(f) => format!("{n} set {f}"),
                    Effect::ClearFlag(f) => format!("{n} clear {f}"),
                    Effect::SetCounter(c, v) => format!("{n} {c}={v}"),
                    _ => continue,
                };
                *writers.entry(k).or_default() += 1;
            }
        }
    }
    assert_eq!(writers.len(), 24);
    let multi: Vec<&String> = writers
        .iter()
        .filter(|(_, n)| **n > 1)
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        multi,
        ["lock clear ashore", "lock set ashore"],
        "a second value has grown a second writer; whether it is a fact about the world is the \
         question to ask before re-pinning this"
    );
    assert_eq!(
        (writers["lock set ashore"], writers["lock clear ashore"]),
        (3, 3),
        "three sites each, and they are the same switch from two sides"
    );
}

/// **With one writer the reduction is free, and the sequence is indistinguishable from the
/// binding.** Appending the effects to the binding that crosses the threshold gives the game the
/// trigger would have given, at no cost in text and none in the report.
#[test]
fn with_one_writer_a_threshold_sequence_is_the_writer() {
    const BASE: &str = "room a \"A\"\nroom b \"B\"\nitem k \"a key\"\n\
                        counter lock 0..2 \"turns of the lock\"\nflag opened latched \"the door gave\"\n\n\
                        room a\n  start\n\
                        \x20 hotspot key \"a key\"\n    is k\n    pick-up:\n      do    give k\n\
                        \x20 hotspot door \"a door\"\n    use-with k:\n      when  lock = 0\n      do    set lock 1\n      else  \"Once is enough.\"\n\
                        \x20 hotspot handle \"a handle\"\n    walk-to:\n      when  lock = 1\n      do    set lock 2\n{SEQ}      else  \"Not yet.\"\n\
                        \x20 hotspot out \"out\"\n    walk-to:\n      when  opened\n      ending\n      else  \"Shut.\"\n\n\
                        room b\n  hotspot back \"back\"\n    walk-to:\n      do    move a\n";
    // the sequence the trigger would have run when `lock` reaches 2
    const SEQ: &str =
        "      say   \"The bolt goes over and the door swings in.\"\n      do    set opened\n";

    let without = BASE.replace("{SEQ}", "");
    let with = BASE.replace("{SEQ}", SEQ);
    assert!(
        !analyse(&load(&without)).winnable,
        "without the sequence nothing opens the door"
    );
    let g = load(&with);
    assert!(analyse(&g).winnable);

    // one binding carries it, and it is the one that writes the value
    let carriers = g
        .bindings
        .iter()
        .filter(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, Effect::SetFlag(f) if f == "opened"))
        })
        .count();
    assert_eq!(carriers, 1);
    assert!(g.bindings.iter().any(|b| {
        b.effects
            .iter()
            .any(|e| matches!(e, Effect::SetCounter(c, 2) if c == "lock"))
            && b.effects
                .iter()
                .any(|e| matches!(e, Effect::SetFlag(f) if f == "opened"))
    }));
}

/// **With three writers it is three copies**, which is the shape
/// `.yidam/decisions/arrival-trigger.yml` admitted `on-enter` for — one authored intent, one copy
/// per door — arriving at the other kind of trigger.
///
/// The difference is what happens when a copy goes missing. `on-enter` had a specimen where the
/// drift changed the game while every finding stayed identical. Here it moves a figure, and only
/// because a binding that writes nothing is not a transition at all: the lever with no `set` under
/// it drops out of the graph. That is detection by accident rather than a check, and it stops
/// working the moment the lever has anything else to do.
#[test]
fn with_three_writers_it_is_three_copies() {
    let game = |seq_on: [bool; 3]| {
        let mut s = String::from(
            "room a \"A\"\nflag alarmed latched \"the alarm has gone off\"\n\nroom a\n  start\n",
        );
        for (i, on) in seq_on.iter().enumerate() {
            s += &format!(
                "  hotspot lever{i} \"a lever\"\n    walk-to:\n{}",
                if *on {
                    "      say   \"Bells, everywhere.\"\n      do    set alarmed\n"
                } else {
                    "      \"You pull it. Nothing.\"\n"
                }
            );
        }
        s += "  hotspot out \"out\"\n    walk-to:\n      when  alarmed\n      ending\n      else  \"Quiet.\"\n";
        s
    };

    let all = load(&game([true, true, true]));
    let copies = all
        .bindings
        .iter()
        .filter(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, Effect::SetFlag(f) if f == "alarmed"))
        })
        .count();
    assert_eq!(copies, 3, "one authored intent, once per writer");

    let (a, b) = (analyse(&all), analyse(&load(&game([true, true, false]))));
    assert!(a.winnable && b.winnable, "the other two still reach it");
    assert_eq!(a.states, b.states, "the missing copy costs no state");
    assert!(
        a.transitions > b.transitions,
        "it costs a transition, and only because a binding that writes nothing is not one: \
         {} against {}",
        a.transitions,
        b.transitions
    );
}
