//! **The fifteen class pages are the only files in this corpus the root commit still owns.**
//!
//! Every class directory holds an `ACTIONS.md`. All fifteen were written in commit one and not
//! one of them has been opened since — 342 commits, thirty-odd phases, an ontology that gained a
//! property value and a class instance, and a checker that grew from nothing to four crates.
//!
//! That would be unremarkable for an operations list, which is what the prelude says the file is:
//! *"a list of operations meaningful for this class — queries that retrieve instances,
//! transitions an instance can undergo, skills that act on it, or calculators that derive from
//! it."* The queries held up. All twenty-nine still resolve, and the only one returning nothing is
//! the triple [`edges.rs`] already pins as declared-but-unauthored.
//!
//! What did not hold up is that the pages also carry **twelve open questions, in two spellings no
//! tool here reads** — `*Open:*` and `*Stub:*`, in Markdown, which [`markers.rs`] does not scan
//! because it scans `.yml`. Its headline ratio is computed on a denominator missing this third
//! place entirely.
//!
//! [`edges.rs`]: edges.rs
//! [`markers.rs`]: markers.rs

use design_gate::repo_root;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Every class page, as `(class, path)`, sorted by class.
fn pages(root: &Path) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(root.join(".yidam/corpus"))
        .expect("the corpus exists")
        .flatten()
    {
        let p = e.path().join("ACTIONS.md");
        if p.is_file() {
            let class = e.file_name().to_string_lossy().into_owned();
            out.push((class, p));
        }
    }
    out.sort();
    out
}

/// The out-edges a class declares, read from the `edges:` block of its `.ont.yml`.
///
/// The same reader [`edges.rs`] uses, and deliberately a copy: a test that shares a parser with
/// the thing it checks agrees with it by construction.
fn declared(root: &Path) -> BTreeMap<String, BTreeSet<(String, String)>> {
    let mut out: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
    for e in std::fs::read_dir(root.join(".yidam/corpus"))
        .expect("the corpus exists")
        .flatten()
    {
        let p = e.path();
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

/// Every `yidam query '<from> -<rel>-> <to>'` a page documents, in page order.
fn documented_queries(src: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for chunk in src.split("yidam query '").skip(1) {
        let Some(q) = chunk.split('\'').next() else {
            continue;
        };
        let Some((from, rest)) = q.split_once(" -") else {
            continue;
        };
        let Some((rel, to)) = rest.split_once("-> ") else {
            continue;
        };
        out.push((from.trim().into(), rel.trim().into(), to.trim().into()));
    }
    out
}

/// The two spellings a class page asks a question in, with the line each sits on.
///
/// Neither is `[open]`. `*Open:*` is a question and `*Stub:*` is a promise about a tool, and the
/// prelude's word for the second — *"stub entries are fine; this file grows over time"* — means a
/// stub **operation**, not a stub claim. Both are read here because both turned out to go stale,
/// and they went stale in opposite directions.
fn marks(src: &str) -> Vec<(usize, &'static str, String)> {
    src.lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let t = l.trim_start();
            let kind = if t.starts_with("- *Open:*") {
                "Open"
            } else if t.starts_with("- *Stub:*") {
                "Stub"
            } else {
                return None;
            };
            Some((i + 1, kind, t.to_string()))
        })
        .collect()
}

