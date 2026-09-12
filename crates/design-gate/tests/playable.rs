//! **Three components, and nothing plays a game.**
//!
//! `engine/` holds two crates, 391 lines of source, five tests and **no binary**. Every test
//! drives one component in isolation: the menu is asked what it shows, the resolver is asked what
//! one binding does. Nothing asks the two of them in turn, which is what playing is.
//!
//! The only thing in this repository that composes the components is `reachability::analyse` — and
//! it is a **search**. It asks the menu for what is offered, applies every one of them, and walks
//! the graph. A player picks one. The difference between those two sentences is the whole of what
//! is missing, and it lives in the instrument rather than in the engine.
//!
//! **The file that would do it cannot be written.** `engine/`'s gate asks every source file under
//! the directory for a `realizes: subsystem/<name>` naming a node the corpus declares. A driver is
//! not one of the four and would not be one if there were forty: it is a *client* of components,
//! and the gate has no spelling for that. *"Silence is not available"* is the rule, and the only
//! way to write this file today is to claim it is a component it is not.
//!
//! **And the guarantee the resolver exists for is unchecked.** `interaction-resolver`'s whole
//! specification is *"never nothing — a silent miss is the failure players read as brokenness"*,
//! and `Outcome::Refused { says: None }` is that failure with a name. Nothing holds it against a
//! game. `.yidam/decisions/authored-text.yml` opened the question and declined to check it;
//! `.yidam/decisions/two-components.yml` gave it a type and did not check it either.
//!
//! These checks pin the state. They are rewritten into the rule in the same phase.

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

/// **Two crates, no binary, and every test drives one component alone.**
#[test]
fn the_engine_has_components_and_no_way_to_run_them() {
    let root = repo_root();
    assert_eq!(
        engine_crates(&root),
        ["interaction-resolver", "verb-menu"],
        "the components that exist"
    );

    let (mut src, mut tests, mut bins) = (0, 0, 0);
    for c in engine_crates(&root) {
        for p in files(&root, &c, "src") {
            src += std::fs::read_to_string(&p)
                .expect("readable")
                .lines()
                .count();
            if p.file_name().is_some_and(|n| n == "main.rs") {
                bins += 1;
            }
        }
        for p in files(&root, &c, "tests") {
            tests += std::fs::read_to_string(&p)
                .expect("readable")
                .matches("\n#[test]")
                .count();
        }
    }
    assert_eq!(
        (src, tests, bins),
        (391, 5, 0),
        "source lines, tests, binaries"
    );

    // and neither component reaches for the other's turn
    let menu = read(&root, "engine/verb-menu/src/lib.rs");
    assert!(
        !menu.contains("interaction_resolver"),
        "the menu does not resolve"
    );
}

/// **The only thing that composes the components is a search, and it is in the instrument.**
///
/// `analyse` asks the menu what is offered and applies *every* one of them. A player picks one.
/// That difference is the slice, and nothing in `engine/` performs it.
#[test]
fn the_one_composition_that_exists_is_a_search() {
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

    for c in engine_crates(&root) {
        for p in files(&root, &c, "src")
            .into_iter()
            .chain(files(&root, &c, "tests"))
        {
            let s = std::fs::read_to_string(&p).expect("readable");
            assert!(
                !s.contains("verb_menu::") || !s.contains("interaction_resolver::"),
                "{}: something in engine/ composes both components",
                p.display()
            );
        }
    }
}

/// **The gate has no spelling for a file that is not a component.**
///
/// A driver composes components and is none of them. The rule asks every source file to name the
/// subsystem it realizes, so the only way to write one today is to claim it is something it is
/// not — and `engine/README.md` says *"silence is not available"*, which is exactly right and
/// leaves nowhere to put this.
#[test]
fn a_file_that_composes_components_cannot_be_written_here() {
    let root = repo_root();
    let gate = read(&root, "crates/design-gate/tests/engine.rs");
    assert!(
        gate.contains("a file under engine/ declaring no `realizes:` in its first ten lines"),
        "every source file must name a subsystem"
    );
    assert!(
        !gate.contains("composes:"),
        "and there is no second spelling for a client of components"
    );

    let readme = read(&root, "engine/README.md");
    assert!(
        flat(&readme).contains("Silence is not available"),
        "which the contract states as the rule rather than as an accident"
    );
    assert!(
        flat(&readme).contains("nothing here has a loop, a clock, or an output"),
        "and the contract already says what is missing"
    );
}

/// **The guarantee the resolver exists for is unchecked.**
///
/// *"Never nothing"* is the whole specification, `Outcome::Refused { says: None }` is the failure
/// it names, and no test anywhere holds it against a game. Two phases have walked past it: one
/// opened the question, the other gave it a type.
#[test]
fn never_nothing_is_promised_and_nothing_checks_it() {
    let root = repo_root();
    let resolver = read(&root, "engine/interaction-resolver/src/lib.rs");
    assert!(
        resolver.contains("Refused { says: Option<&'a str> },"),
        "the failure has a name in the type"
    );
    assert!(
        flat(&resolver).contains("`says: None` is the hole"),
        "and the doc comment says what it is"
    );

    let mut checking: Vec<String> = Vec::new();
    for dir in ["engine", "crates"] {
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n == "target") {
                        continue;
                    }
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "rs")
                    // used versus discussed, for the twelfth time: this file names the failure in
                    // order to say nothing checks it, and a scan of test sources reads itself
                    && p.file_name().is_some_and(|n| n != "playable.rs")
                    && p.components().any(|c| c.as_os_str() == "tests")
                    && std::fs::read_to_string(&p)
                        .is_ok_and(|s| s.contains("Refused") && s.contains("says: None"))
                {
                    checking.push(p.display().to_string());
                }
            }
        }
    }
    assert!(
        checking.is_empty(),
        "something checks it after all: {checking:?}"
    );
}
