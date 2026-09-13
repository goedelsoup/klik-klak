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
///
/// **And it read the same boundary wrong a third time, in a narrower place: it counted `crates/`
/// and called the result a committed test.** The engine holds committed tests too, and when
/// `.yidam/decisions/unbuilt-solver.yml` built `path-solver` the node's evidence became
/// `engine/path-solver/tests/delores.rs` — so a node that gained its first verified claim would
/// have made this number go **down**. `response-presenter` had been uncounted for five phases for
/// the same reason. That is `.yidam/decisions/unread-spelling.yml`'s ceiling one phase later: a
/// hand-written vocabulary that fails silent, where the missing spelling is a directory.
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
                    || std::fs::read_to_string(p)
                        .expect("readable")
                        .contains("engine/")
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
    assert_eq!(citing, 28, "nodes resting some claim on a committed test");
}

/// **Three nodes carry no `[verified]` claim at all, and two of them are on the drawn side.**
///
/// `actor/player-character` is the starkest: it is the node the whole vocabulary is written from —
/// *"the player in every interaction's precondition means this actor's position, inventory and
/// state"* — and it cites no source, no test, and nothing measured. Its locomotion property is
/// three sentences about pathfinding, walk-cycle scale and arrival, none of which this format can
/// state. `room/single-screen` is the same shape.
///
/// The third is not spatial and is here for contrast. `puzzle/lock-and-key` has no verified claim
/// because it is definitional, and its one `[inference]` argues against its own class.
///
/// **`subsystem/path-solver` was the fourth and it left.** It was the component that consumes what
/// the other two describe, and its own `authoring_visibility` named the boundary in passing —
/// *"an author **draws** walkboxes and never touches this"* — which was the reason offered for a
/// node that also said no check would ever read it. `.yidam/decisions/unbuilt-solver.yml` built
/// the component and ran it over two of a shipped game's rooms, and the reason survives the
/// verification: an author still never touches it. **Drawn does not mean unmeasurable**, which is
/// the third time this file has had to separate those, and the second in two phases.
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
    assert_eq!(bare, ["puzzle/lock-and-key", "room/single-screen"]);
}

/// What a record mentioning geometry is doing with it. Hand-made, because whether an argument
/// rests on a premise is not a thing a substring settles — which is the finding this table exists
/// to keep from recurring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Geometry {
    /// The argument needs the premise. Removing it would change what the record decided.
    Rests,
    /// It states the boundary rather than arguing from it.
    Decides,
    /// The word appears — a citation of a geometry record, a class-table row, a comparison with a
    /// format that has geometry — and the argument would stand without it.
    Mentions,
    /// A record about this check rather than about the boundary.
    AboutTheCheck,
}

/// Every record that mentions geometry, and what it is doing with it.
const GEOMETRY: &[(&str, Geometry)] = &[
    ("action-pages", Geometry::Mentions),
    ("adjacency", Geometry::Rests),
    ("containment", Geometry::Rests),
    ("drawn-boundary", Geometry::Decides),
    ("engine-directory", Geometry::Mentions),
    ("escape-hatch", Geometry::Rests),
    ("evidence-kinds", Geometry::Rests),
    ("frame-width", Geometry::Rests),
    ("hotspot-identity", Geometry::Rests),
    ("named-condition", Geometry::Rests),
    ("ontology", Geometry::Mentions),
    ("record-layer", Geometry::Mentions),
    ("room-geometry", Geometry::Rests),
    ("round-trip", Geometry::Mentions),
    ("unbuilt-solver", Geometry::Rests),
    ("unread-spelling", Geometry::AboutTheCheck),
];

