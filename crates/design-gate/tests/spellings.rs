//! **A check built on a hand-written vocabulary reports what it could not classify.**
//!
//! `.yidam/decisions/unread-specimen.yml` found twenty-five tests whose input was a copy of a
//! directory and stated the precondition this method rests on: **a pinned figure is a signal only
//! if the input is the world.** It fixed that for one directory. This is the same defect in the
//! dimension the gate actually spends most of its checks on — not files, but **English**.
//!
//! Seven checks read for a hand-written list of phrasings, and what a missed spelling does to each is
//! not the same thing. A vocabulary that **confirms** is a floor: the material is enumerated some
//! other way and the phrasings account for it, so falling through one is a hole somebody sees. A
//! vocabulary that **detects** is a ceiling: nothing enumerates what it did not match, so the
//! material above it is reported as absent rather than as unread. **Nobody had ever asked which way
//! one of these fails.**
//!
//! Three were ceilings, and `reach.rs` was above its. It counted the records resting on *this
//! vocabulary has no geometry* by filtering `.yidam/decisions/` to those mentioning geometry and
//! matching three phrasings inside them — publishing six of six while computing six of fourteen,
//! and skipping seven records in silence. **Two of the seven rest on the premise**, so the figure
//! was eight.
//!
//! **A longer list would not have found them, which is the finding.** `evidence-kinds` uses a
//! fourth phrasing and a fourth entry would catch it. `adjacency` uses none: it read 84 walkbox
//! polygons out of a shipped game and derived every room's adjacency graph from them **because this
//! vocabulary has none to derive it from** — a record resting on a premise by doing the thing the
//! premise makes necessary. No list of spellings could have found it, and adding spellings is what
//! this check has always done, which is why nobody noticed: each addition makes the number go up,
//! which reads as the check getting better rather than as the ceiling getting higher.
//!
//! **So a ceiling declares its cover.** Each silent vocabulary below names the thing that reports
//! what it did not match — a residue table somebody classifies by hand, a published denominator, or
//! a second check that catches the same defect loudly — and this file holds the naming to being
//! true. `.yidam/decisions/unread-spelling.yml`.

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
    ///
    /// A ceiling is allowed, because there is no `read_dir` for the ways a person might write a
    /// sentence. What is not allowed is an undeclared one: every row of this kind names the thing
    /// that reports its residue, and `every_ceiling_names_what_covers_it` holds the naming.
    Silent,
}

/// Every hand-written vocabulary in the gate, which way it fails, and what covers it.
///
/// Hand-classified, like `resolvers.rs`'s kinds and `subject.rs`'s evidence paths, because the
/// direction is a fact about how the surrounding test is written and not about the constant. The
/// claim is the classification; the check below is that it covers every vocabulary on disk and that
/// every ceiling names something real.
const VOCABULARIES: &[(&str, &str, usize, Miss, &str)] = &[
    // a sentence and what it contradicted, two phrases a row; a sentence corrected fails by name
    ("drift.rs", "STALE", 48, Miss::Loud, ""),
    ("drift.rs", "COPIES", 10, Miss::Loud, ""),
    ("drift.rs", "KEPT", 2, Miss::Loud, ""),
    (
        "enumerated.rs",
        "STALE",
        5,
        Miss::Silent,
        // its own denominator: every living document that mentions the specimens at all
        "living documents read, and living documents that mention the specimens",
    ),
    (
        "markers.rs",
        "CLOSED",
        5,
        Miss::Silent,
        // a second check that catches the same defect loudly, three phases older and unaware
        "the backstop for this check's silence is gone",
    ),
    (
        "reach.rs",
        "PREMISE",
        3,
        Miss::Silent,
        // a residue table: every record the filter admits is classified by hand
        "a record mentioning geometry with no row, or a row with no record",
    ),
    ("retrieval.rs", "READ", 7, Miss::Loud, ""),
    ("retrieval.rs", "WAS_READ", 4, Miss::Loud, ""),
    ("routing.rs", "ROUTES", 8, Miss::Loud, ""),
    // this table, which is one: the three covers above are phrases it looks for in other files.
    // It is a floor — a cover naming something absent fails `every_ceiling_names_what_covers_it`,
    // by name — and it is listed because a census that exempts itself is the defect it is about.
    ("spellings.rs", "VOCABULARIES", 3, Miss::Loud, ""),
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
        .map(|(f, n, c, _, _)| (f.to_string(), n.to_string(), *c))
        .collect();
    assert_eq!(
        found, declared,
        "every hand-written vocabulary needs a row, and a row needs a vocabulary"
    );
}

