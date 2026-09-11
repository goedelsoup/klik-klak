//! **Which way state moves**, and whether the declaration asks for the common case or the rare one.
//!
//! [`world-flag/latched-flag`] proposed a schema change and refused to make it: *"if it is most,
//! `reversible: no` is the wrong default and the property should ask which flags can be* un*set.
//! Nothing here changes the default, because eight names is not enough to invert a schema on."*
//! Those eight names came from a screenshot of a save file in a blog post.
//!
//! A shipped game's source says **32 of 41** boolean state slots are written one way and never
//! back ([delores-source]). So it is most, and the default is the wrong way round in two places:
//! in the ontology's property, and in this format, where `latched` is the word you type to get the
//! behaviour that is already true 78% of the time.
//!
//! What the checker could not see is the direction that costs nothing to fix. It has always
//! reported a flag declared latched that something clears. The reverse — a flag nothing clears
//! that does not say so — is a guarantee the author has and is not claiming, and the four
//! specimens hold two of them. `.yidam/decisions/latch-default.yml`.
//!
//! [`world-flag/latched-flag`]: ../../../.yidam/corpus/world-flag/latched-flag.yml
//! [delores-source]: ../../../.yidam/catalog/delores-source.md

use game_format::Effect;
use reachability::analyse;

const S: [(&str, &str); 4] = [
    ("ferry", include_str!("../../../design/specimen/ferry.game")),
    (
        "lighthouse",
        include_str!("../../../design/specimen/lighthouse.game"),
    ),
    (
        "nightwatch",
        include_str!("../../../design/specimen/nightwatch.game"),
    ),
    (
        "patrol",
        include_str!("../../../design/specimen/patrol.game"),
    ),
];

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "latch.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// **The reverse gear exists and has never been used.** `clear` is one of the eight effects the
/// format admits, and across 662 authored lines it appears zero times: every flag in this corpus
/// is one-way in fact.
///
/// Seven of the nine say so. The two that do not are `door-open`, which the lighthouse's front
/// door sets when it is unlocked, and `asked-key`, scoped to a conversation — and the second is
/// the more interesting, because a conversation-local flag is exactly where an author might
/// reasonably expect to start over.
#[test]
fn nothing_in_this_corpus_ever_clears_a_flag_and_two_flags_decline_to_say_so() {
    let (mut flags, mut declared, mut clears) = (0, 0, 0);
    let mut unclaimed: Vec<String> = Vec::new();
    for (name, src) in S {
        let g = load(src);
        flags += g.flags.len();
        declared += g.flags.iter().filter(|f| f.latched).count();
        clears += g
            .bindings
            .iter()
            .flat_map(|b| &b.effects)
            .filter(|e| matches!(e, Effect::ClearFlag(_)))
            .count();
        let r = analyse(&g);
        assert!(r.latch_violations.is_empty(), "{name}: {r:?}");
        unclaimed.extend(r.unclaimed_latches.iter().map(|u| format!("{name}: {u}")));
    }
    assert_eq!((flags, declared, clears), (9, 7, 0));
    assert_eq!(unclaimed.len(), 2, "{unclaimed:#?}");
    assert!(unclaimed[0].contains("door-open") && unclaimed[1].contains("asked-key"));
}

/// **The declaration the checker asks for is free**, which is what makes asking reasonable. Adding
/// `latched` to the two flags that earned it changes no figure and silences the report, because a
/// property nothing contradicts costs only the word.
#[test]
fn claiming_the_latch_moves_nothing() {
    let src = S[1].1; // lighthouse holds both undeclared flags
    let before = analyse(&load(src));
    let after = analyse(&load(
        &src.replace("flag    door-open   \"", "flag    door-open   latched \"")
            .replace("flag    asked-key   in", "flag    asked-key   latched in"),
    ));
    assert_eq!(before.unclaimed_latches.len(), 2);
    assert!(after.unclaimed_latches.is_empty());
    assert!(after.latch_violations.is_empty());
    assert_eq!(
        (before.states, before.transitions, before.stranded),
        (after.states, after.transitions, after.stranded)
    );
}

/// **The only reverse gear this vocabulary has is a second binding**, and that turns out to be the
/// reason latching is not merely common in a shipped game but very nearly forced.
///
/// Nine boolean slots in Delores are written both ways, and reading where each write sits splits
/// them three ways. **Two are player toggles** — `valve.delores_tightened` and
/// `fireHydrant.isWaterSource`, each written by a `VERB(loosen)` and a `VERB(tighten)` — and those
/// are what this test builds. **Five are derived or spanning**: `lights_on` is recomputed inside
/// `adjustLights()` from a switch's state, the two `sink`/`sinkFaucet` `isWaterSource` writes sit
/// in a `stateChange` callback, and `GLOBAL(taking_photo)` is raised at the top of `takePhoto` and
/// dropped at the bottom. **Two are an actor's own behaviour**: `sheriff.busy` and
/// `sheriff.locked_in_jail`, written inside `script`s run by `startthread`.
///
/// Only the first two are expressible here, and the other seven are reversible for reasons this
/// design refused one at a time — no derived value and no occasion but a binding
/// (`.yidam/decisions/threshold-trigger.yml`), no duration ([`effect-list-semantics`]), no
/// autonomous actor. A vocabulary whose only writer is something the player did is a vocabulary in
/// which state mostly cannot come back.
///
/// What the toggle costs is worth pinning too, because it is small in the direction the corpus
/// once expected to be large: a binding that clears the flag adds **one transition and no
/// states**. Both values were reachable already.
///
/// [`effect-list-semantics`]: ../../../.yidam/decisions/effect-list-semantics.yml
#[test]
fn a_flag_that_goes_back_needs_a_binding_that_puts_it_back() {
    const G: &str = "room a \"A\"\nflag lit{L} \"the lamp is burning\"\n\nroom a\n  start\n\
                     \x20 hotspot lamp \"a lamp\"\n    walk-to:\n      when  not lit\n      do    set lit\n\
                     \x20     else  \"It is already lit.\"\n{OFF}\
                     \x20 hotspot out \"out\"\n    walk-to:\n      when  lit\n      ending\n      else  \"Too dark.\"\n";
    const OFF: &str =
        "  hotspot snuffer \"a snuffer\"\n    walk-to:\n      when  lit\n      do    clear lit\n\
                       \x20     else  \"Nothing to put out.\"\n";

    // one-way: the player can light it and the checker sees a guarantee nobody claimed
    let one = analyse(&load(&G.replace("{L}", "").replace("{OFF}", "")));
    assert_eq!(one.unclaimed_latches.len(), 1);
    assert!(one.unclaimed_latches[0].contains("nothing anywhere clears it"));

    // two-way: a second binding is the whole mechanism, and the report goes quiet
    let two = analyse(&load(&G.replace("{L}", "").replace("{OFF}", OFF)));
    assert!(two.unclaimed_latches.is_empty() && two.latch_violations.is_empty());

    // and what the reverse gear costs is an edge, not a state — both values of `lit` were
    // already reachable, which is the same fact `latched-flag` closed its performance question
    // on: monotonicity removes transitions from the graph and nothing from the set of states
    assert_eq!((one.states, one.transitions), (2, 2));
    assert_eq!((two.states, two.transitions), (2, 3));

    // and the declaration is held against the game rather than believed
    let lied = analyse(&load(&G.replace("{L}", " latched").replace("{OFF}", OFF)));
    assert_eq!(lied.latch_violations.len(), 1);
    assert!(
        lied.unclaimed_latches.is_empty(),
        "it is claimed, just wrongly"
    );
}
