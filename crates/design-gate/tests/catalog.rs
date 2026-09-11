//! **A catalog entry announces an artifact as unretrieved, and the catalog holds it.**
//!
//! `twp-dev-diary.md` closes its unread-list on the Delores source: *"Nobody here has read it. It
//! is the highest-value retrieval this corpus has ever identified."* The artifact it names by URL
//! is `delores-source.md` — `obtained: true`, retrieved the next day, 910 lines, and the evidence
//! base of **27 of this corpus's 43 nodes**, plus a class page. The sentence has been false for
//! 272 commits.
//!
//! It is findable without reading prose. An unread-list naming a URL another entry declares in its
//! `location:` block is a mechanical condition, and across eight entries it fires exactly once.
//!
//! **The convention it breaks is followed by four of the eight and written down nowhere.** When an
//! item on an unread-list gets read, the list says so — in four entries, in four different
//! spellings, none of which any rule required:
//!
//! * *"has now been read and is recorded below"* — `scumm-v5-opcode-reference`
//! * *"has since been read, in part"* — `grumpy-gamer-pdc`
//! * *"have since been read in full"* — `delores-source`
//! * *"was the item at the top of this list and it has been read"* — `ags-manual`
//!
//! This is the ninth undeclared convention this repository has found by measuring, and the second
//! found with an instance already broken. `.yidam/decisions/unread-list.yml`.

use design_gate::repo_root;
use std::path::{Path, PathBuf};

fn entries(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root.join(".yidam/catalog"))
        .expect("readable")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md") && !p.ends_with("README.md"))
        .collect();
    out.sort();
    out
}

fn stem(p: &Path) -> String {
    p.file_name()
        .expect("a name")
        .to_string_lossy()
        .into_owned()
}

/// The URLs an entry declares as the artifact's home, scheme stripped so a prose mention without
/// one still matches.
fn locations(src: &str) -> Vec<String> {
    let fm = src.split("\n---").next().unwrap_or(src);
    fm.lines()
        .filter_map(|l| l.trim().strip_prefix("value: "))
        .filter_map(|v| v.split_once("://"))
        .map(|(_, rest)| rest.trim_end_matches('/').to_string())
        .collect()
}

/// The section an entry keeps for what it has not read.
///
/// Seven of the eight head it *"What this … holds that nobody here has looked at"*; the eighth
/// opens a line with *"Not read:"* instead. The section convention is as undeclared as what goes
/// in it, and the variant spelling is counted rather than corrected.
///
/// Anchored on the phrase and then backed up to its heading. The first version of this matched
/// `"## What th"`, which also catches `delores-source`'s *"What the artifact is"* 835 lines
/// earlier — so the entry that records its own retrieval best was the one the check read wrong.
fn unread(src: &str) -> &str {
    let i = match src.find("nobody here has looked at") {
        Some(at) => src[..at].rfind("\n## ").unwrap_or(0),
        None => src.find("\nNot read:").unwrap_or(src.len()),
    };
    let rest = &src[i..];
    match rest[1..].find("\n## ") {
        Some(j) => &rest[..j + 1],
        None => rest,
    }
}

/// **One unread-list names an artifact the catalog holds, and it is the largest one here.**
///
/// Not a stale figure and not a decayed inference — a sentence asserting that a retrieval has not
/// happened, in a repository where that retrieval is the single most-cited source it has. The
/// mechanism is the one `third-population` found on the other side: *a wrong figure in an
/// unread-list discourages the reading that would correct it*, and this is the same discouragement
/// pointed at a reading that already happened.
#[test]
fn no_unread_list_names_an_artifact_the_catalog_holds() {
    let root = repo_root();
    let all = entries(&root);
    assert_eq!(all.len(), 8, "catalog entries");

    let held: Vec<(String, Vec<String>)> = all
        .iter()
        .map(|p| {
            (
                stem(p),
                locations(&std::fs::read_to_string(p).expect("readable")),
            )
        })
        .collect();

    let mut named = Vec::new();
    for p in &all {
        let src = std::fs::read_to_string(p).expect("readable");
        let section = unread(&src);
        for (other, urls) in &held {
            if other == &stem(p) {
                continue;
            }
            for u in urls {
                if section.contains(u) {
                    named.push(format!("{} names {u} — held by {other}", stem(p)));
                }
            }
        }
    }
    assert_eq!(
        named,
        ["twp-dev-diary.md names github.com/grumpygamer/DeloresDev — held by delores-source.md"],
        "an entry's unread-list naming an artifact this catalog has retrieved"
    );
}

