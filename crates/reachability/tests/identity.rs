//! Whether two hotspots are ever one thing here.
//!
//! `hotspot/static-region` carried this question until the drawing layer closed it: eleven of
//! Delores's twenty delegation sites are one object drawn as several regions, and a region that could say it reads as another
//! would remove all eleven. The node declines to choose among the three relations they want —
//! part-of, same-as, says-the-same — on the grounds that one game is not enough.
//!
//! This is the other game, and it has none of them. What it has is exits that share a destination,
//! which is a different thing: two doors into the corridor are two doors, each written where it
//! stands.
//!
//! **This census reads the source rather than the model, and it has to.** `Binding` carries no
//! response text — the checker discards it, which is why a text-only response is not a transition
//! (`crates/reachability/tests/defaults.rs`). So every text-only `look-at` is identical in the
//! model and distinct on the page, and the page is where identity lives.
//!
//! `.yidam/decisions/hotspot-identity.yml` is the record.

mod common;

use std::collections::HashMap;

/// Every `(hotspot, verb, body-as-written)` in a specimen, with blank lines and indentation
/// normalised away.
fn bindings(src: &str) -> Vec<(String, String, Vec<String>)> {
    let lines: Vec<&str> = src.lines().collect();
    let mut out = Vec::new();
    let mut site = String::new();
    for (i, l) in lines.iter().enumerate() {
        let ind = l.len() - l.trim_start().len();
        let t = l.trim_start();
        if ind == 2 {
            if let Some(rest) = t
                .strip_prefix("hotspot ")
                .or_else(|| t.strip_prefix("actor "))
                .or_else(|| t.strip_prefix("exit "))
            {
                site = rest.split_whitespace().next().unwrap_or("").to_string();
            }
            continue;
        }
        if ind == 4 && t.ends_with(':') && !site.is_empty() {
            let verb = t.trim_end_matches(':').to_string();
            let mut body = Vec::new();
            for next in &lines[i + 1..] {
                if next.trim().is_empty() {
                    break;
                }
                if next.len() - next.trim_start().len() <= 4 {
                    break;
                }
                body.push(next.split_whitespace().collect::<Vec<_>>().join(" "));
            }
            out.push((site.clone(), verb, body));
        }
    }
    out
}

#[test]
fn no_two_hotspots_here_are_one_thing_addressed_twice() {
    let mut shared = 0;
    for (name, src) in common::sources() {
        let mut by_body: HashMap<(String, Vec<String>), Vec<String>> = HashMap::new();
        for (site, verb, body) in bindings(&src) {
            by_body.entry((verb, body)).or_default().push(site);
        }
        for ((verb, body), sites) in by_body.iter().filter(|(_, s)| s.len() > 1) {
            // An exit that shares a destination is not a shared identity. Two doors into the
            // corridor are two doors.
            let is_bare_exit = body.len() == 1 && body[0].starts_with("do move");
            assert!(
                is_bare_exit,
                "{name}: {sites:?} share `{verb}` with body {body:?} — that is the identity \
                 pressure `hotspot/static-region` records, and its arrival here would be the \
                 second independent instance the node says it needs"
            );
            shared += 1;
        }
        assert!(
            !bindings(&src).is_empty(),
            "{name} parsed to nothing, so this census read the wrong thing"
        );
    }
    assert!(
        shared >= 4,
        "the shared exits should still be found, or this test proves nothing; found {shared}"
    );
}
