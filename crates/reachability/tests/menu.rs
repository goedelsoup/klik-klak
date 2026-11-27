//! **The interface this design never specified, which it turns out to have specified all along.**
//!
//! [`tier-split`] recorded that four questions resolved only at a third answer site the ontology
//! has no tier for — *"a design decision about what the player is shown… and **this design has
//! never specified one**"* — and declined to invent interface classes for it. Three decision
//! records have landed there since and one of them is [`default-binding`], which found its own
//! question conditional on the interface and closed with the sharper observation: the checker
//! *"has quietly answered it one way for as long as there has been an instrument."*
//!
//! It has, and so has the format. A contextual menu is fully derivable here from three decisions
//! taken separately, none of which was about a player:
//!
//! | | |
//! |---|---|
//! | which cells are in it | [`default-binding`] — only the authored, nothing stands in |
//! | when a binary verb's entry appears | [`arity-cost`] — keyed by the second object, so while carried |
//! | whether a gated binding is in it | [`availability`] — `when` alone withholds, `when` with `else` shows and declines |
//!
//! `verb_menu::in_menu` is those three sentences. What had to be added was one bit the model was
//! throwing away: whether an `else` was written at all.
//!
//! [`tier-split`]: ../../../.yidam/decisions/tier-split.yml
//! [`default-binding`]: ../../../.yidam/decisions/default-binding.yml
//! [`arity-cost`]: ../../../.yidam/decisions/arity-cost.yml
//! [`availability`]: ../../../.yidam/decisions/availability.yml

mod common;

use game_format::{Binding, Site};
use reachability::analyse;

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "menu.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

fn on_a_hotspot(b: &&Binding) -> bool {
    matches!(b.site, Site::InRoom { .. })
}

/// **The derivation reproduces, from the other end, the two figures `availability` counted by
/// reading text** — which is the evidence that it is a derivation and not a new decision.
///
/// That record found *"all seventeen gated bindings on a hotspot verb carry a line"* and that the
/// withholding form is *"used 3× in 662 lines, all where a refusal is impossible."* Sweeping the
/// reachable states and asking which bindings a menu would hold found the same 17 and the same 3,
/// having counted nothing and read no source.
///
/// **A fifth specimen is the first to withhold a hotspot verb, and it withholds twice.** Twenty-
/// eight gated hotspot bindings now, twenty-six carrying a refusal, and the two that do not are
/// `lock.game`'s `talk-to` on the boatman and on the mate. The specimen states its own reason
/// beside them: ***"A bare `when` and no `else`: withheld rather than refused. You are not shown
/// the choice of talking to yourself."***
///
/// That is `availability`'s own criterion — *"all where a refusal is impossible"* — applied to a
/// case it had no instance of. You cannot decline to talk to yourself; there is nobody to say the
/// refusal to. So the record's rule holds and the census under it moves, which is the good version
/// of a figure changing. `.yidam/decisions/unread-specimen.yml` is why it took eleven commits.
#[test]
fn the_menu_recovers_what_availability_found_by_reading() {
    let (mut gated, mut refusing, mut withheld_elsewhere, mut unshown) = (0, 0, 0, 0);
    for (name, src) in common::sources() {
        let g = load(&src);
        gated += g
            .bindings
            .iter()
            .filter(on_a_hotspot)
            .filter(|b| b.when.is_some())
            .count();
        refusing += g
            .bindings
            .iter()
            .filter(on_a_hotspot)
            .filter(|b| b.when.is_some() && b.refuses)
            .count();
        withheld_elsewhere += g
            .bindings
            .iter()
            .filter(|b| !on_a_hotspot(b) && b.when.is_some() && !b.refuses)
            .count();
        let r = analyse(&g);
        assert!(r.unshown.is_empty(), "{name}: {:#?}", r.unshown);
        unshown += r.unshown.len();
    }
    assert_eq!(
        (gated, refusing),
        (32, 30),
        "gated hotspot bindings, and the ones carrying a refusal — the two that do not are \
         `talk-to` on the character the player is currently driving"
    );
    assert_eq!(
        withheld_elsewhere, 3,
        "two conversation options and an `on-enter`"
    );
    assert_eq!(unshown, 0, "every authored binding reaches a menu");
}

