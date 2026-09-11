//! The default binding, and the three distinctions that were said to collapse into it.
//!
//! `data-format/interaction-table` names the collapse twice — *"a binding never authored, a binding
//! authored as empty, and a binding authored to refuse"* — and `interaction/unary-binding` carries
//! it as `[open]`, on the reading that it is *"a distinction the data format has to be able to
//! make"*.
//!
//! The distinction lands in three different layers and the answer is different in each. The format
//! makes it: the three are three different things to write. The checker cannot make it, and is
//! right not to, which is what the second test holds. And what the distinction is *for* belongs to
//! an interface this design has never specified — `.yidam/decisions/default-binding.yml`.

use reachability::analyse;

const FERRY: &str = include_str!("../../../design/specimen/ferry.game");
const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");
const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");
const PATROL: &str = include_str!("../../../design/specimen/patrol.game");

fn load(source: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(source, "defaults.game");
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{msgs:?}");
    g
}

/// **How much of the table is authored, which is the number the default's whole argument rests on
/// and which this corpus had never computed for itself.**
///
/// `interaction/default-binding` reasons that the default *"handles the great majority of what a
/// player actually triggers, which makes it the single highest-leverage piece of authored content
/// in a game"*. That is true at Delores's scale — 164 objects against 57 verbs is 9,348 cells and
/// 203 are authored, **2.17%** — and it is a fact about the size of the verb vocabulary rather than
/// about adventure games. This design has six verbs.
#[test]
fn the_leverage_of_a_default_is_a_function_of_how_many_verbs_there_are() {
    let mut sites = 0;
    let mut bindings = 0;
    for src in [FERRY, LIGHTHOUSE, NIGHTWATCH, PATROL] {
        let g = load(src);
        sites += g
            .declared
            .values()
            .filter(|(k, _)| matches!(k, game_format::Kind::Hotspot | game_format::Kind::Actor))
            .count();
        bindings += g
            .bindings
            .iter()
            .filter(|b| matches!(b.site, game_format::Site::InRoom { .. }))
            .count();
    }
    let cells = sites * 6;
    assert_eq!((sites, bindings), (42, 87));
    let filled = bindings as f64 / cells as f64;
    assert!(
        (0.33..0.36).contains(&filled),
        "{bindings} of {cells} cells = {filled:.3}; the record quotes 34.5% against Delores's 2.17%"
    );
}

/// **The checker cannot tell the three apart, and the reason it is right not to is the whole
/// answer.** A response that writes nothing is not a state transition, so a hotspot nobody can act
/// on, one with a line of description, and one with a guarded pair of lines are the same object to
/// a reachability search: they add no state and — this is the part worth pinning — no transition
/// either.
///
/// So the collapse `unary-binding` calls a defect of the data format is not one here. The format
/// distinguishes all three by what is written in it. What cannot distinguish them is the thing that
/// asks whether the game is winnable, and that is correct behaviour rather than lost information.
#[test]
fn a_response_that_writes_nothing_is_not_a_transition() {
    let base = analyse(&load(LIGHTHOUSE));
    let anchor = "  hotspot crate \"a nailed-shut crate\"\n";

    let variants = [
        ("never authored", "  hotspot rock \"a rock\"\n"),
        (
            "authored, and does nothing",
            "  hotspot rock \"a rock\"\n    look-at:\n      \"A rock.\"\n",
        ),
        (
            "authored to refuse",
            "  hotspot rock \"a rock\"\n    look-at:\n      when door-open  \"A rock.\"\n      \
             otherwise       \"Too dark to make it out.\"\n",
        ),
    ];
    for (what, insert) in variants {
        let src = LIGHTHOUSE.replace(anchor, &format!("{insert}{anchor}"));
        assert_ne!(src, LIGHTHOUSE, "`{what}` did not apply");
        let r = analyse(&load(&src));
        assert_eq!(
            (r.states, r.transitions, r.winnable, r.strandings.len()),
            (
                base.states,
                base.transitions,
                base.winnable,
                base.strandings.len()
            ),
            "`{what}` was expected to be indistinguishable to the checker"
        );
    }
}

/// **The interface assumption nothing in this design had written down.** `offered` enumerates
/// authored bindings, so a verb the author never wrote for a target is not a transition and no
/// default fires — which is the *contextual menu* Delores ships, where the popup lists only the
/// verbs an object declares, and not the fixed verb bar the corpus's ontology reasons from.
///
/// This test is what would fail if fall-through were ever added, and it exists to make that a
/// decision rather than a drift.
#[test]
fn an_unauthored_cell_is_not_reachable_and_nothing_stands_in_for_it() {
    let base = analyse(&load(LIGHTHOUSE));
    let src = LIGHTHOUSE.replace(
        "  hotspot crate \"a nailed-shut crate\"\n",
        "  hotspot rock \"a rock\"\n  hotspot crate \"a nailed-shut crate\"\n",
    );
    assert_ne!(src, LIGHTHOUSE);
    let g = load(&src);
    assert!(
        g.declared.contains_key("quay.rock"),
        "the hotspot is declared, so this is not a test about parsing"
    );
    assert!(
        !g.bindings.iter().any(
            |b| matches!(&b.site, game_format::Site::InRoom { target, .. } if target == "quay.rock")
        ),
        "and nothing is authored for it"
    );
    let r = analyse(&g);
    assert_eq!(
        (r.states, r.transitions),
        (base.states, base.transitions),
        "six verbs against a new target is six cells, and none of them is a transition"
    );
}

