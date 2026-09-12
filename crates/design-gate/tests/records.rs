//! **There are two corpora in this repository and the tooling knows about one.**
//!
//! | | `.yidam/corpus/` | `.yidam/decisions/` |
//! |---|---|---|
//! | files | 43 | **47** |
//! | bytes of `.yml` | 359 K | **316 K** |
//! | tagged claims | 559 | **321** |
//! | live open questions | 9 | **29** |
//! | citations out | 96 edges + 87 prose | **254** |
//!
//! They were 42 apiece when this file was written. The counting rule is stated in the row because
//! this repository has been bitten twice by a headline figure measured two ways — the corpus row
//! counts `.yml` only, and there are 31 Markdown files under `.yidam/corpus/` that it excludes and
//! `crates/design-gate/tests/pages.rs` is the gate for.
//!
//! `yidam status` says *43 nodes*. `graph-check` checks *43 instances*. `lint` lints them. The
//! index would index them. None of them is a decision record, and none of them is wrong to ignore
//! one: a node asserts something about point-and-click adventure games, a record asserts something
//! about what this project chose. They are different kinds and the vendored tooling is right to
//! count only the first.
//!
//! **`cli/v0.12.0` gave the records a reader**, which is half of what this file was written
//! because they did not have. `yidam decisions-log` lists all 46, newest first, with each
//! `summary:`. It is read-only, owns no REGEN block, and reads exactly the one field this
//! repository has never edited — see `no_record_has_changed_what_it_decided`. So the two indexes
//! of this directory are complementary rather than redundant: the command says what was decided,
//! and the file below says what is still owed.
//!
//! What is not right is that the second kind has had no gate at all while growing to the same
//! size as the first. [`markers.rs`] found the questions had migrated here; [`edges.rs`] found the
//! citations had — **113 of the corpus's 217 prose references now point into this directory
//! against 87 at another node**, a share that has risen in every phase since anything measured it.
//! This file is the gate for where they went.
//!
//! **One symptom is named here and deliberately not built.** The 321 claims in this directory are
//! absent from `yidam status`, which reports the corpus's tally as though it were the artifact's.
//! Counting them here would put a second figure beside the first under a different rule — the two
//! directories are tallied by different code, and this repository has twice been bitten by exactly
//! that (`.yidam/decisions/verb-count.yml`, where two of its own files disagreed on a headline
//! number for six phases). The honest fix is upstream in a binary this repository does not own.
//! It is written down and left.
//!
//! **And the counting rule earns its place on first contact.** There are 42 `[open]` tokens here
//! and **25 live questions**. The other seventeen were the word being discussed rather than used —
//! *"Four `[open]`s, all the same question"*, *"That `[open]` closes"* — which is precisely the
//! failure [`marker-count`] found in the corpus, where a node was reported open by the sentence
//! saying it was not. The rule it chose, that a live marker ends its paragraph, separates all 42
//! with no false positive on its first run against a body it was never tested on.
//!
//! [`marker-count`]: ../../../.yidam/decisions/marker-count.yml
//!
//! [`markers.rs`]: markers.rs
//! [`edges.rs`]: edges.rs

use design_gate::repo_root;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn records(root: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root.join(".yidam/decisions"))
        .expect("the directory exists")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yml"))
        .collect();
    out.sort();
    out
}

fn stem(p: &Path) -> String {
    p.file_stem()
        .expect("a name")
        .to_string_lossy()
        .into_owned()
}

