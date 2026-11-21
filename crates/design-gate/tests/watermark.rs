//! **The watermark was put on the document somebody was reading, and there are two.**
//!
//! `moved-ground` found nine of fourteen premises moved under
//! `design/bounded-expression-language.md`, added `reconciled:` to its front matter, and taught
//! `design_gate::check` to fail when a premise moves past it. That is the instrument working.
//!
//! `design/specimen/README.md` is the other design document. It declares **eighteen** `rests_on`
//! premises — four more than the document that got the watermark — and **thirteen of them have
//! moved**, by +425 lines and −41, across 148 commits. It carries no `reconciled:` and the gate
//! has never said a word about it.
//!
//! **The check is opt-in.** `check` reads the watermark under `if let Some(at)`, so a document
//! that declares none is never asked, and the documents that declare one are the documents whose
//! authors were thinking about drift. That is `carried-claims`'s selection effect — *"writing
//! about your own footing has required being reported once"* — arriving in the gate rather than
//! in the corpus.
//!
//! One of the thirteen is `bounded-expression-language.md` itself, which means the specimen
//! document's ground includes a document whose own ground is watermarked, and the watermark did
//! not travel up the chain.
//!
//! And the drift was not hypothetical. The document said **"Thirty-eight tests"** of the fixtures
//! it describes; there are **158**. It listed four constructs as unexercised *"across both games"*
//! when there are four games and `coverage.rs` counts six. It carried an open question `cutscene`
//! had closed two phases earlier.
//!
//! **The watermark is required now rather than honoured where offered**, and both documents carry
//! one. `.yidam/decisions/second-document.yml`.
//!
//! **What it cannot tell, found by using it.** The mark is a commit count, so *any* commit touching
//! a premise trips it. `engine-home` added one `used-by:` line to four catalog entries — a reverse
//! index, generated content in all but name, changing nothing a design rests on — and both
//! documents came up for reconciliation. The reading took a minute and found nothing, which is the
//! right outcome and not a free one.
//!
//! It is left as it is. Teaching the check to skip front matter would make it skip `retrieved:`
//! and `ttl_days:` too, and a source's expiry is exactly the kind of premise movement a design
//! should be read against. `marker-count`'s rule applies — **make the record truthful rather than
//! ask the tool to be cleverer** — so the price is a reconciliation that sometimes finds nothing,
//! and the alternative is a check that sometimes misses something.
//!
//! **And a document resting on a document takes its watermark one commit later.**
//! `design/specimen/README.md` rests on `bounded-expression-language.md`, so moving the upstream
//! mark *is* a commit touching the downstream premise, and reconciling both in one commit leaves
//! the second red. `second-document` predicted the missing direction of this — a premise of a
//! premise moving is invisible to both marks — and the visible direction is the same structure
//! from the other side. The rule that follows is small and worth stating rather than rediscovering:
//! **the downstream document's mark names a commit after the upstream document's mark moved.**

use design_gate::{front_matter, repo_root};
use std::path::{Path, PathBuf};
use std::process::Command;

const WATERMARKED: &str = "design/bounded-expression-language.md";
const BARE: &str = "design/specimen/README.md";

/// The specimen document's last commit before this phase. A closed window, anchored by subject
/// rather than by `git log -1 --` on the file, because this phase's own revise edits that file —
/// which is the mistake `premises.rs` made and had to be repaired for.
const BEFORE: &str = "^revise: the class was founded twice on the wrong thing";

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

