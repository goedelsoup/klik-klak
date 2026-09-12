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

// ── the second half of the same rule ────────────────────────────────────────
//
// The check above holds a marker's named resolver against the *date* of the document it names.
// What nothing held it against is whether the document has already been read — and that is the
// failure the corpus has been carrying since `class-library` landed.

/// **Documents a catalog entry records as having been read, by the name a marker would call them.**
///
/// Hand-written, like `resolvers.rs`'s kinds and `subject.rs`'s evidence paths, because *this
/// document has been read* is a thing a paragraph says and not a thing a string search settles. The
/// claim is the table; what the first check below does is refuse to let it invent an entry.
///
/// Two entries have a list at all. Every other source in this catalog is one document, retrieved
/// whole, and there is nothing for a marker to name a part of.
const READ: &[(&str, &[&str])] = &[
    (
        ".yidam/catalog/sci-inhouse-docs.md",
        &[
            "SCI Parser Programmer's Reference",
            "Script Classes for Adventure Games",
            "Recommended Procedures for SCI Game Programming Coordinators",
        ],
    ),
    (
        ".yidam/catalog/ags-manual.md",
        &["scripting language reference"],
    ),
];

/// The words an entry uses to say a thing was read, wherever it says it — four spellings, which
/// is the shape `.yidam/decisions/unread-list.yml` found this catalog's conventions arriving in.
const WAS_READ: [&str; 4] = [
    "has been read",
    "read in full",
    "Read 2026",
    "What was read",
];

/// Every `.yml` under a directory.
fn yml(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
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

/// The paragraph a marker lives in — which is the unit `resolver-kind`'s convention is about, since
/// *the question says what would close it* is a sentence beside the token rather than the token.
fn marker_paragraphs(root: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for p in yml(&root.join(".yidam/corpus")) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let lines: Vec<&str> = src.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            if !l.contains("[open]") {
                continue;
            }
            let start = lines[..i]
                .iter()
                .rposition(|x| x.trim().is_empty())
                .map_or(0, |j| j + 1);
            let end = lines[i..]
                .iter()
                .position(|x| x.trim().is_empty())
                .map_or(lines.len(), |j| i + j);
            let name = p
                .strip_prefix(root)
                .unwrap_or(&p)
                .to_string_lossy()
                .into_owned();
            out.push((name, flat(&lines[start..end].join(" "))));
        }
    }
    out
}

/// **The table names documents the catalog says were read, and invents none.**
///
/// The half that keeps the rule below honest. A table of things-already-read that quietly grew a
/// row nothing supports would turn the check into a way of banning words.
#[test]
fn the_table_of_read_documents_is_the_catalogs() {
    let root = repo_root();
    for (entry, titles) in READ {
        let src = flat(&read(&root, entry));
        for title in *titles {
            let mut said = false;
            let mut from = 0;
            while let Some(at) = src[from..].find(title) {
                let at = from + at;
                let lo = at.saturating_sub(160);
                let window = &src[lo..(at + title.len() + 160).min(src.len())];
                if WAS_READ.iter().any(|w| window.contains(w)) {
                    said = true;
                    break;
                }
                from = at + title.len();
            }
            assert!(
                said,
                "{entry} does not say {title:?} was read, so this table has invented a row"
            );
        }
    }
}

