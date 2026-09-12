//! realizes: subsystem/verb-menu
//!
//! **The quay, before the player has done anything.**
//!
//! The node's responsibility is *"given a target and a state, produce the verbs to display and no
//! others"*, and the negative half is the load-bearing one. This walks the four targets standing
//! in `lighthouse.game`'s first room and checks both halves against what an author wrote.
//!
//! The labels are the part that could not have been tested one phase ago: the parse tree discarded
//! every declaration label, so a menu built on it could print `quay.crate` and nothing better.
//! `.yidam/decisions/authored-text.yml`.

use game_format::{parse, resolve, World};

fn lighthouse() -> game_format::Game {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../design/specimen/lighthouse.game"
    );
    let src = std::fs::read_to_string(path).expect("the lighthouse specimen");
    let (mut game, problems) = parse(&src, "lighthouse");
    let more = resolve(&mut game);
    assert!(
        problems.is_empty() && more.is_empty(),
        "the specimen reads clean: {problems:?} {more:?}"
    );
    game
}

/// **What the quay offers at the start, and what it withholds.**
#[test]
fn the_menu_at_the_quay_shows_what_the_author_wrote_and_no_more() {
    let game = lighthouse();
    let w = World::new(&game);
    let s = w.initial();

    let mut targets: Vec<&str> = verb_menu::shown(&w, &s).iter().map(|e| e.target).collect();
    targets.sort();
    targets.dedup();
    assert_eq!(
        targets,
        ["keeper", "quay.crate", "quay.door", "quay.loose-crowbar"],
        "every target standing in the first room"
    );

    // The keeper's name is unqualified and the three hotspots' names are not, because an actor is
    // declared globally and a hotspot belongs to its room. A menu prints the label either way,
    // which is the point of keeping one.
    let keeper = verb_menu::at(&w, "keeper", &s);
    assert_eq!(keeper[0].label, Some("the lighthouse keeper"));

    let crate_ = verb_menu::at(&w, "quay.crate", &s);
    let verbs: Vec<&str> = crate_.iter().map(|e| e.verb).collect();
    assert_eq!(
        verbs,
        ["look-at"],
        "and `use-with crowbar` is not among them, because the crowbar is not carried"
    );

    // the label is what a menu would print, and it exists because the parse tree stopped
    // throwing it away
    assert_eq!(crate_[0].label, Some("a nailed-shut crate"));
    assert_eq!(crate_[0].target, "quay.crate");
    assert!(crate_[0].object.is_none());
}

/// **Picking up the crowbar puts an entry in the crate's menu and takes one out of the rope's.**
///
/// Both halves in one transition: a binary verb's entry is keyed by its second object
/// (`.yidam/decisions/arity-cost.yml`), and an acquisition site whose object has been taken is no
/// longer a site (`verb_menu::live`).
#[test]
fn taking_the_crowbar_moves_two_entries_in_opposite_directions() {
    let game = lighthouse();
    let w = World::new(&game);
    let s = w.initial();

    let take = game
        .bindings
        .iter()
        .find(|b| b.verb == "pick-up")
        .expect("the crowbar is picked up somewhere");
    assert!(verb_menu::in_menu(&w, take, &s), "and it is offered first");

    let after = interaction_resolver_apply(&w, take, &s);
    let verbs: Vec<&str> = verb_menu::at(&w, "quay.crate", &after)
        .iter()
        .map(|e| e.verb)
        .collect();
    assert_eq!(
        verbs,
        ["look-at", "use-with"],
        "the crate gains the entry the crowbar keys"
    );
    assert!(
        verb_menu::at(&w, "quay.loose-crowbar", &after).is_empty(),
        "and the site it came from stops being one"
    );
}

/// Applying an effect list is the resolver's, and this crate must not depend on it — the corpus
/// edge runs the other way. The one transition this fixture needs is written out instead.
fn interaction_resolver_apply(
    w: &World,
    b: &game_format::Binding,
    s: &game_format::State,
) -> game_format::State {
    let mut n = s.clone();
    for e in &b.effects {
        if let game_format::Effect::Give(x, q) = e {
            if let Some(i) = w.items.iter().position(|y| y == x) {
                n.items[i] = (n.items[i] + q).min(w.caps[i]);
            }
        }
    }
    if let game_format::Site::InRoom { target, .. } = &b.site {
        if let Some(i) = w.takeable.iter().position(|t| t == target) {
            n.taken[i] = true;
        }
    }
    n
}
