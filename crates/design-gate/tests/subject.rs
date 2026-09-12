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
        engine: cites(&src, "crates/reachability/") || cites(&src, "crates/game-format/"),
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
    assert_eq!(all.len(), 72, "every record, in the order it was created");

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
        (35, 35, 25),
        "records citing a document, the engine crates, and the gate — they overlap"
    );
    assert_eq!(
        (alone, neither),
        (14, 11),
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
        (0, 0),
        "consecutive newest records citing no catalog entry, and resting on the gate alone"
    );
}
