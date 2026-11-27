//! **A record and the node it argued about disagree in 31 of 241 pairs, in both directions.**
//!
//! [`retraction-reach`] found one correction that was written into a record and never reached the
//! node, and asked how often that happens: how often a record and its node disagree about a fact
//! either could state. It was counted by reading. A record and a node are paired when either links
//! to the other; each of the 241 pairs was read on both sides, and 31 state some shared fact
//! incompatibly. Those 31 come down to the 24 sentences below, because one stale sentence is read by
//! every record that cites its node.
//!
//! **Thirteen are the node behind and eleven are the record behind.** Neither copy is the
//! authority, and that is the answer to the question.
//!
//! This holds what was found rather than finding more. A reading cannot be a gate. What a test can
//! do is keep the finding honest: the pair count is recomputed, and every sentence counted is
//! checked to be where the record says it is.
//!
//! **The phase after it corrected fourteen.** All thirteen node sentences, and the one record
//! sentence that was a live question — `binary-dimension`'s *"unmeasured here too"*, which
//! `arity-cost` had measured. Those are checked gone, except one that `refused.rs` keeps where it
//! was made, which is checked marked as answered instead; the other ten are records of their own
//! moment, each wrong about something written later.
//!
//! **The phase after that annotated the ten.** A record is the record of its moment, so none was
//! rewritten: each sentence stays as it was, and under it a paragraph beginning *"Since
//! corrected."* says what is true now and where it is kept. Each is checked still there and
//! annotated. So is `resolver-kind`'s, the eleventh, found by the testimony sweep outside the count.
//!
//! [`retraction-reach`]: ../../../.yidam/decisions/retraction-reach.yml

use design_gate::repo_root;
use std::collections::BTreeSet;
use std::path::Path;

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `corpus/<class>/<node>.yml` paths a file names, in either spelling a record uses.
fn corpus_links(src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for marker in ["../corpus/", ".yidam/corpus/"] {
        for (i, _) in src.match_indices(marker) {
            let rest = &src[i + marker.len()..];
            let end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || "/_-.".contains(c)))
                .unwrap_or(rest.len());
            let path = &rest[..end];
            if let Some(p) = path.strip_suffix(".yml").filter(|p| p.contains('/')) {
                out.insert(format!("{p}.yml"));
            }
        }
    }
    out
}

/// `decisions/<record>.yml` names a node mentions.
fn record_links(src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (i, _) in src.match_indices("decisions/") {
        let rest = &src[i + "decisions/".len()..];
        let end = rest
            .find(|c: char| !(c.is_ascii_alphanumeric() || "_-".contains(c)))
            .unwrap_or(rest.len());
        if rest[end..].starts_with(".yml") && end > 0 {
            out.insert(format!("{}.yml", &rest[..end]));
        }
    }
    out
}

fn yml(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).expect("dir") {
        let p = e.expect("entry").path();
        if p.is_dir() {
            out.extend(yml(&p));
        } else if p.extension().is_some_and(|x| x == "yml") {
            out.push(p);
        }
    }
    out
}

/// Every (record, node) pair either side links.
fn pairs(root: &Path) -> BTreeSet<(String, String)> {
    let corpus = root.join(".yidam/corpus");
    let mut out = BTreeSet::new();
    for p in yml(&root.join(".yidam/decisions")) {
        let r = p.file_name().unwrap().to_string_lossy().into_owned();
        let src = std::fs::read_to_string(&p).expect("readable");
        for n in corpus_links(&src) {
            out.insert((r.clone(), n));
        }
    }
    for p in yml(&corpus) {
        let n = p
            .strip_prefix(&corpus)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let src = std::fs::read_to_string(&p).expect("readable");
        for r in record_links(&src) {
            out.insert((r, n.clone()));
        }
    }
    out
}

/// Which side is behind.
#[derive(PartialEq)]
enum Behind {
    Node,
    Record,
}
use Behind::*;

