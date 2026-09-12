//! **`verified-unsourced` is a node-level rule over a claim-level property, and four nodes wrote
//! down what that costs before it happened to the last two of them.**
//!
//! The lint reports a node that carries a `[verified]` claim and links no catalog entry. One
//! citation anywhere in a node silences it for the whole node — including for claims that citation
//! has nothing to do with. Four nodes have been reported, and each, on being silenced, recorded
//! that the silence was unearned and named a successor still carrying the warning:
//!
//! | node | what it said | was it true |
//! |---|---|---|
//! | `inventory-item/portable-object` | *"It no longer does… the warning went away without any of the claims it was about becoming better sourced"* | yes, but it named `counter-flag` as the successor |
//! | `interaction/binary-binding` | *"It no longer does, and for the same unearned reason"*, naming `counter-flag` too | same |
//! | `world-flag/boolean-flag` | *"This node now trips `verified-unsourced`… It is left standing"* | **no** |
//! | `world-flag/counter-flag` | *"This node now trips `verified-unsourced`, and the warning is correct"* | **no** |
//!
//! The last two were silenced in one commit two phases later —
//! `revise: four converging guesses about saving` — which added an `ags-manual` citation to each
//! node's `save_semantics` for a claim about save slots. Neither instrument-backed claim moved.
//! `boolean-flag` had named that exact mechanism in the sentence declaring itself a standing
//! instance: *"citing a catalog entry for some other claim would silence the rule for the whole
//! node without earning it."*
//!
//! So the baton was passed three times by hand and dropped on the fourth. All four accounts are
//! corrected, `interaction/consuming-binding` — the only node the lint reports, and the only one
//! that had never mentioned it — now carries the warning and says so, and the two checks below
//! hold what four paragraphs could not.

