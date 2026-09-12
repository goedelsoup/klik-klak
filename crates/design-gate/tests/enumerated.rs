//! **A fifth specimen was written eleven commits ago and twenty-three tests have not read it.**
//!
//! `design/specimen/` holds five games. **Twenty-three test files hold a list of four** —
//! `ferry`, `lighthouse`, `nightwatch`, `patrol` — as `include_str!` constants, which is the set
//! that existed before `.yidam/decisions/control-transfer.yml` wrote `lock.game`. Every census
//! those files take is a census of **662 of 904 authored lines and 91 of 120 bindings**, reported
//! as if it were all of them.
//!
//! **Not one of them failed.** The re-pin cascade every phase here runs on is driven by failures:
//! a test whose figures move gets looked at, and a test whose figures hold does not. A test whose
//! *input is a constant* cannot have its figures move. `walked.rs` and `lost.rs` were extended by
//! hand during that phase — because those two happened to fail, both being about a property the
//! new game had — and the other twenty-three went on passing while reading four fifths of the
//! material.
//!
//! **The gate catches what it reads.** That is the whole finding, and it has two consequences
//! already on disk:
//!
//! - `world-flag/latched-flag` carries a **`[verified]` claim that is false**: `clear` *"used zero
//!   times in 662 authored lines"*, citing `crates/reachability/tests/latch.rs`. `lock.game`
//!   clears a flag three times. The test still reports zero because it cannot see the file.
//! - **Twenty living documents quote a stale specimen figure** — seventeen corpus nodes, two
//!   design documents and one catalog row — and four of the six open questions sit in files
//!   carrying one, including the only question anybody can act on.
//!
//! Six files read the directory instead and are the reason this is not universal:
//! `calculators.rs`, `settled.rs`, `authored.rs`, `coverage.rs`, `declared.rs`, `transfer.rs`.
//! What separates them from the twenty-three is not care. It is that their input is the artifact
//! rather than a copy of it.
//!
//! This file is the census. The rule it becomes is in `.yidam/decisions/unread-specimen.yml`.

use design_gate::repo_root;
use game_format::{parse, resolve, Effect};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Every `tests/*.rs` in either workspace.
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
                    && p.parent().is_some_and(|d| d.ends_with("tests"))
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

/// Path, relative to the root, in the form the census below lists.
fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

/// **Every test file holding a list of specimens that was complete when it was written.**
///
/// Hand-listed rather than computed, because the claim is that *this* is the set and a computed
/// list would only ever agree with itself. Twenty-three, in two crates, and the striking thing is
/// how ordinary they are: they are not a corner of the suite, they are most of what reads a
/// specimen at all.
const INCOMPLETE: &[&str] = &[
    "crates/game-format/tests/cells.rs",
    "crates/game-format/tests/duplication.rs",
    "crates/game-format/tests/ignored.rs",
    "crates/game-format/tests/normal_form.rs",
    "crates/game-format/tests/round_trip.rs",
    "crates/game-format/tests/written.rs",
    "crates/reachability/tests/arity.rs",
    "crates/reachability/tests/arrival.rs",
    "crates/reachability/tests/conditions.rs",
    "crates/reachability/tests/containment.rs",
    "crates/reachability/tests/defaults.rs",
    "crates/reachability/tests/exits.rs",
    "crates/reachability/tests/identity.rs",
    "crates/reachability/tests/latch.rs",
    "crates/reachability/tests/menu.rs",
    "crates/reachability/tests/orphans.rs",
    "crates/reachability/tests/scaling.rs",
    "crates/reachability/tests/silence.rs",
    "crates/reachability/tests/spend.rs",
    "crates/reachability/tests/threshold.rs",
    "crates/reachability/tests/unread.rs",
    "crates/reachability/tests/verbs.rs",
    "crates/reachability/tests/written.rs",
];

/// **Two files hold a list that is complete today, by hand, and will go stale the same way.**
///
/// `walked.rs` and `lost.rs` were extended when `lock.game` landed, and the reason they were is
/// the reason to distrust them: both failed. Six stranding bindings became seven and a transcript
/// count moved. Nothing extended them on purpose; a failure did, and the next specimen may not
/// oblige.
const COMPLETE_BY_HAND: &[&str] = &[
    "crates/reachability/tests/walked.rs",
    "engine/play/tests/lost.rs",
];

/// **And six read the directory.**
const ENUMERATING: &[&str] = &[
    "crates/design-gate/tests/calculators.rs",
    "crates/design-gate/tests/settled.rs",
    "crates/game-format/tests/authored.rs",
    "crates/game-format/tests/coverage.rs",
    "crates/game-format/tests/declared.rs",
    "crates/game-format/tests/transfer.rs",
];

