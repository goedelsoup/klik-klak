//! The gate for `engine/`.
//!
//! `directories.md` asks a new top-level directory for a gate, and *"if what it holds derives from
//! the corpus, the gate is the derivation."* `design/`'s derivation is `rests_on:` and
//! `derives_effects_from:`. This one is a tier down: **a module may not invent a subsystem.**
//!
//! A **source file** under `engine/` declares one of two things in its first ten lines, as
//! markdown front matter or a `//!` doc comment:
//!
//! ```text
//! realizes: subsystem/verb-menu                 // it IS that component
//! composes: verb-menu, interaction-resolver     // it USES those components
//! ```
//!
//! and every name in either must be a subsystem the corpus declares. The unit is a `.rs` or `.md`
//! file and not a file: this asked a crate's `Cargo.toml` for a doc comment until
//! `.yidam/decisions/two-components.yml`, which is the rule that was writable over an empty
//! directory and refused the first thing the directory is for.
//!
//! `composes:` arrived one phase later for the same reason from the other side: a driver is a
//! client of components and none of them, so the only way to write one was to claim it was a
//! component it was not. `.yidam/decisions/playable.yml`.
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
    declaration(src).and_then(|(kind, names)| (kind == Kind::Realizes).then(|| names[0].clone()))
}

#[derive(PartialEq, Eq, Debug)]
enum Kind {
    /// this file is that component
    Realizes,
    /// this file uses those components and is none of them
    Composes,
}

/// What a source file says it is, from its first ten lines.
fn declaration(src: &str) -> Option<(Kind, Vec<String>)> {
    src.lines().take(10).find_map(|l| {
        let l = l.trim();
        let l = l.strip_prefix("//!").map(str::trim).unwrap_or(l);
        if let Some(v) = l.strip_prefix("realizes:") {
            let name = v
                .trim()
                .strip_prefix("subsystem/")?
                .trim_end_matches(".yml");
            return Some((Kind::Realizes, vec![name.to_string()]));
        }
        let v = l.strip_prefix("composes:")?;
        Some((
            Kind::Composes,
            v.split(',')
                .map(|n| n.trim().trim_start_matches("subsystem/").to_string())
                .filter(|n| !n.is_empty())
                .collect(),
        ))
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

/// **Every source file names subsystems the corpus declares, whether it is one or uses them.**
///
/// The whole of the derivation, and it is one rule with two spellings because there is one thing
/// an engine can get structurally wrong that a compiler will not catch: name a component nothing
/// described. The corpus is where components are described, and this is the edge between them.
///
/// The second spelling does not weaken the first. A `composes:` line still cannot name anything
/// the corpus has not declared, and a file with neither is still a failure — *silence is not
/// available* was the rule and still is.
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
        match declaration(&src) {
            Some((kind, names)) => {
                let verb = match kind {
                    Kind::Realizes => "realizes",
                    Kind::Composes => "composes",
                };
                for name in names {
                    if !known.contains(&name) {
                        bad.push(format!(
                            "{rel} {verb} subsystem/{name} — the corpus declares no such node. \
                             Either the component has not been described, or the corpus is \
                             missing one the engine needs"
                        ));
                    }
                }
            }
            None => anonymous.push(rel),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n  "));
    assert!(
        anonymous.is_empty(),
        "a file under engine/ declaring neither `realizes:` nor `composes:` in its first ten \
         lines:\n  {}",
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
/// name in four committed files and `[verified]` against the checker's own fixtures. What that
/// phase built was not the components; it was the place they say they are.
/// `.yidam/decisions/two-components.yml`.
///
/// Only `realizes:` counts here. `engine/play` names both components and is neither, and a list
/// of what is built that counted its clients would say the engine has more of itself than it
/// does.
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

/// **A client composes what it depends on, and nothing more.**
///
/// `engine/play` declares `composes: verb-menu, interaction-resolver` and depends on exactly those
/// two crates. The declaration is prose and the manifest is the build; holding them together is
/// what stops the first from becoming decoration.
///
/// `response-presenter` is deliberately not among them. The slice prints a line and returns, which
/// is that component's job done inline and done wrongly — `.yidam/decisions/playable.yml` states
/// it as a cost rather than hiding it behind a declaration that would claim the component exists.
#[test]
fn a_client_composes_exactly_the_components_it_depends_on() {
    let root = repo_root();
    let src = std::fs::read_to_string(root.join("engine/play/src/lib.rs")).expect("the slice");
    let (kind, names) = declaration(&src).expect("it declares something");
    assert_eq!(kind, Kind::Composes, "and what it declares is a client");
    assert_eq!(names, ["verb-menu", "interaction-resolver"]);

    let manifest = std::fs::read_to_string(root.join("engine/play/Cargo.toml")).expect("readable");
    for name in &names {
        assert!(
            manifest.contains(&format!("\n{name}.workspace = true")),
            "it composes {name} and does not depend on it"
        );
    }
    for other in declared(&root) {
        if !names.contains(&other) {
            assert!(
                !manifest.contains(&format!("\n{other}.workspace = true")),
                "it depends on {other} and does not say so"
            );
        }
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
