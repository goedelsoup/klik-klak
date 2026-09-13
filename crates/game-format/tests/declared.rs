//! **A shipped game reaches two thirds of its inventory by a declaration no binding touches.**
//!
//! [`portable-object`] has asked since it was seeded whether *an item nothing reaches for is an
//! item at all* — an item never required, consumed, or offered as the second object of a binding
//! being *"scenery with an icon"* — and recorded that this corpus has no such instance, so the
//! question had nothing to be asked of. It names a fifth specimen as what would answer it.
//!
//! A shipped game answers it, and the answer is not the one either branch expected. Delores
//! declares **fifteen inventory items. Twelve are named by a binding** — as a second object, a
//! carried requirement, or a consumption. **Three are named by none**, and all three carry
//! photojournalism hooks — `for_story`, `photo_name`, `is_photo` — which a *different subsystem*
//! reads. Nothing in that game is unreached; three things in it are unreached **by any binding**.
//!
//! **The mechanism is not a corner of the game.** `for_story`, `bad_story` and `photo_name` are
//! declared at **165 sites across fourteen of twenty-one room files**, against 222 `VERB(`
//! bindings — comparable in size to the entire binding surface, and none of it is a verb, a
//! binding, or a cell of any interaction table. An object participates by declaring a property,
//! and `Note.dinky` and the camera's `takephoto` read it.
//! `.yidam/catalog/delores-source.md` carries the counts.
//!
//! **This format has no way to say it.** An item here is a name and a ceiling: `Game::items` is a
//! list of strings and `item_cap` is the only thing declared per item. Every reach an item can
//! have is a binding, which is what the three tests below hold — and it is why the corpus's three
//! ways of reaching an item looked exhaustive.
//!
//! [`portable-object`]: ../../../.yidam/corpus/inventory-item/portable-object.yml
//! `.yidam/decisions/declared-reach.yml`.

use game_format::{parse, resolve, Effect, Pred};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn specimens() -> Vec<(String, game_format::Game)> {
    let dir = root().join("design/specimen");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|p| {
            let name = p
                .file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned();
            let src = std::fs::read_to_string(&p).expect("readable");
            let (mut game, mut problems) = parse(&src, &name);
            problems.extend(resolve(&mut game));
            assert!(problems.is_empty(), "{name} does not resolve: {problems:?}");
            (name, game)
        })
        .collect()
}

/// Every item a predicate asks about, however deep in a conjunction.
fn asked(p: &Pred, out: &mut BTreeSet<String>) {
    match p {
        Pred::Carrying(i) | Pred::Count(i, _, _) => {
            out.insert(i.clone());
        }
        Pred::Not(inner) => asked(inner, out),
        Pred::And(a, b) | Pred::Or(a, b) => {
            asked(a, out);
            asked(b, out);
        }
        _ => {}
    }
}

/// The three reaches `portable-object` names, per item, across one game.
fn reaches(g: &game_format::Game) -> BTreeMap<String, (usize, usize, usize)> {
    let mut n: BTreeMap<String, (usize, usize, usize)> =
        g.items.iter().map(|i| (i.clone(), (0, 0, 0))).collect();
    for b in &g.bindings {
        if let Some(o) = &b.object {
            if let Some(e) = n.get_mut(o) {
                e.0 += 1;
            }
        }
        if let Some(w) = &b.when {
            let mut items = BTreeSet::new();
            asked(w, &mut items);
            for i in items {
                if let Some(e) = n.get_mut(&i) {
                    e.1 += 1;
                }
            }
        }
        for eff in &b.effects {
            if let Effect::Spend(i, _) = eff {
                if let Some(e) = n.get_mut(i) {
                    e.2 += 1;
                }
            }
        }
    }
    n
}