/// The 24 sentences, each in the file that is behind, with what it contradicts.
const STALE: &[(Behind, &str, &str, &str)] = &[
    (
        Node,
        "corpus/data-format/interaction-table.yml",
        "`Binding` carries no text",
        "authored-text, effect-list-semantics: `Binding` has `says` and `refusals`",
    ),
    (
        Node,
        "corpus/data-format/interaction-table.yml",
        "one `and`, no `or`",
        "control-transfer, normal-form, or-earns-it: lock.game writes and; relay.game writes or",
    ),
    (
        Node,
        "corpus/data-format/interaction-table.yml",
        "established that every source here is an artifact",
        "testimony: Gilbert's house has testimony; AGS has none",
    ),
    (
        Node,
        "corpus/data-format/interaction-table.yml",
        "gated bindings on a hotspot verb carries a refusal, without exception",
        "unread-specimen: 26 to 2, the two withheld talk-to in lock.game",
    ),
    (
        Node,
        "corpus/interaction/binary-binding.yml",
        "all seventeen gated bindings on a hotspot verb carry a line",
        "unread-specimen: 26 to 2",
    ),
    (
        Node,
        "corpus/interaction/unary-binding.yml",
        "the verb's own default response stands in",
        "default-binding, verb-menu: nothing stands in for an unauthored pair",
    ),
    (
        Node,
        "corpus/prior-art/ags.yml",
        "The full table and what follows from it are in",
        "forty-eight: the table is recorded in the ags-source catalog entry",
    ),
    (
        Node,
        "corpus/puzzle/state-gate.yml",
        "One predicate in thirty-six joins two atoms.",
        "restated-guard, unread-specimen: eight in sixty",
    ),
    (
        Node,
        "corpus/interaction/entry-binding.yml",
        "`enter` carries 237 statements across 17 non-empty bodies",
        "arrival-realizer: 169 statements; 237 is non-blank lines",
    ),
    (
        Node,
        "corpus/prior-art/sierra-sci.yml",
        "eleven documents, one read",
        "class-library: all three have now been opened",
    ),
    (
        Node,
        "corpus/subsystem/path-solver.yml",
        "a region a shipped game hides on entry and never shows again",
        "adjacency: desk_path is hidden; natalie_desk is what it cuts off",
    ),
    (
        Node,
        "corpus/cutscene/scripted-sequence.yml",
        "That is defining the problem away",
        "refused-duration: it is not defined away in the built system",
    ),
    (
        Node,
        "corpus/hotspot/static-region.yml",
        "The eleventh is state rather than geometry",
        "delegation: two are a shut bank's door reading as its sign",
    ),
    (
        Record,
        "decisions/default-binding.yml",
        "46 of its 164 objects",
        "interaction-table: 160 live declarations, 45 with no binding",
    ),
    (
        Record,
        "decisions/default-binding.yml",
        "9,348 cells with 203 authored",
        "default-binding, binary-binding, use-with nodes: 160 objects; 50,676 cells",
    ),
    (
        Record,
        "decisions/binary-dimension.yml",
        "it is unmeasured here too.",
        "binary-binding: arity-cost measured it",
    ),
    (
        Record,
        "decisions/held.yml",
        "indistinguishable from an ontology that overreached",
        "room-manifest: that sentence was withdrawn before held was written",
    ),
    (
        Record,
        "decisions/playable.yml",
        "indistinguishable from an ontology that overreached",
        "room-manifest: the same withdrawn sentence",
    ),
    (
        Record,
        "decisions/named-condition.yml",
        "described was imagined",
        "state-gate: it was not imagined; lock.game writes the gate",
    ),
    (
        Record,
        "decisions/save-boundary.yml",
        "flags, counters, and nothing else",
        "save-state: chosen options and actor rooms are saved too",
    ),
    (
        Record,
        "decisions/threshold-trigger.yml",
        "has exactly one writer",
        "scripted-sequence: sixteen of eighteen",
    ),
    (
        Record,
        "decisions/effect-list-semantics.yml",
        "The edge earns its place when a game needs one",
        "scripted-sequence: both settled, neither wants an edge",
    ),
    (
        Record,
        "decisions/latch-default.yml",
        "effect has never been written here",
        "latched-flag: three clear sites in 1,108 lines",
    ),
    (
        Record,
        "decisions/tier-split.yml",
        "this design has never specified one",
        "verb-menu: this design turned out to have one",
    ),
];

/// **The pairs the reading covered are the pairs there are.** A new link moves this, and the
/// finding is then a count of fewer pairs than exist.
///
/// It has moved once. `one-toggle` and `latched-flag` are the 242nd pair, joined in both
/// directions by the phase that asked whether `clear` reopens the keyword. That pair was read on
/// both sides when it was written, and the two agree: seventeen flags, fourteen declared, one
/// cleared. So the finding stands at 31 of 241 as `retraction-reach` states it, and 31 of 242 is
/// what the corpus holds now.
///
/// It has moved twice. `ags-testimony` is the 78th record and adds four pairs, one to each node it
/// closed, narrowed or left standing: `prior-art/ags`, `interaction-table`, `acquisition-binding`
/// and `orphaned-dialogue-branch`. Each was edited in the same phase that wrote the record, so
/// both sides were read together, and 31 of 246 is what the corpus holds now.
///
/// Three times. `second-game-width` is the 79th record and adds one pair, to `latched-flag`, whose
/// tractability paragraph it revised in the same phase. 31 of 247.
///
/// Four. `forum-witnesses` is the 80th record and adds three pairs, to the three nodes whose AGS
/// markers it narrowed or answered — `prior-art/ags`, `acquisition-binding` and
/// `orphaned-dialogue-branch` — each revised in the same phase. 31 of 250.
///
/// One. `maker-tracker` is the 81st and adds one pair, to `orphaned-dialogue-branch`, the node whose
/// marker it narrowed, revised in the same phase. 31 of 251.
#[test]
fn the_reading_covered_every_linked_pair() {
    let p = pairs(&repo_root());
    let records: BTreeSet<_> = p.iter().map(|(r, _)| r).collect();
    let nodes: BTreeSet<_> = p.iter().map(|(_, n)| n).collect();
    assert_eq!((p.len(), records.len(), nodes.len()), (251, 81, 46));
}

