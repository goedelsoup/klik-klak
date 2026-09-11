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
/// The declaration unions with the byte scan, so it can only add. `.yidam/decisions/testimony.yml`.
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

/// **Seven live questions, one to a node**, which is what the tool now reports because the bytes
/// and the prose finally agree.
///
/// Three of the seven are the same kind and say so: `prior-art/ags`, `acquisition-binding` and
/// `orphaned-dialogue-branch` all ask what an author experienced or intended, which no artifact
/// records. Two more were duplicates of those and are now pointers.
#[test]
fn the_corpus_carries_seven_questions_and_the_count_is_the_token_count() {
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
            "world-flag/latched-flag.yml",
        ]
    );
}

/// **Three quarters of what this corpus still owes is invisible to every tool it has.**
///
/// `yidam open-questions` walks `.yidam/corpus/` and nothing else. The corpus holds 7 live
/// markers; the decision records hold **21 across 16 files**, and they are counted as zero — by
/// `status`, by `due`, by the index, by everything. The migration happened quietly: as questions
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
    assert_eq!((corpus, decisions, records), (7, 21, 16));
    assert_eq!(
        corpus * 100 / (corpus + decisions),
        25,
        "a quarter is visible"
    );
}
