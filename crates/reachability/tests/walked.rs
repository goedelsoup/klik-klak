//! **Every specimen has a way to lose, the checker has reported all of them for phases, and
//! nobody has ever walked into one.**
//!
//! Five games, **six stranding bindings**, each named by line, cause and how many reachable
//! states it kills. The specimens document them in prose —
//! *"unwinnable by consumption if the oil goes into the hinge"*, *"unwinnable by ordering if the
//! river is crossed without the lantern"* — and `ferry.game` states the claim this rests on in a
//! comment beside the trap: ***"Reachable, silent, and the game goes on answering normally."***
//!
//! The checker proved the first word. **`engine/play/scripts/` holds the other two now**, one
//! losing run per specimen beside the winning one, each transcript pinned whole by
//! `engine/play/tests/lost.rs` — which is also where the two halves meet, because that crate takes
//! `reachability` as a **dev-dependency** and never a dependency. The engine cannot ask whether
//! the game is still winnable and must not be able to.
//!
//! What is held here is the checker's side: six bindings, by line, cause and states killed, and a
//! run written down for every one of them. `.yidam/decisions/walked-in.yml`.

mod common;

use reachability::{analyse, Cause};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/reachability is two levels below the root")
        .to_path_buf()
}

fn load(source: &str, name: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(source, name);
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{name}: {msgs:?}");
    g
}

/// Every way to lose, across the five.
fn strandings() -> Vec<(String, usize, String, Cause, usize)> {
    let mut out = Vec::new();
    for (name, src) in common::sources() {
        let r = analyse(&load(&src, &name));
        assert!(
            r.winnable,
            "{name} is winnable, which is what makes a trap one"
        );
        for s in r.strandings {
            let verb = match &s.object {
                Some(o) => format!("{} {o} on {}", s.verb, s.target),
                None => format!("{} on {}", s.verb, s.target),
            };
            out.push((name.clone(), s.line, verb, s.cause, s.states));
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
                "ferry".to_string(),
                64,
                "use-with coin on ferryman".into(),
                Cause::Ordering,
                2
            ),
            (
                "lighthouse".to_string(),
                131,
                "use-with oil-can on base.hinge".into(),
                Cause::Consumption,
                4
            ),
            (
                "lock".to_string(),
                152,
                "walk-to on lockside.paddle-gear".into(),
                Cause::Ordering,
                2
            ),
            (
                "nightwatch".to_string(),
                59,
                "walk-to on yard.hall-door".into(),
                Cause::Consumption,
                1
            ),
            (
                "nightwatch".to_string(),
                153,
                "walk-to on study.hall-door".into(),
                Cause::Consumption,
                1
            ),
            (
                "patrol".to_string(),
                141,
                "walk-to on canteen.corridor-door".into(),
                Cause::Ordering,
                1
            ),
            (
                "relay".to_string(),
                200,
                "use-with line on shed.stack".into(),
                Cause::Consumption,
                28
            ),
        ],
        "every way to lose, by line, cause, and states killed"
    );
}

/// **And every one of them has a run written down.**
///
/// One losing script per specimen, each naming the line the checker blames in its own header, so a
/// reader meets the pairing before running anything. The two nightwatch strandings are the same
/// trap by the two doors into the hall — which is what `on-enter` was introduced for, one binding
/// and two ways in — so one script covers both.
///
/// **Two specimens have a winning run written down as well, and the second is not here to be won.**
/// `lock-won.txt` is the artifact that shows what a change of controlled character looks like to
/// somebody playing, which is nothing: `engine/play/tests/transcript.rs`.
#[test]
fn every_one_of_them_has_a_run_written_down() {
    let dir = root().join("engine/play/scripts");
    let mut scripts: Vec<String> = std::fs::read_dir(&dir)
        .expect("the scripts directory")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    scripts.sort();
    assert_eq!(
        scripts,
        [
            "ferry-lost.txt",
            "lighthouse-lost.txt",
            "lighthouse-won.txt",
            "lock-lost.txt",
            "lock-won.txt",
            "nightwatch-lost.txt",
            "patrol-lost.txt",
        ],
        "every run written down"
    );

    for (name, line) in [
        ("ferry", 64),
        ("lighthouse", 131),
        ("nightwatch", 153),
        ("patrol", 141),
    ] {
        let src = std::fs::read_to_string(dir.join(format!("{name}-lost.txt"))).expect("readable");
        assert!(
            src.contains(&format!("line {line}")),
            "{name}-lost.txt does not name the line the checker blames"
        );
    }

    let ferry = common::source("ferry");
    assert!(
        ferry.contains("Reachable, silent, and the game goes on answering normally."),
        "the specimen states the claim the transcripts now hold"
    );
}