/// What a record's prose points at, split by whether the spelling commits to being a path.
///
/// The distinction is not decoration and it is why this check took a second attempt. A record
/// writes `` `crates/design-gate/tests/markers.rs` `` and `` `markers.rs` `` in the same
/// paragraph, and writes `` `DialogScript.md` `` for a page of a manual that is not in this
/// repository at all. A markdown link and a token beginning with a top-level directory are
/// **citations**: written to be followed, and a failure to resolve one is rot. A bare backticked
/// name is a **mention**, which a reader resolves and a checker cannot. Treating mentions as
/// citations reported twenty-four findings and every one was wrong.
///
/// Records cite four kinds of thing where a corpus node cites two: sibling records, corpus nodes,
/// catalog entries, and **committed test files** — the last with no other check anywhere, so
/// renaming one silently unsources whatever `[verified]` claims rest on it.
fn references(src: &str) -> (BTreeSet<String>, BTreeSet<String>) {
    let (mut cite, mut mention) = (BTreeSet::new(), BTreeSet::new());
    let keep = |s: &str| -> Option<String> {
        let s = s
            .split('#')
            .next()
            .unwrap_or("")
            .trim_end_matches(['.', ',']);
        (s.ends_with(".yml") || s.ends_with(".md") || s.ends_with(".rs")).then(|| s.to_string())
    };
    let rooted = |s: &str| {
        ["crates/", "design/", ".yidam/"]
            .iter()
            .any(|d| s.starts_with(d))
    };

    let b: Vec<char> = src.chars().collect();
    for i in 0..b.len().saturating_sub(1) {
        if b[i] == ']' && b[i + 1] == '(' {
            let t: String = b[i + 2..]
                .iter()
                .take_while(|c| **c != ')' && !c.is_whitespace())
                .collect();
            if let Some(t) = keep(&t) {
                cite.insert(t);
            }
        }
    }
    for chunk in src.split('`').skip(1).step_by(2) {
        if let Some(t) = keep(chunk.trim()) {
            if rooted(&t) { &mut cite } else { &mut mention }.insert(t);
        }
    }
    for w in src.split(|c: char| c.is_whitespace() || "()[],;\"".contains(c)) {
        if rooted(w) {
            if let Some(t) = keep(w) {
                cite.insert(t);
            }
        }
    }
    (cite, mention)
}

fn resolve(root: &Path, from: &Path, r: &str) -> Option<PathBuf> {
    [
        from.parent().unwrap_or(root).join(r),
        root.join(r),
        root.join(".yidam").join(r),
        root.join(".yidam/corpus").join(r),
    ]
    .into_iter()
    .find(|p| p.exists())
}

/// A live question: a line whose prose ends at the token, with the paragraph ending there too.
///
/// The convention is [`marker-count`]'s and it is the only reason the count is readable: a
/// retraction says the question is closed without re-emitting the token. It has been checked on
/// the corpus since `markers.rs` was written and never here, which is where 42 of the 49 markers
/// are.
///
/// [`marker-count`]: ../../../.yidam/decisions/marker-count.yml
fn questions(src: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = src.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter(|(i, l)| {
            l.trim_end().ends_with("`[open]`")
                && lines.get(i + 1).is_none_or(|n| n.trim().is_empty())
        })
        .map(|(i, _)| {
            // the sentence the marker ends, gathered backwards to the paragraph break
            let mut start = i;
            while start > 0 && !lines[start - 1].trim().is_empty() {
                start -= 1;
            }
            let text = lines[start..=i]
                .iter()
                .map(|l| l.trim())
                .collect::<Vec<_>>()
                .join(" ");
            (i + 1, text.trim_end_matches("`[open]`").trim().to_string())
        })
        .collect()
}

fn cites(root: &Path) -> BTreeMap<String, BTreeSet<String>> {
    let names: BTreeSet<String> = records(root).iter().map(|p| stem(p)).collect();
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for p in records(root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let me = stem(&p);
        let mut seen = BTreeSet::new();
        let (cite, mention) = references(&src);
        for r in cite.iter().chain(mention.iter()) {
            let Some(q) = resolve(root, &p, r) else {
                continue;
            };
            if q.extension().is_some_and(|x| x == "yml")
                && q.parent().is_some_and(|d| d.ends_with("decisions"))
            {
                let t = stem(&q);
                if t != me && names.contains(&t) {
                    seen.insert(t);
                }
            }
        }
        out.insert(me, seen);
    }
    out
}

/// The commit that first wrote a file, and the file as it stood there.
///
/// Needs real history. CI checks out with `fetch-depth: 0` for the commit-vocabulary gate, and
/// this is the second thing that depends on it; a shallow clone reports the truncation rather
/// than quietly passing.
/// Whether git is tracking this file at all.
///
/// A record being written right now has no commits, which is not the same condition as a clone
/// with no history — and the first version of this check could not tell them apart, so the first
/// new record after it was written made it panic with a message about shallow clones. Untracked is
/// checked separately and skipped: *"has this changed since the commit that wrote it"* is not yet
/// a question about a file no commit has written.
fn tracked(root: &Path, p: &Path) -> bool {
    let Ok(rel) = p.strip_prefix(root) else {
        return false;
    };
    std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "--error-unmatch", "--"])
        .arg(rel)
        .output()
        .is_ok_and(|o| o.status.success())
}

fn at_creation(root: &Path, p: &Path) -> Option<String> {
    let rel = p.strip_prefix(root).ok()?.to_string_lossy().into_owned();
    let log = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["log", "--format=%H", "--"])
        .arg(&rel)
        .output()
        .ok()?;
    let first = String::from_utf8_lossy(&log.stdout)
        .lines()
        .last()?
        .to_string();
    let show = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("show")
        .arg(format!("{first}:{rel}"))
        .output()
        .ok()?;
    show.status
        .success()
        .then(|| String::from_utf8_lossy(&show.stdout).into_owned())
}

