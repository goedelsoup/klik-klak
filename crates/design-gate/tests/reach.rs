//! **This vocabulary excludes what is drawn rather than typed, and that was a premise in six
//! records before it was a decision in one.**
//!
//! Four symptoms have been read separately, each as a thin patch of a corpus that is thin
//! everywhere. They are one thing:
//!
//! * `walkbox` is the only class in the authoring tier the `.game` surface cannot name at all
//! * `subsystem/path-solver` is the only subsystem carrying no `[verified]` claim, and it is the
//!   component that consumes exactly what the surface leaves out
//! * `actor`, `room` and `walkbox` are the only three classes **no node of which cites
//!   `crates/`** — the instrument has never measured anything they say
//! * `actor/player-character` and `room/single-screen` carry no verified claim and no source, and
//!   what they describe is mostly the excluded half: pathfinding to a clicked point, a walk cycle
//!   scaled by the region it stands in, a coordinate space shared with the background image
//!
//! None of that is neglect. A room in this format is a name, an actor's position is which room it
//! is in, and a hotspot is a name the room gives out — every class keeps the half an author types
//! and loses the half an author draws. `walkbox` is absent because it has no typed half:
//! `.yidam/decisions/ontology.yml` glosses it *"pathable geometry with depth scaling"*, and
//! [room-manifest] states the rule without drawing the conclusion — *"geometry comes from a tool
//! because polygons are drawn, not typed."*
//!
//! `.yidam/decisions/drawn-boundary.yml` states it, and states what it costs: the escape this
//! implies goes to a drawing program rather than to a scripting language, which is a different pen
//! and not a more powerful one.
//!
//! [room-manifest]: ../../../.yidam/corpus/data-format/room-manifest.yml

use design_gate::repo_root;
use game_format::Kind;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Instance nodes, grouped by the directory that names their class.
fn by_class(root: &Path) -> BTreeMap<String, Vec<PathBuf>> {
    let mut out: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
    for e in std::fs::read_dir(root.join(".yidam/corpus"))
        .expect("the corpus exists")
        .flatten()
    {
        let d = e.path();
        if !d.is_dir() {
            continue;
        }
        let class = e.file_name().to_string_lossy().into_owned();
        let mut v: Vec<PathBuf> = std::fs::read_dir(&d)
            .expect("readable")
            .flatten()
            .map(|f| f.path())
            .filter(|p| p.extension().is_some_and(|x| x == "yml"))
            .collect();
        v.sort();
        out.insert(class, v);
    }
    out
}

/// The authoring tier, read from the `tier:` each class declares.
///
/// This used to parse `ontology.yml`'s **prose** — tracking indentation, skipping `verb`'s gloss
/// where it wraps onto a second line — because the three tiers were an argument in a record and no
/// class said which one it was in. `.yidam/decisions/schema-deferral.yml` gave the class a field,
/// and retiring this parser is most of what the field bought. `deferrals.rs` holds the
/// declarations against the record that argues them.
fn authoring_tier(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
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
        if src.lines().any(|l| {
            l.strip_prefix("tier:")
                .is_some_and(|v| v.trim() == "authoring")
        }) {
            out.push(class);
        }
    }
    out.sort();
    out
}

/// The class each `.game` construct is the written form of.
///
/// Exhaustive on purpose: adding a `Kind` without saying which class it realizes will not compile,
/// which is the only part of this file the compiler can hold.
fn realizes(k: Kind) -> &'static str {
    match k {
        Kind::Room => "room",
        Kind::Actor => "actor",
        Kind::Item => "inventory-item",
        Kind::Flag => "world-flag",
        Kind::Counter => "world-flag",
        Kind::Conversation => "conversation",
        Kind::Hotspot => "hotspot",
        Kind::Option_ => "conversation",
    }
}

const KINDS: [Kind; 8] = [
    Kind::Room,
    Kind::Actor,
    Kind::Item,
    Kind::Flag,
    Kind::Counter,
    Kind::Conversation,
    Kind::Hotspot,
    Kind::Option_,
];

