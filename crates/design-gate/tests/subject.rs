//! **Seven consecutive phases put five percent of their lines in the thing this repository is
//! about.**
//!
//! The window is closed and the figures are permanent: from the commit before `phase:
//! spatial-reach` to `phase: priority-edge`, 31 commits and **+2,634 lines**. Of those,
//! `.yidam/corpus/` took **146**. `crates/design-gate/` took **1,721**. `design/`,
//! `crates/reachability/` and `crates/game-format/` took **none at all** — the specimens, the
//! checker and the format were not edited once in seven phases.
//!
//! `design-gate` is the instrument that reads the corpus. It is not the engine and it is not the
//! subject, and `.claude/CLAUDE.md` names the confusion directly: *"`crates/` is not the engine.
//! The engine is what the corpus describes… conflating them is this repository's characteristic
//! mistake."*
//!
//! **The records say the same thing about themselves, permanently and one at a time.** A decision
//! rests on a catalog entry, on the engine crates, or on this gate, and which it cites is a fact
//! about what kind of question it answered. The run reached seven citing no document and five
//! resting on the gate alone, and `forty-eight` broke it.
//!
//! None of those phases was wrong and their findings were real. What they were not is about
//! point-and-click adventure games.

use design_gate::repo_root;
use std::path::Path;
use std::process::Command;

/// The closed window: the last phase whose work reached the corpus in quantity, to the last phase
/// before this one. Named by commit subject rather than by hash, so the anchors are readable and
/// a rebase that rewrites them is a failure rather than a silent drift.
const FROM: &str = "^phase: spatial-reach";
const TO: &str = "^phase: priority-edge";

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("git runs");
    assert!(out.status.success(), "git {args:?} failed");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn phase(root: &Path, grep: &str) -> String {
    let found = git(root, &["log", "--format=%H", "--grep", grep]);
    let sha = found
        .lines()
        .next()
        .unwrap_or_else(|| panic!("no commit matching {grep:?} — the window's anchor is gone"));
    sha.to_string()
}

/// Lines added under a path across the window.
fn added(root: &Path, from: &str, to: &str, path: &str) -> usize {
    git(
        root,
        &["diff", "--numstat", &format!("{from}^1"), to, "--", path],
    )
    .lines()
    .filter_map(|l| l.split('\t').next()?.parse::<usize>().ok())
    .sum()
}

/// Every decision record in the order it was created.
fn records(root: &Path) -> Vec<String> {
    let log = git(
        root,
        &[
            "log",
            "--reverse",
            "--format=%H",
            "--diff-filter=A",
            "--name-only",
            "--",
            ".yidam/decisions/",
        ],
    );
    let mut out: Vec<String> = Vec::new();
    for line in log.lines() {
        if !line.starts_with(".yidam/decisions/") || !line.ends_with(".yml") {
            continue;
        }
        if out.iter().any(|p| p == line) || !root.join(line).exists() {
            continue;
        }
        out.push(line.to_string());
    }
    out
}

/// What a record's own prose points at for evidence.
#[derive(Debug, PartialEq, Eq)]
struct Rests {
    document: bool,
    engine: bool,
    gate: bool,
}

/// A path is cited when something follows it. `catalog/delores-source.md` is a citation;
/// `` `catalog/` `` is a record talking about the catalog, and the two are opposite claims here.
///
/// This distinction was written into this file after the first version of it read `inward-turn` —
/// a record whose whole subject is that it rests on none of these — as resting on all three,
/// because the record names the directories in prose. Six other checks in this crate have had to
/// learn the same thing.
fn cites(src: &str, prefix: &str) -> bool {
    src.match_indices(prefix)
        .any(|(i, _)| src[i + prefix.len()..].starts_with(|c: char| c.is_ascii_alphanumeric()))
}

fn rests(root: &Path, rel: &str) -> Rests {
    let src = std::fs::read_to_string(root.join(rel)).expect("readable");
    Rests {
        document: cites(&src, "catalog/"),
        // `engine/` was added when a record first rested on a component rather than on the
        // checker. The classifier was written before that directory existed, so a record about
        // the engine counted as resting on no evidence path at all — which is the classification
        // `inward-turn` built this to make impossible.
        engine: cites(&src, "crates/reachability/")
            || cites(&src, "crates/game-format/")
            || cites(&src, "engine/"),
        gate: cites(&src, "crates/design-gate/"),
    }
}

