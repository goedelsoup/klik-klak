//! **The proposal's first unsettled item was settled by being built, and nobody told the document.**
//!
//! Section 5 of `design/bounded-expression-language.md` opened: *"**Concrete syntax.** Everything
//! above is abstract. The surface belongs with `interaction-table.yml` and its unresolved tension
//! between being keyed by verb and target and reading like writing."* Three of that section's
//! eight items carry `~~…~~ **Settled.**` with their evidence; this was the one that did not, and
//! it is the item a reader meets first.
//!
//! Everything above is not abstract. `crates/game-format` implements a concrete syntax of **50
//! constructs** — 5 verbs, 19 keywords the parser dispatches on, 8 effects, 9 predicates, 6
//! comparisons, 3 sites — and four specimens totalling 662 lines and 91 bindings are written in
//! it. The same document cites `crates/` **ten times** elsewhere, so this is not a document
//! unaware of the crates; it is one paragraph that nobody re-read.
//!
//! **And the tension it calls unresolved was resolved, in the node it points at.**
//! `interaction-table.yml`'s `authored_by` says a specimen *"resolves it by splitting the two
//! rather than reconciling them: declarations are keyed like data, bindings are written as blocks,
//! and the construct carrying the join is the guarded response list — `when <predicate> "<line>"`,
//! which keys and reads like writing on the same line."* A named construct, in the file the bullet
//! sends the reader to.
//!
//! **That node carries a stale figure of its own, four lines from its answer.** It says *"whether
//! that survives a table with real combinatorics is untested; twenty-three bindings is not a test
//! of scale"*, and the next paragraph is 203 bindings from a shipped game concluding *"the
//! combinatorial explosion this node was seeded to worry about does not appear."* It is also 91
//! bindings across four specimens now rather than twenty-three.
//!
//! Both are corrected. What the checks below hold is the rule rather than the census: **every item
//! in that section is either live or struck through and evidenced**, and the node no longer
//! asserts that a thing it tests four lines later is untested.
//!
//! The last check had to learn used-versus-discussed for the tenth time here, in a spelling none
//! of the other nine had: **a corrected sentence quotes the sentence it corrects**, so the claim
//! is still in the file and must be. What is checked is that no occurrence is asserted.
//! `.yidam/decisions/settled-surface.yml`.

use design_gate::{front_matter, repo_root};
use std::path::Path;

/// Prose wraps, so a quotation from a node is not a substring of it. The eighth undeclared
/// convention this repository found by measuring arrived as exactly this, in `routing.rs`.
fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

const DESIGN: &str = "design/bounded-expression-language.md";
const NODE: &str = ".yidam/corpus/data-format/interaction-table.yml";

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// Constructs, and lines and bindings authored against them.
fn surface(root: &Path) -> (usize, usize, usize, usize) {
    let src = read(root, "crates/game-format/tests/surface.rs");
    let total = src
        .split("verbs.len() + keywords.len()")
        .nth(1)
        .and_then(|t| t.split(',').nth(1))
        .and_then(|n| n.trim().parse::<usize>().ok())
        .expect("the surface test states a total");

    let dir = root.join("design/specimen");
    let mut games: Vec<_> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .collect();
    games.sort();
    let (mut lines, mut bindings) = (0, 0);
    for p in &games {
        let src = std::fs::read_to_string(p).expect("readable");
        lines += src.lines().count();
        // counted off the parse tree rather than by matching line heads, because a heuristic over
        // the text gave 88 against the 91 `crates/reachability/tests/verbs.rs` counts, and a
        // figure that disagrees with a committed one is the defect this repository keeps finding
        let (mut game, _) = game_format::parse(&src, "specimen");
        let _ = game_format::resolve(&mut game);
        bindings += game.bindings.len();
    }
    (total, games.len(), lines, bindings)
}

