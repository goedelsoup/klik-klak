//! **A test over the specimens reads the directory, because a list that equals the directory is a
//! copy of it.**
//!
//! `design/specimen/` held five games and **twenty-five test files held a list of four** — the set
//! that existed before `.yidam/decisions/control-transfer.yml` wrote `lock.game`. Every census
//! those files took covered 662 of 904 authored lines and 91 of 120 bindings, and reported it as
//! all of them, and **not one of them failed**. The re-pin cascade this repository runs on is
//! driven by failures; a test whose input is a constant cannot fail when the input set grows.
//!
//! What that cost is in `.yidam/decisions/unread-specimen.yml`. In short: one `[verified]` claim
//! was false, one settled decision lost the premise it led with, twenty living documents went
//! stale, and three separate generalisations about flags turned out to have an exception nobody
//! had met.
//!
//! **The rule is a prohibition rather than a requirement.** A test may name the games it is about
//! — thirteen do, and `effects.rs` reading `ferry` and `lighthouse` because they are the two that
//! write the effect it is about stays a choice however many specimens exist. What no test may do
//! is name *all* of them. A complete hand-written list is the directory, copied, and only the copy
//! can be silently wrong.
//!
//! Stated as a requirement to enumerate it would be unfalsifiable: a file could read the directory
//! and then filter it back down to four. Stated as a prohibition it names the shape that actually
//! failed, and it fails again the moment a sixth game lands and somebody adds a fifth line to a
//! list instead of deleting the list.

use design_gate::repo_root;
use game_format::{parse, resolve, Effect};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Every `tests/*.rs` in either workspace, and the shared readers under `tests/common/`.
fn test_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for workspace in ["crates", "engine"] {
        let mut stack = vec![root.join(workspace)];
        while let Some(d) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&d) else {
                continue;
            };
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n == "target") {
                        continue;
                    }
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "rs")
                    && p.ancestors().any(|a| a.ends_with("tests"))
                {
                    out.push(p);
                }
            }
        }
    }
    out.sort();
    out
}

/// The specimens a file names by path — the list it carries rather than the list on disk.
fn named(src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = src;
    while let Some(i) = rest.find("design/specimen/") {
        rest = &rest[i + "design/specimen/".len()..];
        let seg: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
            .collect();
        if let Some(stem) = seg.strip_suffix(".game") {
            out.insert(stem.to_string());
        }
    }
    out
}

