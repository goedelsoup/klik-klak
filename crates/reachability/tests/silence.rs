//! **The one guarantee `subsystem/interaction-resolver` exists for, held against four games.**
//!
//! *"Given a composed binding, produce exactly one outcome: an authored response, an authored
//! refusal, or a default. Never nothing. 'Never nothing' is the whole specification — a silent
//! miss is the failure players read as brokenness, and it is entirely preventable at this
//! boundary."*
//!
//! Nothing checked it for six phases, for two reasons in order: there was no response text in the
//! parse tree to be missing (`.yidam/decisions/authored-text.yml`), and no component that answered
//! in outcomes (`.yidam/decisions/two-components.yml`).
//!
//! **The check is narrower than it first looks, and the slice is what narrowed it.** Three turns
//! of a winning run of `lighthouse.game` say nothing — `talk-to keeper`, `walk-to door`,
//! `walk-to stair` — and none is a miss: the world moved and the movement is the response. What
//! counts is a binding the menu **shows**, which **declines** in that state, whose `else` list has
//! no clause covering it. `.yidam/decisions/playable.yml`.

mod common;

use reachability::analyse;

fn load(source: &str, name: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(source, name);
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{name}: {msgs:?}");
    g
}

/// **No binding in any specimen is shown, declines, and says nothing.**
#[test]
fn nothing_authored_here_declines_in_silence() {
    for (name, src) in common::sources() {
        let r = analyse(&load(&src, &name));
        assert!(r.silent.is_empty(), "{name}: {:?}", r.silent);
    }
}

/// **The check is not vacuous, and one of the two clauses it was written for is unreachable.**
///
/// `lighthouse.game`'s lamp refuses the oil can in two worded branches and never bare —
/// `else when lamp-prep = 0` and `else when lamp-prep >= 2` against a firing `= 1` over a `0..3`
/// counter. `.yidam/decisions/authored-text.yml` named it as the one candidate the corpus had and
/// declined to check it in the phase that invented the defect.
///
/// Deleting the first makes the game silent in a reachable state and the report says so.
/// **Deleting the second changes nothing at all**, and the reason is a finding: the binding that
/// raises `lamp-prep` to 2 is the same binding that spends the oil can, so nobody carrying an oil
/// can is ever standing in front of a lamp with `lamp-prep >= 2`. *"The reservoir is full."* is
/// authored text no player can reach — the **fourth** instance of that shape in this format, after
/// a refusal with nothing to refuse, a dead write, and an unreachable value.
#[test]
fn deleting_a_clause_of_a_guarded_refusal_is_reported() {
    let live = "      else when lamp-prep = 0    \"It would run straight through. It wants a wick first.\"\n";
    let dead = "      else when lamp-prep >= 2   \"The reservoir is full.\"\n";
    assert!(
        common::source("lighthouse").contains(live) && common::source("lighthouse").contains(dead)
    );

    let whole = analyse(&load(&common::source("lighthouse"), "lighthouse"));
    assert!(whole.silent.is_empty(), "the specimen is total as written");

    let holed = analyse(&load(
        &common::source("lighthouse").replace(live, ""),
        "no-wick-yet",
    ));
    assert_eq!(holed.silent.len(), 1, "{:?}", holed.silent);
    assert!(
        holed.silent[0].contains("use-with oil-can")
            && holed.silent[0].contains("its `else` says nothing there"),
        "{:?}",
        holed.silent
    );

    // The graph is untouched, which is the point: a refusal writes no state, so this is a defect
    // no reachability figure could ever have reported.
    assert_eq!(
        (
            whole.states,
            whole.transitions,
            whole.winnable,
            whole.stranded
        ),
        (
            holed.states,
            holed.transitions,
            holed.winnable,
            holed.stranded
        ),
        "the same game by every figure the search reports"
    );

    // And the other clause is unreachable: the binding that raises the counter past 1 spends the
    // can that is the key to the binding.
    let without = analyse(&load(
        &common::source("lighthouse").replace(dead, ""),
        "no-full-reservoir",
    ));
    assert!(
        without.silent.is_empty(),
        "deleting it leaves no state in which anything declines in silence: {:?}",
        without.silent
    );
}

