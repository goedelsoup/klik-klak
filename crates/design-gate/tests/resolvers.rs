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
    ///
    /// **Two of its three have closed, and both turned out to be about an artifact.**
    /// `prior-art/ags` asked whether a tool shipped, and `interaction/acquisition-binding` whether a
    /// window reached players, which a released script shows. `.yidam/decisions/ags-pickups.yml`.
    ///
    /// **All three have closed, and the third by reading for the traces a felt loss would leave.**
    /// `failure-mode/orphaned-dialogue-branch` asked whether authors experienced an orphaned option
    /// as a loss. The forum, the tracker and six released games with 71 commits of history show
    /// nobody acting on one, including in the game that ships eleven.
    /// `.yidam/decisions/ags-dialogs.yml`. The column is empty, which this table said it could not be.
    Testimony,
    /// A specific artifact, named, and not yet retrieved.
    ///
    /// **Empty, and it emptied by the phase going and getting something else.** `prior-art/
    /// sierra-sci` was its only member and it named an artifact twice — a document, then a shipped
    /// title's scripts — where what settled it was an **enumeration**: SCI0's kernel function
    /// table, which is the complete list of what the interpreter offered a script.
    /// `.yidam/decisions/two-bars.yml`. So a marker can name a resolver of the wrong *kind*, which
    /// is a failure no check here could have caught and only going and looking finds.
    Retrieval,
    /// Nothing — it is declined, and the marker states the condition that would reopen it.
    ///
    /// **Empty, and its last member closed by meeting its own condition.** `world-flag/latched-flag`
    /// declined to choose between three readings of a measured width until more games were cut.
    /// Seven phases cut four, and `.yidam/decisions/latched-close.yml` closed it.
    Condition,
    /// A game, written in this language, that either reaches for a construct or does not.
    ///
    /// **This column is empty and it emptied by being discharged**, which is the only one of the
    /// six that could. `actor/player-character` was its single member; `.yidam/decisions/
    /// control-transfer.yml` wrote the game and the question closed. The variant stays because the
    /// kind is real and the next question of it will want somewhere to go — and because a column
    /// that vanishes when it empties cannot be told from one nobody thought of.
    Specimen,
    /// A count over work that already exists.
    ///
    /// **Empty, and it emptied the way the other two did — by somebody going and counting.**
    /// `walkbox/scaled-polygon` wanted `found-in`'s test run over 84 real polygons and
    /// `.yidam/decisions/adjacency.yml` ran it; `data-format/interaction-table` wanted the file
    /// that carries the writing measured against the file it is keyed like, and
    /// `.yidam/decisions/reads-like-writing.yml` took it — 84% of a response line is the sentence,
    /// against 66% keyed flat, and 2.02 characters of writing per character of frame against a
    /// shipped game's 0.56.
    ///
    /// **So all three columns whose resolver was a thing somebody could do are empty.** That is
    /// the prediction this table made, complete. What it leaves is a corpus where nothing can be
    /// closed by counting.
    Measurement,
    /// The evidence exists and somebody has to weigh it.
    ///
    /// **New, and it arrived by a route none of the others can: a settled question reopening.**
    /// `puzzle/state-gate` was closed because the multi-flag gate it described did not occur in
    /// the corpus — one compound predicate in thirty-six. A fifth specimen wrote seven more, one
    /// of them joining five atoms, and `.yidam/decisions/unread-specimen.yml` found that the test
    /// counting them had not read that game. Nothing needs retrieving and nothing needs counting;
    /// the count is done. What is missing is a decision about whether four restatements of one
    /// conjunction earn a declarable.
    Judgement,
    /// The marker does not say. This column is empty and the check below is that it stays so.
    Unstated,
}

