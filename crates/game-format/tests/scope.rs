//! Names are scoped to what contains them.
//!
//! The rule is the ontology's containment read as a namespace: `room -contains-> hotspot`
//! makes a hotspot's name the room's to give, and a conversation owns its options — which is
//! what AGS does, numbering options *for the current topic*. Everything else is the world's.
//!
//! These are the cases. A flat gate passed a specimen with a `hotspot lamp` and an `option
//! lamp` in it for two phases, so a green run over one file is not evidence that scoping
//! works.

use game_format::{parse, resolve, Kind, Pred, Site};

const GAOL: &str = r#"
room hall "The Hall"
room cell "The Cell"
actor warden "the warden"
item key "an iron key"
flag freed "the door stands open"
flag asked in north "asked, in the north exchange"
flag asked in south "asked, in the south exchange"

room hall
  start
  hotspot door "a door"
    pick-up:
      do    give key
  actor warden
    talk-to:
      do    enter north

room cell
  hotspot door "another door"
    use-with key:
      do    set freed
      ending

conversation north with warden
  option greet "Hello?"
    when  not asked
    do    set asked
    do    enter south
  option bye "Never mind."
    do    leave

conversation south with warden
  option greet "Again?"
    when  not asked
    do    set asked
  option bye "Never mind."
    do    leave
"#;

fn load(source: &str) -> (game_format::Game, Vec<String>) {
    let (mut game, problems) = parse(source, "test.game");
    let all: Vec<String> = problems
        .into_iter()
        .chain(resolve(&mut game))
        .map(|p| p.to_string())
        .collect();
    (game, all)
}

fn when_of(game: &game_format::Game, option: &str) -> Pred {
    game.bindings
        .iter()
        .find(|b| matches!(&b.site, Site::InConversation { option: o, .. } if o == option))
        .and_then(|b| b.when.clone())
        .unwrap_or_else(|| panic!("no `when` on {option}"))
}

#[test]
fn two_rooms_may_each_have_a_door() {
    let (game, problems) = load(GAOL);
    assert!(problems.is_empty(), "{problems:#?}");
    assert_eq!(
        game.declared.get("hall.door").map(|(k, _)| *k),
        Some(Kind::Hotspot)
    );
    assert_eq!(
        game.declared.get("cell.door").map(|(k, _)| *k),
        Some(Kind::Hotspot)
    );
    // and the bare name is not a declaration of its own
    assert!(!game.declared.contains_key("door"));
}

#[test]
fn a_bare_name_means_the_local_one() {
    let (game, problems) = load(GAOL);
    assert!(problems.is_empty(), "{problems:#?}");
    // both exchanges write `asked`, and they are two different flags
    assert!(game.declared.contains_key("north.asked"));
    assert!(game.declared.contains_key("south.asked"));
    match when_of(&game, "north.greet") {
        Pred::Not(inner) => assert!(matches!(*inner, Pred::Flag(f) if f == "north.asked")),
        other => panic!("{other:?}"),
    }
    match when_of(&game, "south.greet") {
        Pred::Not(inner) => assert!(matches!(*inner, Pred::Flag(f) if f == "south.asked")),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_name_falls_back_to_the_world() {
    // `freed` is declared globally and referenced from inside a room
    let source = GAOL.replace(
        "    use-with key:\n      do    set freed\n",
        "    use-with key:\n      when  not freed\n      do    set freed\n",
    );
    let (game, problems) = load(&source);
    assert!(problems.is_empty(), "{problems:#?}");
    assert!(game.declared.contains_key("freed"));
}

#[test]
fn an_option_is_not_visible_from_a_room() {
    // The case the flat namespace could not express. `greet` exists twice and neither is
    // the world's, so a room asking for it bare is asking for something that is not there.
    let source = GAOL.replace(
        "    pick-up:\n      do    give key\n",
        "    pick-up:\n      when  not chosen greet\n      do    give key\n",
    );
    let (_, problems) = load(&source);
    assert!(
        problems
            .iter()
            .any(|p| p.contains("greet") && p.contains("not declared")),
        "an out-of-scope option must not resolve; got {problems:#?}"
    );
}

#[test]
fn a_qualified_name_reaches_across() {
    let source = GAOL.replace(
        "    pick-up:\n      do    give key\n",
        "    pick-up:\n      when  not chosen north.greet\n      do    give key\n",
    );
    let (game, problems) = load(&source);
    assert!(
        problems.is_empty(),
        "a qualified name must resolve; got {problems:#?}"
    );
    let b = game
        .bindings
        .iter()
        .find(|b| b.verb == "pick-up")
        .expect("the pick-up binding");
    match b.when.clone().unwrap() {
        Pred::Not(inner) => assert!(matches!(*inner, Pred::Chosen(o) if o == "north.greet")),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_collision_inside_one_scope_is_still_a_collision() {
    let source = GAOL.replace(
        "  hotspot door \"a door\"\n",
        "  hotspot door \"a door\"\n    look-at:\n      \"a door\"\n  hotspot door \"the same door\"\n",
    );
    let (_, problems) = load(&source);
    assert!(
        problems
            .iter()
            .any(|p| p.contains("hall.door") && p.contains("already declared")),
        "scoping widens the space, it does not remove the rule; got {problems:#?}"
    );
}
