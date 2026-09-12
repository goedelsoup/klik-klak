//! **A claim's footing is a fact about its node; every check that could exist sees its paragraph.**
//!
//! `evidence-kinds` left that open after measuring where verified claims say they rest: 112 name a
//! catalog entry in their own paragraph, 49 name a committed test, 5 name both, and **150 name
//! nothing** — continuations of passages sourced earlier, which is how anyone writes.
//!
//! What follows from it is a fragility nobody had counted. **Fifteen of the thirty-nine evidenced
//! nodes rest on exactly one distinct catalog entry, and eleven of those on a single line.** Delete
//! that line and the node trips `verified-unsourced`; the claims are unchanged. That is the same
//! node-granularity `evidence-kinds` found adding a line to silence `boolean-flag` and
//! `counter-flag` — **it cuts both ways, and neither direction has anything to do with whether the
//! claims are sourced.**
//!
//! And it says who wrote about their own evidence standing. Four nodes have, and all four are
//! nodes the lint had reported at some point; three of those four are single-line. The other nine
//! single-line nodes have no account of their sourcing and are not better sourced — they happened
//! to carry a line from the beginning. **Writing about your footing has required being reported
//! once.**

use design_gate::repo_root;
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

/// Verified claims that are asserted rather than discussed.
fn claims(src: &str) -> usize {
    src.replace("`[verified]`", "")
        .matches("[verified]")
        .count()
}

/// The catalog entries a node cites, and the lines that cite them.
fn sources(src: &str) -> (Vec<String>, usize) {
    let mut entries = Vec::new();
    let mut lines = 0;
    for l in src.lines() {
        let n = l.matches("catalog/").count();
        if n == 0 {
            continue;
        }
        lines += n;
        for chunk in l.split("catalog/").skip(1) {
            if let Some(e) = chunk.split(['.', ')', ' ', '`']).next() {
                entries.push(e.to_string());
            }
        }
    }
    entries.sort();
    entries.dedup();
    (entries, lines)
}

/// **Fourteen of forty evidenced nodes rest on one source, ten of them on one line.**
///
/// It was sixteen, then fifteen. `class-evidence` brought `interaction/binary-binding` up to the
/// three engines the class claim it carries now rests on, and a node came off this list by gaining
/// evidence rather than by anybody looking at the list — which is the only way a node has ever
/// left it. `.yidam/decisions/orphaned-lines.yml` did it again to
/// `failure-mode/orphaned-dialogue-branch`, and by the same accident: the phase was about counting
/// a defect, and the second source arrived because a shipped engine turned out to declare the
/// thing that would prevent it.
///
/// Not a defect on its own — a node about one shipped game legitimately cites one entry, and
/// `walkbox/scaled-polygon` is exactly that: five claims, all Delores, one link that earns them.
/// What the count is for is the other kind, and the two are indistinguishable from outside the
/// node.
#[test]
fn the_evidence_base_of_fifteen_nodes_is_a_single_source() {
    let root = repo_root();
    let (mut evidenced, mut one_entry, mut one_line, mut held, mut carried) = (0, 0, 0, 0, 0);
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let c = claims(&src);
        if c == 0 {
            continue;
        }
        evidenced += 1;
        let (entries, lines) = sources(&src);
        if entries.len() != 1 {
            continue;
        }
        one_entry += 1;
        held += c;
        one_line += usize::from(lines == 1);
        for para in src.split("\n\n") {
            if !para.contains("catalog/") && !para.contains("crates/") {
                carried += claims(para);
            }
        }
    }
    assert_eq!(
        (evidenced, one_entry, one_line),
        (40, 14, 10),
        "nodes with a verified claim; of those, resting on one catalog entry; of those, on one line"
    );
    assert_eq!(
        (held, carried),
        (86, 38),
        "claims those fourteen carry, and how many name nothing in their own paragraph"
    );
}

