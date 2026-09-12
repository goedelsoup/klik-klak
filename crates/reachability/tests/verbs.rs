//! **Six verbs against forty-nine**, and what a bounded verb vocabulary actually costs.
//!
//! [`prior-art/sierra-sci`] has carried the same `[open]` since genesis, and calls the evidence
//! behind it *"the single most useful piece of evidence available to this corpus"*: Sierra moved
//! from a typed parser to an icon bar within one product line, and *"what was lost is `[open]` and
//! is exactly what the central question needs to know — whether any puzzle expressible to a parser
//! became inexpressible under an icon bar."*
//!
//! It cannot be asked of Sierra here; no Sierra source is in reach. It can be asked of the game
//! whose source is, and the question survives the move, because that game is *itself* the large
//! vocabulary: **57 declared verbs, 49 with a binding, 23 used exactly once**
//! ([delores-source]). Collapsing all 49 onto these six leaves **18 cells holding more than one
//! binding**, and eleven of those are unary, where a cell has nothing but a guard to tell its
//! occupants apart.
//!
//! The bookstore ladders are the case that does not survive it, and this is that case in this
//! language. `.yidam/decisions/verb-count.yml`.
//!
//! [`prior-art/sierra-sci`]: ../../../.yidam/corpus/prior-art/sierra-sci.yml
//! [delores-source]: ../../../.yidam/catalog/delores-source.md

mod common;

use reachability::analyse;

fn parse(src: &str) -> (game_format::Game, Vec<String>) {
    let (mut g, mut p) = game_format::parse(src, "verbs.game");
    p.extend(game_format::resolve(&mut g));
    let m = p.into_iter().map(|x| x.to_string()).collect();
    (g, m)
}

fn load(src: &str) -> game_format::Game {
    let (g, m) = parse(src);
    assert!(m.is_empty(), "{m:#?}");
    g
}

const DECL: &str = "room shop \"The Bookstore\"\nroom loft \"The Upper Shelf\"\n\
                    item ledger \"the shop ledger\"\n\
                    counter rung 0..2 \"how far along the rail the ladder stands\"\n\n";

/// **One object, three actions, one world state — which is the thing the six cannot say.**
///
/// `BookStore.lowerLadder` binds `climb` with no guard, `push_right` under `position != 2` and
/// `push_left` under `position != 0`. At position 1 all three are offered at once, and what picks
/// between them is the verb the player chose and nothing else: the world is identical under all
/// three. Collapse them onto one verb and the cell holds three bindings no predicate separates,
/// which is what `ambiguous_cells` was built to refuse (`.yidam/decisions/arity-cost.yml`).
///
/// Both of the check's reasons are present and it only ever reports the first, because each
/// binding is held against the earliest one in the cell and `climb` is declared first — it is the
/// object's `default_verb`. The second half shows the other reason on its own: the two guards the
/// shipped game does have **both hold at position 1**, so even without `climb` the cell is
/// unreadable.
#[test]
fn three_verbs_on_one_object_become_one_cell_that_cannot_be_read() {
    let rails = "      when  rung != 2\n      do    set rung 2\n      else  \"It is as far right as it goes.\"\n\
                 \x20   walk-to:\n      when  rung != 0\n      do    set rung 0\n      else  \"It is as far left as it goes.\"\n";
    let out = "  hotspot out \"out\"\n    walk-to:\n      ending\n";

    // as the shipped object declares them: the unguarded default verb first
    let (_, all) = parse(&format!(
        "{DECL}room shop\n  start\n  hotspot ladder \"a ladder on a rail\"\n\
         \x20   walk-to:\n      do    move loft\n    walk-to:\n{rails}{out}\nroom loft\n{out}"
    ));
    assert_eq!(all.len(), 2, "{all:#?}");
    assert!(
        all.iter()
            .all(|p| p.contains("already written on `shop/shop.ladder`")
                && p.contains("one of them is unguarded")),
        "{all:#?}"
    );

    // and the two guards alone, which is the reason that survives deleting the default verb
    let (_, pair) = parse(&format!(
        "{DECL}room shop\n  start\n  hotspot ladder \"a ladder on a rail\"\n    walk-to:\n{rails}{out}\nroom loft\n{out}"
    ));
    assert_eq!(pair.len(), 1, "{pair:#?}");
    assert!(
        pair[0].contains("both guards can hold in the same state"),
        "{pair:#?}"
    );
}

