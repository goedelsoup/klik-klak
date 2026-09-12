//! **Three calculators were approved at genesis to read the corpus. All three were answered by
//! reading a game, and two of them are retired without ever having run.**
//!
//! `.yidam/decisions/proposals.yml` approved `vocabulary-coverage`, `collapse-signature` and
//! `reachability`, each written as a stub, *"none is run at genesis because none has enough to
//! read."* One objection was raised, against `reachability`: **"It has nothing to read."**
//!
//! That objection was right, and it was right about all three. The resolution in every case was
//! the same artifact — `design/specimen/`, which did not exist at genesis and is corpus input for
//! none of them:
//!
//! * `reachability` was **built**, and reads a `.game` file.
//! * `vocabulary-coverage`'s question was answered by `.yidam/decisions/unwritten.yml`, counting
//!   constructs across four specimens.
//! * `collapse-signature`'s question — *"the falsifiable form of 'is cutscene just an
//!   interaction?'"* — was answered by `corpus/cutscene/scripted-sequence.yml`, and that skill's
//!   own page says so: *"This skill was scaffolded at genesis to make exactly that argument, and
//!   it was not what produced the answer. A **specimen** was."*
//!
//! Both stubs had come to argue against themselves — not *"there is not enough to read yet"*,
//! which is a reason to wait, but *"the computation I was approved to perform is not the one that
//! answers my question"*. Neither had been edited in 69 and 73 of this repository's 86 phases.
//! `.yidam/decisions/calculators.yml` retires them.
//!
//! And the two prelude domains kept at the vendor step to serve them are cited by nothing.

use design_gate::repo_root;
use std::path::Path;

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **Two of three are retired, and each page says what answered its question.**
///
/// Retired rather than deleted, the way `verb/give-to.yml` is kept: a list that grows by accident
/// needs a record of what it declined. The argument each page had come to make against itself is
/// kept with it — that is the reason, and a reason deleted is a reason somebody re-derives.
#[test]
fn two_of_the_three_calculators_are_retired_and_say_what_answered_them() {
    let root = repo_root();

    let coverage = read(&root, ".yidam/skills/vocabulary-coverage.md");
    let collapse = read(&root, ".yidam/skills/collapse-signature.md");
    let reachability = read(&root, ".yidam/skills/reachability.md");

    assert!(coverage.contains("**Status: retired.**"));
    assert!(collapse.contains("**Status: retired.**"));
    assert!(
        reachability.contains("**Status: implemented.** `crates/reachability`"),
        "and the third is built"
    );

    // the argument each had come to make against itself, kept
    assert!(
        flat(&collapse).contains(
            "The question this skill was built to answer is answerable, and a similarity score \
             over declared vocabularies is not the instrument that answers it."
        ),
        "`collapse-signature` says a score is not the instrument"
    );
    assert!(
        flat(&coverage).contains(
            "A coverage report over verbs cannot tell you which class the gap is in, and this one \
             was read as though it could."
        ),
        "`vocabulary-coverage` says the same thing about its own list"
    );

    // and no marker is left standing on a question that is retired rather than answered
    for (name, page) in [
        ("vocabulary-coverage", &coverage),
        ("collapse-signature", &collapse),
    ] {
        assert!(
            !page.contains("[open]"),
            "{name} still emits a marker for a question nothing is waiting on"
        );
    }
    assert!(
        flat(&coverage).contains("That question is closed by retirement rather than by answer."),
        "and the retraction says which kind of closing it was"
    );
}

/// **The stale sentence is corrected rather than deleted.**
///
/// `vocabulary-coverage` closed on *"The exit primitive it was pointing at is still unwritten"*,
/// which `.yidam/decisions/exit-archetype.yml` made false 73 phases ago. A retired page carrying a
/// false claim about the present is worse than one carrying a superseded argument.
#[test]
fn the_page_no_longer_says_a_closed_gap_is_open() {
    let root = repo_root();
    let coverage = read(&root, ".yidam/skills/vocabulary-coverage.md");
    assert!(
        !coverage.contains("The exit primitive it was pointing at is still unwritten."),
        "the sentence that was false for 73 phases"
    );
    assert!(
        flat(&coverage).contains(
            "**And the exit primitive it was pointing at was written, one phase later.**"
        ),
        "and what replaced it says what happened"
    );
    let parse = read(&root, "crates/game-format/src/parse.rs");
    assert!(
        parse.contains("\"exit\" => {"),
        "the reader dispatches on `exit`"
    );
}