/// **Three authoring classes have no construct in the format, for three different reasons, and
/// only one of them has ever been written down.**
///
/// `cutscene` is **collapsed** — `.yidam/corpus/cutscene/scripted-sequence.yml` reduced it to an
/// interaction on every ground it was seeded on and the class is kept for the argument.
/// `puzzle` is **derived**: `.yidam/corpus/puzzle/lock-and-key.yml` says it in its own words,
/// *"the puzzle has no representation of its own — it is visible only as a pattern across the
/// things it touches"*, which is why `crates/reachability` finds puzzles nothing declares.
///
/// `walkbox` is neither. It is **excluded**, and nothing says so. Six classes reach the format
/// through a `verb` or an `interaction` rather than a `Kind`; walkbox reaches it through nothing,
/// because every word in its gloss is a thing an author draws.
#[test]
fn walkbox_is_the_only_authoring_class_the_format_cannot_name() {
    let root = repo_root();
    let tier = authoring_tier(&root);
    let on_disk: BTreeSet<String> = by_class(&root).keys().cloned().collect();
    for c in &tier {
        assert!(on_disk.contains(c), "{c} is in the tier and not on disk");
    }
    assert_eq!(
        tier.len(),
        11,
        "the authoring tier as ontology.yml declares it"
    );

    let named: BTreeSet<&str> = KINDS.into_iter().map(realizes).collect();
    assert_eq!(named.len(), 6, "eight constructs naming six classes");

    // `verb` and `interaction` are the binding itself rather than a thing a binding names.
    let through_binding = ["verb", "interaction"];
    let mut absent: Vec<&String> = tier
        .iter()
        .filter(|c| !named.contains(c.as_str()) && !through_binding.contains(&c.as_str()))
        .collect();
    absent.sort();
    assert_eq!(
        absent,
        ["cutscene", "puzzle", "walkbox"],
        "collapsed, derived, and the one with no account"
    );
}

/// **One class left, and two left this list in two phases for the same reason.**
///
/// It was three, and the argument for all three was the same: it is not that the checker is weak
/// there, it is that there is nothing for it to read. A `.game` file says an actor is in a room and
/// cannot say where in the room, so no test can measure a claim about a walk cycle or a coordinate
/// space.
///
/// **The argument held for the drawing and not for the questions.** `actor/player-character`'s
/// oldest claim was about control transferring between characters, which is an authoring question
/// and not a rendering one; `walkbox/scaled-polygon`'s was whether adjacency between regions needs
/// a relationship, which is a question about a relationship vocabulary and was answered by counting
/// somebody else's polygons. `.yidam/decisions/control-transfer.yml` and
/// `.yidam/decisions/adjacency.yml` measured them with `crates/game-format/tests/transfer.rs` and
/// `crates/design-gate/tests/adjacency.rs`.
///
/// So two classes left this list in consecutive phases, and neither left because the instrument got
/// stronger. **The boundary was drawn around the format and this check read it as a boundary around
/// the corpus** — a claim a `.game` file cannot express is not thereby a claim nothing can measure,
/// because the corpus reads artifacts the format cannot hold. `room` stays, and its claims are
/// geometry.
///
/// What did measure them is prior art. `walkbox/scaled-polygon` carries five `[verified]` claims
/// from 84 real polygons in a shipped game, and `room/scrolling` six. **This corpus can read
/// geometry it cannot express**, and nothing here distinguishes a claim measured by the instrument
/// from one read out of somebody else's artifact.
#[test]
fn the_classes_the_instrument_never_measured_are_the_drawn_ones() {
    let root = repo_root();
    let mut untouched = Vec::new();
    let mut citing = 0;
    for (class, nodes) in by_class(&root) {
        let n = nodes
            .iter()
            .filter(|p| {
                std::fs::read_to_string(p)
                    .expect("readable")
                    .contains("crates/")
            })
            .count();
        citing += n;
        if n == 0 {
            untouched.push(class);
        }
    }
    assert_eq!(
        untouched,
        ["room"],
        "classes no node of which cites the instrument"
    );
    assert_eq!(citing, 27, "nodes resting some claim on a committed test");
}

