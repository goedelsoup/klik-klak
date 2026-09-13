//! What a condition costs when the vocabulary has no name for it.
//!
//! `puzzle/state-gate` carried this as `[open]` and called it "the strongest live case for
//! *adding* a primitive rather than removing one": *"a gate depending on three flags is stated
//! three times, or more, and nothing checks that the statements agree."*
//!
//! Measured against the four specimens, the shape it describes does not occur — there is one
//! compound predicate in thirty-six — and a different one does. A staged puzzle is a run of
//! bindings each guarded on the value the last one wrote, and the agreement between the write and
//! the read is held by the author alone.
//!
//! So the three ways to break that agreement are enumerated here and each is run. One was
//! invisible before this phase and made the report *better*; it is now refused at resolve time by
//! `unreachable_values` in `crates/game-format/src/resolve.rs`, which needed no new syntax
//! because the steps were already implicit in the bindings.
//!
//! `.yidam/decisions/named-condition.yml` is the record.

mod common;

use game_format::{Cmp, Effect, Game, Pred, Site};
use std::collections::BTreeMap;

fn lighthouse() -> String {
    common::source("lighthouse")
}

fn load(name: &str, src: &str) -> Game {
    let (mut game, problems) = game_format::parse(src, name);
    let problems: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
        .map(|p| p.to_string())
        .collect();
    assert!(
        problems.is_empty(),
        "{name} does not resolve:\n{}",
        problems.join("\n")
    );
    game
}

/// The problems a source produces, without asserting it has none.
fn problems(name: &str, src: &str) -> Vec<String> {
    let (mut game, problems) = game_format::parse(src, name);
    problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
        .map(|p| p.to_string())
        .collect()
}

fn every_pred(g: &Game) -> Vec<&Pred> {
    let mut v = Vec::new();
    for b in &g.bindings {
        if let Some(w) = &b.when {
            v.push(w);
        }
        v.extend(b.guards());
    }
    v
}

fn atom_count(p: &Pred) -> usize {
    match p {
        Pred::Not(q) => atom_count(q),
        Pred::And(a, b) | Pred::Or(a, b) => atom_count(a) + atom_count(b),
        _ => 1,
    }
}

fn subjects(p: &Pred, out: &mut Vec<String>) {
    match p {
        Pred::Not(q) => subjects(q, out),
        Pred::And(a, b) | Pred::Or(a, b) => {
            subjects(a, out);
            subjects(b, out);
        }
        Pred::Flag(x) | Pred::Carrying(x) | Pred::Chosen(x) => out.push(x.clone()),
        Pred::Cmp(x, ..) | Pred::Count(x, ..) => out.push(x.clone()),
        Pred::At(a, _) => out.push(a.clone()),
    }
}

