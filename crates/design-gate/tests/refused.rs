//! **A trade made once, said to be load-bearing in four places, and nobody could count it.**
//!
//! [`threshold-trigger`](../../../.yidam/decisions/threshold-trigger.yml) closed with a marker:
//! *"Whether duration should have been refused. Everything above is downstream of
//! `effect-list-semantics`, and a design that admitted duration would need this trigger, a
//! `skippable` property, and the save-boundary problem the same decision record quotes Thimbleweed
//! Park paying. **That trade was made once and is now load-bearing in four places.**"*
//!
//! Four is not a count of anything. The figure names no places, nothing computes it, and this
//! corpus has a record about precisely that shape —
//! [`collected-verdict`](../../../.yidam/decisions/collected-verdict.yml) on figures that live in
//! prose.
//!
//! **And the reason it could not be counted is that the record draws two lines, not one.**
//! `effect-list-semantics` decided that *`say` is presentation and not an effect* and that *no
//! effect takes time*, and every later phase cites it for one or the other without saying which.
//! A census that reads citations counts both lines together and gets a number about neither.
//!
//! This file is the census. It pins the state as it was found.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Which line of `effect-list-semantics` a file's argument stands on.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Line {
    /// *No effect takes time.* Admit duration and this argument changes.
    Duration,
    /// *`say` is presentation and not an effect.* Admit duration and this argument does not move.
    Presentation,
    /// Both lines, in the same file and usually in the same table.
    Both,
    /// Neither: the file is about the record rather than about what it decided.
    AboutTheRecord,
}

/// Every file in the corpus or the records that cites `effect-list-semantics`, and which of its
/// two decisions the citing argument needs.
///
/// By hand, because no phrase separates them — the citations name the record and not the line.
/// The residue is held at empty below, which is `reach.rs`'s shape: a phase that cites this record
/// lands here and has to be read.
const CITING: &[(&str, Line)] = &[
    ("adjacency", Line::Duration),
    ("arrival-realizer", Line::Presentation),
    ("authored-text", Line::AboutTheRecord),
    ("dialogue-script", Line::Presentation),
    ("effect-half", Line::Both),
    ("entry-binding", Line::Duration),
    ("escape-hatch", Line::Both),
    ("held", Line::Duration),
    ("interaction-resolver", Line::Presentation),
    ("interaction-table", Line::Both),
    ("latch-default", Line::Duration),
    ("presentation-layer", Line::Presentation),
    ("response-presenter", Line::Duration),
    ("save-boundary", Line::Duration),
    ("save-state", Line::Duration),
    ("scaled-polygon", Line::Duration),
    ("scripted-sequence", Line::Duration),
    ("scrolling", Line::Presentation),
    ("thimbleweed-park", Line::Duration),
    ("threshold-trigger", Line::Duration),
    ("two-components", Line::AboutTheRecord),
    ("walked-in", Line::Duration),
];

/// Every `.yml` under the corpus or the records whose text names the record.
fn citing(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for dir in [".yidam/corpus", ".yidam/decisions"] {
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            let mut entries: Vec<_> = std::fs::read_dir(&d)
                .expect("readable")
                .flatten()
                .map(|e| e.path())
                .collect();
            entries.sort();
            for p in entries {
                if p.is_dir() {
                    stack.push(p);
                    continue;
                }
                if p.extension().is_some_and(|x| x == "yml")
                    && std::fs::read_to_string(&p)
                        .is_ok_and(|s| s.contains("effect-list-semantics"))
                {
                    let stem = p
                        .file_stem()
                        .expect("a stem")
                        .to_string_lossy()
                        .into_owned();
                    if stem != "effect-list-semantics" {
                        out.push(stem);
                    }
                }
            }
        }
    }
    out.sort();
    out
}

/// **Fifteen, not four — and the record names no places at all.**
///
/// The trade is load-bearing in every file whose argument would change if an effect could take
/// time. Counted, that is twelve files resting on the duration line alone and three more that rest
/// on both lines of the same record. The figure in prose is under it by a factor of nearly four,
/// and the prose figure cannot be checked because it points at nothing.
#[test]
fn the_trade_is_load_bearing_in_more_places_than_the_record_can_name() {
    let root = repo_root();

    let record = flat(&read(&root, ".yidam/decisions/threshold-trigger.yml"));
    assert!(
        record.contains("That trade was made once and is now load-bearing in four places."),
        "the figure, in prose"
    );
    assert!(
        record.contains(
            "would need this trigger, a `skippable` property, and the save-boundary problem"
        ),
        "and the three things it names, which are not four places"
    );

    let declared: Vec<&str> = CITING.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        citing(&root),
        declared,
        "a file citing the record with no row, or a row with no file"
    );

    let of = |l: Line| CITING.iter().filter(|(_, k)| *k == l).count();
    assert_eq!(
        (
            of(Line::Duration),
            of(Line::Presentation),
            of(Line::Both),
            of(Line::AboutTheRecord)
        ),
        (12, 5, 3, 2),
        "the duration line, the presentation line, both, and neither"
    );
    assert_eq!(
        of(Line::Duration) + of(Line::Both),
        15,
        "places the refusal of duration is load-bearing"
    );
}