/// A record's `summary:` block, whitespace-flattened.
fn summary(src: &str) -> String {
    let Some(i) = src.find("\nsummary:") else {
        return String::new();
    };
    let rest = &src[i + 1..];
    let end = rest
        .match_indices('\n')
        .find(|(j, _)| {
            let line = rest[j + 1..].lines().next().unwrap_or("");
            !line.is_empty() && !line.starts_with(char::is_whitespace) && line.contains(':')
        })
        .map(|(j, _)| j)
        .unwrap_or(rest.len());
    rest[..end].split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **The check that has teeth, pointed at the half of the artifact that never had one.**
///
/// `edges.rs` built this for corpus prose and its first finding was a reference written by the
/// commit that added it. Records carry more citations than nodes do and a wider range of target:
/// a renamed test file under `crates/` silently unsources a `[verified]` claim, and 23 records
/// cite one.
#[test]
fn every_path_written_in_a_record_resolves() {
    let root = repo_root();
    let (mut bad, mut total, mut mentions, mut tests) = (Vec::new(), 0, 0, BTreeSet::new());
    for p in records(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let (cite, mention) = references(&src);
        mentions += mention.len();
        for r in &cite {
            total += 1;
            if r.ends_with(".rs") {
                tests.insert(r.clone());
            }
            if resolve(&root, &p, r).is_none() {
                bad.push(format!("{}  ->  {r}", stem(&p)));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a path written in a decision record must point at something:\n  {}",
        bad.join("\n  ")
    );
    assert_eq!((total, mentions, tests.len()), (589, 97, 74));
}

/// **Eight of eighty-one records are cited by nothing, against three of forty-four nodes.**
///
/// It was sixteen when this check was written and ten when `action-pages` was written; that record
/// cited three of the ten, and every one of the three had been uncited for the same reason — the
/// file asking the question it answers had not been opened since the root commit.
///
/// The corpus's own orphan rule is `yidam lint`'s `orphan-in`, and `prose-edges` showed it finds
/// real defects — two edges owed and written. This layer has four times the orphan rate and no
/// rule at all.
///
/// It is not the same defect. A node with no incoming edge is cut out of the graph; a record with
/// no incoming citation may simply be the last word on something nothing else has needed. What it
/// cannot be is *unnoticed*, which is what it has been.
#[test]
fn a_record_nothing_cites_is_named() {
    let root = repo_root();
    let c = cites(&root);
    let cited: BTreeSet<&String> = c.values().flatten().collect();
    let mut alone: Vec<&String> = c.keys().filter(|k| !cited.contains(k)).collect();
    alone.sort();
    assert_eq!(
        alone,
        [
            "binding-axes",
            "calculators",
            "declared-reach",
            "domain-computer",
            "held",
            "line-format",
            "name-scoping",
            "settled-surface",
        ],
        "eight, and `normal-form` is not here because the record it answers cites it back. \
         A phase that retracts an earlier `[open]` writes into the record that raised it, so \
         the newest record is cited by the oldest thing it touched on the day it lands — which \
         is the one way this list empties from the front. `held` stays for the reason it \
         arrived: it reconciles two standing positions rather than answering a question either \
         of them left open, so nothing had a marker to retract. Reconciling is the one kind of \
         work this list cannot see — and `input-side` showed a second for one phase: a phase \
         whose subject is a retrieval revises a corpus node rather than a record, so it touches \
         nothing here and lands uncited on the day it arrives. Both it and `class-library` came \
         off the front the ordinary way and one commit apart, which is the mechanism twice: \
         `class-library` read the documents `input-side` had named, and `input-side`'s marker \
         now cites the record that corrected it"
    );
    let edges: usize = c.values().map(|v| v.len()).sum();
    assert_eq!((c.len(), edges), (81, 276));
}

/// **The directory holding half the artifact had no index, and now the index is the listing.**
///
/// `.yidam/corpus/` and `.yidam/catalog/` each carry a README with a REGEN block. This directory
/// carried nothing, which is why 42 open questions have been unlistable: there was no command to
/// run and no file to open.
///
/// The block is generated here rather than by `yidam regen`, because a REGEN block is keyed to a
/// yidam subcommand and adding one is a change to a binary this repository does not own. The gate
/// is the same shape regardless — regenerate, compare, fail on stale — and the crate that already
/// hosts corpus hygiene hosts this too.
///
/// `REGEN=1 cargo test -p design-gate --test records` rewrites it.
#[test]
fn the_index_this_directory_never_had_is_current() {
    let root = repo_root();
    let readme = root.join(".yidam/decisions/README.md");
    let mut body = String::new();
    let c = cites(&root);
    let cited: BTreeSet<&String> = c.values().flatten().collect();

    body.push_str("## Open questions\n\n");
    body.push_str(
        "Every live `[open]` in this directory, which no yidam command can enumerate: they are\n\
         counted by `status` only when they sit in a corpus node. The convention is\n\
         [marker-count](marker-count.yml)'s — the token names a live question and a retraction\n\
         does not re-emit it.\n\n",
    );
    let mut total = 0;
    for p in records(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let qs = questions(&src);
        if qs.is_empty() {
            continue;
        }
        body.push_str(&format!("### [{}]({}.yml)\n\n", stem(&p), stem(&p)));
        for (line, text) in qs {
            total += 1;
            let one = text.split_whitespace().collect::<Vec<_>>().join(" ");
            let short = if one.chars().count() > 300 {
                format!("{}…", one.chars().take(300).collect::<String>())
            } else {
                one
            };
            body.push_str(&format!("- **:{line}** — {short}\n"));
        }
        body.push('\n');
    }
    body.push_str(&format!("**{total} open.**\n\n## What nothing cites\n\n"));
    body.push_str(
        "Records no other record points at. Not a defect on its own — a record may be the last\n\
         word on something nothing else has needed — but it is the shape `orphan-in` reports for\n\
         nodes, at four times the rate, and it had never been looked at.\n\n",
    );
    for k in c.keys().filter(|k| !cited.contains(k)) {
        body.push_str(&format!("- [{k}]({k}.yml)\n"));
    }

    // What a record rests on, which it has always declared in its own prose and which nothing
    // showed until `inward-turn` counted it. A path is cited when a filename follows it:
    // `catalog/delores-source.md` is evidence, `` `catalog/` `` is a sentence about the catalog.
    let cites_path = |src: &str, prefix: &str| {
        src.match_indices(prefix).any(|(i, _)| {
            src[i + prefix.len()..].starts_with(|ch: char| ch.is_ascii_alphanumeric())
        })
    };
    let (mut gate_alone, mut unfooted) = (Vec::new(), Vec::new());
    for p in records(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let document = cites_path(&src, "catalog/");
        let engine =
            cites_path(&src, "crates/reachability/") || cites_path(&src, "crates/game-format/");
        let gate = cites_path(&src, "crates/design-gate/");
        if gate && !document && !engine {
            gate_alone.push(stem(&p));
        } else if !gate && !document && !engine {
            unfooted.push(stem(&p));
        }
    }
    body.push_str("\n## What each record rests on\n\n");
    body.push_str(
        "A record cites a catalog entry, the engine crates, or the gate that reads this corpus,\n\
         and which it cites says what kind of question it answered. All three are legitimate.\n\
         The classification was derivable from the day this directory existed and nothing showed\n\
         it, which is [inward-turn](inward-turn.yml)'s finding; the run at the end of the first\n\
         list is the one that record holds as a tripwire.\n\n",
    );
    body.push_str("**On the gate and nothing else** — a question about this repository.\n\n");
    for k in &gate_alone {
        body.push_str(&format!("- [{k}]({k}.yml)\n"));
    }
    body.push_str(
        "\n**On no evidence path named here** — structural, or resting on the prelude.\n\n",
    );
    for k in &unfooted {
        body.push_str(&format!("- [{k}]({k}.yml)\n"));
    }

    let full = format!(
        "# decisions\n\n\
         A record is a claim about what this project chose and why. A [corpus\n\
         node](../corpus/README.md) is a claim about point-and-click adventure games. They are\n\
         different kinds, which is why `yidam status` counts 42 nodes and not 84, and why this\n\
         directory has its own gate instead of moving: `crates/design-gate/tests/records.rs`.\n\n\
         Everything below the line is generated. `REGEN=1 cargo test -p design-gate --test \
         records` rewrites it, and `cargo test` fails when it is stale.\n\n---\n\n{body}"
    );

    if std::env::var("REGEN").is_ok() {
        std::fs::write(&readme, &full).expect("writable");
    }
    let have = std::fs::read_to_string(&readme).unwrap_or_default();
    assert_eq!(
        have, full,
        "the index is stale — REGEN=1 cargo test -p design-gate --test records"
    );
    assert_eq!(total, 56, "live open questions in this directory");
}

/// **What a record decided has never changed. What it owes changes constantly, and the two are
/// different fields.**
///
/// The prelude's lifecycle for this directory is one sentence: *"Decision files are permanent
/// records — they are not updated when a decision is superseded, but a new decision may reference
/// a prior one by `id`."* This repository edits them. **Twenty-six of sixty-one have a commit
/// after the one that wrote them** — and correcting that sentence has been five of the twenty-six,
/// which is the shape of this whole directory.
///
/// Read against the rule that looks like a violation. Measured, it is not: **twenty-two of the
/// twenty-six kept their `summary:` byte-for-byte**, and the rest changed a *figure* —
/// `latch-default` went from `31 of 40` to `32 of 41` when a census defect was found in the
/// extractor that produced it. No record's decision has ever been revised. What the edits do is
/// keep the *account*: what the figures turned out to be, what the record left open, what closed
/// it later.
///
/// So the rule holds in the form that matters and this is the check for it. It also says why the
/// two indexes of this directory do not collide: `yidam decisions-log` reads `summary:`, which is
/// the field nothing here edits, and `.yidam/decisions/README.md` lists open questions, which live
/// entirely in the field that changes.
///
/// The one exception was named rather than excused, and carried the lesson: **a summary should not
/// hold a figure**, because a figure is the one thing in a record that can turn out to be wrong,
/// and the summary is the half the log reads.
///
/// **There are two now, and the second is the convention earning itself one phase later.**
/// `action-pages` put three figures in its summary — *"six answered by later phases, two by a tool
/// that shipped, one by a tense"* — and the first was wrong before the next phase ended: one of
/// the six was a live duplicate of a corpus marker, read as answered. The figures came out of the
/// summary and the summary moved. Both exceptions are corrected measurements and neither is a
/// changed mind, which is the rule holding rather than bending; what the second adds is that the
/// convention was written against exactly this and was not followed by the record written next.
#[test]
fn no_record_has_changed_what_it_decided() {
    let root = repo_root();
    let (mut moved, mut edited, mut uncommitted) = (Vec::new(), 0, Vec::new());
    for p in records(&root) {
        let now = std::fs::read_to_string(&p).expect("readable");
        if !tracked(&root, &p) {
            uncommitted.push(stem(&p));
            continue;
        }
        let then = at_creation(&root, &p).unwrap_or_else(|| {
            panic!("{}: no history — a shallow clone cannot run this", stem(&p))
        });
        if then == now {
            continue;
        }
        edited += 1;
        let (a, b) = (summary(&then), summary(&now));
        assert!(
            a.len() > 40 && b.len() > 40,
            "{}: the summary parse returned nothing, which would make this test agree with \
             itself about everything",
            stem(&p)
        );
        if a != b {
            moved.push(stem(&p));
        }
    }
    // 35 rather than 30 because `parser-silence`, `round-trip`, `calculators` and `normal-form`
    // were each extended after their `decide:` commit and before their phase settled — the gate
    // asked each a question its first draft did not answer, which is the gate working. Three of
    // the four were asked for a source and found one. What this check is for is a record whose
    // *summary* moved, and that is `moved` below.
    //
    // It counts what the history holds rather than what happened: `line-format` and
    // `class-library` were both extended the same way and both extensions were folded into their
    // `decide:` commits with `--amend`, so neither is here. That is the check reading the record it
    // was given, which is the only thing it can do — and a reason to prefer the amend, since a
    // record's history should be the phase's argument rather than the drafting of it.
    //
    // The thirty-fifth is `input-side`, and it is the other kind entirely: a later phase writing
    // into it, which is what `a_record_nothing_cites_is_named` calls the one way that list empties
    // from the front. `class-library` found the marker named a document two years too early to
    // answer it, and the correction went where the marker is. A record edited by the phase that
    // drafted it and a record edited by the phase that read it are both edits, and only the second
    // is the graph doing what it is for.
    assert_eq!(
        edited,
        35,
        "records with a commit after the one that wrote them \
         ({} not yet committed, and not judged: {uncommitted:?})",
        uncommitted.len()
    );
    assert_eq!(
        moved,
        [
            "action-pages",
            "escape-hatch",
            "inert-reports",
            "latch-default"
        ],
        "a record may keep its account current; it may not restate what it decided. All four \
         moved a figure and none moved a decision — the third went stale without the record being \
         touched, because what changed was the thing it described, and the fourth held a \
         prediction, which is a figure that has not happened yet"
    );
}