/// **Every item in every specimen is reached by a binding, because nothing else can reach one.**
///
/// The corpus read this absence as possible evidence that the case does not arise. It is evidence
/// of something narrower: an item in this format has no surface for a non-binding reach, so the
/// four specimens could not have produced a counterexample however they were written. A fifth
/// would not have either, which is what makes the shipped game the right instrument and a specimen
/// the wrong one.
#[test]
fn every_item_in_every_specimen_is_reached_by_a_binding() {
    let mut totals = (0usize, 0usize);
    let mut unreached: Vec<String> = Vec::new();
    for (name, g) in specimens() {
        for (item, (second, required, spent)) in reaches(&g) {
            totals.0 += 1;
            if second + required + spent == 0 {
                unreached.push(format!("{name}/{item}"));
            } else {
                totals.1 += 1;
            }
        }
    }
    assert_eq!(
        totals,
        (16, 16),
        "declared items across five games, and items a binding names"
    );
    assert!(
        unreached.is_empty(),
        "an item no binding reaches: {unreached:?}"
    );
}

/// **An item is a name and a ceiling, and that is the whole of what an item can declare.**
///
/// `Game::items` is `Vec<String>`; `item_cap` is a number per item and exists for termination
/// rather than for authoring. There is no field an author fills in on an item that anything but a
/// binding reads — so the three reaches are not a finding about what authors do, they are the
/// arity of the format.
///
/// Delores's `for_story = "dead_languages"` is one line on an item that wires it into a mechanic
/// with its own state, its own progression and its own failure messages. Here it has nowhere to go.
#[test]
fn the_format_gives_an_item_no_surface_but_its_name() {
    let lib =
        std::fs::read_to_string(root().join("crates/game-format/src/lib.rs")).expect("readable");
    assert!(
        lib.contains("pub items: Vec<String>,"),
        "an item is its name"
    );
    assert!(
        lib.contains("pub item_cap: BTreeMap<String, i64>,"),
        "and a ceiling, which is about termination rather than about the item"
    );

    // Whatever an item is declared with, the parse tree keeps a name and a number.
    let (mut game, mut problems) = parse(
        "room hall \"a hall\"\n\
         item note \"a note\"\n\
         \n\
         room hall\n\
         \x20 start\n\
         \x20 hotspot desk \"a desk\"\n\
         \x20   is note\n\
         \x20   pick-up:\n\
         \x20     do    give note\n\
         \x20   look-at:\n\
         \x20     \"a desk.\"\n\
         \x20 hotspot out \"the way out\"\n\
         \x20   walk-to:\n\
         \x20     ending\n",
        "fixture.game",
    );
    problems.extend(resolve(&mut game));
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(game.items, ["note"], "declared, and nothing else recorded");
    assert!(
        game.item_cap.get("note").is_none_or(|c| *c == 1),
        "the only per-item datum, and it defaults"
    );
}

/// **The corpus's three ways of reaching an item are the format's three, and a shipped game has a
/// fourth.**
///
/// Stated as a comparison rather than as a gap, because the two are not the same claim. That the
/// format cannot express `for_story` is a fact. Whether it *should* is the decision, and
/// `.yidam/decisions/declared-reach.yml` declines to widen the surface on one game's evidence.
///
/// What this pins is the shape: an object reaching a subsystem by declaration is a **fourth axis**
/// beside the three, and every reach this language has runs through `Binding`.
#[test]
fn every_reach_this_language_has_runs_through_a_binding() {
    let mut kinds: BTreeSet<&str> = BTreeSet::new();
    for (_, g) in specimens() {
        for b in &g.bindings {
            if b.object.is_some() {
                kinds.insert("second object");
            }
            if let Some(w) = &b.when {
                let mut items = BTreeSet::new();
                asked(w, &mut items);
                if !items.is_empty() {
                    kinds.insert("carried requirement");
                }
            }
            for e in &b.effects {
                match e {
                    Effect::Spend(..) => {
                        kinds.insert("consumption");
                    }
                    Effect::Give(..) => {
                        kinds.insert("acquisition");
                    }
                    _ => {}
                }
            }
        }
    }
    assert_eq!(
        kinds.into_iter().collect::<Vec<_>>(),
        [
            "acquisition",
            "carried requirement",
            "consumption",
            "second object"
        ],
        "every way an item is touched in four games, and all four are fields of a binding"
    );

    let node =
        std::fs::read_to_string(root().join(".yidam/corpus/inventory-item/portable-object.yml"))
            .expect("readable");
    assert!(
        node.contains("never required, consumed, or offered"),
        "the three the node names"
    );
}