/// **Four nodes carry no `[verified]` claim at all, and three of them are on the drawn side.**
///
/// `actor/player-character` is the starkest: it is the node the whole vocabulary is written from —
/// *"the player in every interaction's precondition means this actor's position, inventory and
/// state"* — and it cites no source, no test, and nothing measured. Its locomotion property is
/// three sentences about pathfinding, walk-cycle scale and arrival, none of which this format can
/// state. `room/single-screen` is the same shape. `subsystem/path-solver` is the component that
/// consumes what they describe, and its own `authoring_visibility` names the boundary in passing:
/// *"an author **draws** walkboxes and never touches this."*
///
/// The fourth is not spatial and is here for contrast. `puzzle/lock-and-key` has no verified claim
/// because it is definitional, and its one `[inference]` argues against its own class.
///
/// **A tag inside backticks is the token being discussed and not a claim**, which this check
/// learned by reporting three of these four as newly-verified the moment they were revised to say
/// they carry no verified claim. The convention holds across all 26 backticked `[verified]` and
/// `[inference]` tokens in the corpus with no exception — *"the tag stays `[inference]` and will
/// not move"*, *"it is `[verified]` now, from this project's own code"* — and nothing anywhere
/// states it or checks it. It is `marker-count`'s finding on the two tallies it was never applied
/// to.
#[test]
fn the_nodes_with_no_verified_claim_are_drawn_or_definitional() {
    let root = repo_root();
    let mut bare = Vec::new();
    for (class, nodes) in by_class(&root) {
        for p in nodes {
            if p.to_string_lossy().ends_with(".ont.yml") {
                continue;
            }
            let src = std::fs::read_to_string(&p).expect("readable");
            if !src.replace("`[verified]`", "").contains("[verified]") {
                let stem = p
                    .file_stem()
                    .expect("a name")
                    .to_string_lossy()
                    .into_owned();
                bare.push(format!("{class}/{stem}"));
            }
        }
    }
    bare.sort();
    assert_eq!(
        bare,
        [
            "puzzle/lock-and-key",
            "room/single-screen",
            "subsystem/path-solver",
        ]
    );
}

/// **Six records rest on *this design has no geometry*, and one decides it.**
///
/// It is load-bearing in each: `hotspot-identity` closes both of a node's open questions with it,
/// `named-condition` separates two hotspot relations by which of them needs it, `frame-width`
/// leaves a named relation unadopted for it, `escape-hatch` lists it beside duration as a pressure
/// a design without it cannot feel, and `room-geometry` names *"a design with geometry in scope"*
/// as the thing that would settle a question a second game would not. `containment` is the
/// mildest and the most telling: it calls a specimen buildable *"needing no geometry, and
/// therefore not blocked on"* it — the boundary used as a scheduling fact.
///
/// That was the shape [`record-lifecycle`] found for `summary:` and [`save-boundary`] found for
/// the state vector: a rule the practice follows exactly, that nothing states, and that a later
/// phase can therefore contradict without noticing. `drawn-boundary` states it, and is partitioned
/// out here rather than counted — a record cannot be evidence that its own premise was unstated.
///
/// [`record-lifecycle`]: ../../../.yidam/decisions/record-lifecycle.yml
/// [`save-boundary`]: ../../../.yidam/decisions/save-boundary.yml
#[test]
fn the_premise_is_load_bearing_in_six_records_and_decided_in_one() {
    // `has none` was in this list for one commit and matched `prose-edges` on a sentence about
    // prose having no schema. The three that remain name the thing.
    const PREMISE: [&str; 3] = ["no geometry", "without geometry", "geometry in scope"];
    let root = repo_root();
    let (mut resting, mut deciding): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    for e in std::fs::read_dir(root.join(".yidam/decisions"))
        .expect("the directory exists")
        .flatten()
    {
        let p = e.path();
        if p.extension().is_none_or(|x| x != "yml") {
            continue;
        }
        let src = std::fs::read_to_string(&p).expect("readable");
        if !src.contains("geometry") {
            continue;
        }
        if !PREMISE.iter().any(|s| src.contains(s)) {
            continue;
        }
        let stem = p
            .file_stem()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        if stem == "drawn-boundary" {
            deciding.push(stem);
        } else {
            resting.push(stem);
        }
    }
    resting.sort();
    assert_eq!(
        deciding,
        ["drawn-boundary"],
        "the record that states the boundary"
    );
    assert_eq!(
        resting,
        [
            "containment",
            "escape-hatch",
            "frame-width",
            "hotspot-identity",
            "named-condition",
            "room-geometry",
        ],
        "records that rest on the boundary; none of them is a record about the boundary"
    );
}
