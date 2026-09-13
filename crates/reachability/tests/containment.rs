//! A thing inside a thing, and whether this vocabulary is missing a way to say so.
//!
//! The previous phase's predicate census turned up `case-open` and `cabinet-open` each read at
//! four sites, two of each four on the hotspot *inside* the container — hotspots named
//! `ledger-in-case` and `bond-in-cabinet`, the relation spelled in the identifier because nothing
//! declares it. It was recorded and not decided.
//!
//! Deciding it needed three measurements, and they are here: what dropping each derived guard
//! costs, what the one-hotspot alternative costs, and how much of the corpus has the shape at all.
//!
//! `.yidam/decisions/containment.yml` is the record. The short version is that the two-hotspot
//! form is not an idiom — it is what `carried-form-of` makes of a container, and it is what makes
//! taking the contents self-limiting.

mod common;

use game_format::Game;
use reachability::analyse;

const PATROL: &str = include_str!("../../../design/specimen/patrol.game");
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

fn rewrite(src: &str, old: &str, new: &str) -> String {
    assert_eq!(src.matches(old).count(), 1, "anchor is not unique");
    src.replace(old, new)
}

/// **Both derived guards are undetectable if dropped, in opposite ways.**
#[test]
fn neither_derived_guard_is_checked() {
    let base = analyse(&load("patrol", PATROL));
    assert_eq!((base.states, base.transitions, base.stranded), (20, 33, 3));

    // The `look-at` guard is text on both branches, so removing it changes nothing at all —
    // a response that writes no state is not a transition (`defaults.rs`).
    let text_only = rewrite(
        PATROL,
        "      when cabinet-open  \"Made out to nobody at all, which is what makes it a bearer bond.\"\n      otherwise          \"Behind the cabinet door.\"\n",
        "      \"Made out to nobody at all, which is what makes it a bearer bond.\"\n",
    );
    let r = analyse(&load("text-only", &text_only));
    assert_eq!(
        (r.states, r.transitions, r.stranded),
        (base.states, base.transitions, base.stranded),
        "a measured footprint of exactly zero"
    );

    // The `pick-up` guard is the one that matters, and dropping it lets the player take the bond
    // through a locked cabinet — which the report describes as a healthier game.
    let ungated = rewrite(
        PATROL,
        "    pick-up:\n      when  cabinet-open\n      say   \"You fold it once and put it away.\"\n      do    give bond\n      else  \"The drawer is shut.\"\n",
        "    pick-up:\n      say   \"You fold it once and put it away.\"\n      do    give bond\n",
    );
    let r = analyse(&load("ungated", &ungated));
    assert!(r.winnable);
    assert_eq!(r.states, 26, "six states the cabinet was holding shut");
    assert_eq!(
        r.stranded, 0,
        "and every stranding gone — the third defect here that presents as good news"
    );
}

/// **The one-hotspot form is expressible, is what the shipped engine does, and is refused.**
///
/// *Delores* has no hotspot for a thing inside a container: `fileDrawer` is one object whose
/// `VERB_OK(lookin)` returns `state == "open"` and whose `VERB(lookin)` calls `pickupObject`
/// directly. Written that way here, the checker names the defect immediately.
#[test]
fn folding_the_container_and_its_contents_into_one_hotspot_is_caught() {
    let folded = rewrite(
        PATROL,
        "  hotspot bond-in-cabinet \"a bearer bond\"\n    is bond\n    look-at:\n      when cabinet-open  \"Made out to nobody at all, which is what makes it a bearer bond.\"\n      otherwise          \"Behind the cabinet door.\"\n\n    pick-up:\n      when  cabinet-open\n      say   \"You fold it once and put it away.\"\n      do    give bond\n      else  \"The drawer is shut.\"\n",
        "",
    );
    let folded = rewrite(
        &folded,
        "      do    set cabinet-open\n      else  \"It is open already.\"\n",
        "      do    set cabinet-open\n      else  \"It is open already.\"\n\n    pick-up:\n      when  cabinet-open\n      say   \"You fold the bond once and put it away.\"\n      do    give bond\n      else  \"The drawer is shut.\"\n",
    );

    let r = analyse(&load("folded", &folded));
    assert_eq!(
        (r.states, r.stranded),
        (20, 3),
        "the same game, by every measure the search reports"
    );
    assert_eq!(
        r.transitions, 34,
        "one transition more: the take is no longer self-limiting"
    );
    assert_eq!(
        r.overflows.len(),
        1,
        "and the checker says so: {:?}",
        r.overflows
    );
    assert!(
        r.overflows[0].contains("bond") && r.overflows[0].contains("declared ceiling"),
        "{}",
        r.overflows[0]
    );
}

/// **How much of the corpus has the shape: two of nine, and both are named for it.**
///
/// A hotspot carrying `is <item>` is the uncarried form of that item. Seven of the nine in these
/// specimens are simply lying about and their `pick-up` is unguarded; two are inside something,
/// and those two are exactly the two whose `pick-up` carries a guard.
///
/// **A fifth game added one loose item and no contained one**, and the naming convention held
/// without anybody being told about it: `lockside.loose-windlass` is lying at the gear, and its
/// `pick-up` is unguarded like the other six.
#[test]
fn seven_are_loose_and_two_are_contained_and_the_names_say_which() {
    let mut loose = Vec::new();
    let mut contained = Vec::new();
    for (name, src) in common::sources() {
        let g = load(&name, &src);
        for hotspot in g.carried_form.keys() {
            let guarded = g.bindings.iter().any(|b| {
                matches!(&b.site, game_format::Site::InRoom { target, .. } if target == hotspot)
                    && b.verb == "pick-up"
                    && b.when.is_some()
            });
            if guarded {
                contained.push(hotspot.clone());
            } else {
                loose.push(hotspot.clone());
            }
        }
    }
    assert_eq!(loose.len(), 11, "{loose:?}");
    assert_eq!(contained.len(), 2, "{contained:?}");
    for c in &contained {
        assert!(
            c.contains("-in-"),
            "the relation is written into the identifier: {c}"
        );
    }
    for l in &loose {
        assert!(
            !l.contains("-in-"),
            "and the loose ones are not named that way: {l}"
        );
    }
}
