//! The gate for `engine/`.
//!
//! `directories.md` asks a new top-level directory for a gate, and *"if what it holds derives from
//! the corpus, the gate is the derivation."* `design/`'s derivation is `rests_on:` and
//! `derives_effects_from:`. This one is a tier down: **a module may not invent a subsystem.**
//!
//! A **source file** under `engine/` declares `realizes: subsystem/<name>` in its first ten
//! lines, as markdown front matter or a `//!` doc comment, and the corpus must declare that node.
//! The unit is a `.rs` or `.md` file and not a file: this asked a crate's `Cargo.toml` for a doc
//! comment until `.yidam/decisions/two-components.yml`, which is the rule that was writable over
//! an empty directory and refused the first thing the directory is for.
//!
//! When the corpus declares no such node the failure is deliberately ambiguous, in the way the
//! effect check is: either the module is a component nothing has described, or the corpus is
//! missing one the engine has just discovered it needs. Both are findings; silence is not
//! available.
//!
//! The second test pins which of the corpus's subsystems are built and which are not. It is an
//! assertion rather than a report because a standing line naming unbuilt components is
//! `inert-reports`'s INFO that everybody learns to scroll past — pinned, it says nothing until the
//! list moves, and then it says so loudly.
//!
//! The third is a rule the first crates made checkable: **a component may not invent a
//! dependency.** A cargo dependency from one engine crate to another must be a `depends-on` edge
//! between the subsystem nodes. The converse is not checked — the resolver also depends on
//! `path-solver` and `response-presenter`, and neither exists.
//!
//! `.yidam/decisions/engine-directory.yml`.

use design_gate::repo_root;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Every **source** file under `engine/`, less the directory's contract.
///
/// A manifest is not a module. `engine/Cargo.toml`, a lockfile and a `.gitignore` make no claim
/// about a subsystem and have nowhere to put one, and asking them for a doc comment was what the
/// rule did before a crate existed to be asked.
fn modules(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.join("engine")];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                stack.push(p);
            } else if p != root.join("engine/README.md")
                && p.extension().is_some_and(|x| x == "rs" || x == "md")
            {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The subsystem a module says it implements, from its first ten lines.
///
/// Two spellings because two kinds of file will need it, and a convention with one spelling per
/// kind is the kind this repository has found nine of by measuring. Both are declared in
/// `engine/README.md` rather than left to be derived from examples.
fn realizes(src: &str) -> Option<String> {
    src.lines().take(10).find_map(|l| {
        let l = l.trim();
        let l = l.strip_prefix("//!").map(str::trim).unwrap_or(l);
        l.strip_prefix("realizes:")
            .map(str::trim)
            .and_then(|v| v.strip_prefix("subsystem/"))
            .map(|v| v.trim_end_matches(".yml").to_string())
    })
}

fn declared(root: &Path) -> BTreeSet<String> {
    std::fs::read_dir(root.join(".yidam/corpus/subsystem"))
        .expect("the subsystem directory")
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            (p.extension()? == "yml").then(|| p.file_stem()?.to_str().map(str::to_string))?
        })
        .collect()
}

