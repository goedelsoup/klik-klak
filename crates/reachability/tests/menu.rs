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
//! `Ix::in_menu` is those three sentences. What had to be added was one bit the model was
//! throwing away: whether an `else` was written at all.
//!
//! [`tier-split`]: ../../../.yidam/decisions/tier-split.yml
//! [`default-binding`]: ../../../.yidam/decisions/default-binding.yml
//! [`arity-cost`]: ../../../.yidam/decisions/arity-cost.yml
//! [`availability`]: ../../../.yidam/decisions/availability.yml

use game_format::{Binding, Site};
use reachability::analyse;

const S: [(&str, &str); 4] = [
    ("ferry", include_str!("../../../design/specimen/ferry.game")),
    (
        "lighthouse",
        include_str!("../../../design/specimen/lighthouse.game"),
    ),
    (
        "nightwatch",
        include_str!("../../../design/specimen/nightwatch.game"),
    ),
    (
        "patrol",
        include_str!("../../../design/specimen/patrol.game"),
    ),
];

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
/// reachable states and asking which bindings a menu would hold finds the same 17 and the same 3,
/// having counted nothing and read no source.
#[test]
fn the_menu_recovers_what_availability_found_by_reading() {
    let (mut gated, mut refusing, mut withheld_elsewhere, mut unshown) = (0, 0, 0, 0);
    for (name, src) in S {
        let g = load(src);
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
        (17, 17),
        "every gated hotspot verb is shown"
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
/// no state — so across the four specimens it reasons about **45** of 91 bindings. A menu holds
/// **88 distinct `(target, verb)` entries**, of which **24 are showing before the player has done
/// anything**. The gap between 45 and 88 is almost exactly the 42 `look-at` bindings, which are
/// the largest single thing the interface displays and contribute nothing to any figure this
/// instrument reports.
///
/// That is not a defect. It is the boundary of what a reachability search is for, stated as a
/// number for the first time, and it is why `default-binding` was right that the question belongs
/// to a different layer. Summed over every reachable state the four games have, the menu holds
/// **698 slots**.
#[test]
fn the_search_reasons_about_half_of_what_the_interface_shows() {
    let (mut bindings, mut acting, mut look_at, mut ever, mut at_start) = (0, 0, 0, 0, 0);
    let mut slots = 0;
    for (_, src) in S {
        let g = load(src);
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
    assert_eq!((bindings, acting, look_at), (91, 45, 42));
    assert_eq!((ever, at_start), (88, 24));

    // and summed over the 145 reachable states of the four games, the interface holds 698
    // target-and-verb slots — the size of the thing nobody had specified
    assert_eq!(slots, 698);
}

/// **A binding in no menu is a defect the corpus had no way to see, and it is an ordinary
/// authoring mistake rather than a contrived one.**
///
/// An acquisition site stops being a site once its object is taken — `Ix::live` says so, and
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
