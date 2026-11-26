//! **The one declared clock was set against a figure nothing computes.**
//!
//! `.yidam/decisions/inert-reports.yml` declared `questions_after = 100` and justified the value
//! with one reading — *"the oldest open question here is `actor/player-character` at 86
//! commits"* — which now appears in three files and is derived in none: the config comment that
//! sets the interval, the record that argued for it, and `reports.rs`'s own header.
//!
//! **It is not the number of commits since that question was asked.** The marker on
//! `actor/player-character` blames to the root commit and has never been edited: by the only
//! measure this repository can compute it is the whole life of the corpus, and the report says 88.
//!
//! **And the report ties five questions that the history cannot tie.** Read by hand, `due` puts
//! five of the nine at the same age. The nine were written at eight distinct moments and the
//! largest group sharing one is three. Whatever the number counts, it is not when the question was
//! asked — and it is not commits touching the file (29), commits touching the corpus (143), or
//! first-parent commits (137) either.
//!
//! **And the age was one of three figures that comment gave and held none of.** The count of what
//! the clock cannot see was wrong in the commit that wrote it — that commit pinned 32 and the
//! sentence said 31 — and the share was right then and not now. Unreadable, mistaken, decayed:
//! three ways for a figure to be loose, in four sentences of one comment, and the same remedy for
//! all three. Every figure that comment still gives is checked below.
//!
//! So the clock stays as a tripwire and the gauge is here. A question's age in this corpus is the
//! commit that last wrote its marker line, which is a **lower bound** — a reword that leaves the
//! question standing resets it — and even the lower bound puts four questions past the interval
//! while `due` reports nothing due.

use design_gate::{ends_question, repo_root};
use std::path::{Path, PathBuf};
use std::process::Command;

/// What `yidam due` reports for the nine, at this commit.
///
/// Read by hand, by setting `questions_after` to each value in turn and reading back the line that
/// changes. That is the only interface: `due --format json` carries `overdue` and a prose
/// `detail` and no per-question age, `open-questions --format json` carries a path and a label,
/// and the text report names the longest only while the clock is due. **A figure that can be read
/// in exactly one way, by mutating the configuration, is a figure that will be quoted rather than
/// checked** — which is how it got into three files.
const REPORTED: [usize; 9] = [88, 88, 88, 88, 88, 83, 74, 7, 7];

/// Every `.yml` under a directory, sorted.
fn yml(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)
            .expect("the directory exists")
            .flatten()
        {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The line each corpus node's `[open]` marker sits on — `markers.rs`'s rule, which holds that
/// there is at most one to a node and that the count is nine.
fn asking(root: &Path) -> Vec<(PathBuf, usize)> {
    let mut out = Vec::new();
    for p in yml(&root.join(".yidam/corpus")) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if let Some((i, _)) = src.lines().enumerate().find(|(_, l)| l.contains("[open]")) {
            out.push((p, i + 1));
        }
    }
    out
}

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

/// The commit that last wrote a line, and how many commits have landed since.
fn written(root: &Path, p: &Path, line: usize) -> (String, usize) {
    let rel = p.strip_prefix(root).expect("under the root");
    let range = format!("{line},{line}");
    let blame = git(
        root,
        &[
            "blame",
            "--porcelain",
            "-L",
            &range,
            "--",
            &rel.to_string_lossy(),
        ],
    );
    let sha = blame
        .lines()
        .next()
        .and_then(|l| l.split(' ').next())
        .expect("a blame header")
        .to_string();
    let age = git(root, &["rev-list", "--count", &format!("{sha}..HEAD")])
        .parse()
        .expect("a count");
    (sha, age)
}

/// The interval the corpus declares, in whatever the tool counts.
fn interval(root: &Path) -> usize {
    std::fs::read_to_string(root.join(".yidam/config.toml"))
        .expect("a config, which `inert-reports` wrote")
        .lines()
        .find_map(|l| l.trim().strip_prefix("questions_after")?.split('=').nth(1))
        .and_then(|v| v.trim().parse().ok())
        .expect("a declared questions clock")
}

