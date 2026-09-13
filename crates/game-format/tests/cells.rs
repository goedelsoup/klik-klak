//! **Is the interaction table's key a key?** Until this check it was a description.
//!
//! [`data-format/interaction-table`] has said since genesis that the format is *keyed by verb and
//! target, with the second object as an optional third key*. Nothing enforced it: a hotspot could
//! carry two `use-with key:` blocks, or two unguarded `pick-up:` blocks, and every figure came
//! back clean.
//!
//! What is refused is narrower than a duplicate, and the narrowing is what a shipped game
//! required. `crates/reachability/tests/delegation.rs` transcribes a Delores shape — a `pick-up`
//! guarded on `not device-seen` beside one guarded on `device-seen` — which is two bindings in one
//! cell and is not a defect, because exactly one of them ever applies. So the rule is about pairs
//! that **cannot be told apart**, and `.yidam/decisions/arity-cost.yml` records what that turns
//! out to have to do with arity.
//!
//! [`data-format/interaction-table`]: ../../../.yidam/corpus/data-format/interaction-table.yml

mod common;

const HEAD: &str = "room a \"A\"\nitem x \"x\"\nitem y \"y\"\nflag seen \"seen\"\n\
                    counter step 0..3 \"how far\"\n\nroom a\n  start\n";
const TAIL: &str = "  hotspot out \"out\"\n    walk-to:\n      ending\n";

fn problems(body: &str) -> Vec<String> {
    let src = format!("{HEAD}  hotspot thing \"a thing\"\n{body}{TAIL}");
    let (mut g, mut p) = game_format::parse(&src, "cells.game");
    p.extend(game_format::resolve(&mut g));
    p.into_iter().map(|p| p.to_string()).collect()
}

fn collisions(body: &str) -> Vec<String> {
    problems(body)
        .into_iter()
        .filter(|m| m.contains("nothing tells the two apart"))
        .collect()
}

/// The plain case: the same cell written twice with nothing to separate the two.
#[test]
fn two_unguarded_bindings_in_one_cell_are_refused() {
    let m = collisions(
        "    use-with x:\n      do    set seen\n    use-with x:\n      say   \"again\"\n",
    );
    assert_eq!(m.len(), 1, "{m:?}");
    assert!(m[0].contains("neither is guarded"), "{m:?}");
}

/// **The shape a shipped game actually writes.** A `pick-up` that also describes the thing the
/// first time, and a bare `pick-up` afterwards. Two bindings, one cell, and the flag decides.
#[test]
fn a_flag_asserted_in_one_and_denied_in_the_other_is_accepted() {
    assert!(collisions(
        "    pick-up:\n      when  not seen\n      do    give x\n      do    set seen\n\
         \x20   pick-up:\n      when  seen\n      do    give x\n"
    )
    .is_empty());
}

/// A guard is not a separator on its own. Guarding one branch and leaving the other bare means the
/// bare one always applies, so the guarded one is decoration.
#[test]
fn one_guarded_and_one_bare_is_refused() {
    let m = collisions("    use-with x:\n      when  seen\n      do    set seen\n    use-with x:\n      say   \"hm\"\n");
    assert_eq!(m.len(), 1, "{m:?}");
    assert!(m[0].contains("one of them is unguarded"), "{m:?}");
}

/// **The case the whole arity question turns on.** Two bindings separated only by which item the
/// player is holding — which is what the second key says for free — and the player can hold both.
#[test]
fn two_bindings_separated_only_by_which_item_is_carried_are_refused() {
    let m = collisions(
        "    use-with:\n      when  carrying x\n      do    set seen\n\
         \x20   use-with:\n      when  carrying y\n      say   \"nothing\"\n",
    );
    assert_eq!(m.len(), 1, "{m:?}");
    assert!(m[0].contains("both guards can hold"), "{m:?}");
}

/// And the same two, told apart by a counter, are fine — which is why the lighthouse's lamp
/// survives the reduction. Its three steps are already ordered.
#[test]
fn two_comparisons_on_one_counter_that_cannot_meet_are_accepted() {
    assert!(collisions(
        "    use-with:\n      when  carrying x and step = 0\n      do    set step 1\n\
         \x20   use-with:\n      when  carrying y and step = 1\n      do    set step 2\n"
    )
    .is_empty());
}

