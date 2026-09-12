//! **The corpus has a second citation graph, and nothing reads it.**
//!
//! [`markers.rs`] found that three quarters of this corpus's open questions sit where no tool
//! looks. This is the same finding on the other axis. A node cites another node in two ways: an
//! entry in its `links:` block, which is an edge and which every tool reads, and a reference in
//! its prose, which is not and which none of them do. The second is three quarters the size of
//! the first.
//!
//! The consequence is visible in `yidam lint`, which reports four nodes as *"nothing links to
//! this node"*. The sentence is false about every one of them and right about all four:
//! `interaction/entry-binding` is quoted at length in four decision records and six corpus nodes.
//! It is not uncited. It is **un-edged**, which is a different defect with the same remedy only
//! sometimes — and this file exists to say which times.
//!
//! [`markers.rs`]: markers.rs

use design_gate::repo_root;
use std::collections::{BTreeMap, BTreeSet};
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

/// A node is a `.yml` under `.yidam/corpus/` that is not a class definition.
fn nodes(root: &Path) -> Vec<PathBuf> {
    yml(&root.join(".yidam/corpus"))
        .into_iter()
        .filter(|p| !p.to_string_lossy().ends_with(".ont.yml"))
        .collect()
}

fn name(root: &Path, p: &Path) -> String {
    p.strip_prefix(root.join(".yidam/corpus"))
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

/// A node's prose is everything above its `links:` block.
fn prose(src: &str) -> &str {
    match src.find("\nlinks:") {
        Some(i) => &src[..i],
        None => src,
    }
}

/// Every `.yml` path a node's prose names, in any of the spellings the corpus writes.
///
/// There are four — a markdown link, a backticked path, a backticked bare filename, and a
/// root-relative path sitting bare in a sentence — and they are read against three bases. That
/// is the whole reason this graph is unreadable: it was never given a form, so it grew one per
/// author-moment. None of them is wrong and no tool could have been written against all four
/// without this file first counting them.
fn references(src: &str) -> BTreeSet<String> {
    let body = prose(src);
    let mut out = BTreeSet::new();
    let mut push = |s: &str| {
        let s = s.split('#').next().unwrap_or("");
        if s.ends_with(".yml") {
            out.insert(s.to_string());
        }
    };
    // `](path)` — a markdown link
    let b: Vec<char> = body.chars().collect();
    for i in 0..b.len().saturating_sub(1) {
        if b[i] == ']' && b[i + 1] == '(' {
            let t: String = b[i + 2..]
                .iter()
                .take_while(|c| **c != ')' && !c.is_whitespace())
                .collect();
            push(&t);
        }
    }
    // `` `path` `` and a bare root-relative path
    for chunk in body.split('`').skip(1).step_by(2) {
        push(chunk.trim());
    }
    for w in body.split(|c: char| c.is_whitespace() || "()[],;".contains(c)) {
        if w.starts_with(".yidam/") {
            push(w.trim_end_matches('.'));
        }
    }
    out
}

/// Where a reference resolves from: its own directory, the corpus root, or the repository root.
fn resolve(root: &Path, from: &Path, r: &str) -> Option<PathBuf> {
    [
        from.parent().unwrap_or(root).join(r),
        root.join(".yidam/corpus").join(r),
        root.join(r),
    ]
    .into_iter()
    .find(|p| p.exists())
    .map(|p| p.canonicalize().unwrap_or(p))
}

/// The declared graph: pairs of nodes joined by a `links:` entry.
fn edges(root: &Path) -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for p in nodes(root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let Some(i) = src.find("\nlinks:") else {
            continue;
        };
        for line in src[i..].lines() {
            let Some(t) = line.trim().strip_prefix("- target:") else {
                continue;
            };
            let t = t.trim();
            if t.ends_with(".ont.yml") {
                continue;
            }
            if let Some(q) = resolve(root, &p, t) {
                out.insert((name(root, &p), name(root, &q)));
            }
        }
    }
    out
}

/// The undeclared graph: pairs of nodes joined by a sentence.
fn citations(root: &Path) -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for p in nodes(root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        for r in references(&src) {
            let Some(q) = resolve(root, &p, &r) else {
                continue;
            };
            if q.to_string_lossy().ends_with(".ont.yml") {
                continue;
            }
            if !q.starts_with(root.join(".yidam/corpus")) {
                continue;
            }
            if q != p.canonicalize().unwrap_or(p.clone()) {
                out.insert((name(root, &p), name(root, &q)));
            }
        }
    }
    out
}

