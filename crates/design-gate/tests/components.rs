//! **Two of the four components the corpus specifies were built before the directory that holds
//! them, and nothing said so.**
//!
//! `engine/` held a contract, a gate and no code
//! ([`engine-directory`](../../../.yidam/decisions/engine-directory.yml)), and the gate pinned
//! four subsystems that nothing implemented. It was wrong about two.
//!
//! `crates/reachability` declared `struct Index<'a>` — not `pub` — with **fourteen methods across
//! 544 lines**, against five public items in the whole crate. `Ix::in_menu` was
//! [`verb-menu`](../../../.yidam/corpus/subsystem/verb-menu.yml)'s responsibility verbatim:
//! *"given a target and a state, the verbs to display and no others"*. `live`, `offered`, `apply`
//! and `settle` were
//! [`interaction-resolver`](../../../.yidam/corpus/subsystem/interaction-resolver.yml).
//!
//! The corpus had been citing them from inside the instrument for four phases —
//! *"`objectTouchable(this, Bank.open ? YES : NO)` is a shipped engine hand-writing
//! `Ix::in_menu`"* is a claim about adventure games resting on a private symbol, and both
//! subsystem nodes carry `[verified]` figures against `crates/reachability/tests/`. A subsystem
//! verified by a checker's fixture is a component that exists, in the one place
//! `.claude/CLAUDE.md` says the engine is not.
//!
//! They are `engine/verb-menu` and `engine/interaction-resolver` now, and the checker depends on
//! them. What follows holds the split: the evaluator is where the corpus says the components are,
//! the search kept only what serves a search, and no crate carries a second copy.
//! `.yidam/decisions/two-components.yml`.

use design_gate::repo_root;
use std::path::Path;

/// Prose wraps, so a quotation from a node is not a substring of it. The eleventh time this crate
/// has had to learn it, and the first in a check about code.
fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// Every free function a file declares, in order.
fn functions(root: &Path, rel: &str) -> Vec<String> {
    read(root, rel)
        .lines()
        .filter_map(|l| {
            let rest = l
                .strip_prefix("pub fn ")
                .or_else(|| l.strip_prefix("fn "))?;
            Some(rest.split(['(', '<']).next()?.trim().to_string())
        })
        .collect()
}

/// **The fourteen methods are in three places, and which one each landed in is the decision.**
///
/// The world and what is true in it went to the format, because no subsystem in this corpus owns
/// state — all four declare none across frames — and a module under `engine/` that held one would
/// be inventing a fifth component. What a player is shown is the menu; what firing one does is the
/// resolver; what only a search needs stayed with the search.
#[test]
fn the_evaluator_is_public_and_split_where_the_corpus_splits_it() {
    let root = repo_root();

    let world = read(&root, "crates/game-format/src/world.rs");
    assert!(
        world.contains("pub struct World<'a> {") && world.contains("pub struct State {"),
        "the world and a state of it are public"
    );
    for m in ["pub fn new", "pub fn initial", "pub fn holds"] {
        assert!(world.contains(m), "the format carries {m}");
    }

    assert_eq!(
        functions(&root, "engine/verb-menu/src/lib.rs"),
        ["at", "shown", "in_menu", "live"],
        "what a player is shown"
    );
    assert_eq!(
        functions(&root, "engine/interaction-resolver/src/lib.rs"),
        [
            "clause",
            "select",
            "resolve",
            "offered",
            "spends",
            "apply_without_spending",
            "apply",
            "settle_after",
            "settle",
            "settle_tracking",
        ],
        "what firing one does"
    );

    let search = functions(&root, "crates/reachability/src/lib.rs");
    assert_eq!(
        search,
        ["describe", "observable", "can_finish", "analyse"],
        "and the search kept only what serves a search"
    );

    // fourteen, still: three moved to the format, two to the menu, six to the resolver, three
    // stayed. `select` and `resolve` are new and `analyse` was always the entry point.
    assert!(
        !read(&root, "crates/reachability/src/lib.rs").contains("struct Index"),
        "nothing is called Index any more"
    );
}

/// **A corpus node is corrected and a record is not, and the difference is what each is for.**
///
/// `Ix::in_menu` is not a symbol any more. `.yidam/corpus/subsystem/interaction-resolver.yml` is
/// living prose about adventure games and says `verb_menu::in_menu` now; the three records that
/// name the old symbol are left exactly as written, because a record is what was decided when it
/// was decided and rewriting one to keep a checker green is the wrong way round.
///
/// The same rule `design-directory` gives for `rests_on:`: *"a corpus node that goes stale is
/// caught by a reader eventually, because the corpus is what gets re-read."* A record is not.
#[test]
fn the_node_names_the_component_and_the_records_still_name_the_method() {
    let root = repo_root();
    let mut stale: Vec<String> = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus"), root.join(".yidam/decisions")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if std::fs::read_to_string(&p).is_ok_and(|s| s.contains("Ix::")) {
                stale.push(
                    p.strip_prefix(&root)
                        .expect("under root")
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    stale.sort();
    assert_eq!(
        stale,
        [
            ".yidam/decisions/arrival-realizer.yml",
            ".yidam/decisions/two-components.yml",
            ".yidam/decisions/unread-state.yml",
            ".yidam/decisions/verb-menu.yml",
        ],
        "records name the method they were written about; no corpus node does"
    );

    let node = read(&root, ".yidam/corpus/subsystem/interaction-resolver.yml");
    assert!(
        flat(&node).contains("hand-writing `verb_menu::in_menu`"),
        "the node names the component"
    );
    for name in ["verb-menu", "interaction-resolver"] {
        let n = read(&root, &format!(".yidam/corpus/subsystem/{name}.yml"));
        assert!(
            n.contains(&format!("engine/{name}")),
            "{name} says where it is built"
        );
        assert!(
            n.contains("crates/reachability/tests/"),
            "and keeps the fixtures its figures are verified against"
        );
    }
}

/// **One implementation, and the dependency runs instrument to engine.**
///
/// A checker with its own evaluator is two systems that can disagree. This corpus holds the case
/// study — AGS ran a declarative layer and a scripting layer side by side — so the checker depends
/// on the engine rather than carrying a copy. Held as a name check against the manifests and the
/// search's own source; the behavioural evidence is that 158 fixtures passed unchanged through the
/// move.
#[test]
fn the_checker_depends_on_the_engine_rather_than_repeating_it() {
    let root = repo_root();
    let manifest = read(&root, "crates/reachability/Cargo.toml");
    for dep in ["verb-menu", "interaction-resolver"] {
        assert!(
            manifest.contains(&format!("{dep} = {{ path = \"../../engine/{dep}\" }}")),
            "the checker depends on {dep}"
        );
    }

    let search = read(&root, "crates/reachability/src/lib.rs");
    for gone in [
        "fn offered",
        "fn in_menu",
        "fn live",
        "fn apply",
        "fn settle",
        "fn holds",
        "fn spends",
    ] {
        assert!(
            !search.contains(gone),
            "the search declares `{gone}` again, which is the second copy this split exists to \
             prevent"
        );
    }
    assert!(
        search.contains("resolver::apply(") && search.contains("verb_menu::in_menu("),
        "and calls the components instead"
    );

    // Nothing in `engine/` reads the corpus, which is the other half of the separation.
    for c in ["verb-menu", "interaction-resolver"] {
        let src = read(&root, &format!("engine/{c}/src/lib.rs"));
        assert!(
            !src.contains(".yidam/corpus/") || src.matches(".yidam/corpus/").count() == 0,
            "an engine crate reading the corpus: {c}"
        );
    }
}