/// Overlapping intervals are not separated, and `>= 1` overlaps `<= 1` at one value.
#[test]
fn comparisons_whose_intervals_meet_are_refused() {
    let m = collisions(
        "    use-with:\n      when  step >= 1\n      do    set seen\n\
         \x20   use-with:\n      when  step <= 1\n      say   \"nothing\"\n",
    );
    assert_eq!(m.len(), 1, "{m:?}");
}

/// Different second objects are different cells, which is the whole of what the third key does.
#[test]
fn different_second_objects_do_not_collide() {
    assert!(collisions(
        "    use-with x:\n      do    set seen\n    use-with y:\n      say   \"nothing\"\n"
    )
    .is_empty());
}

/// **A disjunction is separated by distributing, and a pair that can still meet is still
/// refused.**
///
/// `seen or carrying x` and `not seen` both hold when the flag is clear and `x` is in hand, so
/// this pair is inseparable however the guards are read, and the rule reports it.
#[test]
fn a_disjunction_that_can_meet_is_refused() {
    let m = collisions(
        "    use-with:\n      when  seen or carrying x\n      do    set seen\n\
         \x20   use-with:\n      when  not seen\n      say   \"nothing\"\n",
    );
    assert_eq!(m.len(), 1, "{m:?}");
}

/// **And one that cannot meet is accepted, which it was not for eighty-eight phases.**
///
/// `step = 1` against `step = 2 and seen or step = 2 and carrying x`: the counter cannot hold two
/// values, so no state satisfies both. The flattener used to return `false` on the first `Or` it
/// met, so **one disjunction anywhere in either guard made the whole pair inseparable** — not the
/// disjuncts that overlap, the pair.
///
/// The approximation was admitted deliberately and said why: *"Separating a disjunction wants the
/// cross product, nothing in the corpus writes one in a colliding cell, and asking the author for
/// a clearer guard is the safe direction to be wrong in."* The premise was a census over five
/// games that write `or` **zero** times, which is a claim about a construct nobody had used.
/// `design/specimen/relay.game` is the sixth and writes two, one of them in a colliding cell,
/// and the census stopped being about the language the moment it did.
///
/// It bit at exactly one place and it is the place the construct is for: a **staged** puzzle is a
/// run of bindings on one cell guarded on the counter's current value —
/// `two_comparisons_on_one_counter_that_cannot_meet_are_accepted` is the shape — so any stage
/// wanting a disjunction was refused.
///
/// What separates them now is the cross product the note named: a guard becomes a disjunction of
/// conjunctions, and two guards are disjoint when **every** way of satisfying one contradicts
/// **every** way of satisfying the other. With no `or` on either side that is one pair of atom
/// lists and the same test as before, so nothing that was accepted has moved.
/// `.yidam/decisions/or-earns-it.yml`.
#[test]
fn a_disjunction_that_cannot_meet_is_separated() {
    assert!(collisions(
        "    use-with:\n      when  step = 1\n      do    set step 2\n\
         \x20   use-with:\n      when  step = 2 and seen or step = 2 and carrying x\n               say   \"nothing\"\n"
    )
    .is_empty());

    // the same pair without the disjunction was always accepted, and still is
    assert!(collisions(
        "    use-with:\n      when  step = 1\n      do    set step 2\n\
         \x20   use-with:\n      when  step = 2 and seen\n      say   \"nothing\"\n"
    )
    .is_empty());

    // and distributing does not make everything separable: a disjunct that overlaps still does
    let m = collisions(
        "    use-with:\n      when  step = 1\n      do    set step 2\n\
         \x20   use-with:\n      when  step = 2 and seen or step = 1 and carrying x\n               say   \"nothing\"\n",
    );
    assert_eq!(m.len(), 1, "one disjunct meets the other guard: {m:?}");
}

/// **Every committed specimen resolves clean**, which is what makes the check a rule about what
/// may be written rather than a report about what is.
///
/// It was called *no committed specimen writes two bindings into one cell* and the body never
/// checked that — it checks that nothing is **reported**. Five games happened to key every cell
/// distinctly and the name recorded the accident; `design/specimen/relay.game` hauls a halyard in
/// two stages, which is two `pick-up` bindings on one cell told apart by a counter, and the body
/// stays green while the name goes false. A test whose name outruns its body cannot catch its own
/// claim expiring. `.yidam/decisions/or-earns-it.yml`.
#[test]
fn every_committed_specimen_resolves_clean() {
    for (_, src) in common::sources() {
        let (mut g, mut p) = game_format::parse(&src, "specimen.game");
        p.extend(game_format::resolve(&mut g));
        let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
        assert!(msgs.is_empty(), "{msgs:?}");
    }
}