/// Strip every refusal from all four specimens — every bare `else "<line>"` and every
/// `otherwise` — and nothing moves. Not a state, not a transition, not a stranding.
///
/// That is the same fact as `a_response_that_writes_nothing_is_not_a_transition`, one layer along,
/// and it is why the question this corpus opened twice — whether a failing guard should refuse or
/// simply not offer the verb — **cannot be settled by anything in this repository**. `offered`
/// gates on `when` at all three sites and the model carries no text, so the two readings are the
/// same program. `.yidam/decisions/availability.yml`.
#[test]
fn deleting_every_refusal_moves_no_figure() {
    for src in [FERRY, LIGHTHOUSE, NIGHTWATCH, PATROL] {
        let stripped: String = src
            .lines()
            .filter(|l| {
                let t = l.trim();
                !(t.starts_with("otherwise")
                    || (t.starts_with("else") && !t.starts_with("else when")))
            })
            .map(|l| format!("{l}\n"))
            .collect();
        assert_ne!(stripped, src, "the strip must apply");
        let (a, b) = (analyse(&load(src)), analyse(&load(&stripped)));
        assert_eq!(
            (a.states, a.transitions, a.winnable, a.strandings.len()),
            (b.states, b.transitions, b.winnable, b.strandings.len()),
        );
    }
}

/// **The vocabulary already spells both, and the specimens never choose.**
///
/// A `when` with no refusal under it is legal and occurs three times in 662 authored lines: twice
/// on a conversation option, where an unavailable choice is simply not shown, and once on an
/// `on-enter`, where `interaction/entry-binding` argued a refusal *cannot* exist because nobody
/// tried anything. Every one of the seventeen gated bindings on a hotspot verb carries a refusal —
/// no exceptions, in four games by one hand. (A `when <pred> "<line>"` clause is not a gate: it
/// selects a line inside a response that always applies, which is the other eleven.)
///
/// So the bare `when` means *not offered* at both sites where it appears, and the author has never
/// once reached for it where a refusal was possible.
#[test]
fn a_gate_without_a_refusal_appears_only_where_a_refusal_is_impossible() {
    let (mut bare_verb, mut bare_option, mut bare_enter, mut gated_verb) = (0, 0, 0, 0);
    for src in [FERRY, LIGHTHOUSE, NIGHTWATCH, PATROL] {
        let lines: Vec<&str> = src.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            let head = t
                .strip_suffix(':')
                .map(|h| h.split_whitespace().next().unwrap_or(""));
            let kind = match head {
                Some("on-enter") => "enter",
                Some(h)
                    if [
                        "look-at", "pick-up", "walk-to", "talk-to", "use-with", "give-to",
                    ]
                    .contains(&h) =>
                {
                    "verb"
                }
                _ if t.starts_with("option ") => "option",
                _ => {
                    i += 1;
                    continue;
                }
            };
            let indent = lines[i].len() - lines[i].trim_start().len();
            let (mut gate, mut refusal) = (false, false);
            let mut j = i + 1;
            while j < lines.len() {
                let l = lines[j];
                if l.trim().is_empty() {
                    j += 1;
                    continue;
                }
                if l.len() - l.trim_start().len() <= indent {
                    break;
                }
                let b = l.trim();
                // `when <pred>` alone gates; `when <pred> "<line>"` selects a line
                if b.starts_with("when ") && !b.contains('"') {
                    gate = true;
                }
                if b.starts_with("else") || b.starts_with("otherwise") {
                    refusal = true;
                }
                j += 1;
            }
            if gate {
                match (kind, refusal) {
                    ("verb", true) => gated_verb += 1,
                    ("verb", false) => bare_verb += 1,
                    ("option", false) => bare_option += 1,
                    ("enter", false) => bare_enter += 1,
                    _ => {}
                }
            }
            i = j;
        }
    }
    assert_eq!(
        (bare_option, bare_enter),
        (2, 1),
        "the two sites with no occasion for a refusal"
    );
    assert_eq!(
        bare_verb, 0,
        "and the author never wrote a bare gate where a refusal was possible"
    );
    assert_eq!(gated_verb, 17);
}