/// **The gate `state-gate` describes is exactly the gate the fifth specimen has, and the decision
/// that said it does not occur was resting on four games that had no mechanism.**
///
/// This test read *"there are no multi-flag conditions to give a name to"* and asserted **one
/// compound predicate in thirty-six**. That figure is what `.yidam/decisions/named-condition.yml`
/// declined a primitive on, and what `puzzle/state-gate` carried as `[verified]`.
///
/// It is **eight in sixty**, and seven of the eight are in `lock.game` — which this file could not
/// see for eleven commits, because its input was a list of the four games that existed when it was
/// written (`.yidam/decisions/unread-specimen.yml`).
///
/// **`gates-shut and paddle-up` is written at four sites.** Twice on its own, once with
/// `boatman in chamber` after it, and once inside a five-atom conjunction:
///
/// ```text
/// when gates-shut and paddle-up and boatman in lockside and mate in lockside and not gates-open
/// ```
///
/// That is `state-gate`'s sentence, which was written at genesis and called imagined: *"a gate
/// depending on three flags is stated three times, or more, and nothing checks that the statements
/// agree."* It was not imagined. It was waiting for somebody to write a game with a mechanism in
/// it — a lock with two gates, a paddle and two crew, where being allowed to do a thing depends on
/// the state of the water **and** where two other people are standing.
///
/// **This does not re-decide anything.** `named-condition` declined the primitive for reasons
/// beyond the census — the successor relation is derivable, a static check closed the gap — and
/// weighing those against a corpus that now has the shape is a phase, not a re-pin. What is
/// recorded here is that the premise it led with is gone, and `puzzle/state-gate`'s question is
/// open again.
#[test]
fn the_multi_flag_gate_is_written_and_one_conjunction_is_written_four_times() {
    let mut total = 0;
    let mut compound = 0;
    let mut shown: Vec<String> = Vec::new();
    for (name, src) in common::sources() {
        for p in every_pred(&load(&name, &src)) {
            total += 1;
            if atom_count(p) > 1 {
                compound += 1;
                shown.push(format!("{name}: {} atoms  {p:?}", atom_count(p)));
            }
        }
    }
    assert_eq!(total, 76, "predicates across the specimens");
    assert_eq!(
        compound, 11,
        "predicates joining more than one atom: {shown:#?}"
    );
    assert_eq!(
        shown.iter().filter(|s| s.starts_with("lock:")).count(),
        7,
        "and all but one are in the game with a mechanism"
    );
    assert!(
        shown.iter().any(|s| s.starts_with("lock: 5 atoms")),
        "the widest gate, which is the shape `state-gate` described"
    );

    // and the restatement itself: one conjunction, written into four separate guards
    let mut restated = 0;
    for (name, src) in common::sources() {
        for p in every_pred(&load(&name, &src)) {
            if format!("{p:?}").contains("And(Flag(\"gates-shut\"), Flag(\"paddle-up\"))") {
                restated += 1;
            }
        }
    }
    assert_eq!(
        restated, 4,
        "`gates-shut and paddle-up`, written by hand into four preconditions with nothing \
         checking that the four agree — which is what a named condition would abbreviate"
    );
}

/// **What is restated is one counter's position in a sequence, nine times, on one hotspot.**
#[test]
fn the_pressure_is_a_sequence_and_it_is_all_on_one_target() {
    let g = load("lighthouse", &lighthouse());

    let mut reads: BTreeMap<String, usize> = BTreeMap::new();
    for p in every_pred(&g) {
        let mut s = Vec::new();
        subjects(p, &mut s);
        for x in s {
            *reads.entry(x).or_default() += 1;
        }
    }
    let (worst, n) = reads.iter().max_by_key(|(_, n)| **n).unwrap();
    assert_eq!(worst, "lamp-prep");
    assert_eq!(*n, 9, "sites reading the lamp sequence");

    // Every one of them is on the lamp itself, so this is not a relation between things.
    let on_lamp = g
        .bindings
        .iter()
        .filter(|b| {
            let mut s = Vec::new();
            if let Some(w) = &b.when {
                subjects(w, &mut s);
            }
            for gd in b.guards() {
                subjects(gd, &mut s);
            }
            s.iter().any(|x| x == "lamp-prep")
        })
        .all(|b| matches!(&b.site, Site::InRoom { target, .. } if target == "lamp-room.lamp"));
    assert!(on_lamp, "every reader of lamp-prep is sited on the lamp");
}

/// **The invariant is between a value one binding writes and a value another requires.**
#[test]
fn each_step_writes_the_value_the_next_one_demands() {
    let g = load("lighthouse", &lighthouse());
    let mut writes = Vec::new();
    for b in &g.bindings {
        for e in &b.effects {
            if let Effect::SetCounter(x, n) = e {
                if x == "lamp-prep" {
                    writes.push(*n);
                }
            }
        }
    }
    assert_eq!(writes, vec![1, 2, 3], "wick, then oil, then the match");

    let required: Vec<i64> = every_pred(&g)
        .iter()
        .filter_map(|p| match p {
            Pred::Cmp(x, Cmp::Eq, n) if x == "lamp-prep" => Some(*n),
            _ => None,
        })
        .collect();
    for n in &writes[..2] {
        assert!(
            required.contains(n),
            "{n} is written by one binding and required by another, and nothing relates them"
        );
    }
}