/// **Seven phases, and the corpus took five percent.**
///
/// A closed window, so these numbers are history rather than a pin that decays. What they measure
/// is where seven phases of real work went: `.yidam/decisions/` and the gate that reads it, and
/// **not once** into the specimens, the checker or the format.
///
/// The 146 lines the corpus did take are almost all nodes writing about their own bookkeeping —
/// *"Nothing points at this node"*, *"It was silenced two phases later"* — which is the corpus
/// describing the repository rather than the subject, one level further in.
#[test]
fn seven_phases_put_five_percent_of_their_lines_in_the_corpus() {
    let root = repo_root();
    let (from, to) = (phase(&root, FROM), phase(&root, TO));

    let corpus = added(&root, &from, &to, ".yidam/corpus");
    let decisions = added(&root, &from, &to, ".yidam/decisions");
    let gate = added(&root, &from, &to, "crates/design-gate");
    let total = added(&root, &from, &to, ".");
    assert_eq!(
        (corpus, decisions, gate, total),
        (146, 711, 1721, 2634),
        "the corpus, the records, the instrument that reads them, and everything"
    );
    assert_eq!(
        corpus * 100 / total,
        5,
        "the share that reached the subject"
    );

    for untouched in ["design", "crates/reachability", "crates/game-format"] {
        assert_eq!(
            added(&root, &from, &to, untouched),
            0,
            "{untouched} was edited during the window, which would make this finding smaller"
        );
    }
}

/// **A record says what kind of question it answered, by what it cites.**
///
/// Three evidence paths, and they are not interchangeable. A catalog entry is a document about
/// adventure games. `crates/reachability` and `crates/game-format` are the checker and the format
/// — an argument resting on them is an argument about this design. `crates/design-gate` reads the
/// corpus itself, so a record resting only on it has answered a question about the repository.
///
/// All three are legitimate and the last is the one nothing could count.
#[test]
fn the_evidence_each_record_rests_on() {
    let root = repo_root();
    let all = records(&root);
    assert_eq!(all.len(), 85, "every record, in the order it was created");

    let (mut document, mut engine, mut gate, mut alone, mut neither) = (0, 0, 0, 0, 0);
    for rel in &all {
        let r = rests(&root, rel);
        document += usize::from(r.document);
        engine += usize::from(r.engine);
        gate += usize::from(r.gate);
        alone += usize::from(r.gate && !r.document && !r.engine);
        neither += usize::from(!r.document && !r.engine && !r.gate);
    }
    assert_eq!(
        (document, engine, gate),
        (43, 45, 35),
        "records citing a document, the engine crates, and the gate — they overlap"
    );
    assert_eq!(
        (alone, neither),
        (16, 11),
        "resting on the gate and nothing else, and resting on no evidence path at all"
    );
}

