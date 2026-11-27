//! Where an object starts, and what is allowed to say so.
//!
//! `inventory-item` declares three ways in: `found-in -> room`, `carried-form-of -> hotspot`,
//! `acquired-by -> interaction`. The format realised all three — `holds <item>`, `is <item>`,
//! `give <item>` — and one of them was never information.
//!
//! `.yidam/decisions/found-in.yml` has the measurement. The short version is that `holds
//! <item>` could be omitted, moved to another room, aimed at an item nothing stood for, or
//! made to contradict itself across two rooms, and the analysis of `lighthouse.game` came back
//! 91 states and one stranding every single time. The other two carry the fact between them,
//! and this file is where they are held to it.

use game_format::{parse, resolve};

const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");

/// One room, one actor, one item, one hotspot that stands for the item, one ending.
fn game(room_body: &str) -> Vec<String> {
    let source = format!(
        "room  hall \"The Hall\"\n\
         actor porter \"the porter\"\n\
         item  coin \"a coin\"\n\
         item  lantern \"a lantern\"\n\
         flag  done \"done\"\n\
         \n\
         room hall\n\
         \x20 start\n{room_body}\n\
         \x20 hotspot door \"the door\"\n\
         \x20   walk-to:\n\
         \x20     ending\n"
    );
    let (mut g, mut problems) = parse(&source, "fixture.game");
    problems.extend(resolve(&mut g));
    problems.into_iter().map(|p| p.to_string()).collect()
}

/// The ordinary case, so that every failure below is about the thing it names.
const GOOD: &str = "  hotspot loose-coin \"a coin under a stone\"\n\
                    \x20   is coin\n\
                    \x20   pick-up:\n\
                    \x20     do    give coin\n";

#[test]
fn the_ordinary_case_resolves() {
    assert!(game(GOOD).is_empty(), "{:?}", game(GOOD));
}

#[test]
fn a_room_holds_actors_and_an_item_named_there_is_an_error() {
    let p = game(&format!("  holds coin\n{GOOD}"));
    assert!(
        p.iter().any(|m| m.contains("a room holds actors")),
        "`holds <item>` must be refused; got {p:?}"
    );
}

/// A refusal that does not say what to write instead is a worse refusal than none: the author
/// knows the fact is true and has been told only that this is the wrong place for it.
#[test]
fn the_refusal_names_the_thing_that_does_say_it() {
    let p = game(&format!("  holds coin\n{GOOD}"));
    assert!(
        p.iter().any(|m| m.contains("`is coin` on the hotspot")),
        "the refusal must point at `is`; got {p:?}"
    );
}

/// `holds` never took anything but actors and items, and the third arm should now name only
/// the one that is left.
#[test]
fn holds_pointed_at_something_else_entirely_says_actors_and_stops() {
    let p = game(&format!("  holds done\n{GOOD}"));
    assert!(
        p.iter()
            .any(|m| m.contains("is a flag, and a room holds actors")),
        "{p:?}"
    );
    assert!(
        !p.iter().any(|m| m.contains("holds actors and items")),
        "the old wording promised items; {p:?}"
    );
}

#[test]
fn holds_still_places_an_actor() {
    let mut source = format!("  holds porter\n{GOOD}");
    source.push_str(
        "  actor porter\n\
         \x20   look-at:\n\
         \x20     \"He is reading.\"\n",
    );
    assert!(game(&source).is_empty(), "{:?}", game(&source));
}

/// `carried-form-of` with no `acquired-by` behind it: the hotspot claims to be the uncarried
/// form of something, and nothing on it ever hands that something over. The claim is then a
/// comment — and worse than a comment, because the checker reads it and takes the hotspot out
/// of the room the moment anything else supplies the item.
#[test]
fn a_hotspot_standing_for_an_item_nothing_on_it_gives_is_reported() {
    let p = game(
        "  hotspot loose-coin \"a coin under a stone\"\n\
         \x20   is coin\n\
         \x20   look-at:\n\
         \x20     \"It is there.\"\n",
    );
    assert!(
        p.iter()
            .any(|m| m.contains("stands for `coin`") && m.contains("no binding on it gives")),
        "{p:?}"
    );
}

#[test]
fn the_give_must_name_the_item_the_hotspot_stands_for() {
    let p = game(
        "  hotspot loose-coin \"a coin under a stone\"\n\
         \x20   is coin\n\
         \x20   pick-up:\n\
         \x20     do    give lantern\n",
    );
    assert!(
        p.iter().any(|m| m.contains("stands for `coin`")),
        "picking up a coin and receiving a lantern must be reported; got {p:?}"
    );
}

