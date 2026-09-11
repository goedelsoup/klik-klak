//! **A proposal's ground can move under it and nothing here says so.**
//!
//! `design/bounded-expression-language.md` is 709 lines and `status: proposal`. It declares
//! fourteen `rests_on` premises, which the gate checks — and what it checks is that the paths
//! resolve. `design-gate`'s own header admits the rest: *"`rests_on` proves a path resolves rather
//! than that the node still says what the design thought."*
//!
//! **Nine of the fourteen have changed since the document was last edited**, 102 commits ago,
//! by +410 lines and −22. Three of the nine are catalog entries four phases rewrote: the AGS
//! manual gained the scripting language reference the document's central question is about, the
//! AGS source gained the forty-eight-entry action table, and the SCUMM reference gained eighty-four
//! opcodes.
//!
//! None of that falsified the proposal. That is the point — **a premise moving is not a defect and
//! was not visible either**, and the one time it was caught, *"nobody has tried the middle"* being
//! withdrawn, it was caught by somebody reading.
//!
//! The document now carries `reconciled:`, a commit its premises were last read against, and the
//! gate fails when one of them moves past it. The first test below stays as the census — a closed
//! window on the drift as it stood — and the last holds the watermark.
//!
//! `.yidam/decisions/moved-ground.yml`.

use design_gate::{front_matter, repo_root};
use std::path::Path;
use std::process::Command;

const DESIGN: &str = "design/bounded-expression-language.md";

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

fn premises(root: &Path) -> Vec<String> {
    let src = std::fs::read_to_string(root.join(DESIGN)).expect("readable");
    let fm = front_matter(&src).expect("front matter");
    fm.list("rests_on").to_vec()
}

/// **Fourteen premises, nine of them moved, and the gate reported none of it.**
///
/// A **closed window**, anchored by commit subject to the edit that stood before `moved-ground`
/// opened. The first version of this measured from *the document's last commit* and the phase's
/// own revise moved that anchor out from under it — a baseline that the thing it measures can
/// change is not a baseline, which is the finding of this phase arriving inside its own check.
const BEFORE: &str = "^revise: two sentences that were edges";

#[test]
fn nine_of_fourteen_premises_moved_under_the_proposal() {
    let root = repo_root();
    let found = git(&root, &["log", "--format=%H", "--grep", BEFORE]);
    let last = found
        .lines()
        .next()
        .expect("the anchor commit is gone, so this window cannot be measured")
        .to_string();
    let all = premises(&root);
    assert_eq!(all.len(), 14, "premises the proposal declares");

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
        9,
        "premises edited since the document was: {moved:?}"
    );
    assert!(
        added >= 410 && removed >= 22,
        "lines added and removed under the proposal's feet: +{added}/-{removed}"
    );
    assert!(
        git(&root, &["rev-list", "--count", &format!("{last}..HEAD")])
            .parse::<usize>()
            .expect("a count")
            >= 102,
        "commits since the document was last edited"
    );
}

/// **The gate says out loud that it cannot see this.**
///
/// Not a hidden limitation — `design-gate`'s header states it, `specimen-placement` decided it,
/// and the check that exists is the one a specimen can be held to without a running engine. What
/// is missing is not a stricter reading of the premise; it is any record of when the reading last
/// happened.
#[test]
fn the_gate_checks_that_a_premise_resolves_and_says_that_is_all() {
    let root = repo_root();
    let lib =
        std::fs::read_to_string(root.join("crates/design-gate/src/lib.rs")).expect("readable");
    assert!(
        lib.contains("`rests_on` proves a path resolves rather than that"),
        "the gate's own statement of what it does not check"
    );

    // every premise resolves, which is the whole of what is enforced today
    for p in premises(&root) {
        assert!(
            root.join(&p).exists(),
            "{p} does not resolve, which is the one thing the gate does catch"
        );
    }
}

/// **The document records the commit its premises were last read against, and the gate holds it.**
///
/// A `status: proposal` with fourteen premises and no reconciliation date is a document whose
/// staleness is not unknown but *unmeasurable*, because there is no baseline to measure from.
/// `reconciled:` is the baseline; `design_gate::check` fails when any `rests_on` file has a commit
/// after it.
///
/// Not a date and not an interval — **an event**, and the event is the one that should send
/// somebody back to the document. `unread-clock` established that this repository holds no clock
/// anybody can read; a watermark needs no clock, only a baseline and a comparison, and git has
/// both exactly.
#[test]
fn the_proposal_records_what_it_was_last_read_against() {
    let root = repo_root();
    let src = std::fs::read_to_string(root.join(DESIGN)).expect("readable");
    let fm = front_matter(&src).expect("front matter");
    let at = fm
        .scalar("reconciled")
        .expect("the proposal names the commit its premises were last read against");
    assert_eq!(at.len(), 40, "a full commit sha: {at}");
    assert!(
        git(&root, &["cat-file", "-t", at]) == "commit",
        "and it is a commit in this history"
    );

    for p in premises(&root) {
        let moved: usize = git(
            &root,
            &["rev-list", "--count", &format!("{at}..HEAD"), "--", &p],
        )
        .parse()
        .expect("a count");
        assert_eq!(
            moved, 0,
            "{p} moved since the watermark; the gate says the same thing and this says why"
        );
    }
}