/// **Nine records rest on *this design has no geometry*, one decides it, and for four phases this
/// check could see six.**
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
/// out rather than counted — a record cannot be evidence that its own premise was unstated.
///
/// **Two more were being skipped in silence and the phrasing list is why.** This test filtered the
/// directory to records mentioning geometry and then matched three phrasings inside them; what
/// matched no phrasing was `continue`d past and reported nowhere, so the census published six of
/// six while computing six of fourteen. `.yidam/decisions/unread-spelling.yml`.
///
/// - **`evidence-kinds`** builds a third kind of provenance on `drawn-boundary`'s sentence, quoted:
///   *"this corpus can read geometry it cannot express."* A fourth phrasing, and a fourth entry in
///   the list would have caught it.
/// - **`adjacency`** uses no phrasing at all. It read 84 walkbox polygons out of a shipped game and
///   derived every room's adjacency graph from them, **because this vocabulary has none to derive
///   it from** — a record resting on the premise by doing the thing the premise makes necessary.
///   No list of spellings could have found it.
///
/// **So the fix is not a longer list.** Every record the filter admits is classified by hand and
/// the residue is held at empty, which is the shape `resolvers.rs` gave the corpus markers and
/// `lineage.rs` gave the prior-art nodes. What a phrasing does now is suggest; what settles it is
/// somebody reading the record, and the reading is written down where it can be argued with.
///
/// **The ninth rests on it by finding something built on top of it that does not follow.**
/// `unbuilt-solver` leaves the premise exactly as it is and withdraws a conclusion drawn from it —
/// that a claim no `.game` file can state is a claim nothing can check. The phrasings do catch
/// this one, which is the first useful thing they have done here since the list was written, and
/// they catch it because a record arguing about the premise has to quote it.
///
/// [`record-lifecycle`]: ../../../.yidam/decisions/record-lifecycle.yml
/// [`save-boundary`]: ../../../.yidam/decisions/save-boundary.yml

#[test]
fn the_premise_is_load_bearing_in_nine_records_and_decided_in_one() {
    let root = repo_root();
    // `has none` was in this list for one commit and matched `prose-edges` on a sentence about
    // prose having no schema. The three that remain name the thing — and two of the fifteen below
    // say it in words none of them uses, which is why the list no longer decides anything.
    const PREMISE: [&str; 3] = ["no geometry", "without geometry", "geometry in scope"];

    let mut mentioning: Vec<String> = Vec::new();
    let mut matched: Vec<String> = Vec::new();
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(root.join(".yidam/decisions"))
        .expect("the directory exists")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yml"))
        .collect();
    files.sort();
    for p in files {
        let src = std::fs::read_to_string(&p).expect("readable");
        if !src.contains("geometry") {
            continue;
        }
        let stem = p
            .file_stem()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        if PREMISE.iter().any(|s| src.contains(s)) {
            matched.push(stem.clone());
        }
        mentioning.push(stem);
    }

    // **The residue, published.** Every record the filter admits has a row, and a row needs a
    // record — so a phase that writes about geometry lands here and has to be read, rather than
    // falling through a list of phrasings into a number nobody can question.
    let declared: Vec<&str> = GEOMETRY.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        mentioning, declared,
        "a record mentioning geometry with no row, or a row with no record"
    );

    let of = |k: Geometry| -> Vec<&str> {
        GEOMETRY
            .iter()
            .filter(|(_, g)| *g == k)
            .map(|(n, _)| *n)
            .collect()
    };
    assert_eq!(
        of(Geometry::Rests),
        [
            "adjacency",
            "containment",
            "escape-hatch",
            "evidence-kinds",
            "frame-width",
            "hotspot-identity",
            "named-condition",
            "room-geometry",
            "unbuilt-solver",
        ],
        "records that rest on the boundary; none of them is a record about the boundary"
    );
    assert_eq!(
        of(Geometry::Decides),
        ["drawn-boundary"],
        "the record that states the boundary"
    );
    assert_eq!(
        of(Geometry::Mentions).len(),
        5,
        "the word without the premise"
    );
    assert_eq!(of(Geometry::AboutTheCheck), ["unread-spelling"]);

    // and what the phrasings are worth: they find seven of the nine that rest, miss two, and admit
    // two that do not — a suggestion rather than a classifier. The newest one they do find, and
    // for a reason that does not generalise: a record arguing about the premise has to quote it,
    // so the phrasings catch the records that discuss the boundary and miss the ones that act on
    // it. That is the ceiling, not a hole in the list.
    let rests: Vec<&str> = of(Geometry::Rests);
    let found = rests
        .iter()
        .filter(|r| matched.contains(&r.to_string()))
        .count();
    assert_eq!(
        (found, rests.len(), matched.len()),
        (7, 9, 9),
        "records the phrasings find, records that rest, and everything a phrasing matched"
    );
}