/// **A binding shown without an `else` is withheld rather than silent, and the distinction is
/// authored.**
///
/// `.yidam/decisions/availability.yml` settled that a `when` with no `else` **withholds** and a
/// `when` with one **offers and declines**. So a binding with no refusal list is never in a menu in
/// a state where it would decline, and cannot be silent — which is why the check above reads the
/// menu rather than the binding.
#[test]
fn a_binding_with_no_else_is_never_asked_to_decline() {
    let game = load(&common::source("lighthouse"), "lighthouse");
    let gated_without_else = game
        .bindings
        .iter()
        .filter(|b| b.when.is_some() && !b.refuses)
        .count();
    let gated_with_else = game
        .bindings
        .iter()
        .filter(|b| b.when.is_some() && b.refuses)
        .count();
    assert_eq!(
        (gated_without_else, gated_with_else),
        (2, 6),
        "withholding and refusing, in the specimen that carries both forms"
    );

    // and the two that withhold are conversation options rather than hotspot verbs, which is
    // exactly what `subsystem/verb-menu` counted by reading the text: *"the corpus writes the
    // refusing form seventeen times and the withholding form never on a hotspot verb"*
    assert!(
        game.bindings
            .iter()
            .filter(|b| b.when.is_some() && !b.refuses)
            .all(|b| matches!(b.site, game_format::Site::InConversation { .. })),
        "a hotspot verb that withholds"
    );
    assert!(
        game.bindings
            .iter()
            .filter(|b| b.refuses)
            .all(|b| !b.refusals.is_empty()),
        "and every binding that declines has at least one line to decline with"
    );
}