/// **Four of eight record what has since been read, in four spellings, and nothing asked them to.**
///
/// The discriminator is exact: the four that record a retrieval all write some form of *been
/// read* in the list itself, and the four that do not, do not. That is a convention with a
/// following and no statement — the ninth this repository has found by measurement, after class
/// routing, used-versus-discussed in eight spellings, and the rest.
///
/// Three of the four also name where the reading went: a record, a sibling entry, a section of
/// their own. The fourth names the item's former position in the list. None of them was asked for
/// any of it.
#[test]
fn half_the_unread_lists_record_what_has_since_been_read() {
    let root = repo_root();
    let mut records: Vec<String> = Vec::new();
    let mut silent: Vec<String> = Vec::new();
    for p in entries(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if unread(&src).contains("been read") {
            records.push(stem(&p));
        } else {
            silent.push(stem(&p));
        }
    }
    assert_eq!(
        records,
        [
            "ags-manual.md",
            "delores-source.md",
            "grumpy-gamer-pdc.md",
            "scumm-v5-opcode-reference.md",
        ],
        "unread-lists that say an item on them has since been read"
    );
    assert_eq!(
        silent,
        [
            "ags-272-manual.md",
            "ags-source.md",
            "twp-dev-diary.md",
            "wikipedia-scumm.md",
        ],
        "and the four that say nothing — one of which had the most to say"
    );

    // four instances, four spellings, and a rule would have produced one
    let spellings = [
        (
            "ags-manual.md",
            "the item at the top of this list and it has been read",
        ),
        ("delores-source.md", "have since been read in full"),
        ("grumpy-gamer-pdc.md", "has since been read, in part"),
        (
            "scumm-v5-opcode-reference.md",
            "has now been read and is recorded below",
        ),
    ];
    for (file, phrase) in spellings {
        let src =
            std::fs::read_to_string(root.join(".yidam/catalog").join(file)).expect("readable");
        assert!(
            unread(&src).contains(phrase),
            "{file} no longer spells it {phrase:?}"
        );
    }
}

/// **The artifact called unretrieved is the evidence base of two thirds of the corpus.**
///
/// 27 of 43 nodes cite `delores-source`, and `conversation.ont.yml` besides — more than any other
/// entry, and more than the four AGS entries together. That is what the sentence in the other file
/// denies, and the denial sits in the entry a reader opens *to decide what to retrieve next*,
/// which is the one place where being wrong about this costs something.
#[test]
fn the_artifact_called_unretrieved_is_the_most_cited_source_here() {
    let root = repo_root();
    let mut nodes = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml")
                && !p.to_string_lossy().ends_with(".ont.yml")
            {
                nodes.push(p);
            }
        }
    }
    assert_eq!(nodes.len(), 43, "corpus nodes");

    let mut counts: Vec<(usize, String)> = entries(&root)
        .iter()
        .map(|p| {
            let needle = format!("catalog/{}", stem(p));
            let n = nodes
                .iter()
                .filter(|q| {
                    std::fs::read_to_string(q)
                        .expect("readable")
                        .contains(&needle)
                })
                .count();
            (n, stem(p))
        })
        .collect();
    counts.sort_by(|a, b| b.0.cmp(&a.0));
    assert_eq!(
        counts[0],
        (27, "delores-source.md".to_string()),
        "the most-cited entry in this catalog, against {counts:?}"
    );
    assert!(
        counts[0].0 > counts[1].0 + 3,
        "and it is not close: {counts:?}"
    );
    assert!(
        std::fs::read_to_string(root.join(".yidam/corpus/conversation.ont.yml"))
            .expect("readable")
            .contains("catalog/delores-source"),
        "a class page cites it too, which the node count above excludes"
    );

    let diary =
        std::fs::read_to_string(root.join(".yidam/catalog/twp-dev-diary.md")).expect("readable");
    let delores =
        std::fs::read_to_string(root.join(".yidam/catalog/delores-source.md")).expect("readable");
    assert!(
        delores.contains("obtained: true"),
        "the entry for the artifact the other calls unretrieved"
    );
    assert!(
        unread(&diary).contains("Nobody here has read it"),
        "and the sentence, still standing"
    );

    // And the figure was already computed, in this directory, by a generator this repository
    // runs on every phase. `catalog-audit` prints a row per entry with the node count in it;
    // the row saying 27 and the sentence saying nobody has read it are three files apart and
    // nothing compares them. `inert-reports` called this shape "proximity is not a wire".
    let index = std::fs::read_to_string(root.join(".yidam/catalog/README.md")).expect("readable");
    let row = index
        .lines()
        .find(|l| l.starts_with("| [delores-source.md]"))
        .expect("the generated registry has a row for it");
    assert!(
        row.ends_with("| yes | — | 27 | 1 |"),
        "the generated row already carries the count the sentence denies: {row}"
    );
}
