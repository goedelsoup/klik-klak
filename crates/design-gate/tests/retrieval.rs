//! **A marker one phase old names the document that would settle it, and that document predates
//! the question by two years.**
//!
//! `.yidam/decisions/input-side.yml` closed on this: *"Whether SCI1's icon bar was a game script or
//! an engine feature is answerable and unanswered. Script Classes for Adventure Games is in the
//! retrieved set, unread, and is where a verb bar would be declared if Sierra declared one."*
//!
//! It was read. Jeff Stephenson, **5 April 1988** — SCI0, the parser era. SCI1 and its icon bar
//! shipped in 1990. A document written two years before the thing cannot say whether the thing was
//! an engine feature, and the marker asserted it could.
//!
//! **Naming the artifact that would close a question is the most useful thing a marker can do and
//! it is the thing nothing checks.** Five of this corpus's nine questions are past the interval
//! `.yidam/config.toml` declares; what separates a question waiting for work from one waiting for
//! evidence that does not exist is whether anybody can say what would close it. This one said, and
//! was wrong, and was wrong in the direction that looks like progress.
//!
//! The second check is smaller and is about the catalog. An entry's unread-list has two states — an
//! item is on it, or it came off and the list says where the reading went
//! (`crates/design-gate/tests/catalog.rs`). There is a third now: the *Laura Bow II Programming
//! Style Manual* is a 5.4MB scan with no text layer, and this machine has no OCR. Retrieved, opened,
//! and not readable here is neither of the two states the convention has.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// Prose wraps.
fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **The marker names a document that cannot answer it.**
///
/// This is the census and it pins the defect: `input-side`'s open question points at *Script
/// Classes for Adventure Games*, the catalog dates that document to 1988, and the question is about
/// a 1990 interface. It flips when the marker names something that could settle it.
#[test]
fn the_marker_names_a_document_that_predates_its_own_question() {
    let root = repo_root();
    let record = flat(&read(&root, ".yidam/decisions/input-side.yml"));
    let entry = flat(&read(&root, ".yidam/catalog/sci-inhouse-docs.md"));

    assert!(
        record.contains("Script Classes for Adventure Games"),
        "the marker no longer names the document this check is about"
    );
    assert!(
        record.contains("is where a verb bar would be declared if Sierra declared one"),
        "the claim being pinned: that this document is where the answer would be"
    );

    // And the two dates that make it impossible, both from the catalog rather than from here.
    assert!(
        entry.contains("SCI0 shipped in 1988") && entry.contains("SCI1 in 1990"),
        "the entry dates the two interfaces, which is what the marker had to read and did not"
    );
}

/// **The catalog has two states for an item on an unread-list, and a third has arrived.**
///
/// `catalog.rs` holds the convention: an item is unread, or it came off the list and the list says
/// where the reading went. Both assume that opening a document and reading it are the same act.
///
/// The *Laura Bow II Programming Style Manual* is a scan. There is no text layer in it, there is no
/// `pdftotext` or `pdftoppm` on this machine, and nothing here can read an image. It is retrieved,
/// it was opened, and what it says is unknown — which is worth more than leaving it on the list as
/// though nobody had tried, because the next reader would try the same thing.
#[test]
fn an_unread_list_cannot_say_that_a_reading_was_attempted_and_failed() {
    let root = repo_root();
    let entry = flat(&read(&root, ".yidam/catalog/sci-inhouse-docs.md"));

    // The census: the entry says the manual is unread, and says nothing about having tried.
    assert!(
        entry.contains("Laura Bow II"),
        "the entry no longer names the manual"
    );
    assert!(
        !entry.contains("no text layer"),
        "the entry now records the attempt, which is what this check was pinning the absence of"
    );

    // And the convention it would have to extend, held where it is stated.
    let readme = flat(&read(&root, ".yidam/catalog/README.md"));
    assert!(
        readme.contains("When an item on that list gets read, the list says so and names where the reading went"),
        "the two-state convention, which has no third"
    );
}
