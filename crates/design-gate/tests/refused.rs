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
    ("forced-order", Line::Duration),
    ("held", Line::Duration),
    ("interaction-resolver", Line::Presentation),
    ("interaction-table", Line::Both),
    ("latch-default", Line::Duration),
    ("presentation-layer", Line::Presentation),
    ("refused-duration", Line::AboutTheRecord),
    ("response-presenter", Line::Duration),
    ("runtime-boundaries", Line::Duration),
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

/// **Fifteen, and the figure that named nothing is gone from where it stood.**
///
/// The trade is load-bearing in every file whose argument would change if an effect could take
/// time: twelve resting on the duration line alone and three more on both lines of the same
/// record. `threshold-trigger` said *four places* and named none of them; it carries the count and
/// points at this table now.
#[test]
fn the_refusal_of_duration_is_load_bearing_in_fifteen_places() {
    let root = repo_root();

    let record = flat(&read(&root, ".yidam/decisions/threshold-trigger.yml"));
    assert!(
        !record.contains("is now load-bearing in four places"),
        "the uncountable figure still stands"
    );
    assert!(
        record.contains("[refused-duration](refused-duration.yml) counted: **fifteen**"),
        "and the count is where the question was asked"
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
        (14, 5, 3, 3),
        "the duration line, the presentation line, both, and neither"
    );
    assert_eq!(
        of(Line::Duration) + of(Line::Both),
        17,
        "places the refusal of duration is load-bearing"
    );
}

/// **The one decision is two decisions, and the record says so now.**
///
/// Admit duration tomorrow and `scrolling`, `dialogue-script`, `interaction-resolver`,
/// `arrival-realizer` and `presentation-layer` do not move a word: their argument is that
/// presentation sits outside the state model, which has nothing to do with how long anything
/// takes. That is why *four places* could be written and not checked — the record was cited
/// twenty-three times for two different things and no citation said which.
///
/// It names them now, and asks a later phase to say which it means. This test holds the naming
/// rather than the compliance: what a future citation says is a thing to read, not a string to
/// match, which is why the table above is by hand.
#[test]
fn the_record_names_the_two_lines_it_decides() {
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
    assert!(
        flat(&record).contains("is the **presentation line**")
            && flat(&record).contains("is the **duration line**"),
        "and the record gives them names a citation can use"
    );
    assert!(
        flat(&record).contains("A later phase citing this record should say which line it means."),
        "and asks for one"
    );
}

/// **The ordering that makes a skip safe is stated, and something plays it both ways.**
///
/// `cutscene/scripted-sequence`'s `skippable` is the second of the three costs
/// `threshold-trigger` names, and it was *eliminated* rather than answered: *"atomicity means
/// there is no part-way. **That is defining the problem away.**"* `[inference]`.
///
/// It is not defined away in the built system. It is **ordered** away: `Session::take_with`
/// settles the state and assigns it before one character reaches the player, so a player who skips
/// every line reaches the state of one who reads them all. The node says so now and
/// `engine/play/tests/skipped.rs` plays a specimen both ways to find out.
#[test]
fn the_ordering_that_makes_a_skip_safe_is_stated_and_played() {
    let root = repo_root();

    let node = flat(&read(&root, ".yidam/corpus/cutscene/scripted-sequence.yml"));
    assert!(
        node.contains("atomicity means there is no part-way. That is defining the problem away"),
        "the claim this phase found, kept where it was made"
    );
    assert!(
        node.contains("every write has landed before one character reaches the player"),
        "and the ordering that makes it true rather than merely stipulated"
    );

    // the order is in one file and something reads it now
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

    let played = read(&root, "engine/play/tests/skipped.rs");
    assert!(
        played.contains("fn skipping_every_line_reaches_the_state_of_reading_them_all()")
            && played.contains("patient.state, impatient.state"),
        "and a specimen is played both ways"
    );
    assert!(
        played.contains("fn the_trap_is_sprung_before_the_player_is_told_it_sprang()"),
        "including the one written to spring a trap on an arrival"
    );

    // and the count the marker gave is corrected where it was given
    let record = flat(&read(&root, ".yidam/decisions/threshold-trigger.yml"));
    assert!(
        !record.contains("is now load-bearing in four places"),
        "the uncountable figure still stands"
    );
    assert!(
        record.contains("refused-duration"),
        "and the record that counted it is named where it was asked"
    );
}
