//! **An item that is one thing is in one place, and the specimens were never asked.**
//!
//! `.yidam/decisions/found-in.yml` left this at `[open]`: whether the check that holds a hotspot
//! to giving the item it stands for should also refuse a give of some *other* item there. Its
//! evidence was that every `is` site gave exactly what it stood for, so the corpus had *"no case
//! where a hotspot legitimately hands over something besides itself — and no case where it does
//! not, either"*.
//!
//! There are thirteen sites now, across six games, and that is still true. So this file makes the
//! cases. Every way one `is` hotspot could also hand over what another stands for, in every
//! specimen with two, is written and read back — and before `one-place` every one of them
//! resolved without a word, while the strandings under them moved. Four got smaller, the direction
//! a checker should never move silently. `.yidam/decisions/one-place.yml`.

mod common;

use game_format::{parse, resolve, Effect, Game, Site};
use reachability::analyse;

fn load(source: &str, name: &str) -> (Game, Vec<String>) {
    let (mut g, mut p) = parse(source, name);
    p.extend(resolve(&mut g));
    (g, p.into_iter().map(|p| p.to_string()).collect())
}

/// **Thirteen sites, and every one gives exactly itself, once.**
#[test]
fn every_hotspot_that_stands_for_an_item_gives_only_that_item() {
    let mut sites = 0;
    for (name, src) in common::sources() {
        let (g, p) = load(&src, &name);
        assert!(p.is_empty(), "{name}: {p:?}");
        for (hotspot, item) in &g.carried_form {
            let given: Vec<&String> = g
                .bindings
                .iter()
                .filter(|b| matches!(&b.site, Site::InRoom { target, .. } if target == hotspot))
                .flat_map(|b| &b.effects)
                .filter_map(|e| match e {
                    Effect::Give(x, _) => Some(x),
                    _ => None,
                })
                .collect();
            assert_eq!(given, [item], "{name}: {hotspot}");
            sites += 1;
        }
    }
    assert_eq!(sites, 13, "`is` sites across the specimens");
}

/// **Twenty mutations, nineteen refused, and the twentieth is a counted item.**
///
/// Each mutation adds one line: the `give` on one `is` hotspot is followed by a `give` of the item
/// another hotspot stands for. The one that resolves is `ferry`'s lantern handing over a coin,
/// which is `0..4` and many things by declaration, and it is also the one whose strandings did
/// not move. The four pinned below are the ones where the checker got quieter.
#[test]
fn every_way_to_put_one_thing_in_two_places_is_refused() {
    let mut refused = 0;
    let mut kept = Vec::new();
    let mut quieter = Vec::new();
    for (name, src) in common::sources() {
        let (g, _) = load(&src, &name);
        let before = analyse(&g);
        let items: Vec<&String> = g.carried_form.values().collect();
        for a in &items {
            for b in &items {
                if a == b {
                    continue;
                }
                let from = format!("      do    give {a}\n");
                assert_eq!(src.matches(&from).count(), 1, "{name}: one give of {a}");
                let mutated = src.replace(&from, &format!("{from}      do    give {b}\n"));
                let (m, p) = load(&mutated, &name);
                if p.iter().any(|m| m.contains("which is one thing")) {
                    refused += 1;
                } else {
                    assert!(p.is_empty(), "{name}: {a} gives {b}: {p:?}");
                    kept.push(format!("{name}: {a} gives {b}"));
                }
                let after = analyse(&m);
                let killed =
                    |r: &reachability::Report| r.strandings.iter().map(|s| s.states).sum::<usize>();
                if killed(&after) < killed(&before) {
                    quieter.push((
                        format!("{name}: {a} gives {b}"),
                        killed(&before),
                        killed(&after),
                    ));
                }
            }
        }
    }
    assert_eq!(refused, 19, "mutations refused");
    assert_eq!(
        kept,
        ["ferry: lantern gives coin"],
        "the counted item, and only it"
    );
    assert_eq!(
        quieter,
        [
            ("ferry: coin gives lantern".to_string(), 2, 1),
            ("relay: lamp gives line".to_string(), 28, 14),
            ("relay: link gives line".to_string(), 28, 20),
            ("relay: can gives line".to_string(), 28, 20),
        ],
        "the mutations that made a stranding smaller"
    );
}