/// class -> the `(relationship, target class)` pairs its `.ont.yml` declares as out-edges.
fn declared(root: &Path) -> BTreeMap<String, BTreeSet<(String, String)>> {
    let mut out: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
    for p in yml(&root.join(".yidam/corpus")) {
        let s = p.to_string_lossy().into_owned();
        let Some(class) = s
            .strip_suffix(".ont.yml")
            .map(|c| c.rsplit('/').next().expect("a file name").to_string())
        else {
            continue;
        };
        let src = std::fs::read_to_string(&p).expect("readable");
        let Some(i) = src.find("\nedges:") else {
            continue;
        };
        let (mut rel, mut target, mut dir) = (None, None, "out".to_string());
        let mut seen = BTreeSet::new();
        for line in src[i + 1..].lines().skip(1) {
            if !line.starts_with(' ') && !line.trim().is_empty() {
                break;
            }
            if let Some(v) = line.trim().strip_prefix("- relationship:") {
                if let (Some(r), Some(t)) = (rel.take(), target.take()) {
                    if dir == "out" {
                        seen.insert((r, t));
                    }
                }
                dir = "out".into();
                rel = Some(v.trim().to_string());
            } else if let Some(v) = line.trim().strip_prefix("target:") {
                target = Some(v.trim().to_string());
            } else if let Some(v) = line.trim().strip_prefix("direction:") {
                dir = v.trim().to_string();
            }
        }
        if let (Some(r), Some(t)) = (rel, target) {
            if dir == "out" {
                seen.insert((r, t));
            }
        }
        out.insert(class, seen);
    }
    out
}

fn class_of(root: &Path) -> BTreeMap<String, String> {
    nodes(root)
        .into_iter()
        .map(|p| {
            let src = std::fs::read_to_string(&p).expect("readable");
            let c = src
                .lines()
                .find_map(|l| l.strip_prefix("class:"))
                .expect("every node declares a class")
                .trim()
                .to_string();
            (name(root, &p), c)
        })
        .collect()
}