/// **Every module names a subsystem the corpus declares.**
///
/// The whole of the derivation, and it is one rule because there is one thing an engine can get
/// structurally wrong that a compiler will not catch: build a component nothing described. The
/// corpus is where components are described, and this is the edge between them.
#[test]
fn every_module_names_a_subsystem_the_corpus_declares() {
    let root = repo_root();
    let known = declared(&root);
    let mut bad = Vec::new();
    let mut anonymous = Vec::new();
    for p in modules(&root) {
        let rel = p
            .strip_prefix(&root)
            .unwrap_or(&p)
            .to_string_lossy()
            .into_owned();
        let Ok(src) = std::fs::read_to_string(&p) else {
            continue;
        };
        match realizes(&src) {
            Some(name) if known.contains(&name) => {}
            Some(name) => bad.push(format!(
                "{rel} realizes subsystem/{name} — the corpus declares no such node. Either the \
                 component has not been described, or the corpus is missing one the engine needs"
            )),
            None => anonymous.push(rel),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n  "));
    assert!(
        anonymous.is_empty(),
        "a file under engine/ declaring no `realizes:` in its first ten lines:\n  {}",
        anonymous.join("\n  ")
    );
}

/// **Four subsystems are specified and two are built.**
///
/// The engine's roadmap, derived from the corpus rather than written beside it — and pinned, so it
/// changes only when somebody builds a component or the corpus names one. Both of those should
/// come with a record; neither should arrive silently.
///
/// **It read `built.is_empty()` for one phase and was wrong the whole time.** `verb-menu` and
/// `interaction-resolver` were written — as private methods of `crates/reachability`, cited by
/// name in four committed files and `[verified]` against the checker's own fixtures. What this
/// phase built was not the components; it was the place they say they are.
/// `.yidam/decisions/two-components.yml`.
#[test]
fn the_corpus_specifies_four_components_and_two_are_built() {
    let root = repo_root();
    assert_eq!(
        declared(&root).into_iter().collect::<Vec<_>>(),
        [
            "interaction-resolver",
            "path-solver",
            "response-presenter",
            "verb-menu",
        ],
        "the subsystems this corpus declares"
    );

    let mut built: Vec<String> = modules(&root)
        .iter()
        .filter_map(|p| realizes(&std::fs::read_to_string(p).ok()?))
        .collect();
    built.sort();
    built.dedup();
    assert_eq!(
        built,
        ["interaction-resolver", "verb-menu"],
        "components that exist; `path-solver` and `response-presenter` do not"
    );

    // and the contract's table says the same, so a reader meets it without running anything
    let readme = std::fs::read_to_string(root.join("engine/README.md")).expect("the contract");
    for name in &built {
        assert!(
            readme.contains(&format!(
                "[`{name}`](../.yidam/corpus/subsystem/{name}.yml)"
            ))
                && readme
                    .lines()
                    .any(|l| l.contains(&format!("subsystem/{name}.yml"))
                        && l.ends_with("**built** |")),
            "engine/README.md does not mark {name} built"
        );
    }
}

/// **A component may not invent a dependency.**
///
/// One tier below *may not invent a subsystem*, and checkable only since there were two crates to
/// wire together. A cargo dependency from one engine crate to another must be a `depends-on` edge
/// between the subsystem nodes — `interaction-resolver -depends-on-> verb-menu` is declared, and
/// the crates are wired that way because the corpus says so rather than the other way round.
///
/// One direction only. The resolver also declares `depends-on` `path-solver` and
/// `response-presenter`, and a rule that required those to be cargo dependencies would require
/// building them.
#[test]
fn no_crate_depends_on_a_component_the_corpus_does_not_send_it_to() {
    let root = repo_root();
    let known = declared(&root);
    let mut edges: Vec<(String, String)> = Vec::new();
    for name in &known {
        let manifest = root.join(format!("engine/{name}/Cargo.toml"));
        let Ok(src) = std::fs::read_to_string(&manifest) else {
            continue;
        };
        for other in &known {
            if other != name && src.contains(&format!("\n{other}")) {
                edges.push((name.clone(), other.clone()));
            }
        }
    }
    assert_eq!(
        edges,
        [("interaction-resolver".to_string(), "verb-menu".to_string())],
        "dependencies between engine crates"
    );

    for (from, to) in &edges {
        let node =
            std::fs::read_to_string(root.join(format!(".yidam/corpus/subsystem/{from}.yml")))
                .expect("the node");
        let declared_edge = node
            .split("links:")
            .nth(1)
            .expect("its links")
            .contains(&format!("target: ../subsystem/{to}.yml"));
        assert!(
            declared_edge,
            "{from} depends on {to} in cargo and the corpus declares no such edge — either the \
             dependency is invented or the corpus is missing one the engine needs"
        );
    }
}

/// **The directory states its contract, and the contract says what the gate cannot see.**
///
/// `design/README.md` carries the same section for the same reason, and both were written because
/// this repository has twice overturned a conclusion by re-reading material it already held. A
/// gate's limits recorded beside the gate are the only ones anybody meets.
#[test]
fn the_contract_states_what_this_check_does_not_catch() {
    let root = repo_root();
    let readme = std::fs::read_to_string(root.join("engine/README.md")).expect("the contract");
    for phrase in [
        "It holds material that makes no claims",
        "may not invent a subsystem",
        "## What this gate does not catch",
        "It checks a name, not a component",
        "A manifest is not a module",
    ] {
        assert!(
            readme.contains(phrase),
            "engine/README.md no longer says {phrase:?}"
        );
    }
    assert!(
        readme.contains("realizes: subsystem/"),
        "and shows the declaration a module owes, in both spellings the gate reads"
    );
}
