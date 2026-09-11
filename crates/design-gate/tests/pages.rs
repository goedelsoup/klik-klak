//! **The fifteen class pages were the only files in this corpus the root commit still owned.**
//!
//! Every class directory holds an `ACTIONS.md`. All fifteen were written in commit one and not one
//! had been opened since — 342 commits, thirty-odd phases, an ontology that gained a property
//! value and a class instance, and a checker that grew from nothing to four crates.
//!
//! That would be unremarkable for an operations list, which is what the prelude says the file is:
//! *"a list of operations meaningful for this class — queries that retrieve instances,
//! transitions an instance can undergo, skills that act on it, or calculators that derive from
//! it."* The queries held up. Every one still resolves, and the only one returning nothing is the
//! triple [`edges.rs`] already pins as declared-but-unauthored.
//!
//! What did not hold up is that the pages also carried **twelve open questions, in two spellings
//! no tool here reads** — `*Open:*` and `*Stub:*`, in Markdown, which [`markers.rs`] does not scan
//! because it scans `.yml`. Ten of the twelve were wrong. `.yidam/decisions/action-pages.yml` is
//! the disposition: a page holds operations, the five that were answered became pointers at what
//! answered them, and the two that were live became `[open]` on an instance where a tool can see
//! them. A sixth was live already — a duplicate of the marker on `actor/player-character`, one
//! file away in the same directory, which that record first read as answered.
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

/// **Every path a class page writes resolves.**
///
/// New, and it is what the pages now need that they did not before. They used to assert; they now
/// cite — six of the retired questions became pointers at the record or node that answered them —
/// and a pointer is the kind of claim that rots silently. `edges.rs` checks this for corpus nodes
/// and `records.rs` for decision records; between them they covered every file in `.yidam/` except
/// these fifteen.
///
/// Three bases, because the pages already use two of them: a path relative to the page's own
/// directory, and one relative to the repository root.
#[test]
fn every_path_a_class_page_writes_resolves() {
    let root = repo_root();
    let (mut cites, mut bad) = (0, Vec::new());
    for (class, path) in pages(&root) {
        let src = std::fs::read_to_string(&path).expect("readable");
        let dir = path.parent().expect("a directory").to_path_buf();
        let mut refs: Vec<String> = Vec::new();
        for chunk in src.split("](").skip(1) {
            if let Some(t) = chunk.split(')').next() {
                refs.push(t.to_string());
            }
        }
        for chunk in src.split('`').skip(1).step_by(2) {
            let t = chunk.trim();
            if t.starts_with(".yidam/") || t.starts_with("design/") || t.starts_with("crates/") {
                refs.push(t.to_string());
            }
        }
        for r in refs {
            let r = r.split('#').next().unwrap_or("");
            if r.is_empty() || r.starts_with("http") {
                continue;
            }
            cites += 1;
            if !dir.join(r).exists() && !root.join(r).exists() {
                bad.push(format!("{class}: {r}"));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a class page points at something that is not there:\n  {}",
        bad.join("\n  ")
    );
    assert_eq!(cites, 30, "paths written across the fifteen pages");
}

/// **No question lives on a class page, and the two stubs left are promises rather than questions.**
///
/// It was eight and four. Five of the eight had been answered by a later phase, two were live and
/// are now `[open]` on an instance, one was a duplicate of a marker already on an instance, and
/// two of the four stubs promised `reachability` in the conditional — *"given a game to read"*, *"it has nothing to read"* — after it shipped and read
/// four. A repository gets no warning when a promise it made comes true.
///
/// The two that remain are honest: `collapse-signature` and `vocabulary-coverage` are still
/// unimplemented, and `.yidam/skills/README.md` is the place that tracks them.
///
/// The ratio moves **up** for the first time. [`markers.rs`] has watched the visible share fall a
/// point a phase — 23, 22, 21, 20 — because a phase closes nothing in the corpus and opens one in
/// the record that settles it. Counted honestly against this third place it was 14%. Two questions
/// that no tool could see are now two questions `yidam open-questions` returns.
///
/// [`markers.rs`]: markers.rs
#[test]
fn no_question_lives_on_a_class_page() {
    let root = repo_root();
    let (mut open, mut stub, mut asking) = (0, 0, Vec::new());
    for (class, path) in pages(&root) {
        let src = std::fs::read_to_string(&path).expect("readable");
        let m = marks(&src);
        let o = m.iter().filter(|(_, k, _)| *k == "Open").count();
        stub += m.iter().filter(|(_, k, _)| *k == "Stub").count();
        open += o;
        if o > 0 {
            asking.push(class);
        }
    }
    assert!(
        asking.is_empty(),
        "a class page holds operations. A question about a class goes in a decision record, or \
         `[open]` on an instance where `yidam open-questions` can see it:\n  {}",
        asking.join("\n  ")
    );
    assert_eq!(
        (open, stub),
        (0, 2),
        "was (8, 4) — `.yidam/decisions/action-pages.yml` is the disposition of the other ten"
    );

    // markers.rs's ratio, restated on the denominator that includes this place — and counted
    // rather than repeated, because a literal here would go stale the next time a phase opens a
    // question and nothing would say so.
    let live = |dir: &str, ends: bool| -> usize {
        let mut n = 0;
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "yml") {
                    let src = std::fs::read_to_string(&p).expect("readable");
                    n += src
                        .lines()
                        .enumerate()
                        .filter(|(i, l)| {
                            l.contains("[open]")
                                && (!ends
                                    || (l.trim_end().ends_with("`[open]`")
                                        && src
                                            .lines()
                                            .nth(i + 1)
                                            .is_none_or(|x| x.trim().is_empty())))
                        })
                        .count();
                }
            }
        }
        n
    };
    let (corpus, decisions) = (live(".yidam/corpus", false), live(".yidam/decisions", true));
    assert_eq!((corpus, decisions), (9, 36));
    assert_eq!((corpus * 100) / (corpus + decisions + open + stub), 19);
}

