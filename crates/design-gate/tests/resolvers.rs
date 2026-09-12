//! **The clock came due for the first time, and it cannot tell the four kinds of overdue apart.**
//!
//! `yidam due` read *"none past 100 commit(s)"* for this repository's whole life and now reads
//! *"5 of 9 open question(s) past 100 commit(s)"*. `.yidam/config.toml` declared that interval as a
//! tripwire and `.yidam/decisions/unread-clock.yml` established that the number the report counts is
//! not the age of a question — it says 101 for the genesis question in `actor/player-character.yml`,
//! where the commit that last wrote that marker line is **513** commits back.
//!
//! What is new is not the number. It is that a threshold has been crossed, and crossing it asks the
//! repository to do something about five questions — three of which nothing anybody can do will
//! close.
//!
//! **Three of the nine are waiting for a kind of evidence this corpus has established does not
//! exist.** `prior-art/ags`, `interaction/acquisition-binding` and
//! `failure-mode/orphaned-dialogue-branch` all ask what an author experienced or intended, all three
//! point at `.yidam/decisions/testimony.yml`, and that record's whole finding is that every source
//! here is an artifact and an artifact records what was built rather than what building it was like.
//! A tripwire that fires on a condition nobody can clear is an alarm with no off switch.
//!
//! **And six of the nine already said what would close them, in five spellings, with nothing having
//! asked them to.** That is the shape this repository keeps finding: a convention with a following
//! and no statement — `.yidam/decisions/unread-list.yml` found it in the catalog, `sourcing.rs`
//! found it in four nodes writing about their own evidence standing, and here it was in the markers.
//! `.yidam/decisions/class-library.yml` is why this was looked at: a marker that named its resolver
//! was checkable, was checked, and was wrong.
//!
//! It is a rule now, written in `.yidam/corpus/README.md` beside the claim tags rather than only in
//! a record, and the three that were silent say. `.yidam/decisions/resolver-kind.yml`.

use design_gate::repo_root;
use std::path::{Path, PathBuf};

/// What would close a question, which is a different thing from how old it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Closes {
    /// Testimony: an account of what authoring was like. No artifact supplies it and
    /// `.yidam/decisions/testimony.yml` is where that is argued.
    Testimony,
    /// A specific artifact, named, and not yet retrieved.
    Retrieval,
    /// Nothing — it is declined, and the marker states the condition that would reopen it.
    Condition,
    /// A game, written in this language, that either reaches for a construct or does not.
    Specimen,
    /// A count over work that already exists.
    Measurement,
    /// The marker does not say. This column is empty and the check below is that it stays so.
    Unstated,
}

/// Every corpus question, and what its own marker says would close it.
///
/// Hand-classified, like `subject.rs`'s evidence paths, because what a paragraph *means* is not a
/// thing a string search can settle. The claim is the classification; the check below is that it
/// covers every marker on disk and no other.
const KINDS: &[(&str, Closes)] = &[
    ("actor/player-character.yml", Closes::Specimen),
    ("data-format/interaction-table.yml", Closes::Measurement),
    (
        "failure-mode/orphaned-dialogue-branch.yml",
        Closes::Testimony,
    ),
    ("hotspot/static-region.yml", Closes::Condition),
    ("interaction/acquisition-binding.yml", Closes::Testimony),
    ("prior-art/ags.yml", Closes::Testimony),
    ("prior-art/sierra-sci.yml", Closes::Retrieval),
    ("walkbox/scaled-polygon.yml", Closes::Measurement),
    ("world-flag/latched-flag.yml", Closes::Condition),
];

/// Every corpus node carrying a live marker, in the order the table expects them.
fn asking(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus")];
    let mut files: Vec<PathBuf> = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml") {
                files.push(p);
            }
        }
    }
    files.sort();
    for p in files {
        if std::fs::read_to_string(&p)
            .expect("readable")
            .contains("[open]")
        {
            out.push(
                p.strip_prefix(root.join(".yidam/corpus"))
                    .expect("under the corpus")
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
    out
}

/// **The table covers every question and invents none.**
#[test]
fn the_table_covers_every_question() {
    let root = repo_root();
    let declared: Vec<&str> = KINDS.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        asking(&root),
        declared,
        "every marker needs a row, and a row needs a marker"
    );
}

/// **Every question says what would close it, and the five kinds are not one.**
///
/// This was a census one commit ago: three questions did not say at all, and the report called five
/// of nine overdue without distinguishing any of them. `Unstated` is empty now and the check is that
/// it stays so — the other five kinds are facts about the questions rather than defects, and the
/// point of separating them is that a clock cannot.
///
/// Three wait on testimony and will not close by anybody doing anything, which is the case worth
/// having a column for. One waits on an artifact nobody has retrieved. Two are declined and say what
/// would reopen them. One wants a specimen — a game that switches between characters, which none of
/// the four in `design/specimen` does — and two want a count over work that already exists.
#[test]
fn every_question_says_what_would_close_it() {
    let root = repo_root();
    let count = |k: Closes| KINDS.iter().filter(|(_, c)| *c == k).count();
    assert_eq!(
        count(Closes::Unstated),
        0,
        "a question that does not say what would close it cannot be told from one that is waiting \
         for evidence which does not exist"
    );
    assert_eq!(
        (
            count(Closes::Testimony),
            count(Closes::Retrieval),
            count(Closes::Condition),
            count(Closes::Specimen),
            count(Closes::Measurement),
        ),
        (3, 1, 2, 1, 2),
        "testimony, a named artifact, a condition that would reopen it, a game, and a count"
    );

    // The three that were silent name their resolver and point at the record that asked them to.
    for node in [
        "actor/player-character.yml",
        "data-format/interaction-table.yml",
        "walkbox/scaled-polygon.yml",
    ] {
        let src = std::fs::read_to_string(root.join(".yidam/corpus").join(node)).expect("readable");
        assert!(
            src.contains("What would close it is") && src.contains("resolver-kind"),
            "{node} does not say what would close it"
        );
    }

    // And the convention is written where the claim tags are, rather than only in a record.
    let readme = std::fs::read_to_string(root.join(".yidam/corpus/README.md")).expect("readable");
    assert!(
        readme.contains("The question says what would close it"),
        "the convention has a following and still no statement"
    );

    // The three blocked on testimony each point at the record that argues no artifact supplies it.
    for (node, kind) in KINDS {
        if *kind != Closes::Testimony {
            continue;
        }
        let src = std::fs::read_to_string(root.join(".yidam/corpus").join(node)).expect("readable");
        assert!(
            src.contains("testimony"),
            "{node} is classified as waiting on testimony and does not name the record"
        );
    }

    // And that record still says the thing that makes them unclosable by reading.
    let testimony =
        std::fs::read_to_string(root.join(".yidam/decisions/testimony.yml")).expect("readable");
    let flat = testimony.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("artifact"),
        "testimony.yml no longer argues from the artifact distinction these three rest on"
    );
}