/// **The one decision is two decisions, and a third of what cites it rests on the other one.**
///
/// Admit duration tomorrow and `scrolling`, `dialogue-script`, `interaction-resolver`,
/// `arrival-realizer` and `presentation-layer` do not move a word: their argument is that
/// presentation sits outside the state model, which has nothing to do with how long anything
/// takes. That is why *four places* could be written and not checked — the record is cited
/// twenty-two times for two different things and no citation says which.
#[test]
fn the_record_draws_two_lines_and_no_citation_says_which() {
    let root = repo_root();
    let record = read(&root, ".yidam/decisions/effect-list-semantics.yml");
    assert!(
        record.contains("**Instantaneous.** No effect takes time and no effect yields control."),
        "one line"
    );
    assert!(
        record.contains("**`say` is presentation and not an effect.**"),
        "and the other"
    );

    // and not one citing file names the line rather than the record
    let mut naming: Vec<&str> = Vec::new();
    for (stem, _) in CITING {
        let mut found = None;
        for dir in [".yidam/corpus", ".yidam/decisions"] {
            let mut stack = vec![root.join(dir)];
            while let Some(d) = stack.pop() {
                for e in std::fs::read_dir(&d).expect("readable").flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else if p.file_stem().is_some_and(|s| s == *stem) {
                        found = Some(std::fs::read_to_string(&p).expect("readable"));
                    }
                }
            }
        }
        let src = flat(&found.unwrap_or_else(|| panic!("no file for {stem}")));
        if src.contains("effect-list-semantics`'s duration") || src.contains("the duration half") {
            naming.push(stem);
        }
    }
    assert!(naming.is_empty(), "a citation names the line: {naming:?}");
}

/// **What makes a skip safe is an ordering nothing states and nothing checks.**
///
/// `cutscene/scripted-sequence`'s `skippable` property is the second of the three costs
/// `threshold-trigger` names, and it is *eliminated* rather than answered: *"a sequence that
/// writes state part-way through and is cut short leaves the world in a configuration the author
/// never wrote, and… atomicity means there is no part-way. **That is defining the problem
/// away.**"* `[inference]`.
///
/// It is not defined away in the built system. It is **ordered** away: `Session::take_with`
/// settles the state and assigns it *before* one character reaches the player, so a player who
/// skips every line reaches the state of one who reads them all. That is a stronger claim than the
/// node makes and nothing says it — one file in the build puts the two calls in that order and no
/// test reads the order.
#[test]
fn the_ordering_that_makes_a_skip_safe_is_unstated() {
    let root = repo_root();

    let node = flat(&read(&root, ".yidam/corpus/cutscene/scripted-sequence.yml"));
    assert!(
        node.contains("atomicity means there is no part-way. That is defining the problem away"),
        "the claim, at inference"
    );
    assert!(
        !node.contains("before the player is shown") && !node.contains("settles before"),
        "and the node says nothing about when the writes land"
    );

    // the order is in one file and is not asserted anywhere
    let client = read(&root, "engine/play/src/lib.rs");
    let settle = client
        .find("self.state = settled;")
        .expect("the assignment");
    let present = client
        .find("let held = present_all(")
        .expect("the presentation");
    assert!(
        settle < present,
        "the state settles before anything is shown"
    );

    let mut reading: Vec<String> = Vec::new();
    let mut stack = vec![root.join("engine"), root.join("crates")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n != "target") {
                    stack.push(p);
                }
                continue;
            }
            let is_test = p.components().any(|c| c.as_os_str() == "tests");
            let mine = p.file_name().is_some_and(|n| n == "refused.rs");
            if is_test && !mine && p.extension().is_some_and(|x| x == "rs") {
                let src = std::fs::read_to_string(&p).expect("readable");
                if src.contains("settle_tracking") && src.contains("present") {
                    reading.push(
                        p.file_name()
                            .expect("a name")
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
        }
    }
    assert!(
        reading.is_empty(),
        "something checks the order now: {reading:?}"
    );
}