/// **The cheap form `authored-text` proposed agrees with the search on every specimen, and asks
/// for one line no player reads.**
///
/// The record held the lamp's oil can as *"total, and total only because the counter declares
/// its range"*, and called the check cheap because `resolve.rs` already reasons about guards over
/// declared ranges. Read that way — every slot a binding's guards mention free over what it was
/// declared to hold, the second object carried — all four specimens are total, as the search says.
/// The two readings part on one edit: delete *"The reservoir is full."* and the declared reading
/// reports a hole at `lamp-prep >= 2` that the search, which knows the can is spent at 2, does
/// not. A declared range is an upper bound on what a game reaches, so the static form errs only
/// toward asking for text; the search is exact, and the search is where the check went.
#[test]
fn totality_over_declared_ranges_asks_for_a_line_no_player_reads() {
    use game_format::{Pred, State, World};

    type Domain = Vec<Box<dyn Fn(&mut State)>>;

    fn atoms<'p>(q: &'p Pred, into: &mut Vec<&'p Pred>) {
        match q {
            Pred::Not(a) => atoms(a, into),
            Pred::And(a, b) | Pred::Or(a, b) => {
                atoms(a, into);
                atoms(b, into);
            }
            _ => into.push(q),
        }
    }

    /// Bindings with an `else` list and no unguarded clause, which decline in silence somewhere
    /// over the declared ranges of the slots their guards read.
    fn declared_holes(g: &game_format::Game) -> Vec<String> {
        let w = World::new(g);
        let mut out = Vec::new();
        for b in &g.bindings {
            let Some(when) = &b.when else { continue };
            if !b.refuses || b.refusals.iter().any(|r| r.when.is_none()) {
                continue;
            }
            let mut a = Vec::new();
            atoms(when, &mut a);
            for r in &b.refusals {
                atoms(r.when.as_ref().expect("guarded"), &mut a);
            }
            // one free variable per slot read, over the values it was declared to hold
            let mut vars: Vec<(String, Domain)> = Vec::new();
            for q in a {
                let (key, dom): (String, Domain) = match q {
                    Pred::Flag(x) => {
                        let i = game_format::slot(&w.flags, x);
                        (
                            format!("flag {x}"),
                            (0..2)
                                .map(|v| Box::new(move |s: &mut State| s.flags[i] = v == 1) as _)
                                .collect(),
                        )
                    }
                    Pred::Chosen(x) => {
                        let i = game_format::slot(&w.options, x);
                        (
                            format!("chosen {x}"),
                            (0..2)
                                .map(|v| Box::new(move |s: &mut State| s.chosen[i] = v == 1) as _)
                                .collect(),
                        )
                    }
                    Pred::Carrying(x) | Pred::Count(x, ..) => {
                        let i = game_format::slot(&w.items, x);
                        (
                            format!("item {x}"),
                            (0..=w.caps[i])
                                .map(|v| Box::new(move |s: &mut State| s.items[i] = v) as _)
                                .collect(),
                        )
                    }
                    Pred::Cmp(x, ..) => {
                        let i = game_format::slot(&w.counters, x);
                        let c = g.counters.iter().find(|c| &c.name == x).expect("declared");
                        (
                            format!("counter {x}"),
                            (c.lo..=c.hi)
                                .map(|v| Box::new(move |s: &mut State| s.counters[i] = v) as _)
                                .collect(),
                        )
                    }
                    Pred::At(x, _) => {
                        let i = game_format::slot(&w.actors, x);
                        (
                            format!("actor {x}"),
                            (0..w.rooms.len())
                                .map(|v| Box::new(move |s: &mut State| s.actors[i] = v) as _)
                                .collect(),
                        )
                    }
                    _ => unreachable!("atoms"),
                };
                if !vars.iter().any(|(k, _)| *k == key) {
                    vars.push((key, dom));
                }
            }
            let mut base = w.initial();
            if let Some(o) = &b.object {
                base.items[game_format::slot(&w.items, o)] = 1;
            }
            let mut pick = vec![0usize; vars.len()];
            'every: loop {
                let mut s = base.clone();
                for (v, (_, dom)) in pick.iter().zip(&vars) {
                    dom[*v](&mut s);
                }
                let covered = w.holds(when, &s)
                    || b.refusals
                        .iter()
                        .any(|r| w.holds(r.when.as_ref().unwrap(), &s));
                if !covered {
                    out.push(format!(
                        "{}{}",
                        b.verb,
                        b.object
                            .as_ref()
                            .map(|o| format!(" {o}"))
                            .unwrap_or_default()
                    ));
                    break;
                }
                for (k, (_, dom)) in vars.iter().enumerate() {
                    pick[k] += 1;
                    if pick[k] < dom.len() {
                        continue 'every;
                    }
                    pick[k] = 0;
                }
                break;
            }
        }
        out
    }

    // as written, the two readings agree on every specimen
    for (name, src) in common::sources() {
        let g = load(&src, &name);
        assert!(declared_holes(&g).is_empty(), "{name}");
        assert!(analyse(&g).silent.is_empty(), "{name}");
    }

    // a reachable hole, both see
    let live = "      else when lamp-prep = 0    \"It would run straight through. It wants a wick first.\"\n";
    let holed = load(
        &common::source("lighthouse").replace(live, ""),
        "no-wick-yet",
    );
    assert_eq!(declared_holes(&holed), ["use-with oil-can"]);
    assert_eq!(analyse(&holed).silent.len(), 1);

    // an unreachable one, only the declared reading reports
    let dead = "      else when lamp-prep >= 2   \"The reservoir is full.\"\n";
    let full = load(
        &common::source("lighthouse").replace(dead, ""),
        "no-full-reservoir",
    );
    assert_eq!(
        declared_holes(&full),
        ["use-with oil-can"],
        "a line it would ask for"
    );
    assert!(analyse(&full).silent.is_empty(), "and no player could read");
}