/// **The run is zero, and it reached five.**
///
/// A run, not a total, and that is the point: it rises with every phase that asks a question about
/// the repository and resets the moment one asks about adventure games. It reached seven records
/// citing no document and five resting on the gate alone — the longest it has been — and
/// `forty-eight` reset both by drawing on `.yidam/catalog/ags-source.md`.
///
/// It went to one when `unwritten` rested on the **engine crates** — counting what four authored
/// games did with the language, which is the third evidence path and is subject work — and back to
/// zero when `script-boundary` read a document again. `class-evidence` puts it at one apiece: a
/// record about how this corpus routes a class's argument is a claim about the repository, which
/// is what the gate-alone run is for.
///
/// **This is a tripwire rather than a gauge**, in the sense `unread-clock` settled: it does not
/// say the run is wrong, it says nobody may extend it without noticing. If this assertion has been
/// re-pinned upward, phases went to the repository instead of its subject — which may well be
/// right, and the record that did it should say why.
///
/// **It reached two and the sentence written here said what would break it.** That paragraph read:
/// *"The next phase has an outward target already named: `game-format` discards half of what an
/// author writes, and giving it back is the engine crates rather than this directory. If the run
/// reaches three, that sentence was wrong."* The two records were `engine-directory`, creating a
/// top-level directory, and `settled-surface`, correcting a proposal that called its own
/// implemented surface abstract — both preparation, and preparation for building is repository
/// work by its nature.
///
/// **It went to zero and back to one.** `authored-text` gave the parse tree back the response text
/// and the declaration labels, and rested on all three evidence paths at once: a shipped game
/// where 91 of 203 bindings are a single `sayLine`, the two crates it changed, and a gate it
/// re-pinned. The prediction is the part worth keeping — a tripwire that says in advance what
/// would clear it is testable in a way that one saying only *"do not extend me"* is not.
///
/// `two-components` put it back to one, and **the gate-alone run is the half that stayed at
/// zero**, which is the distinction this check was built to make. That phase moved the evaluator
/// out of the instrument and into `engine/`: it rests on the engine crates, so it is subject work
/// by the third evidence path, and it needed no document because *where a component lives* is not
/// a question any adventure-game source answers. A run of one there was a phase about code rather
/// than about a game, and the record said so.
///
/// **`playable` cleared it by asking whether an engine's main loop belongs in a corpus about
/// authoring vocabularies**, which is a question a document answers: AGS exposes the loop as a
/// script slot and nothing declarative, so the vocabulary has no form of it and the loop is the
/// engine's. A phase can reach for a source to decide what *not* to write down, which is not a use
/// of the catalog this check was built expecting.
///
/// **It went to one and `round-trip` cleared it, by finding the document the neighbouring phase
/// had no use for.** `parser-silence` asked whether this repository's reader keeps a rule this
/// repository's reader states, which no adventure-game source answers — the record says so rather
/// than leaving the count to be read as a gap. `round-trip` asked what happens when a game format
/// is written back out, and a shipped one had already been read: Delores's 22 room files survive
/// their own editor by permissiveness, keeping a dead key nothing writes, and this format takes
/// the other branch at both ends.
///
/// **And it went to one and was cleared by the next phase, for the third time.** `calculators`
/// retires two skills this repository approved against its own corpus at genesis, and no
/// adventure-game source answers whether it should. `normal-form` looked like the same shape — a
/// question about this format's own grammar — and turned out to have a documentary half after
/// all: the construct it declines is one AGS's interaction editor also lacks, in a predicate layer
/// that has no combination at all. Three times now the run has reached one and the phase after it
/// found a source, which is the ratio this check exists to keep visible rather than a number to
/// drive to zero.
///
/// **`input-side` cleared it a fourth time and by a different act: it retrieved the document
/// rather than opening one already held.** Every clearing before it drew on the eight sources that
/// were here on 2026-09-10, and **66 of this repository's 88 phases have run against a catalog that
/// stopped growing** — which is a fact this check cannot see, because a record citing a source is a
/// record citing a source whether the retrieval was a year old or an hour. The run measures whether
/// a phase reached for evidence. Nothing here measures whether the evidence was already in the
/// house, and for two thirds of this repository's life it always was.
///
/// **`line-format` put it to one and `class-library` cleared it, for the fourth time.** The first
/// asks whether a flat line format is better written down as a table or as a grammar — a question
/// about how to specify a reader, which no document describing what an engine offered can answer,
/// and the record says so and names the nearest thing it declined to cite. The second read three of
/// Sierra's in-house documents, and the count moved for a reason worth keeping: the record rested
/// entirely on a catalog entry and **did not link it**, so this check reported a gap that was a
/// missing citation rather than a missing source. It is the first time the tripwire has caught the
/// record rather than the phase.
///
/// **`resolver-kind` puts both halves to one at once, which has not happened since `class-evidence`
/// and is the plainest case either has had.** How a marker should be written, and what a clock can
/// see when it reports five questions overdue, are questions about this artifact's own practice.
/// No adventure-game source answers either, the record says so, and it rests on this crate and
/// nothing else because this crate is what it is about.
///
/// **And that is what happened.** The sentence above named the thing that would clear it, and the
/// thing that would clear it cleared it: `control-transfer` wrote the fifth specimen, rested on
/// `.yidam/catalog/twp-dev-diary.md` and on `engine/play`, and put both runs to zero. This is the
/// second time a prediction written into this comment has been settled by the phase after it —
/// `authored-text` was the first — and the reason to keep writing them is that a tripwire saying
/// only *do not extend me* cannot be wrong, while one that names its own exit can.
///
/// The prediction was cheap to make and it was not trivially true. What cleared the run is the
/// specimen, and a specimen is `design/` — a path this check does not count as an evidence path at
/// all. It cleared by reaching for a source *about* what the specimen found: a diary post titled
/// **switching and inventory**, in a catalog entry read five phases earlier for something else.
///
/// **`adjacency` holds it at zero the same way**, and the two together are a pattern this check
/// cannot see: both reached for a source already in the house and read it a second time for a
/// question the first reading was not aimed at. The unread list says which documents nobody
/// opened. Nothing says what an opened document was not looked at for, and the run cannot tell a
/// retrieval from a re-reading.
///
/// **`retraction-reach` puts it back to one and the record says why.** How a corpus keeps a marker
/// true when the record that corrects it lives somewhere no tool walks is a question about this
/// artifact's practice, and no source about SCI, AGS or SCUMM answers it. It reads a catalog entry
/// to build a table of what has been read and rests no claim about adventure games on it, which is
/// the distinction this check cannot draw and the record has to.
///
/// **And what clears it is named in advance for the second time.** The marker that phase corrected
/// asks for an SCI1-era artifact and nobody holds one, so the phase that goes and gets it is a
/// retrieval. If the run reaches two, that sentence was wrong.
///
/// Two phases on the same tooling, one with a document and one without, is the distinction this
/// check was built to make visible. A phase that touches no source and knows it is not the same
/// as one that should have opened a source and did not, and the only thing that tells them apart
/// is the record.
#[test]
fn the_run_of_records_that_needed_no_document_is_broken() {
    let root = repo_root();
    let all = records(&root);

    let run = |f: &dyn Fn(&Rests) -> bool| {
        all.iter()
            .rev()
            .take_while(|rel| f(&rests(&root, rel)))
            .count()
    };
    let no_document = run(&|r: &Rests| !r.document);
    let gate_alone = run(&|r: &Rests| r.gate && !r.document && !r.engine);
    assert_eq!(
        (no_document, gate_alone),
        (1, 1),
        "consecutive newest records citing no catalog entry, and resting on the gate alone"
    );
}