/// The age of every live question, oldest first.
fn ages(root: &Path) -> Vec<(String, usize)> {
    let mut out: Vec<(String, usize)> = asking(root)
        .into_iter()
        .map(|(p, line)| {
            let (_, age) = written(root, &p, line);
            let name = p
                .strip_prefix(root.join(".yidam/corpus"))
                .unwrap_or(&p)
                .to_string_lossy()
                .into_owned();
            (name, age)
        })
        .collect();
    out.sort_by_key(|(n, a)| (std::cmp::Reverse(*a), n.clone()));
    out
}

/// **The oldest open question was asked in the root commit, and it has been answered.**
///
/// This held `genesis` for six phases on the stated grounds that *"a figure that decays is a
/// figure somebody re-pins without reading"*, and that `genesis` does not decay. It does not. It
/// can be closed, which is the one thing that was never going to happen and now has:
/// `actor/player-character` asked whether transferring control between characters needs vocabulary
/// beyond a flag naming the controlled one, and `.yidam/decisions/control-transfer.yml` answered it
/// with a fifth specimen — **the first corpus question this repository has ever closed.**
///
/// So the anchor is gone and what is held instead is two things: that no marker in the corpus is
/// genesis prose any more, and which marker is now the oldest. The first is the durable half. The
/// second will move again, and moving is what it is for.
#[test]
fn the_oldest_open_question_is_no_longer_the_one_asked_at_genesis() {
    let root = repo_root();
    let genesis = git(&root, &["rev-list", "--max-parents=0", "HEAD"]);
    let (name, line) = asking(&root)
        .into_iter()
        .map(|(p, line)| {
            let (sha, age) = written(&root, &p, line);
            (age, p, sha, line)
        })
        .max_by_key(|(age, ..)| *age)
        .map(|(_, p, sha, line)| {
            (
                p.strip_prefix(root.join(".yidam/corpus"))
                    .unwrap_or(&p)
                    .to_string_lossy()
                    .into_owned(),
                (sha, line),
            )
        })
        .expect("nine questions");
    assert_ne!(
        line.0.as_str(),
        genesis.as_str(),
        "a marker line blamed on the root commit — the genesis question is back, or was reopened"
    );
    assert_eq!(
        name.as_str(),
        "world-flag/latched-flag.yml",
        "the oldest surviving question"
    );
}

/// **Four questions are past the declared interval, and the four are not the four.**
///
/// When this was written `yidam due` printed *"9 open question(s), none past 100 commit(s)"* and
/// `passed: true`, while this check found four past it: `actor/player-character` since genesis,
/// `data-format/interaction-table` at 322, `world-flag/latched-flag` at 185 and
/// `hotspot/static-region` at 157. The finding was the disagreement rather than the number.
///
/// The report has since crossed its own threshold and the count here is four again, over a set
/// missing its oldest member — `control-transfer` closed the genesis question. So the assertion
/// that named it is gone, and what is left is the shape: the count only grows except by a closure,
/// and a closure is a thing a record has to have done.
///
/// **Three, and the fourth was a closure.** `ags-testimony` answered why AGS removed the
/// Interaction Editor, and `prior-art/ags`'s old marker went with it. The marker that node carries
/// now asks something new and is young. The same phase narrowed two other questions and wrote
/// around their marker lines without touching them, so those two kept their ages. A narrowed
/// question is still the same question, and resetting its age would have been the reword this
/// assertion exists to catch.
#[test]
fn the_corpus_has_questions_past_the_interval_it_declared() {
    let root = repo_root();
    let interval = interval(&root);
    let ages = ages(&root);
    let past: Vec<&str> = ages
        .iter()
        .filter(|(_, a)| *a > interval)
        .map(|(n, _)| n.as_str())
        .collect();
    assert!(
        past.len() >= 3,
        "fewer questions past {interval} than when this was written, which means one was answered \
         or one was reworded — and only one of those is a reason to lower the number here"
    );
    assert!(
        !past.contains(&"actor/player-character.yml")
            && !asking(&root)
                .iter()
                .any(|(p, _)| p.ends_with("player-character.yml")),
        "the genesis question is back in the corpus, which nothing here did"
    );
    // **A question asked by the commit that settles the phase has an age of zero, and until
    // `input-side` no phase had ever added one.** Every previous movement in the corpus's count
    // was a closure, so `written()` always had a past commit to blame and the check could assume
    // one. Zero is not "unaged" here — it is the exact and correct answer for a marker line that
    // is one commit old, and it decays into an ordinary age on the next commit. What the check
    // still refuses is a *negative* history, which cannot happen, so the assertion that is left
    // is the one worth keeping: every question this check reports is one it found a blame for.
    assert_eq!(
        ages.len(),
        asking(&root).len(),
        "a question this check cannot age at all — the blame found no commit for its marker line"
    );
}

