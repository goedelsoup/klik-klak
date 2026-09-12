//! realizes: subsystem/interaction-resolver
//!
//! **Never nothing, on the crate in `lighthouse.game`.**
//!
//! The node's whole specification is one guarantee: *"produce exactly one outcome: an authored
//! response, an authored refusal, or a default. Never nothing… a silent miss is the failure
//! players read as brokenness."*
//!
//! Every line asserted below is a line an author wrote. None of them existed in the parse tree one
//! phase ago — `.yidam/decisions/authored-text.yml` — so this is the first fixture in this
//! repository that can check what a game *says* rather than only what it does.

use game_format::{parse, resolve, World};
use interaction_resolver::{resolve as run, Outcome};

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
        "{problems:?} {more:?}"
    );
    game
}

/// **The crate opens once, and says a different thing the second time.**
#[test]
fn the_crate_fires_then_refuses_and_both_are_authored() {
    let game = lighthouse();
    let w = World::new(&game);
    let crowbar = game
        .bindings
        .iter()
        .find(|b| b.verb == "use-with" && b.object.as_deref() == Some("crowbar"))
        .expect("use-with crowbar on the crate");

    // carrying the crowbar, crate shut
    let mut s = w.initial();
    s.items[game_format::slot(&w.items, "crowbar")] = 1;

    let next = match run(&w, crowbar, &s) {
        Outcome::Fired { says, next } => {
            assert_eq!(
                says,
                Some("The lid comes away with a squeal of nails. A can of oil sits in the straw.")
            );
            next
        }
        Outcome::Refused { .. } => panic!("it should open"),
    };

    match run(&w, crowbar, &next) {
        Outcome::Refused { says } => {
            assert_eq!(says, Some("It is open already, and empty."))
        }
        Outcome::Fired { .. } => panic!("it should not open twice"),
    }
}

/// **A guarded response list, read first-match-wins.**
///
/// The crate's `look-at` is `when crate-open "…"` then `otherwise "…"`, and what makes the second
/// a fallback is that it is written last rather than that it is spelled differently.
#[test]
fn the_guarded_response_list_selects_by_the_predicate_beside_the_line() {
    let game = lighthouse();
    let w = World::new(&game);
    let look = game
        .bindings
        .iter()
        .find(|b| {
            b.verb == "look-at"
                && matches!(&b.site, game_format::Site::InRoom { target, .. } if target == "quay.crate")
        })
        .expect("look-at on the crate");

    let shut = w.initial();
    let mut open = shut.clone();
    open.flags[game_format::slot(&w.flags, "crate-open")] = true;

    assert_eq!(
        interaction_resolver::select(&w, &look.says, &shut),
        Some("A crate, nailed shut. Something shifts when you rock it.")
    );
    assert_eq!(
        interaction_resolver::select(&w, &look.says, &open),
        Some("The crate stands open and empty.")
    );
}

/// **The arrival trigger, which has no verb and no target.**
///
/// `settle` fires every `on-enter` binding whose room the player is in, to a fixpoint, and
/// `interaction-resolver` is what realizes it — `.yidam/decisions/arrival-realizer.yml` found that
/// the fourth component the obvious repair would have added has no work to do here.
#[test]
fn settling_a_state_is_a_fixpoint_and_leaves_the_player_somewhere() {
    let game = lighthouse();
    let w = World::new(&game);
    let start = interaction_resolver::settle(&w, w.initial()).expect("the start settles");
    assert_eq!(
        interaction_resolver::settle(&w, start.clone()).expect("and again"),
        start,
        "settling a settled state changes nothing, which is what a fixpoint means"
    );
    assert_eq!(
        w.rooms[start.room], "quay",
        "the specimen starts on the quay"
    );
}
