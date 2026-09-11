//! **A class that argues routes the argument to a node, and the route is written seven ways.**
//!
//! The prelude is categorical: *"A claim tag cannot reach a class definition… a class that has an
//! argument worth counting should be putting it in one."* This corpus obeys it — every class file
//! that states a proposition names the node carrying the tag — and the obedience is **undeclared**.
//! Nine pointers across six class files, in **eight spellings**: *"carries the argument and its
//! tag"*, *"…and its evidence"*, *"The claim is carried, tagged and counted by"*, *"The claim is
//! carried and tagged by"*, *"and its tag are carried by"*, *"is the instance and carries the
//! argument"*, *"…and carries the evidence"*, *"carries the figures"*. Nine routes and eight ways
//! to write one — the corpus has followed a rule it never wrote down, and reinvented the sentence
//! almost every time.
//!
//! **Four of the ten name their target in backticks rather than as a link**, so `edges.rs` cannot
//! resolve them, `graph-check` cannot see them, and a reader cannot click them. The convention is
//! followed and the follow-through is not checkable.
//!
//! **And one carrier is a phase behind.** `interaction.ont.yml` asserts *"in every engine read here
//! the effect list and the presentation list are the same list"* and routes it to
//! `interaction/binary-binding`, which argues it from one engine and 1,328 call sites.
//! `.yidam/decisions/forty-eight.yml` made it two and `.yidam/decisions/third-population.yml` made
//! it three. The class's quantifier grew and the node holding its evidence did not hear.
//!
//! `.yidam/decisions/class-evidence.yml`.

use design_gate::repo_root;
use std::path::{Path, PathBuf};

fn classes(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root.join(".yidam/corpus"))
        .expect("the corpus")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".ont.yml"))
        .collect();
    out.sort();
    out
}

/// The phrases a class file uses to hand its argument to a node.
const ROUTES: [&str; 8] = [
    "carries the argument and its tag",
    "carries the argument and its evidence",
    "claim is carried, tagged and counted by",
    "claim is carried and tagged by",
    "its tag are carried by",
    "is the instance and carries the argument",
    "is the instance and carries the evidence",
    "carries the figures",
];

/// Prose wraps, so a phrase is matched against the text with its line breaks flattened. The first
/// version of this file matched raw bytes and found seven routes of ten: two of the three it
/// missed were one phrase split across a line ending.
fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every route a class file writes, as (class, phrase).
fn routed(root: &Path) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    for p in classes(root) {
        let src = flat(&std::fs::read_to_string(&p).expect("readable"));
        let name = p
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .replace(".ont.yml", "");
        for r in ROUTES {
            for _ in 0..src.matches(r).count() {
                out.push((name.clone(), r));
            }
        }
    }
    out
}

/// **Nine routes, eight spellings, and nothing declares the convention.**
///
/// Pinned as a census. Six class files hand an argument to a node and nearly every one reaches for
/// its own words; the corpus has done this correctly nine times without writing down that it is a
/// rule. That is the eighth undeclared convention this crate has had to learn by measuring it,
/// and the first where following the rule was never the problem.
#[test]
fn a_class_that_argues_routes_to_a_node_in_eight_spellings() {
    let root = repo_root();
    let routes = routed(&root);
    assert_eq!(routes.len(), 9, "routes written: {routes:?}");

    let mut spellings: Vec<&str> = routes.iter().map(|(_, r)| *r).collect();
    spellings.sort();
    spellings.dedup();
    assert_eq!(spellings.len(), 8, "distinct phrasings for one convention");

    let mut files: Vec<&String> = routes.iter().map(|(c, _)| c).collect();
    files.sort();
    files.dedup();
    assert_eq!(
        files,
        [
            "conversation",
            "cutscene",
            "data-format",
            "interaction",
            "verb",
            "world-flag"
        ],
        "class files that route an argument"
    );
}

/// **Four of the ten routes name their target in backticks, where nothing can follow them.**
///
/// `prose-edges` established that a citation written as prose is a citation the graph cannot see,
/// and `edges.rs` holds every path in corpus prose to resolving. A backticked name is not a path,
/// so these four are outside both: the reader is told where the argument lives and given no way to
/// go there.
#[test]
fn four_routes_point_at_a_name_rather_than_a_file() {
    let root = repo_root();
    let mut unlinked = Vec::new();
    for p in classes(&root) {
        let src = flat(&std::fs::read_to_string(&p).expect("readable"));
        for r in ROUTES {
            let mut from = 0;
            while let Some(i) = src[from..].find(r) {
                let at = from + i;
                // the target is named in the sentence around the phrase, either side of it
                let window = &src[at.saturating_sub(120)..(at + r.len() + 120).min(src.len())];
                if !window.contains("](") {
                    unlinked.push(
                        p.file_name()
                            .expect("a name")
                            .to_string_lossy()
                            .replace(".ont.yml", ""),
                    );
                }
                from = at + r.len();
            }
        }
    }
    unlinked.sort();
    assert_eq!(
        unlinked,
        ["data-format", "interaction", "world-flag", "world-flag"],
        "routes whose target is a backticked name rather than a link"
    );
}

/// **The carrier of the corpus's widest claim argues it from one engine.**
///
/// `interaction.ont.yml` quantifies over *every engine read here*. When it was written that was
/// one — Delores, 1,328 call sites, 7.3% state writes — and `interaction/binary-binding` carries
/// that argument with that citation. Two phases have since added AGS's action table and SCUMM's
/// opcode list, so the quantifier now ranges over three and the node holding its evidence names
/// one.
///
/// Nothing was wrong when it was written. What is missing is any way for a node carrying a class's
/// argument to hear that the class's evidence grew.
#[test]
fn the_carrier_names_every_engine_the_class_quantifies_over() {
    let root = repo_root();
    let class =
        std::fs::read_to_string(root.join(".yidam/corpus/interaction.ont.yml")).expect("readable");
    assert!(
        class.contains("in every engine read here"),
        "the class still quantifies over every engine"
    );

    let carrier =
        std::fs::read_to_string(root.join(".yidam/corpus/interaction/binary-binding.yml"))
            .expect("readable");
    let named = ["delores", "ags-source", "scumm-v5"]
        .iter()
        .filter(|s| carrier.contains(*s))
        .count();
    assert_eq!(
        named, 1,
        "sources the carrier cites, against the three the corpus has read"
    );
}
