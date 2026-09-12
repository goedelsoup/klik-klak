//! **A marker one phase old named the document that would settle it, and that document predated the
//! question by two years.**
//!
//! `.yidam/decisions/input-side.yml` closed on this: *"Whether SCI1's icon bar was a game script or
//! an engine feature is answerable and unanswered. Script Classes for Adventure Games is in the
//! retrieved set, unread, and is where a verb bar would be declared if Sierra declared one."*
//!
//! It was read. Jeff Stephenson, **5 April 1988** — SCI0, the parser era. SCI1 and its icon bar
//! shipped in 1990. A document written two years before the thing cannot say whether the thing was
//! an engine feature, and the marker asserted it could. Both dates were in the catalog entry the
//! marker sits beside, and had been since the entry was written.
//!
//! **Naming the artifact that would close a question is the most useful thing a marker can do and
//! it is the thing nothing checked.** Five of this corpus's nine questions are past the interval
//! `.yidam/config.toml` declares; what separates a question waiting for work from one waiting for
//! evidence that does not exist is whether anybody can say what would close it. This one said, and
//! was wrong, and was wrong in the direction that looks like progress — a question with a named
//! resolver reads as nearly answered.
//!
//! **That is the convention working rather than failing.** A marker saying only *this is open*
//! cannot be wrong and cannot be checked. The cost of naming a resolver is that you can name the
//! wrong one; the benefit is that the next phase finds out. `.yidam/decisions/class-library.yml`.
//!
//! The second check is smaller and is about the catalog. An entry's unread-list has two states — an
//! item is on it, or it came off and the list says where the reading went
//! (`crates/design-gate/tests/catalog.rs`). There is a third now: the *Laura Bow II Programming
//! Style Manual* is a 5.4MB scan with no text layer, and this machine has no OCR. Retrieved, opened,
//! and not readable here is neither of the two states the convention has, and the entry writes it
//! out so the next reader does not repeat the attempt.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// Prose wraps.
fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **A marker that names its own resolver names one that could hold the answer.**
///
/// This was a census one commit ago and pinned the defect: `input-side`'s question pointed at
/// *Script Classes for Adventure Games* as *"where a verb bar would be declared if Sierra declared
/// one"*, and that document is dated two years before the interface the question is about.
///
/// It is a rule now, and a narrow one on purpose. It cannot check that a named document *does*
/// answer a question — nothing can, short of reading it, which is the phase rather than the gate.
/// What it can check is the one thing that made this marker impossible rather than merely wrong:
/// that the document named is not dated before the thing it is supposed to describe. The dates come
/// out of the catalog, where they already were when the marker was written.
#[test]
fn the_marker_names_a_resolver_that_is_not_older_than_its_question() {
    let root = repo_root();
    let record = flat(&read(&root, ".yidam/decisions/input-side.yml"));
    let entry = flat(&read(&root, ".yidam/catalog/sci-inhouse-docs.md"));

    // The old claim is quoted rather than deleted, which is this repository's standing practice
    // and is not the re-emission `marker-count` forbids: that convention is about the `[open]`
    // token, and `markers.rs` is where it is held. A correction that hides what it corrects makes
    // the record unreadable against its own history.
    assert!(
        record.contains("this marker named the wrong document to answer it with"),
        "the marker does not say that it was wrong, only that it changed"
    );

    // The marker still names a resolver, and names one that could hold the answer.
    assert!(
        record.contains("What would settle it is an **SCI1-era** artifact"),
        "the marker no longer says what would close it, which is the half worth keeping"
    );
    assert!(
        record.contains("5 April 1988") && record.contains("two years before SCI1 shipped"),
        "and it says why the document it used to name cannot"
    );

    // Both dates are in the catalog and were there the whole time.
    assert!(
        entry.contains("SCI0 shipped in 1988") && entry.contains("SCI1 in 1990"),
        "the entry dates the two interfaces, which is what the marker had to read and did not"
    );
}

/// **An attempted reading that failed says so, and says what would make it succeed.**
///
/// `catalog.rs` holds a two-state convention: an item is unread, or it came off the list and the
/// list says where the reading went. Both assume opening a document and reading it are the same
/// act. The *Laura Bow II Programming Style Manual* is a 5.4MB scan with no text layer, on a
/// machine with no `pdftotext` and no `pdftoppm`, and it is neither state.
///
/// The entry writes the attempt out. That is the convention `unread-list` argued for — a list says
/// where a reading went — applied to a reading that did not happen and was not skipped, and the
/// reason is the same one: without it the next reader repeats the attempt.
///
/// It is **not** promoted to a rule over every entry. One instance is not a convention, and
/// `unread-list` is on record that the wider check it measured *"reports seven of eight and means
/// nothing"*. What is held is this entry, and the reason, so that a second instance has something
/// to match.
#[test]
fn an_attempted_reading_that_failed_says_so() {
    let root = repo_root();
    let entry = flat(&read(&root, ".yidam/catalog/sci-inhouse-docs.md"));

    assert!(
        entry.contains("retrieved, opened, and not readable here"),
        "the third state, in the entry that has the only instance of it"
    );
    // Why, specifically enough that somebody with different tooling can finish the job.
    for reason in ["no text layer", "pdftotext", "5.4MB scan"] {
        assert!(
            entry.contains(reason),
            "the entry does not say {reason:?}, so a reader cannot tell whether their machine \
             would do better"
        );
    }
    // And the location, so they do not have to find it again.
    assert!(
        entry.contains("Laura_Bow_II_-_Programming_Style_Manual.pdf"),
        "the file that could not be read is not linked"
    );

    // The two-state convention it sits beside, unchanged and still stated where it is stated.
    let readme = flat(&read(&root, ".yidam/catalog/README.md"));
    assert!(
        readme.contains("When an item on that list gets read, the list says so and names where the reading went"),
        "the convention this is an exception to"
    );
}