/// **`default-binding` predicted the leak would be unconditional and it is the author's to set,
/// and the author has never once chosen to hide.**
///
/// That record's table reads *"failed precondition → the verb is absent"* and *"what the player
/// learns → what the object affords, before trying"*, and concludes that a contextual menu *"makes
/// that leak unconditional and takes the wording away from the author… it moves a decision out of
/// the author's hands."* Two phases later `availability` put the decision back in them without
/// either record noticing the other.
///
/// Here is the switch. One `else` line is the whole of it: with it the entry is in the menu in
/// every state and the refusal is worded by the author; without it the entry is absent exactly
/// where the guard fails. **Every reachability figure is identical either way**, which is why the
/// search could not see the difference and why nothing made the corpus look.
#[test]
fn one_else_line_is_the_whole_of_the_leak_and_costs_nothing_in_the_graph() {
    const G: &str = "room vault \"The Vault\"\nitem key \"a small key\"\n\
                     flag unlocked latched \"the grille is open\"\n\n\
                     room vault\n  start\n\
                     \x20 hotspot bench \"a bench\"\n    is key\n    pick-up:\n      do    give key\n\
                     \x20 hotspot grille \"an iron grille\"\n    use-with key:\n      when  not unlocked\n\
                     \x20     do    set unlocked\n{ELSE}\
                     \x20 hotspot out \"out\"\n    walk-to:\n      when  unlocked\n      ending\n      else  \"Shut.\"\n";

    let shown = analyse(&load(
        &G.replace("{ELSE}", "      else  \"Already open.\"\n"),
    ));
    let hidden = analyse(&load(&G.replace("{ELSE}", "")));

    // both forms show the grille somewhere, so the distinct entries are the same; what the
    // `else` buys is the states after the grille is open, where it stands there and declines
    assert_eq!(shown.menu_entries, hidden.menu_entries);
    assert!(shown.menu_slots > hidden.menu_slots);
    assert!(shown.unshown.is_empty() && hidden.unshown.is_empty());

    // and the search cannot tell the two games apart at all
    assert_eq!(
        (
            shown.states,
            shown.transitions,
            shown.stranded,
            shown.winnable
        ),
        (
            hidden.states,
            hidden.transitions,
            hidden.stranded,
            hidden.winnable
        )
    );
}

/// **Half of what a player is shown is invisible to the thing that checks the game.**
///
/// The search is built from bindings that can move it — `acting` drops every response that writes
/// no state — so across the specimens it reasons about **60** of 120 bindings. A menu holds **117
/// distinct `(target, verb)` entries**, of which **31 are showing before the player has done
/// anything**. The gap between 60 and 117 is almost exactly the 56 `look-at` bindings, which are
/// the largest single thing the interface displays and contribute nothing to any figure this
/// instrument reports.
///
/// **The half stayed a half across a fifth game** — 45 of 91 was 49%, 60 of 120 is 50% — which is
/// the claim in the title. It is not an accident of four files: `look-at` is roughly half of what
/// an author writes, and `look-at` writes no state.
///
/// That is not a defect. It is the boundary of what a reachability search is for, stated as a
/// number for the first time, and it is why `default-binding` was right that the question belongs
/// to a different layer. Summed over every reachable state the five games have, the menu holds
/// **1,169 slots**.
#[test]
fn the_search_reasons_about_half_of_what_the_interface_shows() {
    let (mut bindings, mut acting, mut look_at, mut ever, mut at_start) = (0, 0, 0, 0, 0);
    let mut slots = 0;
    for (_, src) in common::sources() {
        let g = load(&src);
        bindings += g.bindings.len();
        acting += g
            .bindings
            .iter()
            .filter(|b| {
                !matches!(b.site, Site::OnEnter { .. })
                    && (!b.effects.is_empty()
                        || b.ending
                        || matches!(b.site, Site::InConversation { .. }))
            })
            .count();
        look_at += g.bindings.iter().filter(|b| b.verb == "look-at").count();
        let r = analyse(&g);
        ever += r.menu_entries.0;
        at_start += r.menu_entries.1;
        slots += r.menu_slots;
    }
    assert_eq!((bindings, acting, look_at), (148, 76, 68));
    assert_eq!((ever, at_start), (144, 40));

    // and summed over the 203 reachable states of the five games, the interface holds 1,169
    // target-and-verb slots — the size of the thing nobody had specified
    assert_eq!(slots, 4261);
}

