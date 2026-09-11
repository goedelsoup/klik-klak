//! **`prior-art` has one incoming relationship and only an inventor can receive it.**
//!
//! The class declares three edges and all three point **out** — `implements`, `exhibits`, `reads`.
//! The one thing that can reach it is declared on the other side, `verb -originated-in-> prior-art`,
//! and running it returns three of the four instances: `ags`, `scumm`, `sierra-sci`.
//!
//! `prior-art/thimbleweed-park` is the fourth. It is a 2017 game that `implements` four verbs it
//! did not originate, so the only edge that can point at its class is one it structurally cannot
//! receive — and it is the corpus's second-largest node, 27 verified claims, uncited for 56
//! commits.
//!
//! **So a comparative class cannot reach the systems worth comparing to.** The relationship rewards
//! priority, and the later a system is the more it inherits and the less it can be pointed at. That
//! is a fact about the relationship's semantics rather than about any node.
//!
//! `yidam lint` reports two nodes under `orphan-in` and both have accounts —
//! `world-flag/object-state`'s in `prose-edges`, this one's here — and **neither account is in the
//! node.** A reader who opens either finds a node nothing points at and nothing saying why.

use design_gate::repo_root;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn nodes(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml")
                && !p.to_string_lossy().ends_with(".ont.yml")
            {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn name(root: &Path, p: &Path) -> String {
    p.strip_prefix(root.join(".yidam/corpus"))
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

/// Nodes no other node's `links:` block points at.
///
/// The same condition `yidam lint` reports as `orphan-in`, computed here because the lint misses
/// one — `prose-edges` found it silent about `cutscene/scripted-sequence` — and because a check
/// that has to shell out to a vendored binary to ask cannot hold an answer about it.
fn orphans(root: &Path) -> Vec<String> {
    let mut pointed: BTreeSet<PathBuf> = BTreeSet::new();
    for p in nodes(root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let Some(i) = src.find("\nlinks:") else {
            continue;
        };
        for line in src[i + 1..].lines() {
            let Some(t) = line.trim().strip_prefix("- target:") else {
                continue;
            };
            let t = t.trim();
            if t.ends_with(".ont.yml") {
                continue;
            }
            let at = p.parent().expect("a directory").join(t);
            if let Ok(c) = at.canonicalize() {
                pointed.insert(c);
            }
        }
    }
    let mut out: Vec<String> = nodes(root)
        .into_iter()
        .filter(|p| {
            !p.canonicalize()
                .map(|c| pointed.contains(&c))
                .unwrap_or(false)
        })
        .map(|p| name(root, &p))
        .collect();
    out.sort();
    out
}

/// **Three orphans, and the lint reports two of them.**
///
/// `prose-edges` established the decomposition and the count: five nodes had no incoming edge, one
/// was a defect with a remedy and was fixed, and the rest are structural or a kept refusal. What it
/// did not do is put any of that where a reader of the node would find it.
#[test]
fn the_nodes_nothing_points_at() {
    let root = repo_root();
    assert_eq!(
        orphans(&root),
        [
            "cutscene/scripted-sequence.yml",
            "prior-art/thimbleweed-park.yml",
            "world-flag/object-state.yml",
        ],
        "the third is the one `yidam lint` does not report"
    );
}

/// **Every node nothing points at says why, in the node.**
///
/// All three have accounts and all three accounts are in records — `prose-edges` for the two the
/// lint reports, and `scripted-sequence`'s own collapse argument for the third. A reader who opens
/// one of these nodes to find out why it is unreachable finds nothing; the explanation is one
/// directory away in a file the node does not cite.
///
/// This is the fifth phase in a row to find the same shape and the narrowest instance of it: the
/// corpus knows the answer, the answer is written down, and it is written where the question does
/// not get asked.
#[test]
fn every_orphan_accounts_for_itself() {
    let root = repo_root();
    let mut silent = Vec::new();
    for n in orphans(&root) {
        let src = std::fs::read_to_string(root.join(".yidam/corpus").join(&n)).expect("readable");
        // lowercased: the accounts open a sentence, so the phrase is capitalised where it is used
        // and not where it is quoted — the same used-versus-discussed distinction three other
        // checks in this crate have had to learn, arriving here as a capital letter
        let lower = src.to_lowercase();
        let accounts = ["nothing points at", "nothing links to", "no incoming"]
            .iter()
            .any(|s| lower.contains(s));
        if !accounts {
            silent.push(n);
        }
    }
    assert!(
        silent.is_empty(),
        "a node nothing points at, saying nothing about it:\n  {}",
        silent.join("\n  ")
    );
}

/// **Only an inventor can receive the one relationship that points at `prior-art`.**
///
/// The class's own `edges:` block declares three and all three are `out`. What reaches it is
/// declared on `verb`, and `originated-in` is a historical claim: the later a system is, the more
/// it inherits and the less it can be pointed at. Thimbleweed Park implements four verbs and
/// originated none of them.
///
/// The inverse — `verb -implemented-by-> prior-art` — is refused, and not for want of a use.
/// `prose-edges` read fifteen unedged citations by hand and refused two of them for being *"the
/// inverse of an edge that already exists"*, on the grounds that writing one makes a cycle out of a
/// direction the ontology chose. This would be the third.
#[test]
fn the_only_edge_into_prior_art_rewards_priority() {
    let root = repo_root();
    let ont =
        std::fs::read_to_string(root.join(".yidam/corpus/prior-art.ont.yml")).expect("readable");
    let block = ont.split("\nedges:").nth(1).expect("an edges block");
    assert_eq!(
        block.matches("direction: out").count(),
        3,
        "every relationship the class declares points away from it"
    );
    assert_eq!(
        block.matches("direction: in").count(),
        0,
        "and it declares none pointing at it"
    );

    let mut originated = Vec::new();
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if !src.contains("relationship: originated-in") {
            continue;
        }
        for line in src.lines() {
            if let Some(t) = line.trim().strip_prefix("- target:") {
                if t.contains("prior-art/") {
                    originated.push(t.trim().rsplit('/').next().expect("a name").to_string());
                }
            }
        }
    }
    originated.sort();
    originated.dedup();
    assert_eq!(
        originated,
        ["ags.yml", "scumm.yml", "sierra-sci.yml"],
        "three of the four instances, and the missing one is the 2017 game that inherited its verbs"
    );
}
