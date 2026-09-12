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

/// **`or` is refused rather than approximated.** Separating a disjunction wants the cross product,
/// nothing in the corpus writes one in a colliding cell, and asking the author for a clearer guard
/// is the safe direction to be wrong in.
#[test]
fn a_disjunction_is_treated_as_inseparable() {
    let m = collisions(
        "    use-with:\n      when  seen or carrying x\n      do    set seen\n\
         \x20   use-with:\n      when  not seen\n      say   \"nothing\"\n",
    );
    assert_eq!(m.len(), 1, "{m:?}");
}

/// The four committed specimens key every cell distinctly, so the check is a rule about what may
/// be written rather than a report about what is.
#[test]
fn no_committed_specimen_writes_two_bindings_into_one_cell() {
    for (_, src) in common::sources() {
        let (mut g, mut p) = game_format::parse(&src, "specimen.game");
        p.extend(game_format::resolve(&mut g));
        let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
        assert!(msgs.is_empty(), "{msgs:?}");
    }
}