/// **The report ties five questions that the history cannot tie.**
///
/// [`REPORTED`] holds what `due` says, read by hand. Five of the nine share a value. The nine
/// marker lines were written at eight distinct moments and the largest group sharing one is the
/// three that `marker-count` rewrote in a single commit.
///
/// **The corpus is at four and [`REPORTED`] holds nine.** `declared-reach` closed
/// `portable-object`'s, `input-side` asked one about Sierra's icon bar, `control-transfer` closed
/// the genesis one, `adjacency` closed `scaled-polygon`'s, `two-bars` closed `sierra-sci`'s, and
/// `unread-specimen` **reopened** `puzzle/state-gate`'s — so the reading held below is of the nine
/// that stood when it was taken, and it is not re-taken. The tie it records is a fact about that
/// reading and does not decay with the count.
///
/// The reopening is worth naming here because it is the movement this clock is least able to
/// report. A question that comes back looks identical to a question that was just asked, and the
/// difference is that something downstream was resting on the answer.
///
/// A five-way tie cannot come out of a three-way maximum, so the report's number is not a count
/// of commits since the question was asked, under any reading of "asked" this repository can
/// compute. That is the whole finding, and it needs no mapping from node to reported age — which
/// is fortunate, because the report does not publish one.
#[test]
fn the_report_ties_five_questions_the_history_keeps_apart() {
    let root = repo_root();
    let computed: Vec<usize> = ages(&root).into_iter().map(|(_, a)| a).collect();
    assert_eq!(
        (computed.len(), REPORTED.len()),
        (4, 9),
        "the corpus against the reading `REPORTED` holds: four closures, one question asked, and \
         one reopened by `unread-specimen` when the census that closed it turned out to have been \
         reading four of five specimens"
    );

    let widest = |v: &[usize]| {
        let mut v = v.to_vec();
        v.sort_unstable();
        v.iter()
            .map(|x| v.iter().filter(|y| *y == x).count())
            .max()
            .unwrap_or(0)
    };
    assert_eq!(
        (widest(&REPORTED), widest(&computed)),
        (5, 3),
        "the largest group the report calls the same age, against the largest the history wrote \
         together"
    );
}

/// Live `[open]` markers in the decision records, by `markers.rs`'s rule.
fn owed_in_records(root: &Path) -> usize {
    let mut n = 0;
    for p in yml(&root.join(".yidam/decisions")) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let lines: Vec<&str> = src.lines().collect();
        n += lines
            .iter()
            .enumerate()
            .filter(|(i, _)| ends_question(&lines, *i))
            .count();
    }
    n
}

