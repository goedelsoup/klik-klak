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
//! that does not say so — is a guarantee the author has and is not claiming, and the specimens
//! hold two of them. `.yidam/decisions/latch-default.yml`.
//!
//! **The `clear` count went from zero to three and the argument got better.**
//! `.yidam/decisions/unread-specimen.yml` is why this file could not see the fifth game for
//! eleven commits. What it says now is sharper than what it said: fourteen flags, eleven declaring
//! the latch, two holding it silently, and **exactly one that is genuinely two-way**.
//!
//! [`world-flag/latched-flag`]: ../../../.yidam/corpus/world-flag/latched-flag.yml
//! [delores-source]: ../../../.yidam/catalog/delores-source.md

mod common;

use game_format::Effect;
use reachability::analyse;

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "latch.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// **The reverse gear is used exactly once, and what it writes is not a fact about the world.**
///
/// This test read *"the reverse gear exists and has never been used… across 662 authored lines it
/// appears zero times"* for eleven commits after that stopped being true, because its input was a
/// list of four games and there were five (`.yidam/decisions/unread-specimen.yml`).
///
/// The corrected census is a better argument than the zero was. **Fourteen flags across the
/// specimens, and the three `clear` sites all write one of them: `ashore`.** Eleven of the
/// remaining thirteen declare `latched`; the other two hold the guarantee without claiming it, and
/// are reported.
///
/// `ashore` is the flag `lock.game` uses to record **which character the player is driving** —
/// `.yidam/decisions/control-transfer.yml` found that this format has no site for a control switch
/// and an author has to spend a flag on it. So the division is not *most flags latch*. It is:
///
/// - a flag recording something about the world is latched, because a door that has been unlocked
///   does not re-lock itself and nothing in this vocabulary would say so if it did;
/// - the one flag recording something about the **player** is cleared, because you can stop being
///   who you were.
///
/// Thirteen of fourteen, and the exception is the one flag that is not a fact about the world at
/// all. That is the case `world-flag/latched-flag` was arguing and it could not have been made
/// before somebody wrote the game.
///
/// The two that hold the guarantee silently are `door-open`, which the lighthouse's front door
/// sets when it is unlocked, and `asked-key`, scoped to a conversation — and the second is the
/// more interesting, because a conversation-local flag is exactly where an author might reasonably
/// expect to start over.
#[test]
fn the_one_flag_anything_clears_is_the_one_that_is_not_about_the_world() {
    let (mut flags, mut declared, mut clears) = (0, 0, 0);
    let mut unclaimed: Vec<String> = Vec::new();
    for (name, src) in common::sources() {
        let g = load(&src);
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
    assert_eq!((flags, declared, clears), (17, 14, 3));
    assert_eq!(unclaimed.len(), 2, "{unclaimed:#?}");
    assert!(unclaimed[0].contains("door-open") && unclaimed[1].contains("asked-key"));

    // and every clear writes the same flag, which is the whole of the argument above
    let mut cleared: Vec<String> = Vec::new();
    for (_, src) in common::sources() {
        for b in &load(&src).bindings {
            for e in &b.effects {
                if let Effect::ClearFlag(f) = e {
                    cleared.push(f.clone());
                }
            }
        }
    }
    assert_eq!(
        cleared,
        ["ashore", "ashore", "ashore"],
        "a second flag has been cleared, and whether it is a fact about the world is the question          to ask before re-pinning this"
    );
}

/// **The declaration the checker asks for is free**, which is what makes asking reasonable. Adding
/// `latched` to the two flags that earned it changes no figure and silences the report, because a
/// property nothing contradicts costs only the word.
#[test]
fn claiming_the_latch_moves_nothing() {
    let src = common::source("lighthouse"); // it holds both undeclared flags
    let before = analyse(&load(&src));
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