/// **Deleting one line makes a node unsourced, and the claims do not move.**
///
/// The destructive half, and the point of the phase. `evidence-kinds` found one added line
/// silencing a node for claims it had nothing to do with; this is the same sensitivity from the
/// other side. For every node whose whole evidence base is one citation, removing that citation
/// flips `verified-unsourced` from silent to loud — with the same claims, the same tags, and the
/// same tests behind them.
///
/// A lint that answers differently before and after a line about a different claim is answering a
/// question about the node, and the question everybody reads it as is about the claims.
#[test]
fn one_line_is_the_difference_between_sourced_and_not() {
    let root = repo_root();
    let (mut flipped, mut attributed) = (Vec::new(), Vec::new());
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if claims(&src) == 0 {
            continue;
        }
        let (_, lines) = sources(&src);
        if lines != 1 {
            continue;
        }
        // the lint fires on a verified claim and no catalog link at all
        let without: String = src
            .lines()
            .filter(|l| !l.contains("catalog/"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !without.contains("catalog/"),
            "{}: the simulated deletion did not remove the citation",
            name(&root, &p)
        );
        // Some nodes put the citation and the claim it supports on one line, so the deletion
        // takes a claim with it. That is the attribution working — those are the claims a checker
        // *can* place — and it is counted rather than excluded.
        if claims(&without) < claims(&src) {
            attributed.push(name(&root, &p));
        }
        flipped.push(name(&root, &p));
    }
    assert_eq!(flipped.len(), 10);
    assert_eq!(
        attributed,
        [
            "failure-mode/unwinnable-state.yml",
            "interaction/conversational-binding.yml",
            "interaction/unary-binding.yml",
            "inventory-item/stackable-item.yml",
            "prior-art/sierra-sci.yml",
            "walkbox/scaled-polygon.yml",
            "world-flag/boolean-flag.yml",
            "world-flag/counter-flag.yml",
        ],
        "of the twelve, the ones whose single citation shares a line with a claim — so deleting it \
         removes a claim as well as the source, which is the only case where a checker can see \
         which claim the line was for"
    );
    assert!(
        flipped.contains(&"walkbox/scaled-polygon.yml".to_string()),
        "the node whose single citation genuinely earns all five of its claims is in the same list \
         as the ones whose does not, which is the finding rather than an oversight"
    );
}

/// **Two single-source nodes are silenced by a citation for an unrelated claim and neither says so.**
///
/// `puzzle/state-gate` cites one entry — a designer's essay on puzzle dependency charts — and
/// rests two of its eight claims on committed tests. `interaction/unary-binding` cites one entry
/// and names `crates/` four times. Both are `boolean-flag`'s position exactly, and `boolean-flag`
/// wrote three paragraphs about being in it.
///
/// The difference is that `boolean-flag` had been **reported**. Four nodes in this corpus discuss
/// their own evidence standing and all four are nodes the lint named at some point; the ones that
/// have always carried a line have no account of their sourcing and are not better sourced.
/// `.yidam/decisions/carried-claims.yml`.
///
/// `failure-mode/orphaned-dialogue-branch` passed through this list without stopping.
/// `.yidam/decisions/orphaned-lines.yml` gave it its first test — it had cited one entry and named
/// none for eleven phases — and in the same phase a second catalog entry, so it left by the door
/// the list is for before anything could count it at the door it came in by.
#[test]
fn a_single_source_node_that_also_rests_on_the_instrument_is_named() {
    let root = repo_root();
    let mut both = Vec::new();
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if claims(&src) == 0 {
            continue;
        }
        let (entries, _) = sources(&src);
        if entries.len() == 1 && src.contains("crates/") {
            both.push(name(&root, &p));
        }
    }
    assert_eq!(
        both,
        [
            "failure-mode/unwinnable-state.yml",
            "hotspot/static-region.yml",
            "interaction/conversational-binding.yml",
            "interaction/unary-binding.yml",
            "inventory-item/portable-object.yml",
            "prior-art/sierra-sci.yml",
            "puzzle/state-gate.yml",
            "subsystem/verb-menu.yml",
            "world-flag/boolean-flag.yml",
            "world-flag/counter-flag.yml",
        ],
        "one document entry between them and the instrument behind the rest"
    );
}
