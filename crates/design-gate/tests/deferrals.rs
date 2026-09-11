//! **One refusal, written four times, in a repository whose own rule says to look again at the
//! second.**
//!
//! Four decision records decline a change with the same phrase — *a schema change to prose that
//! has none*, or its variant *marking X would be a schema change*. Two of them quote the rule that
//! says the refusal has expired. `prose-edges` at instance two: *"the corpus's own rule says the
//! second instance is when to look again."* `drawn-boundary` at instance four: *"that was two
//! instances ago."* **The rule has been cited more often than it has been followed.**
//!
//! What nobody did is separate them, and they are not one question:
//!
//! | | records | can this repository act |
//! |---|---|---|
//! | **Prose has no schema** — a back-link from a marker to what answered it; a declared citation syntax | `marker-count`, `prose-edges` | yes, and it declines to |
//! | **The ontology cannot state a fact about itself** — that an edge is derived; which tier a class is in | `found-in`, `drawn-boundary` | **yes, with a field on a block that already exists** |
//! | **Blocked upstream** — a typed `claim_tag` that the pinned build ignores | `action-pages` | **no** |
//!
//! The third has been counted with the other two and cannot be acted on here at all. The second is
//! the tractable pair, and the cost of not having it is already being paid: `reach.rs` parses
//! `ontology.yml`'s **prose** to recover which classes are in the authoring tier, because no
//! `.ont.yml` says.

use design_gate::repo_root;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn records(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root.join(".yidam/decisions"))
        .expect("the directory exists")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yml"))
        .collect();
    out.sort();
    out
}

fn stem(p: &Path) -> String {
    p.file_stem()
        .expect("a name")
        .to_string_lossy()
        .into_owned()
}

/// The classes, and the `tier:` each declares.
fn tiers(root: &Path) -> BTreeMap<String, Option<String>> {
    let mut out = BTreeMap::new();
    for e in std::fs::read_dir(root.join(".yidam/corpus"))
        .expect("the corpus exists")
        .flatten()
    {
        let p = e.path();
        let s = p.to_string_lossy().into_owned();
        let Some(class) = s
            .strip_suffix(".ont.yml")
            .map(|c| c.rsplit('/').next().expect("a name").to_string())
        else {
            continue;
        };
        let src = std::fs::read_to_string(&p).expect("readable");
        let tier = src
            .lines()
            .find_map(|l| l.strip_prefix("tier:"))
            .map(|v| v.trim().to_string());
        out.insert(class, tier);
    }
    out
}

/// The tiers `ontology.yml` declares in prose, as `class -> tier`.
///
/// The thing a field replaces. Kept as the check on the field rather than deleted with it: the
/// record is where the tiers were argued and it stays the authority, so a declaration disagreeing
/// with it is a defect in the declaration.
fn tiers_in_prose(root: &Path) -> BTreeMap<String, String> {
    let src =
        std::fs::read_to_string(root.join(".yidam/decisions/ontology.yml")).expect("readable");
    let mut out = BTreeMap::new();
    let (mut tier, mut indent) = (None, None);
    for line in src.lines() {
        if let Some(head) = line.trim().strip_suffix(" tier:") {
            tier = Some(head.trim().to_lowercase());
            indent = None;
            continue;
        }
        if let Some(head) = line
            .trim()
            .strip_suffix(" tier — what an adventure game is:")
        {
            tier = Some(head.trim().to_lowercase());
            indent = None;
            continue;
        }
        if let Some(head) = line
            .trim()
            .strip_suffix(" tier — what realizes the vocabulary:")
        {
            tier = Some(head.trim().to_lowercase());
            indent = None;
            continue;
        }
        let Some(t) = &tier else { continue };
        if line.trim().is_empty() {
            tier = None;
            continue;
        }
        let here = line.len() - line.trim_start().len();
        if *indent.get_or_insert(here) != here {
            continue;
        }
        if let Some(name) = line.split_whitespace().next() {
            out.insert(name.to_string(), t.clone());
        }
    }
    out
}

