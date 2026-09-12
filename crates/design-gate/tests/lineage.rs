//! **One of the four systems this corpus holds a `prior-art` node on has no source about itself.**
//!
//! `prior-art/sierra-sci.yml` makes eight claims about Sierra's two engines and cites exactly one
//! catalog entry: `delores-source.md`, a game written by different people on a different engine in
//! a different decade. Its own `source_standing` says so — *"Nothing retrieved… None of it has been
//! located or read, and the era range above is recollection"* — and that sentence has stood since
//! genesis while the node above it grew to argue about what Sierra's transition proves.
//!
//! The other three nodes each cite a source about their own subject. `ags` cites three, `scumm`
//! cites two, `thimbleweed-park` cites the diary its subject was written in. Nothing made the
//! difference visible, because a node that cites *any* catalog entry looks sourced to every check
//! here — which is the same shape `sourcing.rs` found in `verified-unsourced` and did not
//! generalise: one citation anywhere silences a node, including for the claims it has nothing to
//! do with.
//!
//! **This is that rule one level up.** `verified-unsourced` asks whether a node cites anything.
//! This asks whether it cites anything *about the thing it is a node about*, and the two came
//! apart on exactly one node, from genesis to this phase's eighty-eighth.

use design_gate::repo_root;
use std::path::Path;

/// Which catalog entries are **about** each `prior-art` node's subject.
///
/// Aboutness is not a property any file here carries and could not be made into one cheaply: a
/// catalog entry's frontmatter says what it *is* and its `used-by` says who draws on it, and
/// neither is the same as what system it documents. So it is declared, here, and the declaration
/// is the claim — an entry appearing under a node is an assertion that reading it tells you about
/// that system.
///
/// `delores-source.md` appears under no node. It is cited by all four and is about none of them:
/// it is a fifth system, and the reason this table exists is that it was standing in for a sixth.
const ABOUT: &[(&str, &[&str])] = &[
    (
        "ags.yml",
        &["ags-manual.md", "ags-272-manual.md", "ags-source.md"],
    ),
    (
        "scumm.yml",
        &["scumm-v5-opcode-reference.md", "wikipedia-scumm.md"],
    ),
    (
        "sierra-sci.yml",
        &["agi-specifications.md", "sci-inhouse-docs.md"],
    ),
    ("thimbleweed-park.yml", &["twp-dev-diary.md"]),
];

/// Every catalog entry a node names in its prose, by filename.
fn cited(root: &Path, node: &str) -> Vec<String> {
    let src = std::fs::read_to_string(root.join(".yidam/corpus/prior-art").join(node))
        .expect("the node is readable");
    let mut out: Vec<String> = Vec::new();
    let mut rest = src.as_str();
    while let Some(i) = rest.find("catalog/") {
        rest = &rest[i + "catalog/".len()..];
        let end = rest.find(".md").filter(|e| {
            rest[..*e]
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
        });
        if let Some(e) = end {
            let name = format!("{}.md", &rest[..e]);
            if !out.contains(&name) {
                out.push(name);
            }
        }
    }
    out
}

/// Every `prior-art` node on disk, so the table cannot fall behind the corpus.
fn nodes(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(root.join(".yidam/corpus/prior-art"))
        .expect("the class directory")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".yml"))
        .collect();
    out.sort();
    out
}

/// **The table covers every node, and names only entries that exist.**
///
/// A declaration that silently stops applying is worse than none, and this one is hand-written.
#[test]
fn the_aboutness_table_is_complete() {
    let root = repo_root();
    let declared: Vec<&str> = ABOUT.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        nodes(&root),
        declared,
        "every prior-art node needs a row, and a row needs a node"
    );

    for (node, entries) in ABOUT {
        for e in *entries {
            assert!(
                root.join(".yidam/catalog").join(e).exists(),
                "{node} names {e}, which is not in the catalog"
            );
        }
    }
}

/// **Three nodes rest on a source about themselves and one rests on somebody else's game.**
///
/// This is the census, and it pins the defective state rather than the rule: the assertion below
/// says `sierra-sci` cites none of its own sources, which is a fact about this repository and not
/// a thing anybody wants to keep true.
///
/// **The sources are already in the catalog** — `agi-specifications` and `sci-inhouse-docs` landed
/// one commit before this file — so what is pinned here is the gap between having retrieved a thing
/// and having read it into the node. That gap is the whole of what this phase closes, and it flips
/// to the rule: *every prior-art node cites at least one source about its own subject.*
#[test]
fn one_node_rests_only_on_a_source_about_a_different_system() {
    let root = repo_root();

    let mut without: Vec<&str> = Vec::new();
    for (node, own) in ABOUT {
        let cites = cited(&root, node);
        if !own.iter().any(|e| cites.iter().any(|c| c == e)) {
            without.push(node);
        }
    }

    assert_eq!(
        without,
        ["sierra-sci.yml"],
        "prior-art nodes citing no source about the system they describe"
    );

    // And what the one without rests on instead, so the substitution is on the record by name
    // rather than by absence.
    assert_eq!(
        cited(&root, "sierra-sci.yml"),
        ["delores-source.md"],
        "every catalog entry the Sierra node cites"
    );
}