/// **Every item in the unsettled section is live or struck through, and a struck one says what
/// settled it.**
///
/// The convention was already there — three items carried it and a fourth did not, which is what a
/// convention with no check produces. The figures are pinned alongside because the thing that
/// settled the first item is a size, and a size that drifts would leave the strikethrough
/// asserting something no longer true.
#[test]
fn every_settled_item_says_what_settled_it() {
    let root = repo_root();
    assert_eq!(
        surface(&root),
        (50, 5, 904, 120),
        "constructs, specimens, lines, bindings"
    );

    let doc = read(&root, DESIGN);
    assert!(
        !doc.contains("**Concrete syntax.** Everything above is abstract."),
        "the item that stood while the thing it called abstract was built"
    );

    let section = doc
        .split("## 5. What this does not settle")
        .nth(1)
        .expect("the section");
    let items: Vec<&str> = section.lines().filter(|l| l.starts_with("- ")).collect();
    assert_eq!(items.len(), 9, "items in the section");
    let struck: Vec<&&str> = items.iter().filter(|l| l.starts_with("- ~~")).collect();
    assert_eq!(struck.len(), 4, "struck through: {struck:?}");
    for l in &struck {
        assert!(
            l.contains("**Settled") || l.contains("**Answered"),
            "a struck item that does not say what settled it: {l}"
        );
    }
    assert!(
        doc.matches("crates/").count() >= 10,
        "in a document that cites the crates throughout"
    );
}

/// **The tension the bullet calls unresolved is resolved in the node the bullet points at.**
///
/// The bullet's whole content is a forwarding address, and the address has the answer. Nothing
/// brought a reader back — which is `design-directory`'s own statement of why `rests_on:` exists:
/// *"a corpus node that goes stale is caught by a reader eventually, because the corpus is what
/// gets re-read. A design document is not."*
#[test]
fn the_node_the_bullet_forwards_to_carries_the_resolution() {
    let root = repo_root();
    let node = read(&root, NODE);
    assert!(
        flat(&node).contains("resolves it by splitting the two rather than reconciling them"),
        "the node says the tension was resolved"
    );
    assert!(
        flat(&node).contains("keys and reads like writing on the same line"),
        "and names the construct that resolves it"
    );

    let doc = read(&root, DESIGN);
    assert!(
        flat(&doc).contains("keys and reads like writing on the same line"),
        "and the proposal carries it now, quoted from the address it was forwarding to"
    );

    // The node is a premise of the proposal, so the gate has been watching the path all along
    // and has nothing to say about what is written on either end of it.
    let fm = front_matter(&doc).expect("front matter");
    assert!(
        fm.list("rests_on").iter().any(|p| p == NODE),
        "the proposal names this node as a premise"
    );
}

/// **The node no longer calls untested a thing it tests four lines later.**
///
/// `authored_by` said scale was untested and the next paragraph tested it at 203 bindings. The
/// sentence is corrected rather than deleted — what it was right about is that a specimen count is
/// not a scale test, which is why the shipped game was read.
#[test]
fn the_node_no_longer_calls_scale_untested_above_the_paragraph_that_tests_it() {
    let root = repo_root();
    let node = read(&root, NODE);
    let prop = node
        .split("  authored_by: |")
        .nth(1)
        .expect("the property")
        .split("\n  escape_hatch:")
        .next()
        .expect("its end");
    let text = flat(prop);

    // Used versus discussed, for the tenth time in this crate and in a spelling none of the
    // other nine had: **a corrected sentence that quotes the sentence it corrects.** The phrase
    // is still in the file and must be, because deleting it would leave the correction correcting
    // nothing. What must not survive is an *asserted* occurrence.
    const CLAIM: &str = "is untested; twenty-three bindings is not a test of scale";
    for (i, _) in text.match_indices(CLAIM) {
        let before = &text[i.saturating_sub(120)..i];
        assert!(
            before.contains("This paragraph read *\""),
            "an asserted occurrence of the claim, not a quoted one: …{before}"
        );
    }
    assert_eq!(
        text.matches(CLAIM).count(),
        1,
        "quoted exactly once, by the correction"
    );
    assert!(
        text.contains(
            "It is five specimens and 120 bindings now, and the answer is the shipped \
                       game rather than the specimens"
        ),
        "the claim, corrected in place rather than deleted"
    );
    assert!(
        text.find("This paragraph read").expect("the correction")
            < text
                .find("The combinatorial explosion this node was seeded to worry about does not appear")
                .expect("the paragraph that answers it"),
        "and it still sits above the paragraph that answers it, which is the point"
    );
    assert!(
        text.contains("That is 203 bindings rather than 23"),
        "the shipped game's count stands where it was"
    );
}
