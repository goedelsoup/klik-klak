//! **Three reports nag about one absence and none of them can be acted on from here.**
//!
//! `yidam status` has said *"index not initialized"* in every line since genesis. `yidam due`'s
//! index clock is one of three that *"can never come due"*. `yidam doctor` runs clean except for
//! a single warning, and the warning is the index. `yidam index-build` answers all three:
//!
//! > `index-build` needs the `index` feature — reinstall with `cargo install yidam --features index`
//!
//! and the pinned build's features are `reports, export-graph, tonpa, serve-http, vault-s3,
//! catalog-fetch`. The prelude's own kuten already knew: *"`due.index_after` — nobody holds one;
//! one corpus declined it in writing, because no released binary can build the index the clock
//! measures."* This repository is the second to decline it, which under that file's
//! pre-registration is data rather than an omission.
//!
//! The other two clocks are declined for reasons of their own and the fourth is declared.
//! `.yidam/decisions/inert-reports.yml` carries all four.
//!
//! **And `due` measures age while this repository measures commits.** Three days, 350-odd commits,
//! thirty-odd phases. A clock in days is the wrong instrument for a corpus that moves this fast,
//! and saying so is most of what declaring one is worth.

use design_gate::repo_root;
use std::path::Path;

fn config(root: &Path) -> String {
    std::fs::read_to_string(root.join(".yidam/config.toml")).unwrap_or_default()
}

/// Live `[open]` markers, corpus and records, by the rules the two existing checks use.
fn owed(root: &Path) -> (usize, usize) {
    let (mut corpus, mut decisions) = (0, 0);
    for (dir, ends) in [(".yidam/corpus", false), (".yidam/decisions", true)] {
        let mut stack = vec![root.join(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(&d).expect("readable").flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                    continue;
                }
                if p.extension().is_none_or(|x| x != "yml") {
                    continue;
                }
                let src = std::fs::read_to_string(&p).expect("readable");
                let lines: Vec<&str> = src.lines().collect();
                let n = lines
                    .iter()
                    .enumerate()
                    .filter(|(i, l)| {
                        l.contains("[open]")
                            && (!ends
                                || (l.trim_end().ends_with("`[open]`")
                                    && lines.get(i + 1).is_none_or(|x| x.trim().is_empty())))
                    })
                    .count();
                *if ends { &mut decisions } else { &mut corpus } += n;
            }
        }
    }
    (corpus, decisions)
}

/// **Every clock is declared or declined, and the declines say why in the config itself.**
///
/// `due` reported three clocks with *"no interval declared and can never come due"* for 350
/// commits, and the report is right in a way that reads as an oversight and is not one: two of the
/// three cannot mean anything here, and the third is worth having.
///
/// * `index_after` — **declined.** No released binary can build the index it measures.
/// * `phases_after` — **declined.** It counts inquiry refs in flight, and phases here settle
///   inside the session that opens them, so the count is structurally zero.
/// * `questions_after` — **declared**, at the `inquiry` kuten's proposed 100 days.
/// * `catalog.ttl_days` — already carried per entry, which the kuten records as the primary form.
///
/// The declines are in the file rather than only in the record because `due`'s own line — *"3
/// clock(s) have no interval declared and can never come due"* — cannot tell a decision from an
/// omission, and a reader hitting that line goes to the config first.
#[test]
fn every_due_clock_is_declared_or_declined_in_writing() {
    let root = repo_root();
    // The census: no config file at all, so `due` reports three clocks that can never come due
    // and nothing anywhere says whether that is a decision. The rule replaces this in the phase.
    let cfg = config(&root);
    assert!(cfg.is_empty(), "there is no .yidam/config.toml");
}

/// **The questions clock sees a quarter of what the repository owes.**
///
/// `yidam open-questions` walks `.yidam/corpus/` and nothing else, so the clock a declared
/// `questions_after` starts is a clock over 9 markers while 31 more sit in the decision records.
/// [`markers.rs`] measured that ratio and `.yidam/decisions/README.md` lists the larger share; what
/// this pins is that **declaring the clock does not widen it**, so a green `due` is a statement
/// about a quarter of the questions and the record says so.
///
/// [`markers.rs`]: markers.rs
#[test]
fn the_clock_that_is_declared_covers_a_quarter_of_what_is_owed() {
    let root = repo_root();
    let (corpus, decisions) = owed(&root);
    assert_eq!((corpus, decisions), (9, 32));
    assert_eq!(
        corpus * 100 / (corpus + decisions),
        21,
        "what a declared questions clock can see"
    );
}

/// **`doctor` runs in the gate, and its one warning is the one thing nobody here can fix.**
///
/// It had been referenced exactly once in this repository — in a `mise.toml` comment observing
/// that *"`yidam doctor` has no check for"* a corpus that failed to fetch. Somebody read it,
/// found it did not cover their case, and did not add it for the eleven cases it does: the
/// provenance pin, the binary the PATH resolves, how old the vendored prelude is, whether every
/// REGEN block holds what its generator produces, whether a catalog entry has expired, whether the
/// corpus is readable at all.
///
/// It exits 0 on a warning, so adding it to `mise run ci` gates on breakage and reports the rest.
#[test]
fn the_gate_runs_doctor() {
    let root = repo_root();
    let mise = std::fs::read_to_string(root.join("mise.toml")).expect("readable");
    let ci = mise
        .split("[tasks.ci]")
        .nth(1)
        .and_then(|s| s.split("\n[tasks.").next())
        .expect("a ci task");
    // The census, for the same reason.
    assert!(!ci.contains("doctor"), "the gate does not run doctor");
}
