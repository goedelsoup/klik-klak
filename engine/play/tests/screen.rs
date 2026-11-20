//! composes: verb-menu, interaction-resolver, response-presenter, path-solver

use play::load;
use play::screen::{Hold, Scene, Screen};

const GAME: &str = include_str!("../../../design/specimen/patrol.game");
const SCENE: &str = include_str!("../assets/patrol.scene");

fn finish(screen: &mut Screen) {
    if matches!(screen.hold, Hold::Walking { .. }) {
        screen.command("arrive", "").unwrap();
    }
    while matches!(screen.hold, Hold::Speaking { .. }) {
        screen.command("dismiss", "").unwrap();
    }
}

#[test]
fn walking_and_speech_hold_the_world_and_restore_does_not_arrive() {
    let game = load(GAME, "patrol.game").unwrap();
    let mut screen = Screen::new(&game, Scene::parse(SCENE, &game).unwrap()).unwrap();
    let before = screen.session.state.clone();
    screen.command("choose", "walk-to inner-door").unwrap();
    assert_eq!(screen.session.state, before);
    assert!(screen.command("choose", "walk-to inner-door").is_err());
    assert!(screen.command("save", "").is_err());
    finish(&mut screen);
    assert_eq!(screen.session.here(), "The Corridor");
    assert!(
        screen.command("arrive", "").is_err(),
        "a walk cannot commit twice"
    );
    for choice in ["walk-to canteen-door", "pick-up tin", "walk-to switch"] {
        screen.command("choose", choice).unwrap();
        finish(&mut screen);
    }
    assert!(screen.frame("", None).contains("the night porter"));
    let saved = screen.command("save", "").unwrap().unwrap();
    screen.command("choose", "look-at kettle").unwrap();
    screen.command("arrive", "").unwrap();
    assert!(matches!(screen.hold, Hold::Speaking { .. }));
    assert!(screen.command("load", &saved).is_err());
    assert!(screen.command("walk", "200 450").is_err());
    finish(&mut screen);
    screen.command("load", &saved).unwrap();
    assert!(matches!(screen.hold, Hold::Idle));
    assert!(screen.session.opening.is_empty());
    assert_eq!(screen.position, screen.scene.rooms["canteen"].spawn);
    let before = screen.position;
    assert!(screen.command("walk", "0 0").is_err());
    assert_eq!(screen.position, before);
}

#[test]
fn geometry_must_resolve_both_directions_and_stand_on_valid_floor() {
    let game = load(GAME, "patrol.game").unwrap();
    for bad in [
        SCENE.replace("lobby.inner-door", "lobby.missing"),
        SCENE.replace("765 440 door", "765 0 door"),
        SCENE.replace("room lobby passage 470 450", "room lobby passage 0 0"),
        SCENE.replace("figure canteen porter 680 425", ""),
        SCENE
            .lines()
            .filter(|l| !l.starts_with("target lobby lobby.inner-door"))
            .collect::<Vec<_>>()
            .join("\n"),
    ] {
        assert!(Scene::parse(&bad, &game).is_err());
    }
}

#[test]
fn the_inventory_display_is_a_reader_even_without_a_puzzle_predicate() {
    let source = "room hall \"Hall\"\nitem pebble \"a pebble\"\nroom hall\n  start\n  hotspot stone \"stone\"\n    is pebble\n    pick-up:\n      do give pebble\n  hotspot door \"door\"\n    walk-to:\n      ending\n";
    let geometry = "floor floor 0 340 960 340 960 540 0 540\nroom hall floor 200 450\ntarget hall hall.stone 300 300 30 30 315 450 tin\ntarget hall hall.door 600 200 100 200 650 450 door\n";
    let game = load(source, "inventory.game").unwrap();
    assert!(game_format::World::new(&game).items.is_empty());
    let mut screen = Screen::new(&game, Scene::parse(geometry, &game).unwrap()).unwrap();
    screen.command("choose", "pick-up stone").unwrap();
    finish(&mut screen);
    assert_eq!(screen.session.state.items, [1]);
    assert!(screen
        .frame("", None)
        .contains("\"label\":\"a pebble\",\"count\":1"));
    let saved = screen.session.save().unwrap();
    screen.session.restore(&saved).unwrap();
    assert_eq!(screen.session.state.items, [1]);
}