/// **Every query the pages document is a relationship the ontology declares.**
///
/// This is the half of these files that worked, and it is worth saying why it worked: a query is
/// written against the *class* vocabulary, which is the part of this corpus a phase changes only
/// by widening. Nothing here has ever removed a relationship, so a page naming one in commit one
/// still names one at commit 343 — and the two widenings since (`data-format -encodes-> world-flag`,
/// `interaction.shape`'s fourth value) added to what a page could say without invalidating what it
/// already said.
///
/// The check is the one that would have caught the other kind of drift, and this is the commit
/// that admits it was never run.
#[test]
fn every_query_a_class_page_documents_is_a_declared_triple() {
    let root = repo_root();
    let d = declared(&root);
    let (mut total, mut distinct, mut bad) = (0, BTreeSet::new(), Vec::new());
    for (class, path) in pages(&root) {
        let src = std::fs::read_to_string(&path).expect("readable");
        for (from, rel, to) in documented_queries(&src) {
            total += 1;
            distinct.insert((from.clone(), rel.clone(), to.clone()));
            let declares = d
                .get(&from)
                .is_some_and(|s| s.contains(&(rel.clone(), to.clone())));
            if !declares {
                bad.push(format!("{class}: `{from} -{rel}-> {to}` is not declared"));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a documented query names a relationship the ontology does not have:\n  {}",
        bad.join("\n  ")
    );
    assert_eq!(
        (total, distinct.len()),
        (37, 29),
        "37 documented queries, 29 of them distinct — the pages repeat each other's edges more \
         often than they repeat their own"
    );
}

/// **Twelve questions in two spellings, on pages nothing has reopened since the root commit.**
///
/// [`markers.rs`] reports *"7 open in the corpus, 28 in the records, and three quarters of what
/// this corpus owes is invisible to every tool it has."* The denominator is short by these
/// twelve. The visible share is not 20% but **7 of 47, fifteen percent** — and the twelve are the
/// least visible of the three populations, because the other two are at least `.yml` and at least
/// spelled the way the one existing check spells it.
///
/// [`markers.rs`]: markers.rs
#[test]
fn the_class_pages_ask_in_a_spelling_no_tool_reads() {
    let root = repo_root();
    let (mut open, mut stub, mut carrying) = (0, 0, Vec::new());
    for (class, path) in pages(&root) {
        let src = std::fs::read_to_string(&path).expect("readable");
        let m = marks(&src);
        let o = m.iter().filter(|(_, k, _)| *k == "Open").count();
        let s = m.iter().filter(|(_, k, _)| *k == "Stub").count();
        open += o;
        stub += s;
        if o + s > 0 {
            carrying.push(class);
        }
    }
    assert_eq!((open, stub), (8, 4));
    assert_eq!(
        carrying.len(),
        12,
        "twelve of fifteen pages carry one; the three that do not are the three with nothing left \
         to promise — data-format, prior-art, subsystem"
    );

    // markers.rs's ratio, restated on the denominator that includes this place.
    let (corpus, decisions) = (7, 28);
    assert_eq!((corpus * 100) / (corpus + decisions + open + stub), 14);
}

/// **Every class page's first commit is this repository's first commit, and its last is too.**
///
/// Which is the finding. These are not stale because somebody stopped maintaining them; they were
/// never maintained. The genesis author wrote an operations list, put questions in it because
/// there was nowhere else to put a question about a *class* — `.ont.yml` cannot carry a claim tag
/// and an instance node is the wrong scope — and every phase since has answered questions
/// somewhere else.
///
/// Five of the twelve have answers sitting in records written after the question. Two of those
/// records — `conversation-reduction` and `conversation-boundary` — are on `records.rs`'s orphan
/// list, cited by nothing. **That is one finding from both ends**: the file that asks the question
/// was never reopened, so the record that answered it was never cited.
#[test]
fn no_class_page_has_been_opened_since_the_root_commit() {
    let root = repo_root();
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(&root)
        .args(["rev-list", "--max-parents=0", "HEAD"])
        .output()
        .expect("git is on PATH");
    let genesis = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(
        genesis.len(),
        40,
        "the lifecycle check needs real history — a shallow clone has no root commit to compare \
         against, and reports that rather than passing"
    );

    let (mut untouched, mut wrong_origin) = (0, Vec::new());
    for (class, path) in pages(&root) {
        let rel = path
            .strip_prefix(&root)
            .expect("under the root")
            .to_string_lossy()
            .into_owned();
        let log = std::process::Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["log", "--format=%H", "--"])
            .arg(&rel)
            .output()
            .expect("git is on PATH");
        let commits: Vec<String> = String::from_utf8_lossy(&log.stdout)
            .lines()
            .map(str::to_string)
            .collect();
        match commits.last() {
            Some(first) if *first == genesis => {}
            other => wrong_origin.push(format!("{class}: written by {other:?}, not the root")),
        }
        untouched += usize::from(commits.len() == 1);
    }
    assert!(
        wrong_origin.is_empty(),
        "a class page not written at genesis:\n  {}",
        wrong_origin.join("\n  ")
    );
    assert_eq!(
        untouched, 15,
        "all fifteen, and the number is pinned so that reopening one is visible as a change here"
    );
}
