//! **Three of the gate's six hand-written vocabularies fail silent, and one of them is skipping
//! seven records right now.**
//!
//! `.yidam/decisions/unread-specimen.yml` found twenty-five tests whose input was a copy of a
//! directory and stated the precondition the method here rests on: **a pinned figure is a signal
//! only if the input is the world.** It fixed that for one directory. This is the same defect in
//! the dimension the gate actually spends most of its checks on — not files, but **English**.
//!
//! Six checks are built on a hand-written list of phrasings, and what a missed spelling does to
//! each is not the same thing:
//!
//! | vocabulary | file | a phrasing nobody listed |
//! |---|---|---|
//! | `ROUTES`, 8 | `routing.rs` | **loud** — the set of routing classes is pinned, so one dropping out fails |
//! | `WAS_READ`, 4 | `retrieval.rs` | **loud** — it confirms, so a miss reads as *the catalog does not say it* |
//! | `READ`, 4 titles | `retrieval.rs` | **loud** on the titles, and its list of *entries* is the unheld table below |
//! | `CLOSED`, 6 | `markers.rs` | **silent** — a retraction worded newly is not detected |
//! | `PREMISE`, 3 | `reach.rs` | **silent** — a record is dropped out of the census |
//! | `STALE`, 5 | `enumerated.rs` | **silent** — a stale figure worded newly is not found |
//!
//! **The direction is the whole of it, and nobody had ever asked which way one of these fails.** A
//! vocabulary that *confirms* is safe: the list is a floor, and falling through it is a failure. A
//! vocabulary that *detects* is not: the list is a ceiling, and what is above it is reported as
//! nothing at all.
//!
//! **`reach.rs` is above its ceiling today.** It counts the records resting on *this vocabulary has
//! no geometry* by taking every record that mentions geometry and matching three phrasings.
//! Fourteen records mention geometry. It classifies seven — six resting, one deciding — and
//! **silently skips the other seven** — among them `adjacency`, the phase that read 84 walkbox polygons out of a shipped
//! game, which is the record most about geometry this corpus has.
//!
//! And one hand table of **files** is unheld: `retrieval.rs`'s `READ` states that two catalog
//! entries list the documents they contain and never reads `.yidam/catalog/` to check. That was
//! offered as the pattern this phase would sweep and it is one instance, which
//! `.yidam/decisions/unread-list.yml` is on record as not being a convention. It is fixed here and
//! the finding is the other one. `.yidam/decisions/unread-spelling.yml`.

use design_gate::repo_root;
use std::path::{Path, PathBuf};

/// What a phrasing nobody listed does to the check that reads for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Miss {
    /// The check fails. The vocabulary is a floor: the material is enumerated some other way and
    /// the phrasings account for it, so falling through one is a hole somebody sees.
    Loud,
    /// The check passes. The vocabulary is a ceiling: nothing enumerates what it did not match, so
    /// the material above it is reported as absent rather than as unread.
    Silent,
}

/// Every hand-written vocabulary in the gate, and which way it fails.
///
/// Hand-classified, like `resolvers.rs`'s kinds and `subject.rs`'s evidence paths, because the
/// direction is a fact about how the surrounding test is written and not about the constant. The
/// claim is the classification; the check below is that it covers every vocabulary on disk.
const VOCABULARIES: &[(&str, &str, usize, Miss)] = &[
    ("enumerated.rs", "STALE", 5, Miss::Silent),
    ("markers.rs", "CLOSED", 5, Miss::Silent),
    ("reach.rs", "PREMISE", 3, Miss::Silent),
    ("retrieval.rs", "READ", 4, Miss::Loud),
    ("retrieval.rs", "WAS_READ", 4, Miss::Loud),
    ("routing.rs", "ROUTES", 8, Miss::Loud),
];