/// Every corpus question, and what its own marker says would close it.
///
/// Hand-classified, like `subject.rs`'s evidence paths, because what a paragraph *means* is not a
/// thing a string search can settle. The claim is the classification; the check below is that it
/// covers every marker on disk and no other.
const KINDS: &[(&str, Closes)] = &[];

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
/// would reopen them, and one wants a count over work that already exists.
///
/// **The specimen column is empty, the measurement column is down to one, and that is this table's
/// first result.** The specimen column had one member — a game that switches between characters,
/// which none of the four in `design/specimen` did — and the phase after this table was written
/// wrote the fifth (`.yidam/decisions/control-transfer.yml`). The phase after *that* ran the
/// measurement `walkbox/scaled-polygon` had been waiting for
/// (`.yidam/decisions/adjacency.yml`).
///
/// **Sorting questions by what would close them turned the list into a queue.** Three consecutive
/// closures, after eleven phases with none, and all three were the kinds whose resolver is a thing
/// somebody can go and do rather than a thing somebody has to find — a specimen, a measurement, a
/// retrieval. That is the argument for the classification stated as an outcome rather than as a
/// hope.
///
/// **What is left is what the table said would be left.** Three questions wait on testimony and two
/// are declined with a condition; between them they are five of the six, and not one will close by
/// anybody doing anything. `.yidam/decisions/resolver-kind.yml` predicted exactly that shape and
/// the three emptied columns are the prediction complete: specimen, retrieval and measurement were
/// the three whose resolver was a thing somebody could go and do, and all three are gone.
///
/// **Nothing in this corpus can now be closed by counting.**
///
/// **The seventh is a reopening, which is a direction this table had no way to expect.** Every
/// closure it sorted for was a question leaving the list. `puzzle/state-gate` came back onto it
/// because `.yidam/decisions/unread-specimen.yml` found that the census closing it had been
/// reading four of five specimens — so a question can be resolved by evidence that is wrong, and
/// nothing in a resolver kind says how the evidence was gathered. The new column is `Judgement`:
/// the counting is done and somebody has to weigh it.
///
/// **And a declined question closed, by a route this table said would not exist.**
/// `hotspot/static-region` was declined on the condition that this vocabulary has no geometry.
/// `.yidam/decisions/drawn-boundary.yml` made that condition permanent, and the scene file then
/// moved the question to the drawing layer, where `engine/play/tests/screen.rs` finds nothing that
/// asks for it. No new evidence arrived. Two later decisions had already answered it, and nobody had
/// read the marker against them. So "not one will close by anybody doing anything" was wrong for a
/// condition: a condition can be overtaken, and the way to find that out is to re-read it.
///
/// **And a testimony question closed, which this table also said would not happen.**
/// `prior-art/ags` asked whether a tool had shipped, and that turned out to be a question about an
/// artifact asked of testimony: `.yidam/decisions/forum-witnesses.yml` read the release thread and
/// a later statement that the tool was never finished. The two left in the column ask what authors
/// experienced, and the same reading narrowed them without closing either.
///
/// **And the last declined question closed, by the measuring its condition asked for.**
/// `world-flag/latched-flag` was declined until the width was measured in more games. Seven phases
/// cut four Sierra games into parts and counted the state at their joins, and
/// `.yidam/decisions/latched-close.yml` closed it. What is left is two questions, both testimony.
///
/// **And a second testimony question closed by an artifact.** `interaction/acquisition-binding`
/// asked whether AGS authors pay for the missing pickup bundle, which reads as a question about
/// what they experienced. The window it predicted is in a script, so released scripts could show
/// it, and `.yidam/decisions/ags-pickups.yml` read eleven games' and found none shipped. What is
/// left is one question, and it is testimony.
///
/// **And the last closed the same way.** `failure-mode/orphaned-dialogue-branch` asked what authors
/// felt about an orphaned option, and a felt loss leaves traces: a thread, a request, a fix.
/// `.yidam/decisions/ags-dialogs.yml` read every place those would be and found none. The table is
/// empty. Every column it sorted for has emptied, and the one it said would never empty emptied by
/// its questions turning out to be about artifacts.
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
            count(Closes::Judgement),
        ),
        (0, 0, 0, 0, 0, 0),
        "testimony, a named artifact, a condition that would reopen it, a game, a count, and a \
         decision somebody has to take"
    );

    // Every question has been answered, and the one that came back was re-decided. Nothing is
    // left. A question opened later must still say what would close it, and a declined one must
    // name the condition that would reopen it.
    for (node, kind) in KINDS {
        if *kind != Closes::Condition {
            continue;
        }
        let src = std::fs::read_to_string(root.join(".yidam/corpus").join(node)).expect("readable");
        assert!(
            src.contains("condition") || src.contains("would reopen"),
            "{node} is declined and does not say what would reopen it"
        );
    }

    // And the convention is written where the claim tags are, rather than only in a record.
    let readme = std::fs::read_to_string(root.join(".yidam/corpus/README.md")).expect("readable");
    assert!(
        readme.contains("The question says what would close it"),
        "the convention has a following and still no statement"
    );

    // Any question blocked on testimony points at the record that argues no artifact supplies it.
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