/// The derivation is *sited*: `reachability` removes the hotspot when a binding **on that
/// hotspot** gives the item, and nowhere else. So the check has to be sited too, or it would
/// pass games the checker still gets wrong.
#[test]
fn a_give_from_somewhere_else_does_not_satisfy_it() {
    let p = game(
        "  hotspot loose-coin \"a coin under a stone\"\n\
         \x20   is coin\n\
         \x20   look-at:\n\
         \x20     \"It is there.\"\n\
         \x20 hotspot till \"a till\"\n\
         \x20   pick-up:\n\
         \x20     do    give coin\n",
    );
    assert!(
        p.iter().any(|m| m.contains("stands for `coin`")),
        "the give is on `till`, not on `loose-coin`; got {p:?}"
    );
}

/// The measurement this check was written from, run against a real game rather than a fixture.
///
/// `loose-wire` stands for `wire`; make its own `pick-up` hand over the ledger instead. Before
/// this check the mutation resolved without a word, and the analysis went from **14 states and
/// two strandings to 6 and none** — still reporting the game winnable, because handing the
/// player the ledger at the front gate wins it. A defect that makes the report *shorter* is
/// the one an author has no way to notice.
#[test]
fn the_mutation_that_made_the_checker_quieter_is_now_refused() {
    let mutated = NIGHTWATCH.replace("      do    give wire\n", "      do    give ledger\n");
    assert_ne!(mutated, NIGHTWATCH, "the mutation must actually apply");

    let (mut clean, mut p) = parse(NIGHTWATCH, "nightwatch.game");
    p.extend(resolve(&mut clean));
    assert!(p.is_empty(), "the specimen itself must resolve: {p:?}");

    let (mut g, mut p) = parse(&mutated, "nightwatch.game");
    p.extend(resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(
        msgs.iter()
            .any(|m| m.contains("stands for `wire`") && m.contains("no binding on it gives")),
        "{msgs:?}"
    );
}

/// **The other direction: what else may hand the item over.** `.yidam/decisions/one-place.yml`.
///
/// The checks above hold that a hotspot standing for an item gives it. Nothing asked whether
/// anything *else* did, and when something does, the item is in two places at once — carried,
/// and still lying in the room offering itself. The rule is the item's: a single item that has a
/// carried form is given by that form and nowhere else.
#[test]
fn a_single_item_given_anywhere_but_where_it_lies_is_reported() {
    let p = game(&format!(
        "{GOOD}\
         \x20 hotspot till \"a till\"\n\
         \x20   pick-up:\n\
         \x20     do    give coin\n"
    ));
    assert!(
        p.iter().any(
            |m| m.contains("gives `coin`, which is one thing") && m.contains("hall.loose-coin")
        ),
        "the coin lies under the stone and comes out of the till as well; got {p:?}"
    );
}

/// The shape the question was asked about, and the one it would have caught.
#[test]
fn a_hotspot_handing_over_what_another_hotspot_stands_for_is_reported() {
    let p = game(&format!(
        "{GOOD}\
         \x20     do    give lantern\n\
         \x20 hotspot hook \"a lantern on a hook\"\n\
         \x20   is lantern\n\
         \x20   pick-up:\n\
         \x20     do    give lantern\n"
    ));
    assert!(
        p.iter()
            .any(|m| m.contains("gives `lantern`, which is one thing")),
        "{p:?}"
    );
}

/// **And the broad rule, refused.** A hotspot that stands for one thing may hand over another
/// that stands nowhere: picking up a coat and finding a key in its pocket is one place for each.
#[test]
fn a_hotspot_may_hand_over_something_that_lies_nowhere_else() {
    let p = game(&format!("{GOOD}\x20     do    give lantern\n"));
    assert!(p.is_empty(), "{p:?}");
}

/// A counted item is many things by declaration. `ferry.game`'s jar gives three coins beside a
/// loose one, and that is the specimen this exemption keeps.
#[test]
fn a_counted_item_may_come_from_more_than_one_place() {
    let source = "room  hall \"The Hall\"\n\
                  item  coin 0..4 \"a coin\"\n\
                  \n\
                  room hall\n\
                  \x20 start\n\
                  \x20 hotspot loose-coin \"a coin under a stone\"\n\
                  \x20   is coin\n\
                  \x20   pick-up:\n\
                  \x20     do    give coin\n\
                  \x20 hotspot jar \"a jar\"\n\
                  \x20   pick-up:\n\
                  \x20     do    give coin 3\n\
                  \x20 hotspot door \"the door\"\n\
                  \x20   walk-to:\n\
                  \x20     ending\n";
    let (mut g, mut p) = parse(source, "fixture.game");
    p.extend(resolve(&mut g));
    assert!(p.is_empty(), "{p:?}");
}
