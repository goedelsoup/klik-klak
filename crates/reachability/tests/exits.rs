//! `exit <name> to <room>`, and what it costs a checker.
//!
//! The first primitive this corpus has admitted on the second question in `AGENTS.md` rather
//! than the first. Each individual exit is sayable without it — a hotspot with a `walk-to` that
//! moves — and *an exit* is not, which is why `crates/reachability/tests/scaling.rs` writes that
//! shape out once per room in Rust.
//!
//! `.yidam/decisions/exit-archetype.yml` is the record. The tests below are its evidence: an
//! archetype that expands into the ordinary form before anything reads the game costs the checker
//! nothing and leaves every exception sayable. That is not a property of archetypes in general —
//! it is a property of expanding early, and `delores-source` is where the alternative shows.

use game_format::{Effect, Site};
use reachability::analyse;

const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");

fn load(source: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(source, "exits.game");
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{msgs:?}");
    g
}

fn problems(source: &str) -> Vec<String> {
    let (mut g, mut p) = game_format::parse(source, "exits.game");
    p.extend(game_format::resolve(&mut g));
    p.into_iter().map(|p| p.to_string()).collect()
}

#[test]
fn an_exit_expands_into_the_binding_an_author_would_have_written() {
    let g = load(
        "room a \"A\"\nroom b \"B\"\n\nroom a\n  start\n  exit door to b\n\n\
         room b\n  hotspot out \"out\"\n    walk-to:\n      ending\n",
    );
    let b = g
        .bindings
        .iter()
        .find(|b| matches!(&b.site, Site::InRoom { target, .. } if target == "a.door"))
        .expect("the exit declares a hotspot and hangs a binding on it");
    assert_eq!(b.verb, "walk-to");
    assert!(matches!(&b.effects[..], [Effect::Move(None, r)] if r == "b"));
    assert!(b.when.is_none());
}

/// The claim that makes the archetype safe: it is gone before anything reads the game. Rewriting
/// the specimen's three bare exits as declarations must not move a single figure.
#[test]
fn a_declared_exit_analyses_exactly_like_the_written_one() {
    let rewritten = LIGHTHOUSE
        .replace(
            "  hotspot stair \"a spiral stair\"\n    look-at:\n      \"Iron treads, worn to a shine in the middle, turning up into the dark.\"\n\n    walk-to:\n      do    move lamp-room\n",
            "  exit stair to lamp-room\n    look-at:\n      \"Iron treads, worn to a shine in the middle, turning up into the dark.\"\n",
        )
        .replace(
            "  hotspot door \"the iron door, from the inside\"\n    look-at:\n      \"It stands open on the quay, and the grey water beyond it.\"\n\n    walk-to:\n      do    move quay\n",
            "  exit door to quay\n    look-at:\n      \"It stands open on the quay, and the grey water beyond it.\"\n",
        );
    assert_ne!(rewritten, LIGHTHOUSE, "the rewrite must actually apply");

    let a = analyse(&load(LIGHTHOUSE));
    let b = analyse(&load(&rewritten));
    assert_eq!(
        (a.states, a.transitions, a.winnable, a.strandings.len()),
        (b.states, b.transitions, b.winnable, b.strandings.len()),
        "an exit that expands early is invisible to the checker:\n{}\n---\n{}",
        a.render(),
        b.render()
    );
}

/// **What `measured-width` found against archetypes, tested here.** Delores's two gated exits were
/// missed on a first reading because neither gate is where preconditions are written: one is a
/// `VERB_PRE` on a binding the engine generates and never writes down. The gate hid from a
/// *reader*. It cannot hide from a checker here, because the expansion happens at parse time and
/// what the checker reads is the ordinary binding with its guard on it.
#[test]
fn a_gate_written_under_an_exit_reaches_the_checker() {
    let src = "room a \"A\"\nroom b \"B\"\nflag open \"the door is open\"\n\n\
               room a\n  start\n  exit door to b\n    when  open\n    else  \"Locked.\"\n\
               \x20 hotspot lever \"a lever\"\n    walk-to:\n      do    set open\n\n\
               room b\n  hotspot out \"out\"\n    walk-to:\n      ending\n"
        .to_string();
    let g = load(&src);
    let b = g
        .bindings
        .iter()
        .find(|b| matches!(&b.site, Site::InRoom { target, .. } if target == "a.door"))
        .unwrap();
    assert!(b.when.is_some(), "the guard reached the binding");
    assert!(matches!(&b.effects[..], [Effect::Move(None, r)] if r == "b"));

    // and the checker sees the ordering: pull the lever's write and the ending is unreachable
    let broken = src.replace("      do    set open\n", "      \"Nothing happens.\"\n");
    assert_ne!(broken, src);
    assert!(analyse(&load(&src)).winnable);
    assert!(
        !analyse(&load(&broken)).winnable,
        "a gate declared under an exit gates the game"
    );
}

/// The census the primitive was admitted on, held against both bodies of authored work.
#[test]
fn the_bare_exit_is_the_dominant_shape_in_both_bodies_of_work() {
    let mut total = 0;
    let mut bare = 0;
    for src in [
        include_str!("../../../design/specimen/ferry.game"),
        LIGHTHOUSE,
        include_str!("../../../design/specimen/nightwatch.game"),
        include_str!("../../../design/specimen/patrol.game"),
    ] {
        let lines: Vec<&str> = src.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            if l.trim() != "walk-to:" {
                continue;
            }
            let ind = l.len() - l.trim_start().len();
            let mut body = Vec::new();
            for next in &lines[i + 1..] {
                if next.trim().is_empty() || next.len() - next.trim_start().len() <= ind {
                    break;
                }
                body.push(next.trim());
            }
            if !body
                .iter()
                .any(|b| b.starts_with("do") && b.contains("move"))
            {
                continue;
            }
            total += 1;
            if body.len() == 1 {
                bare += 1;
            }
        }
    }
    // Delores ships 20 exits, 18 of them a bare declaration and 2 gated
    // (`.yidam/catalog/delores-source.md`). Four specimens written without reference to it:
    assert_eq!((total, bare), (20, 15), "the exit census the record quotes");
}

#[test]
fn an_exit_that_does_not_say_where_it_goes_is_refused() {
    let bad = "room a \"A\"\n\nroom a\n  start\n  exit door b\n";
    assert!(
        problems(bad)
            .iter()
            .any(|m| m.contains("the `to` is what names where it goes")),
        "{:?}",
        problems(bad)
    );
    let outside = "room a \"A\"\n  exit door to a\n";
    assert!(problems(outside)
        .iter()
        .any(|m| m.contains("outside a room section")));
}