/// **Twenty-three tests carry a list of four, and the directory holds five.**
///
/// The partition is the census: what a file names against what is there. A file naming a proper
/// subset chose one — `effects.rs` reads `ferry` and `lighthouse` because they are the two that
/// write the effect it is about, and that stays a choice however many specimens are written. A
/// file naming **all of them** is not making a choice. It is holding a copy of the directory, and
/// the copy is what goes stale.
#[test]
fn twenty_three_tests_read_four_of_the_five_specimens() {
    let root = repo_root();
    let all = on_disk(&root);
    assert_eq!(all.len(), 5, "specimens on disk");

    let (mut incomplete, mut by_hand, mut enumerating, mut study) =
        (Vec::new(), Vec::new(), Vec::new(), 0usize);
    for p in test_files(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let names = named(&src);
        let reads_dir = src.contains("join(\"design/specimen\")");
        if reads_dir && names.is_empty() {
            enumerating.push(rel(&root, &p));
        } else if names.is_empty() {
            continue;
        } else if names == all {
            by_hand.push(rel(&root, &p));
        } else if names.len() > 1 && all.iter().filter(|n| !names.contains(*n)).count() == 1 {
            // the set that was complete before `lock.game`, and is not now
            incomplete.push(rel(&root, &p));
        } else {
            study += 1;
        }
    }

    assert_eq!(incomplete, INCOMPLETE, "files holding a list of four");
    assert_eq!(by_hand, COMPLETE_BY_HAND, "files holding a list of five");
    assert_eq!(enumerating, ENUMERATING, "files reading the directory");
    assert_eq!(study, 13, "files naming the games they are about");
}

/// **What the twenty-three measure, against what there is to measure.**
///
/// Every figure in this repository that begins *"across the specimens"* and was computed by one of
/// those files is this ratio: **73% of the lines and 76% of the bindings.** The remaining quarter
/// is a whole game, with three rooms, two actors, a switch of controlled character and a committed
/// ordering defect — which is to say the material added precisely to be unlike the other four.
#[test]
fn the_census_those_tests_take_covers_three_quarters_of_the_material() {
    let root = repo_root();
    let dir = root.join("design/specimen");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .collect();
    paths.sort();

    let (mut all, mut four) = ((0usize, 0usize, 0usize), (0usize, 0usize, 0usize));
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
        let row = (src.lines().count(), g.bindings.len(), g.declared.len());
        all = (all.0 + row.0, all.1 + row.1, all.2 + row.2);
        if name != "lock" {
            four = (four.0 + row.0, four.1 + row.1, four.2 + row.2);
        }
    }
    assert_eq!(
        all,
        (904, 120, 104),
        "lines, bindings and declared names on disk"
    );
    assert_eq!(four, (662, 91, 81), "and what the twenty-three see");
}

/// **A `[verified]` claim in the corpus is false, and the test it cites still reports the number
/// that makes it true.**
///
/// `world-flag/latched-flag` argues that a latched flag is the normal case and an unlatched one
/// wants a reason, and offers as evidence that `clear` — admitted to the language, one of eight
/// effects — had **never been written**. `lock.game` writes it three times.
///
/// The argument survives and is sharpened by what it clears: `ashore`, the flag naming which
/// character the player is driving, which is the one flag in five games that is not a fact about
/// the world (`.yidam/decisions/control-transfer.yml`). A latch is still the normal case. But the
/// sentence carrying the claim says zero, says it in the `[verified]` tag, and cites a test that
/// agrees with it only by not reading the file.
#[test]
fn the_claim_that_no_author_has_ever_cleared_a_flag_is_false() {
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
        "and every one of them writes the flag naming the controlled character: {cleared:?}"
    );

    let node = std::fs::read_to_string(root.join(".yidam/corpus/world-flag/latched-flag.yml"))
        .expect("readable");
    let flat = node.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("zero times in 662 authored lines"),
        "the false sentence is the one this test is about; if it has been rewritten, this check \
         is the thing to rewrite with it"
    );
}

/// **Twenty living documents quote a figure the fifth specimen moved.**
///
/// Records are not counted: a record says what was decided when it was written and
/// `.yidam/decisions/record-lifecycle.yml` is why they are not edited to stay true. A corpus node
/// is the opposite — it is the copy every tool reads, which is the finding
/// `.yidam/decisions/retraction-reach.yml` reached one phase ago by a different route.
///
/// **Four of the six open questions are in files carrying one of these**, and one of them is
/// `data-format/interaction-table`, whose marker describes the material for the measurement that
/// would close it: *"Four specimens, 662 lines and 91 bindings are authored in that syntax."*
#[test]
fn twenty_living_documents_quote_a_stale_specimen_figure() {
    let root = repo_root();
    const STALE: [&str; 6] = [
        "four specimens",
        "four committed specimen",
        "662",
        "91 bindings",
        "81 declared",
        "105 authored",
    ];
    let mut quoting: Vec<String> = Vec::new();
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
            if STALE.iter().any(|s| flat.contains(s)) {
                quoting.push(rel(&root, &p));
            }
        }
    }
    assert_eq!(
        quoting.len(),
        20,
        "documents quoting a stale figure: {quoting:#?}"
    );

    // and the question that would be answered by counting is described in the stale figures
    let marker =
        std::fs::read_to_string(root.join(".yidam/corpus/data-format/interaction-table.yml"))
            .expect("readable");
    let flat = marker.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("Four specimens, 662 lines and 91 bindings"),
        "the only actionable question describes its own material, and describes it wrong"
    );
}
