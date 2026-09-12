//! **Every specimen has a way to lose, the checker has reported all of them for phases, and
//! nobody has ever walked into one.**
//!
//! Four games, **five stranding bindings**, each named by line, cause and how many reachable
//! states it kills. The specimens document them in prose —
//! *"unwinnable by consumption if the oil goes into the hinge"*, *"unwinnable by ordering if the
//! river is crossed without the lantern"* — and `ferry.game` states the claim this rests on in a
//! comment beside the trap: ***"Reachable, silent, and the game goes on answering normally."***
//!
//! The checker proved the first word. Nothing has ever shown the other two, because until
//! `.yidam/decisions/playable.yml` nothing could play a game at all — and the one script that
//! exists wins.
//!
//! These checks pin that. They are rewritten into the rule in the same phase.

use reachability::{analyse, Cause};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/reachability is two levels below the root")
        .to_path_buf()
}

const ALL: [(&str, &str); 4] = [
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

fn load(source: &str, name: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(source, name);
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{name}: {msgs:?}");
    g
}

/// Every way to lose, across the four.
fn strandings() -> Vec<(&'static str, usize, String, Cause, usize)> {
    let mut out = Vec::new();
    for (name, src) in ALL {
        let r = analyse(&load(src, name));
        assert!(
            r.winnable,
            "{name} is winnable, which is what makes a trap one"
        );
        for s in r.strandings {
            let verb = match &s.object {
                Some(o) => format!("{} {o} on {}", s.verb, s.target),
                None => format!("{} on {}", s.verb, s.target),
            };
            out.push((name, s.line, verb, s.cause, s.states));
        }
    }
    out
}

/// **Five bindings, and each one is a sentence a player reads as progress.**
#[test]
fn every_specimen_is_stranded_somewhere() {
    assert_eq!(
        strandings(),
        [
            (
                "ferry",
                64,
                "use-with coin on ferryman".into(),
                Cause::Ordering,
                2
            ),
            (
                "lighthouse",
                131,
                "use-with oil-can on base.hinge".into(),
                Cause::Consumption,
                4
            ),
            (
                "nightwatch",
                59,
                "walk-to on yard.hall-door".into(),
                Cause::Consumption,
                1
            ),
            (
                "nightwatch",
                153,
                "walk-to on study.hall-door".into(),
                Cause::Consumption,
                1
            ),
            (
                "patrol",
                141,
                "walk-to on canteen.corridor-door".into(),
                Cause::Ordering,
                1
            ),
        ],
        "every way to lose, by line, cause, and states killed"
    );
}

/// **And not one of them has been played.**
///
/// `engine/play/scripts/` holds one file and it wins. The specimens' own prose is the strongest
/// statement of what is untested: `ferry.game` says the failure is *"silent, and the game goes on
/// answering normally"*, and that is a claim about a transcript nobody has produced.
#[test]
fn nothing_has_walked_into_one() {
    let dir = root().join("engine/play/scripts");
    let mut scripts: Vec<String> = std::fs::read_dir(&dir)
        .expect("the scripts directory")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    scripts.sort();
    assert_eq!(
        scripts,
        ["lighthouse-won.txt"],
        "every run ever written down"
    );

    let ferry = ALL[0].1;
    assert!(
        ferry.contains("Reachable, silent, and the game goes on answering normally."),
        "the specimen states the claim beside the trap"
    );
}