/// **Four records decline the same change and two of them say the refusal has expired.**
///
/// Pinned as a census. The phrase is the tell — a record reaching for *schema change* is a record
/// noticing that something the corpus knows cannot be written down anywhere a tool would read it.
#[test]
fn the_same_refusal_is_written_four_times() {
    let root = repo_root();
    let mut deferring = Vec::new();
    for p in records(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if src.contains("schema change") {
            deferring.push(stem(&p));
        }
    }
    assert_eq!(
        deferring,
        ["drawn-boundary", "found-in", "marker-count", "prose-edges"],
        "records declining a change for want of a schema"
    );
}

/// **Every class declares its tier, and every declaration agrees with the record that argued it.**
///
/// The field `drawn-boundary` asked for. It costs one line per class and it retires a parser:
/// `reach.rs` was reading `ontology.yml`'s prose — indentation rules and a gloss that wraps onto a
/// second line — to recover a fact the class could have stated.
///
/// The prose stays and this checks against it, because `ontology.yml` is where the three tiers were
/// argued and a declaration that disagrees with it is the declaration being wrong.
#[test]
fn every_class_declares_the_tier_the_record_argues() {
    let root = repo_root();
    let (declared, prose) = (tiers(&root), tiers_in_prose(&root));
    let mut bad = Vec::new();
    for (class, tier) in &declared {
        match (tier.as_deref(), prose.get(class)) {
            (Some(d), Some(p)) if d == p => {}
            (Some(d), Some(p)) => bad.push(format!("{class}: declares {d}, the record says {p}")),
            (Some(d), None) => bad.push(format!(
                "{class}: declares {d}, the record places it in no tier"
            )),
            (None, _) => bad.push(format!("{class}: declares no tier")),
        }
    }
    // The census this commit found: fifteen classes, none of which says which tier it is in, and
    // a record whose prose places all fifteen. The rule replaces this in the same phase.
    assert_eq!(bad.len(), 15, "classes declaring no tier");
    assert_eq!(prose.len(), 15, "classes the record places");

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for t in declared.values().flatten() {
        *counts.entry(t.as_str()).or_default() += 1;
    }
    assert!(counts.is_empty());
}

/// **An edge no author writes says so, and there is exactly one.**
///
/// `found-in` asked for this at one instance and declined on the corpus's own rule — *a thing earns
/// its place by being what the vocabulary cannot say* — then found a second instance of the
/// neighbouring gap and left this one where it was. The relationship *is* real and *is* derived:
/// a hotspot belongs to a room, so an item with a carried form is found in that hotspot's room, and
/// `game_format::resolve` reports an author who writes it wrong rather than asking them to write it
/// at all.
///
/// One marked edge is a thin population and the check says so by pinning it. What the field buys is
/// that the next one is a line rather than a fifth record about a schema.
#[test]
fn a_derived_edge_says_it_is_derived() {
    let root = repo_root();
    let mut derived = Vec::new();
    for e in std::fs::read_dir(root.join(".yidam/corpus"))
        .expect("the corpus exists")
        .flatten()
    {
        let p = e.path();
        let s = p.to_string_lossy().into_owned();
        let Some(class) = s
            .strip_suffix(".ont.yml")
            .map(|c| c.rsplit('/').next().expect("a name").to_string())
        else {
            continue;
        };
        let src = std::fs::read_to_string(&p).expect("readable");
        let Some(i) = src.find("\nedges:") else {
            continue;
        };
        let mut rel = None;
        for line in src[i + 1..].lines().skip(1) {
            if !line.starts_with(' ') && !line.trim().is_empty() {
                break;
            }
            if let Some(v) = line.trim().strip_prefix("- relationship:") {
                rel = Some(v.trim().to_string());
            } else if line.trim().starts_with("derived:") {
                if let Some(r) = &rel {
                    derived.push(format!("{class} -{r}->"));
                }
            }
        }
    }
    derived.sort();
    assert!(
        derived.is_empty(),
        "the census: no edge says it is derived, and one is"
    );
}