/// Every document the gate reads as a design — `design/**/*.md` less the directory's contract.
fn designs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.join("design")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "md") && p != root.join("design/README.md")
            {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn premises(root: &Path, doc: &str) -> Vec<String> {
    let src = std::fs::read_to_string(root.join(doc)).expect("readable");
    front_matter(&src)
        .expect("front matter")
        .list("rests_on")
        .to_vec()
}

/// **Every design document records the commit its premises were last read against.**
///
/// It was one of two, and not because the instrument could not reach the other — the same `check`,
/// the same front matter, the same `rests_on` list read by the same call. What decided which one
/// got it is which one the phase was reading, which is a population a check must never be allowed
/// to choose.
#[test]
fn every_design_document_records_what_it_was_last_read_against() {
    let root = repo_root();
    let all = designs(&root);
    let expected = [WATERMARKED, "design/graphical-client.md", BARE];
    assert_eq!(
        all.iter()
            .map(|p| p
                .strip_prefix(&root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned())
            .collect::<Vec<_>>(),
        expected,
        "the documents the gate reads as designs"
    );

    for doc in expected {
        let src = std::fs::read_to_string(root.join(doc)).expect("readable");
        let fm = front_matter(&src).expect("front matter");
        let at = fm
            .scalar("reconciled")
            .unwrap_or_else(|| panic!("{doc} names no commit its premises were last read against"));
        assert_eq!(at.len(), 40, "{doc}: a full commit sha: {at}");
        assert_eq!(
            git(&root, &["cat-file", "-t", at]),
            "commit",
            "{doc}: and it is a commit in this history"
        );
        for premise in fm.list("rests_on") {
            let moved: usize = git(
                &root,
                &["rev-list", "--count", &format!("{at}..HEAD"), "--", premise],
            )
            .parse()
            .expect("a count");
            assert_eq!(
                moved, 0,
                "{doc}: {premise} moved since the watermark; the gate says the same and this says why"
            );
        }
    }
}

/// **The document without the watermark rests on more and has drifted further.**
///
/// Nineteen premises against fifteen, sixteen moved against eleven, +425/−41 against +410/−22,
/// 148 commits against 102. Worse on every measure the phase that built the instrument used to
/// argue the instrument was needed.
///
/// **Only the near end of the window is fixed.** The far end is HEAD, so the count of premises
/// that have moved rises whenever a phase edits one of the eighteen, and this assertion is
/// re-pinned rather than held. It went to fourteen when `authored-text` took a needless escape out
/// of `ferry.game`, and to fifteen when `calculators` retired `vocabulary-coverage` — a premise
/// that moved because the thing it described stopped existing, which is a kind of drift this
/// measure cannot tell from any other. Both documents gained `actor/player-character` when
/// `control-transfer` found it had been a premise of each for eleven phases without being
/// declared, so the two counts rose together and the gap between them did not move. What the check is for is the comparison against the
/// watermarked document, not the number — and the comparison has widened, which is the direction
/// the finding predicted.
#[test]
fn the_second_document_rests_on_more_and_has_drifted_further() {
    let root = repo_root();
    let found = git(&root, &["log", "--format=%H", "--grep", BEFORE]);
    let last = found
        .lines()
        .next()
        .expect("the anchor commit is gone, so this window cannot be measured")
        .to_string();

    let all = premises(&root, BARE);
    assert_eq!(all.len(), 19, "premises the specimen document declares");
    assert_eq!(
        premises(&root, WATERMARKED).len(),
        15,
        "against the document that got the watermark"
    );

    let mut moved = Vec::new();
    let (mut added, mut removed) = (0usize, 0usize);
    for p in &all {
        let n: usize = git(
            &root,
            &["rev-list", "--count", &format!("{last}..HEAD"), "--", p],
        )
        .parse()
        .expect("a count");
        if n == 0 {
            continue;
        }
        moved.push(p.clone());
        for line in git(
            &root,
            &["diff", "--numstat", &format!("{last}..HEAD"), "--", p],
        )
        .lines()
        {
            let mut f = line.split('\t');
            added += f.next().and_then(|x| x.parse::<usize>().ok()).unwrap_or(0);
            removed += f.next().and_then(|x| x.parse::<usize>().ok()).unwrap_or(0);
        }
    }
    assert_eq!(
        moved.len(),
        16,
        "premises edited since this document was: {moved:?}"
    );
    assert!(
        moved.iter().any(|p| p == WATERMARKED),
        "one of them is the watermarked document, so the specimen's ground includes ground that \
         is itself watched — and the watermark did not travel up the chain"
    );
    assert!(
        added >= 425 && removed >= 41,
        "lines moved under the second document: +{added}/-{removed}"
    );
    assert!(
        git(&root, &["rev-list", "--count", &format!("{last}..HEAD")])
            .parse::<usize>()
            .expect("a count")
            >= 148,
        "commits since the second document was last edited"
    );
}

/// **A document that declares no watermark fails, verified against a document that does not.**
///
/// `check` read the mark under `if let Some(at)`, so silence was what a document got for declaring
/// none — and the population that survived was the one whose author had thought about drift. That
/// is `carried-claims`'s selection effect arriving in the gate, and the fix is a `match` with a
/// `None` arm rather than a stricter comparison.
///
/// The rest is what the silence cost: a test count four times under, and a coverage list written
/// when there were two specimens and never revisited when there were four. Both are derived here
/// instead of stated there.
#[test]
fn a_document_that_declares_no_watermark_fails() {
    let root = repo_root();
    // A design document with front matter and no `reconciled:`, in a root of its own, so the
    // finding is produced by `check` rather than asserted about its source.
    let tmp = std::env::temp_dir().join(format!("design-gate-watermark-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(tmp.join("design")).expect("a temp root");
    std::fs::write(
        tmp.join("design/bare.md"),
        "---\ntitle: a design that names no baseline\nstatus: proposal\nrests_on:\n  - .yidam/corpus/verb.ont.yml\n---\n\nnothing here.\n",
    )
    .expect("writable");
    let report = design_gate::check(&tmp).report();
    std::fs::remove_dir_all(&tmp).expect("removable");
    assert!(
        report.contains("declares no reconciled:"),
        "a design naming no baseline is a finding:\n{report}"
    );

    let doc = std::fs::read_to_string(root.join(BARE)).expect("readable");
    assert!(
        !doc.contains("Thirty-eight tests"),
        "the figure the document stated about its own fixtures, derived nowhere"
    );
    let actual = ["crates/reachability/tests", "crates/game-format/tests"]
        .iter()
        .map(|d| {
            std::fs::read_dir(root.join(d))
                .expect("readable")
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "rs"))
                .map(|p| {
                    std::fs::read_to_string(p)
                        .expect("readable")
                        .matches("\n#[test]")
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    assert_eq!(
        actual, 220,
        "the fixtures the document describes carry this many tests"
    );
    assert!(
        doc.contains(&format!("**{actual} tests**")),
        "and the document states the figure this check derives"
    );

    assert!(
        !doc.contains("Unexercised across both games"),
        "a coverage list written at two specimens and never revisited at five"
    );
    assert!(
        doc.contains("Unexercised across all six games: **none.**"),
        "the list is over every game now, and it is empty"
    );
    let coverage = std::fs::read_to_string(root.join("crates/game-format/tests/coverage.rs"))
        .expect("readable");
    assert!(
        coverage.contains("never.is_empty()"),
        "and the check that counts them over all six holds the list at empty"
    );
}
