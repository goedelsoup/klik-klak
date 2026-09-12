//! **A hundred and five authored lines, and nothing asks which of them a player can reach.**
//!
//! [`failure-mode/orphaned-dialogue-branch`](../../../.yidam/corpus/failure-mode/orphaned-dialogue-branch.yml)
//! is the only failure mode in this corpus that nothing detects. It says so about shipped games
//! — *"they simply never see it — which is why it survives in shipped games and **why nobody
//! counts it**"* — and its own `detectable_at` property says `authoring`, which nothing here has
//! ever made true.
//!
//! The parse tree has held the lines since `.yidam/decisions/authored-text.yml` and the search
//! has enumerated the reachable states since genesis. Nothing has put the two together.
//!
//! This census pins the defect: the lines exist, the states exist, and the report has no field
//! between them.

use game_format::{parse, resolve, Game};
use std::path::{Path, PathBuf};

const FERRY: &str = include_str!("../../../design/specimen/ferry.game");
const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");
const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");
const PATROL: &str = include_str!("../../../design/specimen/patrol.game");

fn load(source: &str, name: &str) -> Game {
    let (mut game, problems) = parse(source, name);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    let problems = resolve(&mut game);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    game
}

fn specimens() -> Vec<(&'static str, Game)> {
    [
        ("ferry", FERRY),
        ("lighthouse", LIGHTHOUSE),
        ("nightwatch", NIGHTWATCH),
        ("patrol", PATROL),
    ]
    .into_iter()
    .map(|(n, s)| (n, load(s, n)))
    .collect()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/reachability is two levels below the root")
        .to_path_buf()
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **The lines are in the tree, with the line numbers that would name them.**
///
/// 105 clauses across four specimens, every one carrying where it was written. A census of what
/// is available to be counted, so that the count has something to be measured against.
#[test]
fn a_hundred_and_five_authored_lines_carry_where_they_were_written() {
    let mut total = 0;
    for (name, game) in specimens() {
        let clauses: Vec<_> = game
            .bindings
            .iter()
            .flat_map(|b| b.says.iter().chain(b.refusals.iter()))
            .collect();
        assert!(
            clauses.iter().all(|r| r.line > 0),
            "{name}: a clause with no line"
        );
        total += clauses.len();
    }
    assert_eq!(
        total, 105,
        "authored response clauses in the four specimens"
    );
}

/// **Nothing counts them, and the report is where the count would go.**
///
/// Every other failure mode this corpus names has a field: `strandings` is `unwinnable-state`,
/// `silent` is the resolver's guarantee, `unshown` is a binding written and never offered. A
/// line written and never selected has none, and `unshown` is not it — a binding can be shown in
/// every state of the game and still carry a clause no state selects.
#[test]
fn the_report_has_no_field_for_a_line_nobody_reads() {
    let root = repo_root();
    let src =
        std::fs::read_to_string(root.join("crates/reachability/src/lib.rs")).expect("the search");
    let fields = src
        .split("pub struct Report {")
        .nth(1)
        .expect("the report")
        .split("\n}")
        .next()
        .expect("its end");
    for absent in ["orphan", "unreachable_line", "unread_line"] {
        assert!(
            !fields.contains(absent),
            "the report has a {absent} field now — this census has been answered"
        );
    }

    // and the resolver cannot name a clause even if it were asked: it returns the text
    let resolver =
        std::fs::read_to_string(root.join("engine/interaction-resolver/src/lib.rs")).expect("it");
    assert!(
        resolver.contains(
            "pub fn select<'a>(w: &World, list: &'a [Response], s: &State) -> Option<&'a str>"
        ),
        "selection answers in text, so a caller counting lines cannot say which line"
    );
    assert!(
        !resolver.contains("pub fn clause"),
        "nothing hands back the clause"
    );
}

/// **The corpus says the mode belongs to branching trees, and this design has no branching tree.**
///
/// Pinned because it is the claim the census will be read against. If orphaning needs a path
/// through a tree, a vocabulary whose conversations are topic lists has designed the mode out and
/// the count should be zero.
#[test]
fn the_corpus_scopes_orphaning_to_a_construct_this_vocabulary_refused() {
    let root = repo_root();
    let node = std::fs::read_to_string(
        root.join(".yidam/corpus/failure-mode/orphaned-dialogue-branch.yml"),
    )
    .expect("the node");
    assert!(
        flat(&node).contains(
            "It belongs to `branching-tree` specifically rather than to conversations generally. \
             A topic list has no paths, so it cannot orphan anything; a tree can."
        ),
        "the scope claim"
    );
    assert!(
        flat(&node).contains("why nobody counts it"),
        "and the claim that nobody counts it"
    );
    assert!(
        node.contains("detectable_at: authoring"),
        "and where it says it could be caught"
    );
}