/// A `const NAME: [&str; N]` whose members are phrases rather than paths — the shape the table
/// above is about, found by reading the sources rather than by being told.
fn vocabularies(root: &Path) -> Vec<(String, String, usize)> {
    let dir = root.join("crates/design-gate/tests");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the gate's tests")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    files.sort();

    let mut out = Vec::new();
    for p in files {
        let name = p
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        let src = std::fs::read_to_string(&p).expect("readable");
        // a declaration, not the text of one: `naming.rs` asserts that a component's source
        // contains the string `const NAMES: [(&str, &str); 6] = [`, and a scanner that reads
        // bytes cannot tell a constant from a quotation of one.
        for (offset, _) in src.match_indices("const ") {
            // a declaration starts its line; anything else is a quotation of one
            let indent_only = src[..offset]
                .rsplit_once('\n')
                .is_some_and(|(_, head)| head.chars().all(char::is_whitespace));
            if !indent_only {
                continue;
            }
            let rest = &src[offset + 6..];
            let Some(colon) = rest.find(':') else {
                continue;
            };
            let ident = &rest[..colon];
            if ident.is_empty() || !ident.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                continue;
            }
            let Some(eq) = rest.find('=') else { continue };
            let ty = &rest[colon..eq];
            if !ty.contains("&str") || !ty.contains('[') {
                continue;
            }
            let Some(open) = rest[eq..].find('[') else {
                continue;
            };
            let Some(close) = rest[eq + open..].find("];") else {
                continue;
            };
            let body = &rest[eq + open..eq + open + close];
            // phrases: a literal with a space in it and no path separator
            let phrases = body
                .split('"')
                .skip(1)
                .step_by(2)
                .filter(|l| l.contains(' ') && !l.contains('/'))
                .count();
            if phrases >= 2 {
                out.push((name.clone(), ident.to_string(), phrases));
            }
        }
    }
    out
}

/// **The table covers every vocabulary and invents none.**
///
/// The check `resolvers.rs` wrote for the markers, pointed at the gate's own constants. A sixth
/// vocabulary added without a row fails here, which is the one thing a hand classification can be
/// held to.
#[test]
fn the_table_covers_every_vocabulary_in_the_gate() {
    let root = repo_root();
    let found = vocabularies(&root);
    let declared: Vec<(String, String, usize)> = VOCABULARIES
        .iter()
        .map(|(f, n, c, _)| (f.to_string(), n.to_string(), *c))
        .collect();
    assert_eq!(
        found, declared,
        "every hand-written vocabulary needs a row, and a row needs a vocabulary"
    );
}

/// **Three of six fail silent, which is a thing nobody had asked about any of them.**
///
/// Pinned as a census rather than as a rule, because two of the three are silent for reasons that
/// take a different fix each and one of them is silent and harmless. The rule this becomes is in
/// `.yidam/decisions/unread-spelling.yml`.
#[test]
fn three_of_six_vocabularies_fail_silent() {
    let count = |m: Miss| VOCABULARIES.iter().filter(|(_, _, _, x)| *x == m).count();
    assert_eq!(
        (count(Miss::Loud), count(Miss::Silent)),
        (3, 3),
        "vocabularies whose miss is a failure, and whose miss is a pass"
    );
}