/// **A marker may name a document the catalog has read, and must then say so.**
///
/// This was a census one commit ago and pinned the defect. `prior-art/sierra-sci`'s
/// `source_standing` read *"Script Classes for Adventure Games is in the retrieved set,
/// **unread**, and is where a verb bar would be declared if Sierra declared one"* — while ninety
/// lines above it, the same node quoted that document at length, dated it 5 April 1988, and cited
/// the record that had refuted the marker two phases earlier.
///
/// **The rule is not that a marker may not name a read document.** Quoting what you correct is this
/// repository's standing practice, and the corrected marker named it again in order to say it was
/// the wrong one. What a marker may not do is name a document the catalog has read **and call it
/// unread**, because that sentence is not stale — it is false against a file in the same
/// directory, and it makes the question look cheap.
///
/// **The instance is gone: `two-bars` answered that question one phase later and the marker with
/// it.** So this holds zero markers today and governs every one written after it.
///
/// Narrow on purpose, like the date check above. It cannot know whether a named document answers a
/// question; nothing can, short of reading it, which is a phase and not a gate. What it knows is
/// that the artifact is already in the house.
///
/// The shape is `.yidam/decisions/stale-markers.yml`'s — *"the prose was updated and the marker was
/// not"* — on its third occurrence, and the first where a check was possible at all. It is possible
/// because `resolver-kind` made a marker say what would close it, which is what gave the sentence
/// something to be wrong about. `.yidam/decisions/retraction-reach.yml`.
#[test]
fn a_marker_naming_a_read_document_says_it_has_been_read() {
    let root = repo_root();
    let read_titles: Vec<&str> = READ.iter().flat_map(|(_, t)| t.iter().copied()).collect();

    let mut naming: Vec<(String, &str)> = Vec::new();
    for (node, para) in marker_paragraphs(&root) {
        for t in &read_titles {
            if !para.contains(t) {
                continue;
            }
            naming.push((node.clone(), t));
            assert!(
                WAS_READ.iter().any(|w| para.contains(w)),
                "{node}'s marker names {t:?} and does not say it has been read"
            );
            assert!(
                !para.contains("unread") && !para.contains("has not been read"),
                "{node}'s marker names {t:?} and calls it unread, which is false against \
                 the catalog entry the node cites"
            );
        }
    }
    // **Zero, one phase after the rule was written, because the question closed.** The marker that
    // motivated this named *Script Classes for Adventure Games*; `.yidam/decisions/two-bars.yml`
    // answered the question and retracted the marker, and no marker names a read document now.
    //
    // That is the ordinary life of a guard and it is worth being exact about what it means here.
    // The loop above governs every marker in the corpus and fires on the next one; what it has
    // never done is catch something nobody already knew, because the instance was repaired by the
    // phase that wrote the check. A guard with no current violation is not the defect
    // `.yidam/decisions/unread-list.yml` warns about — that one is a *measurement* reporting seven
    // of eight and meaning nothing — but the two are easy to confuse and the difference is whether
    // the number is load-bearing. Here it is not; the loop is.
    assert!(
        naming.is_empty(),
        "a marker names a document the catalog has read: {naming:?}"
    );
}

/// A question, the record that owns it, and the phrase both must carry about what would close it.
///
/// Hand-written, because which record owns a question is not a thing a string search settles. One
/// row, and it is the row this rule was built from.
const AGREES: &[(&str, &str, &str)] = &[(
    ".yidam/corpus/prior-art/sierra-sci.yml",
    ".yidam/decisions/input-side.yml",
    "SCI1-era",
)];

/// **The node and the record say the same thing about what would close the question.**
///
/// A census one commit ago: `input-side.yml` said *"What would settle it is an **SCI1-era**
/// artifact"* and the node said a document that had been read. **The record is the copy that gets
/// edited**, because a phase is about its record. **The node is the copy that gets read** — by
/// `yidam open-questions`, by `status`, by `due`, and by anybody following the corpus rather than
/// the history. For two phases the wrong one of the two was right.
///
/// `.yidam/decisions/marker-count.yml` found the same fact doing the opposite damage: 56 of 63 live
/// questions sit where no command walks. The directory nothing reads is also the directory every
/// correction gets written into.
#[test]
fn the_node_and_the_record_agree_about_what_would_close_it() {
    let root = repo_root();
    for (node, record, phrase) in AGREES {
        let n = flat(&read(&root, node));
        let r = flat(&read(&root, record));
        assert!(
            r.contains(phrase),
            "{record} no longer names the resolver ({phrase:?})"
        );
        assert!(
            n.contains(phrase),
            "{node} does not say what {record} says would close its question ({phrase:?})"
        );
    }
}
