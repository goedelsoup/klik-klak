//! **A component of `State` exists only if some predicate can read it** — the rule
//! [`observed-state`] states and three vectors were narrowed to obey, one at a time, each after a
//! published figure came out wrong in the flattering direction.
//!
//! It left one question: *"whether the same rule wants enforcing rather than observing — a check
//! that fails when a component is added that no predicate reads. It would have caught all three,
//! and there is no fourth vector to try it on."*
//!
//! There was a fourth, a fifth and a sixth, and they were the three nobody had looked at. `items`,
//! `flags` and `counters` were still built by enumerating declarations rather than by asking what
//! the game reads — `game.flags.iter().map(|f| f.name.clone())` — which is the same defect as the
//! `chosen` bits in the one place the rule had never been applied.
//!
//! And the answer to the question is that the rule wants both, decided by **who wrote the state**.
//! `.yidam/decisions/unread-state.yml`.
//!
//! [`observed-state`]: ../../../.yidam/decisions/observed-state.yml

mod common;

use reachability::analyse;

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "unread.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// **The committed corpus reads everything it declares**, which is why narrowing the three vectors
/// moved no figure in this repository and why the check arrives preventive rather than corrective.
///
/// Nine flags, one counter and eleven items across the four specimens, and every one of them is
/// named by a predicate or taken as a binary verb's second object. Worth pinning precisely because
/// it is the state the previous three narrowings were found in — a green run that stayed green.
#[test]
fn nothing_in_this_corpus_declares_state_it_cannot_read() {
    for (name, src) in common::sources() {
        let r = analyse(&load(&src));
        assert!(r.unread.is_empty(), "{name}: {:#?}", r.unread);
    }
}

/// **An unread flag would have doubled the reachable set, and now costs nothing**, which is the
/// whole of the defect and the whole of the fix in one comparison.
///
/// Three games, identical but for one line. In the first the lever's flag is read by the door's
/// guard and is real state. In the second nothing reads it — so it is not state, the search does
/// not split on it, and the report says why. The third deletes the flag outright and agrees with
/// the second exactly, which is the test that the narrowing is a narrowing and not a change of
/// meaning.
#[test]
fn a_flag_nothing_reads_costs_no_states_and_is_reported() {
    const G: &str = "room hall \"The Hall\"\n{DECL}\n\
                     room hall\n  start\n\
                     \x20 hotspot lever \"a brass lever\"\n    walk-to:\n      say   \"It goes over with a clunk.\"\n{SET}\
                     \x20 hotspot door \"the door\"\n    walk-to:\n{GUARD}      ending\n";
    // a refusal needs a guard to refuse against, so the ungated forms carry none
    const GATE: &str = "      when  thrown\n";
    const ELSE: &str = "      else  \"It will not shift.\"\n";

    let read = analyse(&load(
        &G.replace("{DECL}", "flag thrown latched \"the lever is over\"\n")
            .replace("{SET}", "      do    set thrown\n")
            .replace("      ending\n", &format!("      ending\n{ELSE}"))
            .replace("{GUARD}", GATE),
    ));
    let unread = analyse(&load(
        &G.replace("{DECL}", "flag thrown latched \"the lever is over\"\n")
            .replace("{SET}", "      do    set thrown\n")
            .replace("{GUARD}", ""),
    ));
    let absent = analyse(&load(
        &G.replace("{DECL}", "")
            .replace("{SET}", "")
            .replace("{GUARD}", ""),
    ));

    // read: the lever is a move, and the door waits on it
    assert!(read.unread.is_empty());
    assert_eq!((read.states, read.transitions), (2, 3));

    // unread: the same two lines of authored text, and no state at all
    assert_eq!(unread.unread.len(), 1);
    assert!(
        unread.unread[0].contains("`flag thrown`")
            && unread.unread[0].contains("no predicate reads it")
    );
    assert_eq!(
        (unread.states, unread.transitions),
        (absent.states, absent.transitions),
        "identical to having written neither line"
    );
    assert!(unread.states < read.states, "the bit it would have cost");

    // and the write costs no transition either, which needed a second narrowing: `acting` used
    // to admit any binding with an effect, so a write nothing reads was a self-loop in a
    // published figure
    assert_eq!((unread.states, unread.transitions), (1, 1));

    // and the advice that would have been wrong is withheld: a flag to delete is not a flag to
    // declare latched
    assert!(unread.unclaimed_latches.is_empty());
    assert!(
        read.unclaimed_latches.is_empty(),
        "`thrown` says `latched` itself"
    );
}

/// **Counters and items go the same way, and an item is read where `offered` reads it.**
///
/// A binary verb requires its second object carried, so naming an item after `use-with` is a read
/// even where no predicate mentions it — which is why the four specimens have no unread items
/// despite `carrying` appearing in only some of them. What is *not* a read is `spend`: reducing a
/// count nothing consults changes nothing anybody can observe.
#[test]
fn an_item_is_read_where_a_binary_verb_asks_for_it() {
    const G: &str = "room cell \"The Cell\"\nitem key \"a bent key\"\nitem stone \"a loose stone\"\n\
                     counter turns 0..2 \"turns of the key\"\n\n\
                     room cell\n  start\n\
                     \x20 hotspot ledge \"a ledge\"\n    is key\n    pick-up:\n      do    give key\n\
                     \x20 hotspot rubble \"rubble\"\n    is stone\n    pick-up:\n      do    give stone\n\
                     \x20 hotspot lock \"the lock\"\n    use-with key:\n      say   \"It turns once.\"\n      do    set turns 1\n\
                     \x20 hotspot out \"out\"\n    walk-to:\n{GUARD}      ending\n{BAR}";

    // `key` is read by `use-with key` and `turns` by the exit's guard; `stone` is read by nothing
    let r = analyse(&load(
        &G.replace("{GUARD}", "      when  turns = 1\n")
            .replace("{BAR}", "      else  \"Locked.\"\n"),
    ));
    assert_eq!(r.unread.len(), 1);
    assert!(r.unread[0].contains("`item stone`"));

    // drop the guard and the counter joins it — two findings, and the exit no longer waits
    let both = analyse(&load(&G.replace("{GUARD}", "").replace("{BAR}", "")));
    assert_eq!(both.unread.len(), 2);
    assert!(both.unread.iter().any(|u| u.contains("`counter turns`")));
    assert!(both.states < r.states);
}