/// **`reach.rs` filters fourteen records and reads seven of them.**
///
/// The live instance, and the one that makes this a defect rather than a shape. That test counts
/// the records resting on *this vocabulary has no geometry* by filtering `.yidam/decisions/` to
/// those mentioning geometry and then matching three phrasings inside them. The filter finds
/// fourteen. The phrasings match seven. **The other seven are `continue`d past and reported
/// nowhere**, so the census the test publishes is drawn from half of what it looked at.
///
/// Its own comment records the near miss in the other direction — a fourth phrasing, `has none`,
/// was in the list for one commit and matched `prose-edges` on a sentence about prose having no
/// schema. The author who took that one out looked for false positives and there was no way to look
/// for false negatives, because nothing counted the denominator.
///
/// `adjacency` is in the seven. It is the phase that read 84 walkbox polygons out of a shipped
/// game's source and derived every room's adjacency graph from them — whether it rests on the
/// boundary this test enumerates is exactly the question the test exists to answer, and the test
/// has never seen it.
#[test]
fn the_geometry_census_skips_seven_of_the_fourteen_records_it_filters() {
    let root = repo_root();
    // Read out of `reach.rs` rather than copied here. The first version of this test copied the
    // three phrasings, and the scanner above found them — a census of hand-written vocabularies
    // holding a hand-written copy of the vocabulary it was measuring, which is the defect
    // `adjacency` named as *a copy of something that is not constant* and `unread-specimen`
    // generalised. Caught by its own check on the first run, which is the one way this shape has
    // ever been caught here.
    let premise: Vec<String> = {
        let src = std::fs::read_to_string(root.join("crates/design-gate/tests/reach.rs"))
            .expect("readable");
        let at = src
            .find("const PREMISE:")
            .expect("the vocabulary this test is about");
        let body = &src[at..at + src[at..].find("];").expect("its end")];
        body.split('"')
            .skip(1)
            .step_by(2)
            .map(str::to_string)
            .collect()
    };
    assert_eq!(
        premise.len(),
        3,
        "the phrasings `reach.rs` reads for: {premise:?}"
    );
    let (mut mentioning, mut matched, mut skipped) = (0usize, 0usize, Vec::new());
    let mut files: Vec<PathBuf> = std::fs::read_dir(root.join(".yidam/decisions"))
        .expect("the directory exists")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yml"))
        .collect();
    files.sort();
    for p in files {
        let src = std::fs::read_to_string(&p).expect("readable");
        if !src.contains("geometry") {
            continue;
        }
        mentioning += 1;
        if premise.iter().any(|s| src.contains(s.as_str())) {
            matched += 1;
        } else {
            skipped.push(
                p.file_stem()
                    .expect("a name")
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
    assert_eq!(
        (mentioning, matched, skipped.len()),
        (14, 7, 7),
        "records mentioning geometry, records a phrasing matched, records dropped in silence"
    );
    assert_eq!(
        skipped,
        [
            "action-pages",
            "adjacency",
            "engine-directory",
            "evidence-kinds",
            "ontology",
            "record-layer",
            "round-trip",
        ],
        "and none of them has ever been looked at by the check that enumerates this premise"
    );
}

/// **One hand table of files is unheld, and one is not a pattern.**
///
/// `retrieval.rs`'s `READ` names two catalog entries and the documents each records as read, under
/// a header stating that *"two entries have a list at all"* — a claim about `.yidam/catalog/`, made
/// without reading it. Twelve files are there. A twelfth source arriving with a list of documents
/// inside it is invisible to that check in exactly the way `lock.game` was invisible to
/// twenty-five others.
///
/// **It is one instance and it was offered as a pattern.** The survey that opened this phase looked
/// for hand tables of files across both workspaces and found that almost every one is a citation —
/// a test naming the two records it is about — rather than a copy of a listing, and that the two
/// that are copies (`resolvers.rs`, `lineage.rs`) already hold themselves against a directory scan
/// without either of them arguing for it. `.yidam/decisions/unread-list.yml` is on record that one
/// instance is not a convention. It is repaired below and it is not the finding.
#[test]
fn one_table_of_files_claims_a_directory_it_does_not_read() {
    let root = repo_root();
    let src = std::fs::read_to_string(root.join("crates/design-gate/tests/retrieval.rs"))
        .expect("readable");
    let entries = std::fs::read_dir(root.join(".yidam/catalog"))
        .expect("the catalog")
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "md"))
        .count();
    assert_eq!(entries, 12, "files in the catalog");
    assert!(
        !src.contains("join(\".yidam/catalog\")"),
        "`retrieval.rs` reads the catalog now, which is what this census was pinning the absence of"
    );

    // and the two that do it right, neither of which says so
    for (file, marker) in [
        (
            "resolvers.rs",
            "every marker needs a row, and a row needs a marker",
        ),
        ("lineage.rs", "The table covers every node"),
    ] {
        let src = std::fs::read_to_string(root.join("crates/design-gate/tests").join(file))
            .expect("readable");
        assert!(
            src.contains(marker),
            "{file} no longer holds its table to a scan"
        );
    }
}