/// **The three ways to break the sequence, and what each one costs.**
///
/// The middle one was always caught and the last one always shouted. The first was invisible: it
/// removed every stranding the specimen has and left the ending reachable, so the report got
/// *better*. That is the mutation an author is most likely to make.
#[test]
fn breaking_the_agreement_three_ways() {
    // 1. Write the wrong next value: the oil step becomes unreachable.
    let skip = lighthouse().replace("do    set lamp-prep 1", "do    set lamp-prep 2");
    assert_ne!(skip, lighthouse(), "the mutation applied");
    let found = problems("skip", &skip);
    assert_eq!(found.len(), 2, "refused at two sites: {found:?}");
    for f in &found {
        assert!(
            f.contains("no value `lamp-prep` can hold satisfies `lamp-prep = 1`"),
            "unexpected problem: {f}"
        );
    }

    // Without the check it resolves cleanly and analyses *better* than the specimen.
    let base = reachability::analyse(&load("lighthouse", &lighthouse()));
    assert_eq!(base.states, 91);
    assert_eq!(base.stranded, 16, "the specimen is deliberately trapped");

    // 2. Require the wrong value: two steps accept the same one, and reachability catches it.
    let race = lighthouse().replace(
        "when  lamp-prep = 1\n      say   \"You fill",
        "when  lamp-prep = 0\n      say   \"You fill",
    );
    assert_ne!(race, lighthouse());
    assert!(
        problems("race", &race).is_empty(),
        "resolves — this one is a reachability finding"
    );
    let r = reachability::analyse(&load("race", &race));
    assert!(
        r.winnable,
        "still winnable, which is why it needs the stranding report"
    );
    assert_eq!(r.stranded, 40, "up from the specimen's 16");
    assert!(
        r.strandings.iter().any(|s| s.target == "lamp-room.lamp"),
        "and it names the lamp"
    );

    // 3. Require a value only this binding writes: the ending goes.
    let orphan = lighthouse().replace(
        "when  lamp-prep = 2\n      say   \"The wick catches",
        "when  lamp-prep = 3\n      say   \"The wick catches",
    );
    assert_ne!(orphan, lighthouse());
    assert!(problems("orphan", &orphan).is_empty());
    assert!(
        !reachability::analyse(&load("orphan", &orphan)).winnable,
        "the ending is unreachable and the report says so"
    );
}

/// **A second pressure the census turned up, and it is not a condition at all.**
///
/// `case-open` is read at four sites and `cabinet-open` at four, and in both games two of the four
/// are on a *different* hotspot — the thing inside the container, asking about the container.
/// Those hotspots are named `ledger-in-case` and `bond-in-cabinet`, which is the containment
/// written into the identifier because the vocabulary cannot state it.
#[test]
fn containment_is_spelled_in_the_hotspot_name() {
    for (game, flag, container, contents) in [
        (
            "nightwatch",
            "case-open",
            "study.case",
            "study.ledger-in-case",
        ),
        (
            "patrol",
            "cabinet-open",
            "strongroom.cabinet",
            "strongroom.bond-in-cabinet",
        ),
    ] {
        let src = common::source(game);
        let g = load(game, &src);
        let mut sites: Vec<&str> = Vec::new();
        for b in &g.bindings {
            let mut s = Vec::new();
            if let Some(w) = &b.when {
                subjects(w, &mut s);
            }
            for gd in b.guards() {
                subjects(gd, &mut s);
            }
            if s.iter().any(|x| x == flag) {
                if let Site::InRoom { target, .. } = &b.site {
                    sites.push(target);
                }
            }
        }
        assert_eq!(sites.len(), 4, "{game}: sites reading {flag}");
        assert_eq!(
            sites.iter().filter(|t| **t == container).count(),
            2,
            "{game}: on the container"
        );
        assert_eq!(
            sites.iter().filter(|t| **t == contents).count(),
            2,
            "{game}: on what is inside it — a relation stated by naming"
        );
        assert!(contents.contains("-in-"), "and stated in the name");
    }
}