/// **Every class page was written by this repository's first commit, and for 342 commits its last
/// was too.**
///
/// Which is the finding. These were not stale because somebody stopped maintaining them; they were
/// never maintained. The genesis author wrote an operations list, put questions in it because
/// there was nowhere else to put a question about a *class* — `.ont.yml` cannot carry a claim tag
/// and an instance node is the wrong scope — and every phase since answered questions somewhere
/// else.
///
/// Five of the twelve had answers sitting in records written after the question. Two of those
/// records — `conversation-reduction` and `conversation-boundary` — were on `records.rs`'s orphan
/// list, cited by nothing. **That is one finding from both ends**: the file asking the question was
/// never reopened, so the record that answered it was never cited. Citing them from
/// `.yidam/decisions/action-pages.yml` took the orphan list from ten to eight.
///
/// The origin assertion is permanent; the untouched count is a census that this phase moved from
/// fifteen to four, and the four are the pages that had nothing wrong with them.
#[test]
fn every_class_page_was_written_by_the_root_commit() {
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

        // Compared by content rather than by commit count, which is the second time in one phase
        // that a git-backed check had to stop asking git a question the working tree answers:
        // `records.rs` could not see an uncommitted record, and counting commits here could not
        // see the edit that changes the answer until after it was committed. A page is untouched
        // when what is on disk is what the root commit wrote.
        let show = std::process::Command::new("git")
            .arg("-C")
            .arg(&root)
            .arg("show")
            .arg(format!("{genesis}:{rel}"))
            .output()
            .expect("git is on PATH");
        let then = String::from_utf8_lossy(&show.stdout).into_owned();
        let now = std::fs::read_to_string(&path).expect("readable");
        untouched += usize::from(show.status.success() && then == now);
    }
    assert!(
        wrong_origin.is_empty(),
        "a class page not written at genesis:\n  {}",
        wrong_origin.join("\n  ")
    );
    assert_eq!(
        untouched, 4,
        "was fifteen. The four still untouched are the pages that carried nothing wrong — \
         data-format, hotspot, prior-art, subsystem"
    );
}
