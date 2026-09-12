//! **The engine plays a game, and the file that does it is not a component.**
//!
//! `engine/` held two crates, 391 lines of source, five tests and **no binary**. Every test drove
//! one component in isolation: the menu asked what it shows, the resolver asked what one binding
//! does. Nothing asked the two of them in turn, which is what playing is.
//!
//! The only thing in this repository that composes the components is `reachability::analyse` — and
//! it is a **search**. It asks the menu for what is offered, applies every one of them, and walks
//! the graph. A player picks one. The difference between those two sentences is the whole of what
//! is missing, and it lives in the instrument rather than in the engine.
//!
//! **The file that would do it could not be written.** The gate asked every source file for a
//! `realizes: subsystem/<name>`, and a driver is not one of the four and would not be one if there
//! were forty: it is a *client* of components. `composes:` is the second spelling, and it invents
//! nothing — every name in it must still be a subsystem the corpus declares, so *"silence is not
//! available"* survives intact. `.yidam/decisions/playable.yml`.
//!
//! **And the guarantee the resolver exists for is checked now.** *"Never nothing — a silent miss
//! is the failure players read as brokenness"* is that node's whole specification, and
//! `Report::silent` holds it against four games. Two phases walked past it: one opened the
//! question, the other gave it a type.
//!
//! The slice narrowed the check on its first run. Three turns of a winning run of
//! `lighthouse.game` say nothing and none is a miss — the world moved, and the movement is the
//! response.

use design_gate::repo_root;
use std::path::Path;

/// Prose wraps. Twelfth time.
fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

fn engine_crates(root: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(root.join("engine"))
        .expect("the engine directory")
        .flatten()
        .filter(|e| e.path().is_dir() && e.file_name() != "target")
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    out.sort();
    out
}

fn files(root: &Path, crate_: &str, dir: &str) -> Vec<std::path::PathBuf> {
    let mut out: Vec<std::path::PathBuf> =
        std::fs::read_dir(root.join(format!("engine/{crate_}/{dir}")))
            .map(|d| d.flatten().map(|e| e.path()).collect())
            .unwrap_or_default();
    out.sort();
    out
}

/// **Three crates, a binary, and something that drives both components.**
#[test]
fn the_engine_has_components_and_a_client_that_runs_them() {
    let root = repo_root();
    assert_eq!(
        engine_crates(&root),
        ["interaction-resolver", "play", "verb-menu"],
        "two components and a client"
    );

    let mut bins = 0;
    for c in engine_crates(&root) {
        for p in files(&root, &c, "src") {
            if p.file_name().is_some_and(|n| n == "main.rs") {
                bins += 1;
            }
        }
    }
    assert_eq!(bins, 1, "`cargo run -p play -- <game> [script]`");

    // and neither component reaches for the other's turn — the composition is the client's
    let menu = read(&root, "engine/verb-menu/src/lib.rs");
    assert!(
        !menu.contains("interaction_resolver"),
        "the menu does not resolve"
    );
    let resolver = read(&root, "engine/interaction-resolver/src/lib.rs");
    assert!(
        !resolver
            .lines()
            .any(|l| !l.trim_start().starts_with("//") && l.contains("verb_menu::in_menu")),
        "and the resolver does not decide what is shown — it names the function in prose only"
    );
}

/// **Two compositions now, and the difference between them is the whole of what was missing.**
///
/// `reachability::analyse` asks the menu what is offered and applies *every* one of them.
/// `play::Session::take` asks the same question and applies the one that was picked. A search and
/// a player, over one evaluator.
#[test]
fn the_search_applies_every_offer_and_the_client_applies_one() {
    let root = repo_root();

    let search = read(&root, "crates/reachability/src/lib.rs");
    assert!(
        search.contains("verb_menu::in_menu(") && search.contains("resolver::apply("),
        "the search drives both components"
    );
    assert!(
        search.contains("VecDeque") && search.contains("q.pop_front()"),
        "by walking a queue rather than by being told what to do"
    );

    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client.contains("shown(&self.world, &self.state)")
            && client.contains("respond(&self.world"),
        "and the client drives both components too"
    );
    assert!(
        !client.contains("VecDeque"),
        "by being told which one, which is the only difference between them"
    );

    // The state is the client's and no component's — all four subsystems declare none across
    // frames, which is what `.yidam/decisions/playable.yml` answered for `two-components`.
    assert!(
        client.contains("pub state: State,"),
        "the client owns the world"
    );
    for c in ["verb-menu", "interaction-resolver"] {
        let src = read(&root, &format!("engine/{c}/src/lib.rs"));
        assert!(
            !src.contains("pub struct") || !src.contains(": State,"),
            "{c} holds a state between calls"
        );
    }
}

/// **The gate has a spelling for a file that is not a component, and it invents nothing.**
///
/// `composes:` names components rather than claiming to be one, and every name in it is checked
/// against the corpus exactly as `realizes:` is. A file with neither still fails.
#[test]
fn a_file_that_composes_components_says_so_and_names_only_declared_ones() {
    let root = repo_root();
    let gate = read(&root, "crates/design-gate/tests/engine.rs");
    assert!(
        gate.contains("composes:") && gate.contains("Kind::Composes"),
        "the gate reads the second spelling"
    );
    assert!(
        gate.contains(
            "a file under engine/ declaring neither `realizes:` nor `composes:` in its first ten"
        ),
        "and a file with neither is still a failure"
    );

    let readme = read(&root, "engine/README.md");
    assert!(
        flat(&readme).contains("Silence is not available"),
        "which the contract states as the rule rather than as an accident"
    );
    assert!(
        flat(&readme).contains("composes: verb-menu, interaction-resolver"),
        "and shows the declaration in the form the gate reads"
    );

    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client.lines().take(10).any(|l| l.contains("composes:")),
        "the client declares what it composes"
    );
}

/// **Never nothing, held against four games.**
///
/// `Report::silent` is the guarantee as a check, and the mutation test beside it is what makes it
/// bite: deleting one clause of the lamp's guarded refusal leaves a reachable state in which the
/// oil can is offered, declines, and says nothing — while **every figure the search reports stays
/// identical**, because a refusal writes no state.
#[test]
fn never_nothing_is_promised_and_something_checks_it() {
    let root = repo_root();
    let search = read(&root, "crates/reachability/src/lib.rs");
    assert!(
        search.contains("pub silent: Vec<String>,"),
        "the report carries it"
    );
    assert!(
        search.contains("Shown, declining, and saying nothing:"),
        "and renders it"
    );

    let fixture = read(&root, "crates/reachability/tests/silence.rs");
    assert!(
        fixture.contains("fn nothing_authored_here_declines_in_silence")
            && fixture.contains("fn deleting_a_clause_of_a_guarded_refusal_is_reported"),
        "the guarantee and the mutation that would break it"
    );

    // and the client tells a silent turn from a wordless one, which is the distinction the slice
    // found and neither the corpus nor the checker had
    let client = read(&root, "engine/play/src/lib.rs");
    assert!(
        client.contains("pub fn silent(&self) -> bool {")
            && flat(&client).contains("self.said.is_none() && self.moved.is_none()"),
        "a turn is silent when nothing was said and nothing visible changed"
    );
}