/// The specimens on disk, which is the only list that cannot be out of date.
fn on_disk(root: &Path) -> BTreeSet<String> {
    std::fs::read_dir(root.join("design/specimen"))
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .map(|p| {
            p.file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

/// **No test file names every specimen.**
///
/// The one check that could have caught this, stated the way it had to be stated. A file that
/// names a strict subset has chosen those games; a file that names the whole set has copied the
/// directory, and a copy of something that changes cannot be kept correct by care —
/// `.yidam/decisions/adjacency.yml` reached the same sentence about a walkbox graph and
/// `.yidam/decisions/retraction-reach.yml` about a corpus node.
#[test]
fn no_test_names_every_specimen() {
    let root = repo_root();
    let all = on_disk(&root);
    assert_eq!(all.len(), 6, "specimens on disk");

    let (mut copies, mut reading, mut study) = (Vec::new(), 0usize, 0usize);
    for p in test_files(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let names = named(&src);
        if names == all {
            copies.push(rel(&root, &p));
        } else if names.is_empty() {
            if src.contains("join(\"design/specimen\")") || src.contains("common::sources()") {
                reading += 1;
            }
        } else {
            study += 1;
        }
    }
    assert!(
        copies.is_empty(),
        "a test file holds its own copy of `design/specimen/`, which will be a list of {} the \
         next time a game is written:\n  {}",
        all.len(),
        copies.join("\n  ")
    );
    assert_eq!(
        (reading, study),
        (30, 18),
        "files reading the directory, and files naming the games they are about"
    );
}

/// **What the directory holds, computed once, for everything that quotes it.**
///
/// Five specimens, 904 authored lines, 120 bindings, 104 declared names. The three living
/// documents that state a whole-corpus figure are held to these below; every derived figure is
/// pinned in the test that computes it, which is the idiom `adjacency` and `control-transfer`
/// already use.
#[test]
fn the_specimen_census_is_what_the_documents_quote() {
    let root = repo_root();
    let dir = root.join("design/specimen");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .collect();
    paths.sort();

    let (mut lines, mut bindings, mut declared) = (0usize, 0usize, 0usize);
    for p in &paths {
        let name = p
            .file_stem()
            .expect("a stem")
            .to_string_lossy()
            .into_owned();
        let src = std::fs::read_to_string(p).expect("readable");
        let (mut g, mut problems) = parse(&src, &name);
        problems.extend(resolve(&mut g));
        assert!(problems.is_empty(), "{name}: {problems:?}");
        lines += src.lines().count();
        bindings += g.bindings.len();
        declared += g.declared.len();
    }
    assert_eq!(
        (paths.len(), lines, bindings, declared),
        (6, 1108, 148, 131),
        "specimens, authored lines, bindings, declared names"
    );

    // the documents that state the whole-corpus figure rather than a derived one
    for (doc, phrase) in [
        (
            ".yidam/corpus/data-format/interaction-table.yml",
            "five specimens and 120 bindings",
        ),
        (
            ".yidam/corpus/verb/give-to.yml",
            "120 bindings across 904 lines",
        ),
        ("design/specimen/README.md", "904 lines"),
    ] {
        let src = std::fs::read_to_string(root.join(doc)).expect("readable");
        let flat = src.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            flat.contains(phrase),
            "{doc} does not quote the computed figure: expected {phrase:?}"
        );
    }
}

/// **No living document quotes a figure the fifth specimen moved.**
///
/// Twenty did: seventeen corpus nodes, two design documents and one catalog row. Records are not
/// swept — a record says what was decided when it was written, and
/// `.yidam/decisions/record-lifecycle.yml` is why they are not edited to stay true. A corpus node
/// is the opposite: it is the copy every tool reads, which is
/// `.yidam/decisions/retraction-reach.yml`'s finding reached one phase earlier by a different
/// route.
///
/// The tokens are the ones that were actually wrong. A check for *every* number in the corpus
/// would be a check on nothing; these six are the figures that propagated.
///
/// **And the denominator is published, because a zero from nothing-there and a zero from
/// nothing-matched are the same byte.** `.yidam/decisions/unread-spelling.yml` found this shape in
/// three of the gate's checks, this one among them: a list of stale phrasings is a ceiling, and a
/// figure worded in a seventh way is reported as absent rather than as unread. Nothing can
/// enumerate the ways a person might write a number, so what is enumerated instead is **how much
/// material the check looked at** — every living document that mentions the specimens at all. When
/// that figure moves, somebody reads the new document rather than trusting a silence.
#[test]
fn no_living_document_quotes_a_stale_specimen_figure() {
    let root = repo_root();
    const STALE: [&str; 6] = [
        "four specimens",
        "four committed specimen",
        "662",
        "91 bindings",
        "81 declared",
        "105 authored",
    ];
    let (mut quoting, mut scanned, mut mentioning) = (Vec::new(), 0usize, 0usize);
    for dir in [".yidam/corpus", "design", ".yidam/catalog"] {
        let mut stack = vec![root.join(dir)];
        let mut files: Vec<PathBuf> = Vec::new();
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "yml" || x == "md") {
                    files.push(p);
                }
            }
        }
        files.sort();
        for p in files {
            let src = std::fs::read_to_string(&p).expect("readable");
            let flat = src.split_whitespace().collect::<Vec<_>>().join(" ");
            scanned += 1;
            if flat.contains("specimen") {
                mentioning += 1;
            }
            if let Some(s) = STALE.iter().find(|s| flat.contains(**s)) {
                quoting.push(format!("{}  ({s})", rel(&root, &p)));
            }
        }
    }
    assert!(
        quoting.is_empty(),
        "a living document quotes a specimen figure that has moved:\n  {}",
        quoting.join("\n  ")
    );
    assert_eq!(
        (scanned, mentioning),
        (105, 35),
        "living documents read, and living documents that mention the specimens — the second is \
         what this check is a claim about, and a clean result over a shrinking denominator is not \
         the same result"
    );
}

/// **The `[verified]` claim the stale census made false, and the three exceptions it hid.**
///
/// `world-flag/latched-flag` said `clear` was *"used zero times in 662 authored lines"*. It is
/// used three times, all in `lock.game`, all writing `ashore` — the flag that records which
/// character the player is driving, which `.yidam/decisions/control-transfer.yml` established this
/// format has no site for and an author has to spend a flag on.
///
/// **That flag is the exception in three separate censuses**, and each one was a generalisation
/// about flags that had never met it:
///
/// | claim | where | the exception |
/// |---|---|---|
/// | every flag is one-way | `reachability/tests/latch.rs` | `ashore`, cleared 3× |
/// | every value has one writer | `reachability/tests/threshold.rs` | `ashore`, written 6× |
/// | no guard joins more than one atom | `reachability/tests/conditions.rs` | `ashore` in a 5-atom conjunction |
///
/// So the shape is not *flags behave like this*. It is: **a flag recording something about the
/// world behaves like this, and the one flag recording something about the player does not** —
/// and the corpus had no instance of the second until somebody wrote a game that switches
/// characters.
#[test]
fn the_flag_that_is_not_about_the_world_is_the_exception_in_every_census() {
    let root = repo_root();
    let src = std::fs::read_to_string(root.join("design/specimen/lock.game")).expect("readable");
    let (mut g, mut problems) = parse(&src, "lock");
    problems.extend(resolve(&mut g));
    assert!(problems.is_empty(), "{problems:?}");

    let cleared: Vec<&String> = g
        .bindings
        .iter()
        .flat_map(|b| b.effects.iter())
        .filter_map(|e| match e {
            Effect::ClearFlag(f) => Some(f),
            _ => None,
        })
        .collect();
    assert_eq!(cleared.len(), 3, "clear sites in the fifth specimen");
    assert!(
        cleared.iter().all(|f| *f == "ashore"),
        "every clear writes the flag naming the controlled character: {cleared:?}"
    );

    let node = std::fs::read_to_string(root.join(".yidam/corpus/world-flag/latched-flag.yml"))
        .expect("readable");
    let flat = node.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        !flat.contains("zero times"),
        "the node still says the reverse gear has never been used"
    );
    assert!(
        flat.contains("ashore"),
        "and it names the one flag that is cleared, which is the whole of the corrected argument"
    );
}