use design_gate::repo_root;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn nodes(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.join(".yidam/corpus")];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("readable").flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "yml")
                && !p.to_string_lossy().ends_with(".ont.yml")
            {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn name(root: &Path, p: &Path) -> String {
    p.strip_prefix(root.join(".yidam/corpus"))
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

/// What a `[verified]` claim names in its own paragraph.
///
/// **A lower bound, deliberately.** The corpus argues in long passages that establish a source
/// once and carry it — *"counted over the whole game — 203 live bindings across 21 rooms"* — so a
/// claim citing nothing in its own paragraph is usually a continuation and not an unsourced
/// assertion. Attributing those would mean reading 316 claims. What this counts is the claims that
/// **say** what they rest on, which is the population any checker could ever act on.
#[derive(Default, Debug, PartialEq, Eq)]
struct Rests {
    catalog: usize,
    instrument: usize,
    both: usize,
    carried: usize,
}

fn rests(src: &str) -> Rests {
    let mut r = Rests::default();
    for para in src.split("\n\n") {
        // a backticked tag is the token being discussed, not a claim — `reach.rs` learned this
        let n = para
            .replace("`[verified]`", "")
            .matches("[verified]")
            .count();
        if n == 0 {
            continue;
        }
        let (cat, ins) = (para.contains("catalog/"), para.contains("crates/"));
        *match (cat, ins) {
            (true, true) => &mut r.both,
            (true, false) => &mut r.catalog,
            (false, true) => &mut r.instrument,
            (false, false) => &mut r.carried,
        } += n;
    }
    r
}

/// **The instrument is a kind of evidence the register has no slot for, and it is not a corner.**
///
/// The strongest evidence this corpus produces about its own language is a committed test that can
/// be re-run, and its tag vocabulary has no way to say so —
/// `.yidam/decisions/found-in.yml` opened that at two instances and
/// `.yidam/corpus/world-flag/boolean-flag.yml` called itself the third.
///
/// `.yidam/decisions/evidence-kinds.yml` measured 49 and says 49. The two since are that record's
/// own finding being written into the nodes it was about, each resting on this file — which is why
/// the count is pinned here and not in a summary, and is the lesson `record-lifecycle` drew from
/// the only other record whose figure moved.
///
/// The catalog is not the remedy. An entry for `crates/` would silence `verified-unsourced` on
/// every node at once, which is the unearned silencing the four nodes below documented, applied
/// globally and on purpose. `.yidam/decisions/evidence-kinds.yml`.
#[test]
fn the_instrument_is_a_kind_of_evidence_the_register_has_no_slot_for() {
    let root = repo_root();
    let mut total = Rests::default();
    let (mut instrument_only, mut silenced) = (Vec::new(), Vec::new());
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        let r = rests(&src);
        total.catalog += r.catalog;
        total.instrument += r.instrument;
        total.both += r.both;
        total.carried += r.carried;
        if r.instrument > 0 {
            instrument_only.push(name(&root, &p));
            if src.contains("catalog/") {
                silenced.push(name(&root, &p));
            }
        }
    }
    assert_eq!(
        total,
        Rests {
            catalog: 122,
            instrument: 60,
            both: 5,
            carried: 160,
        }
    );
    assert_eq!(
        (instrument_only.len(), silenced.len()),
        (23, 22),
        "nodes with a claim resting on the instrument alone, and how many of them cite a catalog \
         entry for something else — which is every one but the node the lint reports"
    );
}

/// **A node that says it trips the lint must trip it, and a node that says it stopped must have.**
///
/// The check `boolean-flag` wanted and could not have. It named the mechanism that would silence
/// it wrongly, declared itself left standing, and was silenced two phases later by a commit about
/// save semantics that had no reason to look at it — along with `counter-flag`, which the two
/// earlier nodes had both named as the one still carrying the warning.
///
/// A claim about a tool's output is a claim, and this repository checks its claims everywhere
/// except about its own reports. That is [`markers.rs`]'s opening sentence with a different noun
/// in it.
///
/// [`markers.rs`]: markers.rs
#[test]
fn no_node_claims_a_lint_report_it_does_not_have() {
    let root = repo_root();
    let mut wrong = Vec::new();
    let mut claiming: BTreeMap<String, bool> = BTreeMap::new();
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        if !src.contains("verified-unsourced") {
            continue;
        }
        // the lint fires on a node with a verified claim and no catalog link at all
        let reported =
            src.replace("`[verified]`", "").contains("[verified]") && !src.contains("catalog/");
        let n = name(&root, &p);
        claiming.insert(n.clone(), reported);

        let says_trips = ["now trips `verified-unsourced`", "still trips"]
            .iter()
            .any(|s| src.contains(s));
        // `stopped` was in this list and matched a node describing what happened to *other*
        // nodes. A claim about this node's own report says so in the first person.
        let says_stopped = ["It no longer does", "it no longer does"]
            .iter()
            .any(|s| src.contains(s));
        if says_trips && !reported {
            wrong.push(format!("{n}: says it trips the lint; it does not"));
        }
        if says_stopped && reported && !says_trips {
            wrong.push(format!(
                "{n}: says the lint stopped reporting it; it has not"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "a claim about a tool's report is a claim:\n  {}",
        wrong.join("\n  ")
    );
    assert_eq!(
        claiming.len(),
        6,
        "nodes that say something about this lint. It was four, all of them nodes the lint had \
         named; `consuming-binding` joined while it is true, and `state-gate` joined without ever \
         having been reported — see `.yidam/decisions/carried-claims.yml`"
    );
}

/// **Every node named as *the one still carrying the warning* is a node the lint still reports.**
///
/// Two nodes hand the warning on by name — `portable-object` to `counter-flag`, `binary-binding`
/// to `counter-flag` — which is a pointer maintained by hand, correct when written, and with
/// nothing anywhere to notice when its target stops qualifying. Both pointed at the same node and
/// both went wrong in the same commit.
///
/// This is the fourth hand-maintained pointer this repository has found and the second it has
/// found broken: `action-pages` found class pages pointing at questions answered years of commits
/// earlier, and `prose-edges` found the corpus's prose citations were a second graph nothing read.
#[test]
fn the_node_carrying_the_warning_is_carrying_it() {
    let root = repo_root();
    let mut bad = Vec::new();
    let mut pointers = 0;
    for p in nodes(&root) {
        let src = std::fs::read_to_string(&p).expect("readable");
        for para in src.split("\n\n") {
            if !para.contains("carrying the warning") {
                continue;
            }
            // A pointer is the markdown link **immediately before** the phrase. Scanning the
            // whole paragraph matched a node that merely discusses the phrase, and `rsplit` on a
            // paragraph with no link at all returns the paragraph — two ways to be wrong that the
            // first version of this check was, on the first prose written after it.
            let before = &para[..para.find("carrying the warning").expect("just matched")];
            let Some(target) = before
                .rsplit("](")
                .next()
                .filter(|_| before.contains("]("))
                .and_then(|t| t.split(')').next())
                .filter(|t| t.ends_with(".yml"))
            else {
                continue;
            };
            pointers += 1;
            let at = p
                .parent()
                .expect("a directory")
                .join(target.split('#').next().unwrap_or(""));
            let Ok(other) = std::fs::read_to_string(&at) else {
                bad.push(format!(
                    "{}: hands the warning to {target}, which is not a file",
                    name(&root, &p)
                ));
                continue;
            };
            let reported = other.replace("`[verified]`", "").contains("[verified]")
                && !other.contains("catalog/");
            if !reported {
                bad.push(format!(
                    "{}: hands the warning to {target}, which the lint does not report",
                    name(&root, &p)
                ));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a pointer at *the node still carrying the warning*:\n  {}",
        bad.join("\n  ")
    );
    assert_eq!(pointers, 2, "nodes handing the warning on by name");
}
