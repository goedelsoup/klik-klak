//! **The corpus's largest block is a property, and the record that collected its verdict has the
//! verdict's headline figure wrong.**
//!
//! `data-format/interaction-table.escape_hatch` is **484 lines — 10% of every block in the corpus,
//! and 2.1× the next largest**. It carries two `---` horizontal rules, which is the tell: an
//! author reaching for a section break inside a YAML scalar is an author whose field has stopped
//! being one field.
//!
//! `.yidam/decisions/escape-hatch.yml` is 81 lines and its own `context:` says why: *"the property
//! grew to eleven screens of argument beneath a sentence saying the question was open. This record
//! collects the verdict and states what it cost."* **The collection happened and the source was
//! never trimmed**, so the corpus states one verdict twice at two sizes — and they have drifted.
//!
//! | | `surface.rs` | the property | the record |
//! |---|---|---|---|
//! | constructs | **50** | 50 | **51** |
//! | withdrawn | **2** | 2 | **1** |
//!
//! Two of three agree. The odd one is the record, in its `summary:` — the permanent half, the one
//! `yidam decisions-log` reads, and the field `record-lifecycle` wrote *a summary should not hold a
//! figure* about three phases later.
//!
//! `surface.rs` had already named the remedy in its own header: *"A change here is not a failure.
//! It is a signal that `.yidam/decisions/escape-hatch.yml` needs reading again, which is the whole
//! reason the figures are pinned rather than described."* The change happened — `give-to` folded
//! into `use-with`, six verbs became five — and nobody carried the signal. This file is the wire.

use design_gate::repo_root;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Every top-level block and property of every node, with its line count.
fn blocks(root: &Path) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus")];
    let mut files = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml")
                && !p.to_string_lossy().ends_with(".ont.yml")
            {
                files.push(p);
            }
        }
    }
    files.sort();
    for p in files {
        let src = std::fs::read_to_string(&p).expect("readable");
        let lines: Vec<&str> = src.lines().collect();
        let name = p
            .strip_prefix(root.join(".yidam/corpus"))
            .unwrap_or(&p)
            .to_string_lossy()
            .into_owned();
        let top: Vec<(usize, &str)> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| {
                l.starts_with(|c: char| c.is_ascii_lowercase())
                    && l.contains(':')
                    && !l.starts_with(' ')
            })
            .map(|(i, l)| (i, l.split(':').next().unwrap_or("")))
            .collect();
        for (n, (i, key)) in top.iter().enumerate() {
            let end = top.get(n + 1).map(|(j, _)| *j).unwrap_or(lines.len());
            if ["class", "label", "tier", "links"].contains(key) {
                continue;
            }
            if *key == "properties" {
                let sub: Vec<(usize, &str)> = lines[*i..end]
                    .iter()
                    .enumerate()
                    .filter(|(_, l)| {
                        l.starts_with("  ") && !l.starts_with("   ") && l.trim().contains(':')
                    })
                    .map(|(k, l)| (k + i, l.trim().split(':').next().unwrap_or("")))
                    .collect();
                for (m, (k, prop)) in sub.iter().enumerate() {
                    let stop = sub.get(m + 1).map(|(l, _)| *l).unwrap_or(end);
                    out.push((stop - k, format!("{name}  properties.{prop}")));
                }
            } else {
                out.push((end - i, format!("{name}  {key}")));
            }
        }
    }
    out.sort_by(|a, b| b.0.cmp(&a.0));
    out
}

/// The surface figures, read off the test that pins them rather than restated.
///
/// `surface.rs` is the authority by construction: it enumerates the verbs and keywords from the
/// parser and holds the enums with exhaustive matches, so a tenth predicate does not compile until
/// somebody names it. Reading its assertions means this check cannot drift from it either.
fn surface(root: &Path) -> BTreeMap<String, usize> {
    let src = std::fs::read_to_string(root.join("crates/game-format/tests/surface.rs"))
        .expect("the surface test exists");
    let mut out = BTreeMap::new();
    for line in src.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("assert_eq!(") else {
            continue;
        };
        let mut parts = rest.split(',');
        let (Some(lhs), Some(rhs)) = (parts.next(), parts.next()) else {
            continue;
        };
        let Ok(n) = rhs
            .trim()
            .trim_end_matches([')', ';'])
            .trim()
            .parse::<usize>()
        else {
            continue;
        };
        let key = lhs.trim().trim_end_matches("()").replace(".len", "");
        out.insert(key, n);
    }
    // the total lives on its own multi-line assertion and is summed rather than parsed
    let total: usize = [
        "verbs",
        "keywords",
        "effects",
        "predicates",
        "comparisons",
        "sites",
    ]
    .iter()
    .filter_map(|k| out.get(*k))
    .sum();
    out.insert("constructs".into(), total);
    out
}