/// **The pairs were not the whole of it.** The same stale fact written a second time, in a node no
/// record links or in `design/` and the catalog, is outside every pair the reading covered. The
/// testimony sweep found ten such copies by searching for each corrected sentence's figure rather
/// than by following links, and corrected them. Each is checked gone; the path is from the root.
const COPIES: &[(&str, &str)] = &[
    (
        ".yidam/corpus/failure-mode/orphaned-dialogue-branch.yml",
        "and every source this corpus holds is an artifact",
    ),
    (
        ".yidam/corpus/interaction/acquisition-binding.yml",
        "no source this corpus holds reports what using the thing was like",
    ),
    (
        ".yidam/corpus/prior-art/ags.yml",
        "The code, the manual and the interview are artifacts",
    ),
    (
        ".yidam/corpus/subsystem/verb-menu.yml",
        "The corpus writes the refusing form seventeen times",
    ),
    (
        ".yidam/corpus/subsystem/verb-menu.yml",
        "twenty-six to two toward showing here",
    ),
    (
        ".yidam/catalog/delores-source.md",
        "a hotspot verb carries a line.",
    ),
    (
        ".yidam/catalog/delores-source.md",
        "`when` with an `else` — **26**",
    ),
    (
        ".yidam/catalog/delores-source.md",
        "seventeen to nothing toward showing here",
    ),
    (
        "design/bounded-expression-language.md",
        "fifteen `and` and two `or`",
    ),
    (
        "design/bounded-expression-language.md",
        "twenty-six of the twenty-eight gated bindings",
    ),
];

/// **Every copy found outside the pairs is corrected.**
#[test]
fn every_copy_outside_the_pairs_is_corrected() {
    let root = repo_root();
    for (file, sentence) in COPIES {
        let src = flat(&std::fs::read_to_string(root.join(file)).expect("readable"));
        assert!(!src.contains(sentence), "{file} says {sentence:?} again");
    }
    assert_eq!(COPIES.len(), 10);
    let record = flat(
        &std::fs::read_to_string(root.join(".yidam/decisions/retraction-reach.yml"))
            .expect("readable"),
    );
    assert!(
        record.contains("ten copies outside every pair"),
        "retraction-reach states the undercount"
    );
}

/// A corrected sentence another test keeps as the claim it was, and the mark that answers it.
const KEPT: &[(&str, &str)] = &[(
    "That is defining the problem away",
    "as written; the built system does not define it away, and it is answered below, by an ordering",
)];

/// What opens the paragraph that corrects a record without rewriting it.
const SINCE: &str = "*Since corrected.*";

/// The record sentence the testimony sweep found outside the count, kept and annotated the same way.
const ELEVENTH: (&str, &str) = (
    "decisions/resolver-kind.yml",
    "every source this corpus holds is an artifact",
);

/// **The eleventh stands as written and is annotated.**
#[test]
fn the_record_sentence_outside_the_count_is_annotated() {
    let (file, sentence) = ELEVENTH;
    let src =
        flat(&std::fs::read_to_string(repo_root().join(".yidam").join(file)).expect("readable"));
    assert!(src.contains(sentence) && src.contains(SINCE), "{file}");
}

/// Corrected in the phase after the count: every node row, and the one record row that was open.
fn corrected(side: &Behind, file: &str) -> bool {
    *side == Node || file == "decisions/binary-dimension.yml"
}

/// **Every disagreement counted is accounted for, and the split was thirteen to eleven.** Fourteen
/// are gone and ten stand as written, each with its correction under it.
#[test]
fn every_counted_disagreement_is_corrected_or_still_stands() {
    let root = repo_root();
    for (side, file, sentence, against) in STALE {
        let src = flat(&std::fs::read_to_string(root.join(".yidam").join(file)).expect("readable"));
        if let Some((_, mark)) = KEPT.iter().find(|k| k.0 == *sentence) {
            assert!(
                src.contains(sentence) && src.contains(mark),
                "{file} keeps {sentence:?} and marks it answered"
            );
        } else if corrected(side, file) {
            assert!(
                !src.contains(sentence),
                "{file} says {sentence:?} again, which contradicted {against}"
            );
        } else {
            assert!(
                src.contains(sentence) && src.contains(SINCE),
                "{file} keeps {sentence:?} as the record of its moment and says under it that \
                 {against}"
            );
        }
    }
    let node = STALE.iter().filter(|s| s.0 == Node).count();
    let fixed = STALE.iter().filter(|s| corrected(&s.0, s.1)).count();
    assert_eq!((STALE.len(), node, STALE.len() - node), (24, 13, 11));
    assert_eq!((fixed, STALE.len() - fixed), (14, 10));

    let record = flat(
        &std::fs::read_to_string(root.join(".yidam/decisions/retraction-reach.yml"))
            .expect("readable"),
    );
    for figure in [
        "241 pairs across 76 records and 46 nodes",
        "31 pairs disagree",
        "24 distinct sentences",
        "13 are the node behind",
        "11 are the record behind",
        "fourteen were corrected",
        "the ten are annotated",
    ] {
        assert!(
            record.contains(figure),
            "retraction-reach states {figure:?}"
        );
    }
}
