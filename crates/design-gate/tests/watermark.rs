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
//! And the drift is not hypothetical. The document says **"Thirty-eight tests"** of the fixtures
//! it describes; there are **158**. It lists four constructs as unexercised *"across both games"*
//! when there are four games and `coverage.rs` counts six.

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

/// **Two design documents, one watermark.**
///
/// Not a document the instrument could not reach — the same `check`, the same front matter, the
/// same `rests_on` list read by the same call. What decided which one got it is which one the
/// phase was reading.
#[test]
fn the_instrument_covers_one_of_the_two_documents_it_could() {
    let root = repo_root();
    let all = designs(&root);
    assert_eq!(
        all.iter()
            .map(|p| p
                .strip_prefix(&root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned())
            .collect::<Vec<_>>(),
        [WATERMARKED, BARE],
        "the documents the gate reads as designs"
    );

    let watermarked: Vec<&str> = all
        .iter()
        .filter(|p| {
            let src = std::fs::read_to_string(p).expect("readable");
            front_matter(&src).is_some_and(|fm| fm.scalar("reconciled").is_some())
        })
        .map(|p| p.strip_prefix(&root).unwrap_or(p).to_str().expect("utf-8"))
        .collect();
    assert_eq!(
        watermarked,
        [WATERMARKED],
        "documents recording the commit their premises were last read against"
    );
}

/// **The document without the watermark rests on more and has drifted further.**
///
/// Eighteen premises against fourteen, thirteen moved against nine, +425/−41 against +410/−22,
/// 148 commits against 102. Worse on every measure the phase that built the instrument used to
/// argue the instrument was needed.
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
    assert_eq!(all.len(), 18, "premises the specimen document declares");
    assert_eq!(
        premises(&root, WATERMARKED).len(),
        14,
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
        13,
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

/// **A document that declares no watermark is never asked, and two of its figures have decayed.**
///
/// The first half is the gate's shape: `check` opens the comparison with `if let Some(at)`, so
/// silence is what a document gets for not declaring one. The second half is what that silence
/// cost here — a test count four times under, and a coverage list written when there were two
/// specimens and never revisited when there were four.
#[test]
fn a_document_that_declares_no_watermark_is_never_asked() {
    let root = repo_root();
    let lib =
        std::fs::read_to_string(root.join("crates/design-gate/src/lib.rs")).expect("readable");
    assert!(
        lib.contains("if let Some(at) = fm.scalar(\"reconciled\")"),
        "the watermark comparison is reached only when the document declares one"
    );
    assert!(
        !lib.contains("declares no reconciled:"),
        "and nothing requires that it does"
    );

    let doc = std::fs::read_to_string(root.join(BARE)).expect("readable");
    assert!(
        doc.contains("Thirty-eight tests"),
        "the figure the document states about its own fixtures"
    );
    let actual = ["crates/reachability/tests", "crates/game-format/tests"]
        .iter()
        .map(|d| {
            std::fs::read_dir(root.join(d))
                .expect("readable")
                .flatten()
                .map(|e| {
                    std::fs::read_to_string(e.path())
                        .expect("readable")
                        .matches("\n#[test]")
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    assert_eq!(
        actual, 158,
        "the fixtures the document describes carry this many tests"
    );

    assert!(
        doc.contains("Unexercised across both games: `or`, `!=`, `<=`, `>`"),
        "four constructs, across two games, when there are four games"
    );
    let coverage = std::fs::read_to_string(root.join("crates/game-format/tests/coverage.rs"))
        .expect("readable");
    assert!(
        coverage.contains(r#"["!=", "<=", ">", "clear-flag", "exit", "or"]"#),
        "and the check that counts them over all four finds six"
    );
}