/// **One property is nine percent of the corpus and twice the next largest block.**
///
/// It was ten percent for seven phases. The block has not moved; the corpus grew around it.
///
/// It is not a rule and there is nothing here to fix by splitting a file: `interaction-table` is
/// one concept and the prelude's shape has no unit between a node and a property. What the pin does
/// is make the size visible, because the size is what produced the defect below — a field nobody
/// reads from the top is a field whose first sentence can disagree with its last screen, which is
/// exactly what `escape-hatch.yml` was written to fix and exactly what happened to `escape-hatch.yml`.
#[test]
fn the_largest_block_in_the_corpus_is_a_property() {
    let root = repo_root();
    let b = blocks(&root);
    let total: usize = b.iter().map(|(n, _)| n).sum();
    assert_eq!(
        (b[0].0, b[0].1.as_str()),
        (
            484,
            "data-format/interaction-table.yml  properties.escape_hatch"
        )
    );
    assert_eq!(
        (b[0].0 * 100 / total, b[0].0 / b[1].0),
        (9, 2),
        "share of every block in the corpus, and how many times the next largest"
    );
    assert_eq!(
        std::fs::read_to_string(root.join(".yidam/corpus/data-format/interaction-table.yml"))
            .expect("readable")
            .matches("\n    ---\n")
            .count(),
        2,
        "horizontal rules inside one YAML scalar — a field that has stopped being one field"
    );
}

/// **Every statement of the authoring surface's size agrees with the test that pins it.**
///
/// The wire `surface.rs` asked for. It says a change in its figures *"is a signal that
/// `escape-hatch.yml` needs reading again"*, the change came, and the signal went nowhere: six
/// verbs became five when `give-to` folded into `use-with`, the property was corrected, and the
/// record's `summary:` still said 51.
///
/// This is [`verb-count`]'s failure, which is the one this repository has been bitten by most —
/// *"two of its own files disagreed on a headline number for six phases"* — recurring in the field
/// `record-lifecycle` singled out as the one that must not hold a figure.
///
/// [`verb-count`]: ../../../.yidam/decisions/verb-count.yml
#[test]
fn every_statement_of_the_surface_size_agrees_with_the_test() {
    let root = repo_root();
    let s = surface(&root);
    assert_eq!(
        (s["constructs"], s["verbs"], s["keywords"], s["effects"]),
        (50, 5, 19, 8),
        "read off crates/game-format/tests/surface.rs"
    );

    let mut wrong = Vec::new();
    let mut seen = 0;
    let mut files: Vec<PathBuf> = Vec::new();
    // `design/` joined the population when a design document first stated the size. It is the
    // likeliest place for the figure to drift — the proposal is written once and re-read least,
    // which is `design-directory`'s own argument for `rests_on:` — and it was outside the scan
    // for as long as no document had a number in it. `.yidam/decisions/settled-surface.yml`.
    let mut stack = vec![
        root.join(".yidam/corpus"),
        root.join(".yidam/decisions"),
        root.join("design"),
    ];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml" || x == "md") {
                files.push(p);
            }
        }
    }
    files.sort();
    for p in files {
        let src = std::fs::read_to_string(&p).expect("readable");
        let shown = p.strip_prefix(&root).unwrap_or(&p).display().to_string();
        for w in src.split_whitespace().collect::<Vec<_>>().windows(2) {
            // A figure inside single asterisks, quotes or backticks is one being *reported* — a
            // record saying what another file got wrong. Double asterisks are emphasis and the
            // figure is being asserted, which is the distinction this missed on its first run and
            // which the corpus makes consistently. Three separate checks have now had to learn the
            // same rule for three different tokens: `marker-count` for `[open]`, `reach.rs` for
            // `[verified]`, this one for a number. It is written down nowhere.
            let quoted = w[0].starts_with(['"', '`', '\''])
                || (w[0].starts_with('*') && !w[0].starts_with("**"));
            if quoted {
                continue;
            }
            let Ok(n) = w[0]
                .trim_matches(|c: char| !c.is_ascii_digit())
                .parse::<usize>()
            else {
                continue;
            };
            let noun = w[1].trim_matches(|c: char| !c.is_ascii_alphabetic());
            if noun != "constructs" {
                continue;
            }
            seen += 1;
            if n != s["constructs"] {
                wrong.push(format!(
                    "{shown}: says {n} constructs; the test pins {}",
                    s["constructs"]
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a figure about the authoring surface that the test does not support:\n  {}",
        wrong.join("\n  ")
    );
    assert_eq!(
        seen, 2,
        "places that state the surface size in full: the corpus property, and the proposal, which \
         states it because the sentence that used to stand there called the surface abstract. It \
         was three once — the property and a record twice — and the remedy then was the one taken \
         again here: a record names the test rather than restating it"
    );
}
