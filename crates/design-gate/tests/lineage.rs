//! **One of the four systems this corpus holds a `prior-art` node on had no source about itself,
//! from genesis to the eighty-eighth phase.**
//!
//! `prior-art/sierra-sci.yml` made eight claims about Sierra's two engines and cited exactly one
//! catalog entry: `delores-source.md`, a game written by different people on a different engine in
//! a different decade. Its own `source_standing` said so — *"Nothing retrieved… None of it has been
//! located or read, and the era range above is recollection"* — and that sentence stood while the
//! node above it grew to argue about what Sierra's transition proves.
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

/// **Every prior-art node cites a source about the system it describes.**
///
/// This was a census when it was written one commit ago, and it named `sierra-sci` as the one node
/// that did not: its only catalog citation was `delores-source.md`, a game by different people on
/// a different engine in a different decade. It is a rule now, and the flip is the phase.
///
/// The rule is weaker than it looks and deliberately so. It does not say a node is *well* sourced,
/// or that its claims are carried by what it cites — `verified-unsourced` already tries the first
/// and `sourcing.rs` records what that costs. It says only that somewhere in the node is a pointer
/// to a document about its own subject, which is the floor, and which one node sat below from
/// genesis to the eighty-eighth phase with nothing able to say so.
#[test]
fn every_prior_art_node_cites_a_source_about_itself() {
    let root = repo_root();

    let mut without: Vec<&str> = Vec::new();
    for (node, own) in ABOUT {
        let cites = cited(&root, node);
        if !own.iter().any(|e| cites.iter().any(|c| c == e)) {
            without.push(node);
        }
    }

    assert!(
        without.is_empty(),
        "prior-art nodes citing no source about the system they describe: {without:?}"
    );

    // And the node that used to be the whole of that list, by name, so a regression is legible as
    // one rather than as an empty vector becoming non-empty.
    let sierra = cited(&root, "sierra-sci.yml");
    for e in ["agi-specifications.md", "sci-inhouse-docs.md"] {
        assert!(
            sierra.contains(&e.to_string()),
            "the Sierra node no longer cites {e}, which is what this phase was"
        );
    }
}
