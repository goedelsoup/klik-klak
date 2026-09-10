//! `cargo test -p design-gate` is the gate for `design/`.
//!
//! It was a shell script until `crates/` existed. `.yidam/decisions/design-directory.yml`
//! wrote down the condition for moving it and this is the move.

use design_gate::{check, front_matter, repo_root};

#[test]
fn design_still_derives_from_the_corpus() {
    let f = check(&repo_root());
    assert!(
        f.0.is_empty(),
        "design/ does not derive cleanly:\n{}",
        f.report()
    );
}

#[test]
fn a_missing_premise_is_caught() {
    // The check that matters most, verified against a case rather than a green run: the
    // shell gate it replaced found nothing on the documents it was written for either.
    let root = repo_root();
    let doc = root.join("design/bounded-expression-language.md");
    let source = std::fs::read_to_string(&doc).expect("the design exists");
    let fm = front_matter(&source).expect("it has front matter");
    let rests = fm.list("rests_on");
    assert!(!rests.is_empty());
    for p in rests {
        assert!(root.join(p).exists(), "{p} should exist right now");
    }
    assert!(!root.join(".yidam/corpus/nothing-here.yml").exists());
}

#[test]
fn an_effect_the_ontology_does_not_declare_is_caught() {
    let root = repo_root();
    let ont = std::fs::read_to_string(root.join(".yidam/corpus/interaction.ont.yml")).unwrap();
    let declared = design_gate::declared_relationships(&ont);
    assert!(
        declared.iter().any(|r| r == "consumes"),
        "the one relationship this repo added"
    );
    assert!(!declared.iter().any(|r| r == "teleports"));
}
