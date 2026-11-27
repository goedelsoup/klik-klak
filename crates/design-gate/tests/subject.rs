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
///
/// `forum-witnesses` is the 101st and the 53rd to cite a document: `catalog/ags-forums`, read again
/// for eleven threads.
///
/// `maker-tracker` is the 102nd and the 54th: `catalog/ags-tracker`, the maintainers' issues.
///
/// `puzzle-width` is the 103rd and the 55th: `catalog/lsl5-source` again, for its hint book and
/// its points awards.
///
/// `house-width` is the 104th and the 56th: `catalog/sierra-agi-source`, three AGI games' logic.
///
/// `pq-acts` is the 105th and the 57th: `catalog/sierra-agi-source` again, Police Quest by region.
///
/// `pq-stops` is the 106th and the 58th: `catalog/sierra-agi-source` a third time, the patrol by
/// incident.
///
/// `larry-rooms` is the 107th and the 59th: `catalog/sierra-agi-source` a fourth time, Larry by
/// room.
#[test]
fn the_evidence_each_record_rests_on() {
    let root = repo_root();
    let all = records(&root);
    assert_eq!(all.len(), 107, "every record, in the order it was created");

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
        (59, 59, 45),
        "records citing a document, the engine crates, and the gate — they overlap"
    );
    assert_eq!(
        (alone, neither),
        (16, 12),
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
/// **And what clears it is named in advance for the second time — and it did.** The marker
/// `retraction-reach` corrected asked for an SCI1-era artifact, the next phase went and retrieved
/// one, and `two-bars` rests on it. Three predictions written into this comment, three settled by
/// the phase after. The run is zero.
///
/// **This clearing is the first by a retrieval of something that had never been in the house.**
/// `input-side` retrieved too, and `control-transfer` and `adjacency` cleared by re-reading sources
/// already held — which this check counts identically, and should not. A phase that opens a
/// document nobody had opened and a phase that opens one for a second question are different work,
/// and both read here as *cited a document*.
///
/// Two phases on the same tooling, one with a document and one without, is the distinction this
/// check was built to make visible. A phase that touches no source and knows it is not the same
/// as one that should have opened a source and did not, and the only thing that tells them apart
/// is the record.
///
/// **`unread-specimen` puts it to one, and the reason is the one this check keeps proving it
/// cannot see.** The phase found that twenty-three tests had stopped reading the fifth specimen,
/// which is a question about this repository's instrument and no source about adventure games
/// answers it. But it reads as **engine** rather than gate-alone, because the defect falsified a
/// claim about authoring: `latched-flag` said no author had ever cleared a flag, and the game
/// nobody had read clears one three times. So the gate-alone run stays at zero while the
/// no-document run goes to one — the two halves disagreeing, which is what they are for.
///
/// **The evidence was already in the house and nobody had to go anywhere for it.** That is a third
/// way to clear nothing at all: not a retrieval, not a re-reading, but a source this repository
/// wrote itself and then failed to open. 242 lines of authored game, committed eleven commits
/// earlier, unread by three quarters of the suite.
///
/// **`reads-like-writing` clears it the next phase, and clears it by re-reading.** The measurement
/// it took needed a scale — 84% of a line being a sentence means nothing on its own — and
/// `delores-source.md` had one, taken by `frame-width` for a different question and never read for
/// this one. So the run is zero and this check counts that identically to going out and retrieving
/// something nobody had.
///
/// **Most clearances here are re-readings, and the count is computed below rather than estimated.**
/// The unread list says which documents nobody opened. Nothing says what an opened document was
/// **not** looked at for, and on the evidence of the last eight phases that is where most of the
/// answers have been.
///
/// **`unread-spelling` puts both runs to one, and the gate-alone run is the one that matters.** It
/// is the first record since `class-evidence` to rest on the gate and nothing else, and the reason
/// is the subject: whether a check built on a list of English phrasings can report what it did not
/// recognise is a question about this repository's instrument, and no source about SCI, AGS or
/// SCUMM answers it. The record says so, which is what this tripwire asks of a phase that extends
/// the run rather than clearing it.
///
/// **It is also the case the run was built to allow rather than to forbid.** `unread-clock`
/// settled that this is a tripwire and not a gauge — it does not say the run is wrong, it says
/// nobody may extend it without noticing. One phase on the instrument, immediately after two that
/// corrected what the instrument had been measuring, is the sequence somebody would choose. A run
/// of two would be a different conversation.
///
/// **It did not reach two.** `restated-guard` cleared both runs the next phase by re-deciding a
/// question against `.yidam/catalog/delores-source.md` — a shipped game's guard census, taken by
/// the phase that read that source for what an object declares, and read here for whether a
/// language needs a name for a condition. `the_last_eight_phases_mostly_re_read` holds the shape:
/// **four of eight rested on a document already in the house, one retrieved a new one, and three
/// rested on no document at all.** This check counts the first two identically.
///
/// **It is at one again, and the tripwire is doing what it is for.** `declared-edge`'s subject is
/// this repository's own architecture — three `depends-on` edges the corpus declares against the
/// one the build takes — and no source about SCI, AGS or SCUMM says whether an edge somebody drew
/// at genesis survives a compiler. It rests on the engine and the gate, which is why the
/// gate-alone run stays zero: the evidence is a built thing rather than a check reading itself.
///
/// That is the second time in five phases the run has been extended by a phase whose subject is
/// the instrument or the engine, and both times the record said so. A run of two would be a
/// different conversation and this is not one.
///
/// `runtime-boundaries` raises the no-document run to one and keeps gate-alone at zero. Its
/// evidence is the implemented engine and its regression cases: initialization, arrival, save
/// restoration and floor routing are contracts already stated here, so correcting their
/// implementation requires no new claim about another engine.
///
/// **`one-toggle` makes it two, which is the different conversation, and the record has it.** Its
/// question is which polarity this format's `latched` keyword should take, and the only evidence
/// that can weigh it is what the six specimens write, which this repository wrote itself. No
/// source about another engine declares whether a flag latches. It rests on `latch.rs`, so it reads
/// as engine and the gate-alone run stays at zero. Two engine phases in a row is the run this check
/// was built to make somebody notice, and what clears it is the next phase that has a question a
/// document can answer.
///
/// **`edge-policy` makes it three, and it is not a phase.** A re-vendor's `yidam doctor` asked
/// every class to say whether its relationship vocabulary is closed, and the answer was a census
/// of this corpus's own edges, so no document could weigh it. It rests on no evidence path at
/// all, which is why `neither` went up with it. The tripwire is extended by maintenance rather
/// than by a phase that went to the repository instead of its subject, and the next phase is
/// still the one that clears it. The record does not say this itself: it was pushed before the
/// gate counted it, and an edit after its `decide:` would be a fifty-third edited record.
///
/// **`ags-testimony` clears it, and by the route this comment has named least often: a retrieval
/// of something that had never been in the house.** `testimony` had sorted three questions as
/// ones no artifact answers and said where the testimony was. The phase went to the AGS forums,
/// waited out a rate limit, and rests on `.yidam/catalog/ags-forums.md`. Both runs are zero.
/// The fifty-third edited record did arrive, but in `testimony` rather than here.
///
/// `second-game-width` keeps both at zero by retrieving a second game's source,
/// `.yidam/catalog/lsl5-source.md`.
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

/// **Four of the last eight phases answered a question with a source already in the house.**
///
/// The figure two records state, computed instead of counted by hand. Both got it wrong the first
/// time and in the same direction — `reads-like-writing` said *third in five* and `restated-guard`
/// said *fourth in six*, where the windows are six and eight — which is the ordinary failure of a
/// ratio asserted in prose, and `.yidam/decisions/unread-specimen.yml` is the record about pinning
/// a figure to something that cannot compute it.
///
/// Whether a phase **retrieved** a document or **re-read** one it already had is the distinction
/// the run above cannot draw, so it is drawn here by hand and held against the directory. The
/// catalog's unread list says which documents nobody opened; nothing says what an opened document
/// was not looked at for, and six of eight phases is what that costs.
///
/// **And the window has gone to zero.** `two-bars` fell off the end and nothing has replaced it:
/// every phase in the last eight that rested on a document rested on one already in the house. The
/// last six phases re-read, and the reason is legible: five were engine phases, which cannot find
/// out what an author writes by retrieving another manual, and the sixth read this corpus's own
/// records against each other. That is a real change of mode rather than a lapse, and it is the
/// first time this figure has been able to say so.
///
/// `runtime-boundaries` shifts that window to five re-readings and three records needing no
/// document; its contribution is implementation against the existing design.
///
/// `one-toggle` pushes `unread-spelling` out of the window and both rest on no document, so the
/// figures stand: five re-readings, nothing retrieved, three needing no document.
///
/// `edge-policy` pushes `restated-guard` out and rests on no document, so the window holds four
/// re-readings, nothing retrieved, and four needing no document.
///
/// `ags-testimony` pushes `unbuilt-solver` out and is the first retrieval in the window since
/// `two-bars` left it: three re-readings, one retrieval, four needing no document.
///
/// `second-game-width` pushes `or-earns-it` out and is a second retrieval in a row: a game's source
/// that had never been in the house. Two re-readings, two retrievals, four needing no document.
///
/// `forum-witnesses` pushes `declared-edge` out and is a third retrieval in a row. Its catalog
/// entry was already in the house, but every thread it rests on was fetched for the first time,
/// which is what this column means by new. Two re-readings, three retrievals, three needing no
/// document.
///
/// `maker-tracker` pushes `dismissed-hold` out and is a fourth retrieval in a row, from a kind of
/// document the house had not held: the AGS maintainers' tracker. One re-reading, four retrievals,
/// three needing no document.
///
/// `puzzle-width` pushes `refused-duration` out, and both re-read: it went back to the LSL5 clone
/// for the hint book and the points awards, which were in the house and unopened. The figures stand.
///
/// `house-width` pushes `runtime-boundaries` out and is a retrieval: three games' source that had
/// never been in the house. One re-reading, five retrievals, two needing no document.
///
/// `pq-acts` pushes `one-toggle` out and re-reads the Police Quest clone already in the house. Two
/// re-readings, five retrievals, one needing no document.
///
/// `pq-stops` pushes `edge-policy` out and re-reads the same clone again. Three re-readings and five
/// retrievals, and every one of the eight rests on a document.
///
/// `larry-rooms` pushes `ags-testimony` out and re-reads the same clone a fourth time. Four
/// re-readings and four retrievals.
#[test]
fn the_last_eight_phases_mostly_re_read() {
    /// Each of the last eight records, and whether the document it rests on was new to this
    /// repository when the phase opened. `None` where the phase rested on no document.
    const RECENT: [(&str, Option<bool>); 8] = [
        ("second-game-width", Some(true)),
        ("forum-witnesses", Some(true)),
        ("maker-tracker", Some(true)),
        ("puzzle-width", Some(false)),
        ("house-width", Some(true)),
        ("pq-acts", Some(false)),
        ("pq-stops", Some(false)),
        ("larry-rooms", Some(false)),
    ];

    let root = repo_root();
    let all = records(&root);
    let tail: Vec<String> = all[all.len() - RECENT.len()..]
        .iter()
        .map(|rel| {
            std::path::Path::new(rel)
                .file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        tail,
        RECENT
            .iter()
            .map(|(n, _)| n.to_string())
            .collect::<Vec<_>>(),
        "the eight most recent records, in creation order"
    );

    // the classification is by hand; that a row's record does or does not cite a catalog entry is
    // not, and the two have to agree
    for (name, retrieved) in RECENT {
        let src =
            std::fs::read_to_string(root.join(".yidam/decisions").join(format!("{name}.yml")))
                .expect("readable");
        assert_eq!(
            cites(&src, "catalog/"),
            retrieved.is_some(),
            "{name} is classified as resting on a document and does not cite one, or the reverse"
        );
    }

    let doc = RECENT.iter().filter(|(_, r)| r.is_some()).count();
    let fresh = RECENT.iter().filter(|(_, r)| *r == Some(true)).count();
    assert_eq!(
        (doc, fresh, doc - fresh),
        (8, 4, 4),
        "phases resting on a document, on one newly retrieved, and on one already in the house"
    );
}