/// **Nothing in the corpus checks that a path written in prose points at anything.**
///
/// `design-gate` has resolved every link in `design/` since it was a shell script, and the corpus
/// — the thing `design/` derives *from* — has never had the same check. 215 references across 43
/// nodes and 12 class definitions, and the reason none of them is broken is luck plus a small
/// number of hands. The first thing it caught was a reference written by the commit that added it,
/// to a decision record one commit in the future.
///
/// **The three bases are why no tool was ever written for it.** A reference resolves from the
/// node's own directory (105), from the repository root (105), or from `.yidam/corpus/` (5), and
/// the first two split the corpus nearly in half: `../verb/walk-to.yml` beside
/// `.yidam/decisions/found-in.yml` in the same paragraph. A checker written against either one
/// reports about half the corpus as rot, which is a worse report than none and is presumably why
/// nobody shipped it.
///
/// **And more than half of what the corpus points at is not in the corpus.** 112 of the 215 land
/// in `.yidam/decisions/`, 92 on another node, 11 on a class definition. The share has grown in
/// every phase since this was first counted, which is the migration continuing under the test
/// that measures it. `markers.rs` found the
/// open questions migrating into the decision records where no tool counts them; this is the
/// citations doing it, and it is the larger half.
#[test]
fn every_path_written_in_corpus_prose_resolves() {
    let root = repo_root();
    let mut bad = Vec::new();
    let (mut own, mut repo, mut corpus, mut decisions) = (0, 0, 0, 0);
    for p in yml(&root.join(".yidam/corpus")) {
        let src = std::fs::read_to_string(&p).expect("readable");
        for r in references(&src) {
            if r.contains("/decisions/") {
                decisions += 1;
            }
            match resolve(&root, &p, &r) {
                None => bad.push(format!("{}  ->  {r}", name(&root, &p))),
                Some(_) if p.parent().unwrap_or(&root).join(&r).exists() => own += 1,
                Some(_) if root.join(".yidam/corpus").join(&r).exists() => corpus += 1,
                Some(_) => repo += 1,
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a path written in prose must point at something:\n  {}",
        bad.join("\n  ")
    );
    assert_eq!(
        (own, repo, corpus),
        (135, 135, 5),
        "the three bases, all load-bearing"
    );
    assert_eq!((own + repo + corpus, decisions), (275, 155));
}

/// **99 declared edges, 97 prose citations, and 18 pairs in both.**
///
/// So **79 pairs of nodes are joined by a sentence and by nothing a tool can read**, against 81
/// joined by an edge and not by a sentence. Two graphs of nearly the same size over the same 44
/// nodes, overlapping by a sixth — and one of them is the corpus.
///
/// This heading said `(96, 86, 18)` while the assertion below said `(99, 93, 18)`: the pin was
/// re-pinned and the sentence explaining it was not, which is the failure mode this whole crate
/// exists to catch happening inside the crate. The derived figures — 68 and 78 — were arithmetic
/// on the stale pair and were therefore wrong twice over.
///
/// It was (88, 76, 14) when this file was written. The two the reading below found are the
/// difference, and they are the only two it found in sixty-two.
///
/// The number is held here rather than fixed, for the reason `markers.rs` holds its own: nothing
/// can tell that a sentence *should* have been an edge, short of reading. What a pinned count
/// does is make the divergence move visibly when somebody adds to either side.
#[test]
fn the_graph_the_tools_read_is_not_the_graph_the_corpus_wrote() {
    let root = repo_root();
    let (e, c) = (edges(&root), citations(&root));
    let both = e.intersection(&c).count();
    assert_eq!((e.len(), c.len(), both), (99, 98, 18));
}

/// **Three nodes have no incoming edge, and every one of them is cited in prose.**
///
/// It was five. Two were defects with remedies and both are fixed: `data-format/dialogue-script`
/// was owed a `reads` edge by the node that calls it *"this phase's largest finding"*, and
/// `interaction/entry-binding` was owed `subsystem -realizes->` by a component that did not exist
/// until somebody asked what such a component does — `.yidam/decisions/arrival-realizer.yml`.
/// That second one is the condition this file set for writing an available edge, met: a node says
/// so now, and the node had to be read against a shipped engine before it could.
///
/// **The other three cannot be fixed the same way, and the reasons are different.**
///
/// * `cutscene/scripted-sequence` — **its class has no incoming relationship at all.** `cutscene`
///   is the only one in this ontology that has none; every other class has somewhere it can be
///   pointed at from. `design/bounded-expression-language.md` refuses the obvious mirror,
///   `interaction -enters-> cutscene`, *"until a game needs one"*, so this orphan is a design
///   decision showing through the graph rather than an oversight.
/// * `prior-art/thimbleweed-park` — `prior-art` can be reached only by `verb -originated-in->`,
///   and no verb in this corpus originated in a 2017 game that inherited them. The class points
///   outward by construction and this instance can receive nothing.
/// * `world-flag/object-state` — `interaction -sets-> world-flag` is available and authored, to
///   `boolean-flag` and `counter-flag`, and this is the dominant form of state in the one game
///   whose source this corpus can read. No binding here sets one because the node declines to
///   propose it: it is *"a record of what a real system keeps and why, and not a primitive
///   proposed here."* A kept refusal is invisible in the graph unless somebody else shipped the
///   thing, which is why `give-to` and `walk-to`, kept on the same grounds, are not orphans.
///
/// They are named rather than counted because a fourth appearing is the event worth failing on.
#[test]
fn the_nodes_nothing_points_at_are_pointed_at_in_prose() {
    let root = repo_root();
    let (e, c) = (edges(&root), citations(&root));
    let mut orphans: Vec<String> = nodes(&root)
        .iter()
        .map(|p| name(&root, p))
        .filter(|n| !e.iter().any(|(_, t)| t == n))
        .collect();
    orphans.sort();
    assert_eq!(
        orphans,
        [
            "cutscene/scripted-sequence.yml",
            "prior-art/thimbleweed-park.yml",
            "world-flag/object-state.yml",
        ]
    );
    for o in &orphans {
        assert!(
            c.iter().any(|(_, t)| t == o),
            "{o} has no edge and no citation either — that is a real orphan and a different finding"
        );
    }
}

/// **A relationship being available is not evidence that an edge is missing. It is 13% of one.**
///
/// Of the 68 citations with no edge, **14 join two classes the ontology already gives a
/// relationship for** — `prior-art -reads-> data-format`, `verb -subsumed-by-> verb`,
/// `interaction -sets-> world-flag`. That looks like a rule waiting to be written. Reading all
/// fifteen, it is not:
///
/// * **Three were edges that should exist** and are edges now. Two were sentences that should
///   have been links — `prior-art/ags -reads->
///   interaction-table`, whose 48-action table that node quotes from AGS's own source, and
///   `thimbleweed-park -reads-> dialogue-script`, which is Yack and is that node's *"largest
///   finding"*. Both citing nodes already author `reads` elsewhere in the same block, so neither
///   was a missing vocabulary or a missing habit; they were written on a day when the author was
///   thinking about the paragraph. The third is different in kind and took a phase:
///   `interaction-resolver -realizes-> entry-binding` was unwritten because **nothing realized
///   arrival**, and what made it writable was reading what such a component does in a shipped
///   engine and finding that 161 of its 169 statements have no counterpart here.
/// * **Two are the inverse of an edge that already exists.** `interaction-resolver -depends-on->
///   verb-menu` is written; the citation runs the other way and adding it would make a cycle out
///   of a direction the ontology chose.
/// * **Two the ontology forbids.** `interaction -applies-> verb` is declared *"mutually exclusive
///   with `triggered-by`"*, which is the one edge `entry-binding` has; and an interaction table
///   keyed by verb and target is defined by not indexing arrival.
/// * **One is denied by the sentence making it.** `walk-to` cites `give-to` to say *"nothing here
///   collapses into anything"* — the citation is there to refuse the relationship.
/// * **Eight are pointers to an argument**, which is what most prose citation is and what no
///   relationship in this ontology means.
///
/// So a check that wrote an edge wherever the vocabulary had one available would have written 13
/// false edges, two of them into a mutual exclusion. The count is pinned; the reading is not
/// mechanisable and is recorded in `.yidam/decisions/prose-edges.yml`.
///
/// **And reading one for whether it should be an edge is not reading it for whether it is true.**
/// `prose-edges` quotes its own first example of the eight verbatim — *"the node still carrying
/// the warning"* — classified it correctly as a pointer rather than an owed edge, and had no
/// occasion to notice that the pointer named a node that had stopped qualifying four phases
/// earlier. `.yidam/decisions/evidence-kinds.yml` found that and retargeted it, which is why the
/// availability count moved: two such citations left and one arrived.
#[test]
fn the_vocabulary_had_a_relationship_available_and_was_right_twice() {
    let root = repo_root();
    let (e, c, d, k) = (
        edges(&root),
        citations(&root),
        declared(&root),
        class_of(&root),
    );
    let mut available = 0;
    for (s, t) in c.difference(&e) {
        let (cs, ct) = (&k[s], &k[t]);
        if d[cs].iter().any(|(_, target)| target == ct) {
            available += 1;
        }
    }
    assert_eq!((c.difference(&e).count(), available), (80, 19));
}

/// **A declared relationship no instance authors: the design document says two and there is one.**
///
/// `design/bounded-expression-language.md` refuses `interaction -enters-> cutscene` on the
/// grounds that *"a declared relationship no instance authors is a defect this corpus already
/// carries two of."* Counted as class-to-class triples rather than as relationship names, 43 are
/// declared and 42 are authored. The survivor is `failure-mode -prevented-by-> subsystem`: a
/// failure mode prevented by a written *form* is authored once, and no node claims an engine
/// component makes one unreachable.
///
/// The figure was right when written and nothing told anybody it had moved, which is the shape
/// `.yidam/decisions/stale-markers.yml` named and the reason this is a test rather than a
/// sentence.
#[test]
fn one_declared_relationship_has_no_instance() {
    let root = repo_root();
    let (e, d, k) = (edges(&root), declared(&root), class_of(&root));
    let mut unauthored = Vec::new();
    for (class, rels) in &d {
        for (rel, target) in rels {
            let used = e.iter().any(|(s, t)| {
                k.get(s).is_some_and(|c| c == class) && k.get(t).is_some_and(|c| c == target)
            });
            if !used {
                unauthored.push(format!("{class} -{rel}-> {target}"));
            }
        }
    }
    let declared_triples: usize = d.values().map(|v| v.len()).sum();
    assert_eq!(declared_triples, 44);
    assert_eq!(unauthored, ["failure-mode -prevented-by-> subsystem"]);
}