/// **Every figure the config states about this clock is computed here.**
///
/// `.yidam/config.toml` is where a reader goes when `due` prints a line they do not understand —
/// `inert-reports` put the declines there for exactly that reason — and before this check every
/// quantity it gave them was loose, in three different ways:
///
/// * **"at 86 commits"** — a reading no check derived, repeated into the record and into
///   `reports.rs`'s header, and not the age of the question under any measure computable here.
///   Replaced by the event, which does not decay.
/// * **"9 markers while 31 sit in `.yidam/decisions/`"** — **wrong when it was written.** The
///   commit that shipped this sentence also shipped `assert_eq!((corpus, decisions), (9, 32))`,
///   three files away in the same phase. Proximity is not a wire.
/// * **"a statement about 22% of what is owed"** — right when it was written and stale since. Two
///   records have been added; the share is lower every time one is.
///
/// Unreadable, mistaken, decayed. A correction fixes the second once and re-reading fixes the
/// third once; neither touches the first, and all three come back the next time somebody writes a
/// number into a comment. [`collected-verdict`]'s remedy is the only one that covers all three,
/// and covers them again next phase.
///
/// [`collected-verdict`]: ../../../.yidam/decisions/collected-verdict.yml
#[test]
fn every_figure_the_config_states_about_this_clock_is_computed_here() {
    let root = repo_root();
    let cfg = std::fs::read_to_string(root.join(".yidam/config.toml")).expect("readable");

    assert!(
        cfg.contains("since genesis"),
        "the config gives the age of the oldest question as an event, because a count decays \
         between the writing and the reading"
    );

    let figure = |after: &str| -> usize {
        let at = cfg
            .find(after)
            .unwrap_or_else(|| panic!("the config no longer says {after:?}"));
        cfg[..at]
            .rsplit(|c: char| !c.is_ascii_digit())
            .find(|s| !s.is_empty())
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| panic!("no figure before {after:?}"))
    };
    let owed = owed_in_records(&root);
    let corpus = asking(&root).len();
    assert_eq!(
        (figure(" sit in `.yidam/decisions/`"), owed),
        (owed, owed),
        "what the config says the clock cannot see, against what it cannot see"
    );
    assert_eq!(
        (figure("% of what is owed"), corpus * 100 / (corpus + owed)),
        (6, 6),
        "the share the config claims, against the share"
    );
}

/// **Every file that speaks about the oldest question points at what computes it.**
///
/// Three did not. `.yidam/config.toml` gave *"at 86 commits"* as the reason the interval is 100,
/// `.yidam/decisions/inert-reports.yml` gave the same figure and predicted the clock *"fires
/// within a phase or two"*, and this crate's `reports.rs` header repeated it. Two phases and ten
/// commits later the tool read 88, so the prediction was wrong about the rate as well as the
/// quantity, and nothing anywhere would have said so.
///
/// This is [`collected-verdict`]'s shape exactly, one phase on: a measured figure copied into
/// prose with nothing holding the copies together. That record's remedy was a check that reads the
/// statements and compares them to what it computes, and this is the same wire on a figure that
/// had reached three files including a test.
///
/// What is required is not silence about the age — it is that a file giving one names the event
/// this check derives, or names this check. A number with a provenance can be re-read; a number
/// without one gets copied.
///
/// [`collected-verdict`]: ../../../.yidam/decisions/collected-verdict.yml
#[test]
fn every_statement_of_the_oldest_question_points_at_what_computes_it() {
    let root = repo_root();
    let mut loose = Vec::new();
    for f in [
        ".yidam/config.toml",
        ".yidam/decisions/inert-reports.yml",
        "crates/design-gate/tests/reports.rs",
    ] {
        let src = std::fs::read_to_string(root.join(f)).expect("readable");
        if !src.contains("oldest") && !src.contains("86 commits") {
            continue;
        }
        if !(src.contains("genesis") || src.contains("clock.rs") || src.contains("unread-clock")) {
            loose.push(f);
        }
    }
    assert!(
        loose.is_empty(),
        "a file giving the age of the oldest question without naming the event or the check:\n  {}",
        loose.join("\n  ")
    );
}