/// **The reduction works, and what it converts is verbs into hotspots and steps into bindings.**
///
/// Split the one object into the thing and the two ways of moving it, and every cell is singly
/// occupied, the checker is quiet and the game plays. Nothing became inexpressible.
///
/// Two costs, and the second was not the one this test was written to find. Three verb bodies
/// become **three hotspots**, which is the exchange the icon bar makes everywhere: a distinction
/// that was carried by the verb is carried by the thing pointed at, and there is no bound on how
/// many things a room may hold. But `SetCounter` is absolute and this language has no increment,
/// so a rail the ladder *steps* along costs **one binding per step in each direction** — four
/// where the shipped game wrote two function bodies. The verb vocabulary was never the binding
/// constraint; the effect vocabulary is.
///
/// It is the same exchange [`use-with`] made in the other direction — `binary-dimension` reduced a
/// binary verb to unary verbs over richer state and paid 15% more frame — and it is the reason the
/// icon bar was affordable at all.
///
/// [`use-with`]: ../../../.yidam/corpus/verb/use-with.yml
#[test]
fn splitting_the_object_buys_the_verbs_back() {
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
    assert!(r.winnable, "{}", r.render());

    let hotspots = g
        .declared
        .values()
        .filter(|(k, _)| matches!(k, game_format::Kind::Hotspot))
        .count();
    // three hotspots in the shop where the shipped game had one object, and four rail bindings
    // where it had two function bodies
    assert_eq!(hotspots, 5);
    assert_eq!(g.bindings.iter().filter(|b| b.verb == "walk-to").count(), 6);
    assert_eq!(
        r.stranded, 0,
        "the rail runs both ways, so nothing is stranded on it"
    );
}

/// **The bound was not binding from the other side either: one of the six had never been used.**
///
/// `give-to` carried no binding in 904 authored lines. [`verb/give-to`] assessed it in the corpus
/// two phases after it was seeded and found it *"subsumed at the engine tier"* — AGS routes it
/// through `Use inventory on character` — and kept the node as a record of what was declined. The
/// parser was never told, so `surface.rs` counted six verbs where the corpus had decided on five,
/// and the specimens had been voting with their silence the whole time.
///
/// That is what withdrew it; the surface is five verbs and fifty constructs now, and this test is
/// the census that keeps the other five honest.
///
/// [`verb/give-to`]: ../../../.yidam/corpus/verb/give-to.yml
#[test]
fn the_five_that_remain_are_all_written_and_the_sixth_never_was() {
    let mut used = std::collections::BTreeSet::new();
    let mut total = 0;
    for (_, src) in common::sources() {
        for b in &load(&src).bindings {
            used.insert(b.verb.clone());
            total += 1;
        }
    }
    assert_eq!(total, 120);
    for v in ["look-at", "use-with", "talk-to", "pick-up", "walk-to"] {
        assert!(used.contains(v), "{v}");
    }
    // `on-enter` and `option` are binding forms rather than verbs; they are how the parser
    // records a room entry and a conversation choice, and neither is on the verb bar
    assert_eq!(used.len(), 7);
    assert!(used.contains("on-enter") && used.contains("option"));
    assert!(
        !used.contains("give-to"),
        "withdrawn, and nothing was written in it"
    );
}

/// **`give-to` is withdrawn, and this is the demonstration that withdrew it.**
///
/// [`verb/give-to`] concluded two phases after the verb was seeded that it is `use-with` applied to
/// an actor — AGS's manual says so outright: *"Use inventory on character… could be used to allow
/// the player to give items to characters."* The node was kept as a record of what was declined and
/// the parser was never told, so for six phases the surface counted a verb the corpus had decided
/// against.
///
/// Three things closed it. It carries no binding in 662 authored lines. Folding Delores's `giveto`
/// into `use-with` creates **no new collision** — 198 cells and 18 of them shared, identical either
/// way — so it earns nothing even in the game that has it, where both its bindings sit on inventory
/// items whose only other verb is `lookat` ([delores-source]). And while both existed they were two
/// cells for one intent with nothing to tell them apart: the same game written either way gave the
/// same states, the same transitions and the same report, and no check had anything to say.
///
/// That last is what this test now holds, from the far side. The verb is gone; the game it
/// expressed is unchanged.
///
/// [`verb/give-to`]: ../../../.yidam/corpus/verb/give-to.yml
#[test]
fn the_game_give_to_expressed_is_unchanged_without_it() {
    let src = "room hall \"The Hall\"\nitem coin \"a bent coin\"\nactor beggar \"a beggar\"\n\n\
               room hall\n  start\n  actor beggar\n\
               \x20 hotspot purse \"a dropped purse\"\n    is coin\n    pick-up:\n      do    give coin\n\
               \x20 hotspot out \"out\"\n    walk-to:\n      when  paid\n      ending\n      else  \"Not yet.\"\n";
    let decl = "flag    paid latched \"the beggar has been paid\"\n";
    let give = "  actor beggar\n    give-to coin:\n      when  carrying coin\n      say   \"He takes it without looking up.\"\n      do    spend coin\n      do    set paid\n      else  \"You have nothing for him.\"\n";
    let with = give.replace("give-to coin:", "use-with coin:");

    // the verb is no longer a construct, and the parser says which line stopped being one
    let (_, gone) = parse(&format!("{decl}{}", src.replace("  actor beggar\n", give)));
    assert!(
        gone[0].contains("`give-to` is not a construct this format has"),
        "{gone:#?}"
    );

    // and the exchange it was written for still plays, on the verb that subsumed it
    let g = load(&format!("{decl}{}", src.replace("  actor beggar\n", &with)));
    let r = analyse(&g);
    assert!(r.winnable, "{}", r.render());
    assert_eq!(
        g.bindings.iter().filter(|x| x.verb == "use-with").count(),
        1
    );
    assert_eq!(r.stranded, 0);
}