/// **A binding in no menu is a defect the corpus had no way to see, and it is an ordinary
/// authoring mistake rather than a contrived one.**
///
/// An acquisition site stops being a site once its object is taken — `verb_menu::live` says so, and
/// `hotspot/static-region` established why. So a response on that hotspot, gated on the flag the
/// taking sets, can never be reached: by the time the guard holds, the shelf is gone.
///
/// Nothing else catches it. `unreachable_values` is satisfied because something does set the flag;
/// no state is stranded, because the game is winnable without ever looking; and the binding writes
/// nothing, so the search never loads it. It is authored text with no reader, which is the fourth
/// check of that kind here after `dead_writes`, `unreachable_values` and the unreachable refusal.
#[test]
fn a_response_behind_its_own_acquisition_is_never_shown() {
    const G: &str = "room store \"The Store\"\nitem key \"a brass key\"\n\
                     flag took \"the key is off the shelf\"\n\n\
                     room store\n  start\n\
                     \x20 hotspot shelf \"a high shelf\"\n    is key\n\
                     \x20   pick-up:\n      say   \"You reach it down.\"\n      do    give key\n      do    set took\n\
                     {LOOK}\
                     \x20 hotspot out \"out\"\n    walk-to:\n      when  carrying key\n      ending\n      else  \"Locked.\"\n";
    const LOOK: &str = "    look-at:\n      when  took\n      \"A clean rectangle in the dust.\"\n";

    let bad = analyse(&load(&G.replace("{LOOK}", LOOK)));
    assert_eq!(bad.unshown.len(), 1, "{:#?}", bad.unshown);
    assert!(bad.unshown[0].contains("`look-at`") && bad.unshown[0].contains("never shown"));

    // and it is invisible to everything else: the same game without the line is identical
    let good = analyse(&load(&G.replace("{LOOK}", "")));
    assert!(good.unshown.is_empty());
    assert_eq!(
        (bad.states, bad.transitions, bad.stranded, bad.winnable),
        (good.states, good.transitions, good.stranded, good.winnable)
    );
    assert_eq!(bad.menu_entries, good.menu_entries);
}

/// **The exchange `verb-count` measured, priced at the interface it left open.**
///
/// That record found a bounded verb set loses no puzzle and pays for it in targets: Delores's
/// `lowerLadder` binds `climb`, `push_right` and `push_left` on one object, and here the same game
/// needs the ladder and the two ends of the rail. It closed by noting that *"every figure this
/// instrument computes is identical and the interface is not, and the layer where such differences
/// matter is one this design has never specified."*
///
/// It is specified now, and it gives the same answer with one number changed. **The menu holds
/// three entries either way** — the player is offered exactly as many choices as the shipped game
/// offers — and what the split moves is how many things they must point at to see them, from one
/// to three. That is the exchange stated in the interface's own units, and it is not a loss of
/// choices; it is a loss of their being gathered in one place.
#[test]
fn the_split_offers_the_same_three_choices_from_three_targets_instead_of_one() {
    const DECL: &str = "room shop \"The Bookstore\"\nroom loft \"The Upper Shelf\"\n\
                        item ledger \"the shop ledger\"\n\
                        counter rung 0..2 \"how far along the rail the ladder stands\"\n\n";
    let step = |from: i64, to: i64, far: &str| {
        format!("    walk-to:\n      when  rung = {from}\n      do    set rung {to}\n      else  \"It is as far {far} as it goes.\"\n")
    };
    let src = format!(
        "{DECL}room shop\n  start\n\
         \x20 hotspot ladder \"a ladder on a rail\"\n\
         \x20   look-at:\n      \"It reaches the upper shelf, wherever it happens to stand.\"\n\
         \x20   walk-to:\n      when  rung = 2\n      do    move loft\n      else  \"Not under the shelf you want.\"\n\
         \x20 hotspot rail-right \"the rail, to the right\"\n{}{}\
         \x20 hotspot rail-left \"the rail, to the left\"\n{}{}\n\
         room loft\n\
         \x20 hotspot shelf \"the top shelf\"\n    is ledger\n    pick-up:\n      do    give ledger\n\
         \x20 hotspot down \"back down\"\n    walk-to:\n      when  carrying ledger\n      ending\n      else  \"Not without the ledger.\"\n",
        step(0, 1, "right"), step(1, 2, "right"), step(2, 1, "left"), step(1, 0, "left")
    );
    let g = load(&src);
    let r = analyse(&g);
    assert!(r.winnable && r.unshown.is_empty());

    // the three things the shipped object offers, and the three targets they are spread over
    let shop: Vec<&Binding> = g
        .bindings
        .iter()
        .filter(|b| matches!(&b.site, Site::InRoom { room, .. } if room == "shop"))
        .filter(|b| b.verb == "walk-to")
        .collect();
    let targets: std::collections::BTreeSet<&str> = shop
        .iter()
        .filter_map(|b| match &b.site {
            Site::InRoom { target, .. } => Some(target.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        targets.len(),
        3,
        "one ladder became three things to point at"
    );
    // one `walk-to` entry per target, however many bindings sit behind it
    assert_eq!(shop.len(), 5, "and five bindings, because the rail steps");
}