/// **The drawing layer is where a `part-of` would show, and the patrol scene has none of it.**
///
/// `hotspot/static-region` declined `part-of` on a condition: this vocabulary has no geometry, so
/// the relation would join two hotspots with nothing drawn under either. `drawn-boundary` then made
/// the first half of that permanent for `.game`, and the scene file made the second half false.
/// *Delores* declares `parent` in its drawing files and nowhere else, so this is the layer the
/// pressure would arrive in. Here, every target is drawn as exactly one rectangle (a second is
/// refused), no two rectangles in a room overlap, and the one contained hotspot is drawn beside
/// its container rather than inside it.
///
/// That is a tripwire, not a ban. A scene that wants two regions for one target, or one region
/// drawn inside another, fails here first. That is the evidence `graphical-client.md` says a
/// worked game has to supply before the drawing format grows.
#[test]
fn a_target_is_one_region_and_no_region_is_drawn_inside_another() {
    let game = load(GAME, "patrol.game").unwrap();
    let twice = format!("{SCENE}\ntarget lobby lobby.inner-door 20 20 40 40 765 440 door\n");
    assert!(
        Scene::parse(&twice, &game).is_err(),
        "a second region for one target is a part-of, and the scene format does not hold one"
    );

    let scene = Scene::parse(SCENE, &game).unwrap();
    let overlaps = |a: [i32; 4], b: [i32; 4]| {
        a[0] < b[0] + b[2] && b[0] < a[0] + a[2] && a[1] < b[1] + b[3] && b[1] < a[1] + a[3]
    };
    let mut drawn = 0;
    for (room, r) in &scene.rooms {
        drawn += r.targets.len();
        for (i, a) in r.targets.iter().enumerate() {
            for b in &r.targets[i + 1..] {
                assert!(
                    !overlaps(a.rect, b.rect),
                    "{room}: {} and {} overlap. That is the drawn part-of pressure \
                     `hotspot/static-region` closed on the absence of",
                    a.name,
                    b.name
                );
            }
        }
    }
    assert_eq!(drawn, 13, "the census read a different scene");

    let strongroom = &scene.rooms["strongroom"].targets;
    let rect = |suffix: &str| {
        strongroom
            .iter()
            .find(|t| t.name.ends_with(suffix))
            .unwrap()
            .rect
    };
    assert!(
        !overlaps(rect(".bond-in-cabinet"), rect(".cabinet")),
        "containment is a guard here, not geometry (`.yidam/decisions/containment.yml`)"
    );
}

/// **A save holds what the session's readers read, and the spot a player stands on is not one.**
///
/// `save-boundary` left one candidate for state no predicate reads: within-room position. The
/// graphical client answered it from both sides. The inventory display is a reader with no
/// predicate, and its quantities are in the snapshot. Position is the client's own, so two saves
/// taken at different spots in one room are the same bytes, and a load puts the player at the
/// room's spawn.
#[test]
fn where_the_player_stands_in_a_room_is_not_part_of_the_save() {
    let game = load(GAME, "patrol.game").unwrap();
    let mut screen = Screen::new(&game, Scene::parse(SCENE, &game).unwrap()).unwrap();
    let spawn = screen.scene.rooms["lobby"].spawn;
    assert_eq!(screen.position, spawn);
    let at_spawn = screen.command("save", "").unwrap().unwrap();
    screen.command("walk", "120 480").unwrap();
    finish(&mut screen);
    assert_ne!(screen.position, spawn);
    let elsewhere = screen.command("save", "").unwrap().unwrap();
    assert_eq!(at_spawn, elsewhere);
    screen.command("load", &elsewhere).unwrap();
    assert_eq!(screen.position, spawn);
}