/// **Each was answered by a specimen, which is corpus input for none of them.**
///
/// `collapse-signature` says so on its own page. `vocabulary-coverage`'s question was answered by
/// `unwritten.yml` — six constructs written by nothing, twenty written twice or fewer — which
/// reads `design/specimen/` and is pinned in `crates/game-format/tests/coverage.rs`. Both skills
/// declare what they read, and neither list includes a game.
#[test]
fn each_question_was_answered_by_an_artifact_the_skill_does_not_read() {
    let root = repo_root();

    let coverage = read(&root, ".yidam/skills/vocabulary-coverage.md");
    assert!(
        flat(&coverage).contains("The specimen is **not corpus input**."),
        "`vocabulary-coverage` says the answer came from a file it does not read"
    );
    let collapse = read(&root, ".yidam/skills/collapse-signature.md");
    assert!(
        flat(&collapse).contains("it was not what produced the answer. A **specimen** was"),
        "and `collapse-signature` says the same"
    );

    // and the answer that was produced is committed, over four games
    let unwritten = read(&root, ".yidam/decisions/unwritten.yml");
    assert!(
        flat(&unwritten).contains("six constructs never at all"),
        "the coverage question, answered"
    );
    let cutscene = read(&root, ".yidam/corpus/cutscene/scripted-sequence.yml");
    assert!(
        cutscene.contains("specimen"),
        "the collapse question, answered against a specimen"
    );
}

/// **Both prelude domains kept at the vendor step are cited by nothing.**
///
/// `proposals.yml` closes on `prelude_domains: [similarity, graph-topology]` — *"Thirteen of the
/// fifteen domain libraries are dropped at the vendor step."* `similarity` was kept for
/// `collapse-signature`, which is a stub. `graph-topology` was kept for `reachability`, which was
/// built and does not use it: that crate depends on `game-format`, `verb-menu` and
/// `interaction-resolver`, and on nothing else.
///
/// 208 KB of vendored code, reached for by no line in either workspace.
#[test]
fn the_two_domains_kept_for_the_calculators_are_used_by_neither() {
    let root = repo_root();

    let proposals = read(&root, ".yidam/decisions/proposals.yml");
    assert!(
        proposals.contains("prelude_domains: [similarity, graph-topology]"),
        "the two kept"
    );

    for domain in ["similarity", "graph-topology"] {
        assert!(
            root.join(".yidam/.vendor/prelude/domains")
                .join(domain)
                .is_dir(),
            "{domain} is vendored"
        );
    }

    let mut citing = Vec::new();
    for workspace in ["crates", "engine"] {
        let mut stack = vec![root.join(workspace)];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n == "target") {
                        continue;
                    }
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "rs" || x == "toml")
                    // this file names both domains in order to ask about them, which is
                    // discussing and not using — the distinction that has caught a census here
                    // before
                    && p.file_name().is_some_and(|n| n != "calculators.rs")
                {
                    let Ok(src) = std::fs::read_to_string(&p) else {
                        continue;
                    };
                    for domain in ["similarity", "graph_topology", "graph-topology"] {
                        if src.contains(domain) {
                            citing.push(format!("{}: {domain}", p.display()));
                        }
                    }
                }
            }
        }
    }
    assert!(
        citing.is_empty(),
        "a domain is used now, which changes what retiring it would cost: {citing:?}"
    );

    let manifest = read(&root, "crates/reachability/Cargo.toml");
    assert!(
        !manifest.contains("graph"),
        "the calculator the domain was kept for does not depend on it"
    );
}

/// **The objection raised at genesis was true of all three, and named only one.**
///
/// *"`reachability` calls `graph-topology`, and it was approved over a stated objection. **It has
/// nothing to read.** … this corpus holds the vocabulary a game would be authored in, not any
/// game."*
///
/// The corpus still holds the vocabulary a game would be authored in. What changed is that
/// `design/` holds four games, and every one of the three questions was answered there.
#[test]
fn the_objection_named_one_calculator_and_was_true_of_three() {
    let root = repo_root();
    let proposals = flat(&read(&root, ".yidam/decisions/proposals.yml"));
    assert!(
        proposals.contains(
            "**It has nothing to read.** Unwinnable-state detection is a reachability computation \
             over an authored game's state graph, and this corpus holds the vocabulary a game \
             would be authored in, not any game."
        ),
        "the objection, as it was written"
    );
    assert!(
        proposals.contains("none is run at genesis because none has enough to read"),
        "and what was said about all three"
    );

    // four games now, and the corpus is not where any of them lives
    let games = std::fs::read_dir(root.join("design/specimen"))
        .expect("the specimen directory")
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "game"))
        .count();
    assert_eq!(games, 4, "specimens, in `design/` and not in the corpus");
}