/// **Three of ten fail silent, and every one of the three names what covers it.**
///
/// The rule. A ceiling is not a defect — there is no `read_dir` for the ways a person might write a
/// sentence, and a list of phrasings is the only way in. What is a defect is an *undeclared*
/// ceiling: a check reporting a clean result over material it never looked at, with nothing saying
/// how much that was.
///
/// The three covers are three different shapes, which is the reason this is a rule about declaring
/// rather than a rule about mechanism:
///
/// - **a residue table** — `reach.rs` names every record its filter admits and classifies each by
///   hand, so a phase that writes about geometry has to be read rather than falling through;
/// - **a published denominator** — `enumerated.rs` reports how many living documents mention the
///   specimens at all, so a clean result over a shrinking denominator is not the same result;
/// - **a loud second check** — `markers.rs` keeps a silent check and names `resolvers.rs`, where a
///   marker it let through has no row and fails by name.
#[test]
fn three_of_six_vocabularies_fail_silent_and_each_names_its_cover() {
    let count = |m: Miss| {
        VOCABULARIES
            .iter()
            .filter(|(_, _, _, x, _)| *x == m)
            .count()
    };
    assert_eq!(
        (count(Miss::Loud), count(Miss::Silent)),
        (7, 3),
        "vocabularies whose miss is a failure, and whose miss is a pass"
    );
    for (file, name, _, miss, cover) in VOCABULARIES {
        assert_eq!(
            *miss == Miss::Silent,
            !cover.is_empty(),
            "{file}'s {name} is a ceiling with no declared cover, or declares one and is not"
        );
    }
}

/// **Every ceiling names something that is actually there.**
///
/// The half that keeps the declaration from being a comment. Each silent vocabulary names a string
/// its own file must contain — the assertion or message that publishes the residue — and a phase
/// that deletes the cover while leaving the row fails here rather than going quiet again.
#[test]
fn every_ceiling_names_what_covers_it() {
    let root = repo_root();
    for (file, name, _, miss, cover) in VOCABULARIES {
        if *miss != Miss::Silent {
            continue;
        }
        let src = std::fs::read_to_string(root.join("crates/design-gate/tests").join(file))
            .expect("readable");
        assert!(
            src.contains(cover),
            "{file}'s {name} declares that {cover:?} reports what it could not match, and the \
             file does not say it"
        );
    }
}

/// **`reach.rs` publishes its residue now, and the phrasings find six of the eight that rest.**
///
/// The instance this phase was about, held from outside the file it repairs. Fifteen records
/// mention geometry; every one has a row in `reach.rs`'s hand-made table; eight rest on the
/// premise, and the three phrasings find six of them. **The gap between six and eight is the
/// measurement that could not be taken before**, because nothing counted what the filter admitted
/// and the phrasings missed.
///
/// `evidence-kinds` is the one a fourth spelling would have caught. `adjacency` is the one no
/// spelling could: it rests on the premise by acting on it.
#[test]
fn the_geometry_check_publishes_what_its_phrasings_miss() {
    let root = repo_root();
    let src =
        std::fs::read_to_string(root.join("crates/design-gate/tests/reach.rs")).expect("readable");
    for held in [
        "const GEOMETRY: &[(&str, Geometry)]",
        "a record mentioning geometry with no row, or a row with no record",
        "records the phrasings find, records that rest, and everything a phrasing matched",
    ] {
        assert!(src.contains(held), "`reach.rs` no longer {held:?}");
    }
    for (name, why) in [
        ("adjacency", "Geometry::Rests"),
        ("evidence-kinds", "Geometry::Rests"),
    ] {
        assert!(
            src.contains(&format!("(\"{name}\", {why})")),
            "{name} was found resting on the premise by this phase and is no longer classified so"
        );
    }
}

/// **One hand table of files was unheld, and one is not a pattern.**
///
/// `retrieval.rs`'s `READ` named two catalog entries and the documents each records as read, under
/// a header stating that *"two entries have a list at all"* — a claim about `.yidam/catalog/`, made
/// without reading it. It reads it now: `SHAPE` covers all fourteen files and `READ` must name
/// exactly the two classified as a list.
///
/// **It was offered as the pattern this phase would sweep and it is one instance.** The survey
/// looked for hand tables of files across both workspaces and found that almost every one is a
/// *citation* — a test naming the two records it is about — rather than a copy of a listing, and
/// that the two which are copies (`resolvers.rs`, `lineage.rs`) already hold themselves against a
/// directory scan without either arguing for it. `.yidam/decisions/unread-list.yml` is on record
/// that one instance is not a convention, and that standard applies to a claim this repository
/// makes about its own work as much as to one it makes about adventure games.
#[test]
fn the_one_table_of_files_reads_its_directory() {
    let root = repo_root();
    let src = std::fs::read_to_string(root.join("crates/design-gate/tests/retrieval.rs"))
        .expect("readable");
    let entries = std::fs::read_dir(root.join(".yidam/catalog"))
        .expect("the catalog")
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "md"))
        .count();
    assert_eq!(entries, 16, "files in the catalog");
    assert!(
        src.contains("a catalog entry with no row, or a row with no entry"),
        "`retrieval.rs` claims two entries have a document list and does not read the catalog"
    );

    // and the two that did it right all along, neither of which says so
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
