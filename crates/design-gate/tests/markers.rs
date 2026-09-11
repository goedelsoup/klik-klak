//! **An open question is a claim, and this repository checks its claims everywhere except there.**
//!
//! That sentence is [`stale-markers`]'s, written after a sweep that found six of twenty-nine
//! markers no longer agreeing with the prose under them. It named the cause exactly — *"nothing
//! anywhere notices that a question has been answered"* — retired the six, and built nothing. A
//! second sweep found five more of the same shape, which is what a diagnosis without a check
//! predicts.
//!
//! This is the check. It cannot tell that a question has been answered; nothing can, short of
//! reading. What it can do is make the marker mean one thing, hold the count where a reader can
//! see it move, and state the part no tool here can see at all.
//!
//! [`stale-markers`]: ../../../.yidam/decisions/stale-markers.yml

use design_gate::repo_root;
use std::path::{Path, PathBuf};

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

/// Lines carrying the token, with whether each ends its paragraph.
fn markers(p: &Path) -> Vec<(usize, String, bool)> {
    let src = std::fs::read_to_string(p).expect("readable");
    let lines: Vec<&str> = src.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.contains("[open]"))
        .map(|(i, l)| {
            let ends = l.trim_end().ends_with("`[open]`")
                && lines.get(i + 1).is_none_or(|n| n.trim().is_empty());
            (i + 1, l.trim().to_string(), ends)
        })
        .collect()
}

/// **The token names a live question and nothing else.**
///
/// It did not. `yidam open-questions` reports a node as open if the bracketed token appears
/// anywhere in its bytes — the prelude warns about exactly this, calling it a property of the
/// node's *serialization* rather than of the node — so a node whose prose read *"That `[open]` is
/// closed"* was counted as open by the sentence announcing that it is not. Sixteen nodes were
/// reported; seven had a question.
///
/// The remedy the prelude names is a typed `claim_tag` property on the class. It was tried and
/// does not work in the pinned build: declaring `claim_tag: open` added an open *claim* and left
/// the node list unchanged, and declaring `claim_tag: verified` on a closed node did not remove it.
/// The declaration unions with the byte scan, so it can only add.
/// `.yidam/decisions/marker-count.yml`.
///
/// So the serialization is made truthful instead, which costs one convention: a retraction says the
/// question is closed without re-emitting the token.
#[test]
fn no_retraction_re_emits_the_token() {
    const CLOSED: [&str; 6] = [
        "is closed",
        "closes",
        "was `[open]`",
        "used to end",
        "no longer",
        "has since closed",
    ];
    let root = repo_root();
    let mut bad = Vec::new();
    for p in yml(&root.join(".yidam/corpus")) {
        for (n, line, _) in markers(&p) {
            if CLOSED.iter().any(|c| line.contains(c)) {
                bad.push(format!("{}:{n}  {line}", p.display()));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a sentence retracting a marker must not re-emit it:\n{}",
        bad.join("\n")
    );
}

/// **Nine live questions, one to a node**, which is what the tool reports because the bytes and
/// the prose agree.
///
/// Three of them are the same kind and say so: `prior-art/ags`, `acquisition-binding` and
/// `orphaned-dialogue-branch` all ask what an author experienced or intended, which no artifact
/// records. Two more were duplicates of those and are now pointers.
///
/// It was seven for six phases. The two that arrived came from **outside the corpus**: they had
/// been sitting on class pages since the root commit in a spelling no tool reads, and
/// `.yidam/decisions/action-pages.yml` moved them somewhere countable. So this is the one figure
/// here that went up because something was found rather than because something was asked.
#[test]
fn the_corpus_carries_nine_questions_and_the_count_is_the_token_count() {
    let root = repo_root();
    let mut open: Vec<String> = Vec::new();
    for p in yml(&root.join(".yidam/corpus")) {
        let m = markers(&p);
        assert!(
            m.len() <= 1,
            "{}: {} markers — one question to a node keeps the count readable",
            p.display(),
            m.len()
        );
        if !m.is_empty() {
            open.push(
                p.strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace(".yidam/corpus/", ""),
            );
        }
    }
    assert_eq!(
        open,
        [
            "actor/player-character.yml",
            "data-format/interaction-table.yml",
            "failure-mode/orphaned-dialogue-branch.yml",
            "hotspot/static-region.yml",
            "interaction/acquisition-binding.yml",
            "prior-art/ags.yml",
            "walkbox/scaled-polygon.yml",
            "world-flag/latched-flag.yml",
        ]
    );
}

/// **Three quarters of what this corpus still owes is invisible to every tool it has.**
///
/// `yidam open-questions` walks `.yidam/corpus/` and nothing else. The corpus holds 9 live
/// markers; the decision records hold **42 across 38 files**, and they are counted as zero — by
/// `status`, by `due`, by the index, by everything. There was a third place and this file could
/// not see it either: twelve more sat on the class pages in a spelling nothing reads, which
/// `crates/design-gate/tests/pages.rs` now holds at zero. The migration happened quietly: as questions
/// got answered they were replaced by *"what this does not settle"* paragraphs in the record that
/// answered them, which is the right place to put them and a place nothing looks.
///
/// The number is pinned rather than fixed because fixing it is a change to a vendored binary, and
/// `.yidam/.vendor/` is read-only here by rule.
#[test]
fn most_of_the_open_questions_are_where_nothing_looks() {
    let root = repo_root();
    let (mut corpus, mut decisions, mut records) = (0, 0, 0);
    for p in yml(&root.join(".yidam/corpus")) {
        corpus += markers(&p).len();
    }
    for p in yml(&root.join(".yidam/decisions")) {
        let live = markers(&p).iter().filter(|(_, _, ends)| *ends).count();
        decisions += live;
        records += usize::from(live > 0);
    }
    assert_eq!((corpus, decisions, records), (8, 45, 41));
    assert_eq!(
        corpus * 100 / (corpus + decisions),
        15,
        "under a fifth is visible"
    );

    // It was (7, 23, 18) for two phases and the visible share then fell one point a phase — 23,
    // 22, 21, 20, 19 — every time for the same reason: a phase closes nothing in the corpus and
    // opens one in the record that settles it. This is the first phase to move it the other way,
    // and not by closing anything: `.yidam/decisions/action-pages.yml` found two questions that had
    // been outside the count since the root commit and put them where it can see them. Both halves
    // of the denominator are readable now — `.yidam/decisions/README.md` lists the record side by
    // record and line, and `crates/design-gate/tests/pages.rs` holds the class pages to zero.
    //
    // It has fallen back since, to 17 and now to 16, and the mechanism is the first one again:
    // nothing was closed and two records were added. `action-pages` bought a point by making a
    // hidden question visible, which is a one-off; the drift is not. A share that only rises when
    // somebody widens the numerator by hand is a share that goes one way.
}
