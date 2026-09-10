//! A counter's range is declared and its steps are not.
//!
//! `resolve` already refuses a comparison outside the declared range — `lamp-prep = 7` against
//! `0..3`. What it could not see is that the values *inside* the range are not all reachable: a
//! staged puzzle is a run of bindings each guarded on the value the last one wrote, and that
//! agreement is held by the author alone.
//!
//! `unreachable_values` closes the gap without new syntax, because the steps are already implicit
//! in the bindings. `.yidam/decisions/named-condition.yml` is the record, and
//! `crates/reachability/tests/conditions.rs` is where the same check meets a real specimen.

use game_format::{parse, resolve};

/// One counter, one room, and whatever bindings the case needs.
fn game(body: &str) -> Vec<String> {
    let source = format!(
        "room    hall \"The Hall\"\n\
         item    wick \"a wick\"\n\
         counter step 0..3 \"how far along\"\n\
         \n\
         room hall\n\
         \x20 start\n\
         \x20 hotspot lamp \"the lamp\"\n{body}\n\
         \x20 hotspot door \"the door\"\n\
         \x20   walk-to:\n\
         \x20     ending\n"
    );
    let (mut g, mut p) = parse(&source, "sequences.game");
    p.extend(resolve(&mut g));
    p.into_iter().map(|x| x.to_string()).collect()
}

#[test]
fn a_step_whose_value_is_written_is_accepted() {
    let found = game(
        "    use-with wick:\n\
         \x20     when  step = 0\n\
         \x20     do    set step 1\n\
         \x20   look-at:\n\
         \x20     when step = 1   \"Wicked.\"\n\
         \x20     otherwise       \"Bare.\"",
    );
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_step_whose_value_nothing_writes_is_refused() {
    let found = game(
        "    use-with wick:\n\
         \x20     when  step = 0\n\
         \x20     do    set step 2\n\
         \x20   look-at:\n\
         \x20     when step = 1   \"Wicked.\"\n\
         \x20     otherwise       \"Bare.\"",
    );
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].contains("no value `step` can hold satisfies `step = 1`")
            && found[0].contains("only ever set to 0, 2"),
        "{}",
        found[0]
    );
}

/// The starting value counts as holdable even though no binding writes it.
#[test]
fn the_declared_floor_is_a_value_the_counter_can_hold() {
    let found = game(
        "    use-with wick:\n\
         \x20     when  step = 0\n\
         \x20     do    set step 1",
    );
    assert!(
        found.is_empty(),
        "a guard on the initial value is fine: {found:?}"
    );
}

/// A range comparison is satisfiable if *any* holdable value satisfies it, not only an equality.
#[test]
fn a_range_comparison_needs_one_value_in_it() {
    let ok = game(
        "    use-with wick:\n\
         \x20     when  step = 0\n\
         \x20     do    set step 1\n\
         \x20   look-at:\n\
         \x20     when step >= 1   \"Underway.\"\n\
         \x20     otherwise        \"Bare.\"",
    );
    assert!(ok.is_empty(), "{ok:?}");

    let dead = game(
        "    use-with wick:\n\
         \x20     when  step = 0\n\
         \x20     do    set step 1\n\
         \x20   look-at:\n\
         \x20     when step >= 2   \"Underway.\"\n\
         \x20     otherwise        \"Bare.\"",
    );
    assert_eq!(dead.len(), 1, "{dead:?}");
    assert!(dead[0].contains("`step >= 2`"), "{}", dead[0]);
}

/// The range check that was already there still fires, and this one does not double up on it.
#[test]
fn a_value_outside_the_declared_range_is_still_one_problem() {
    let found = game(
        "    use-with wick:\n\
         \x20     when  step = 0\n\
         \x20     do    set step 1\n\
         \x20   look-at:\n\
         \x20     when step = 7   \"Impossible.\"\n\
         \x20     otherwise       \"Bare.\"",
    );
    assert_eq!(
        found.len(),
        2,
        "both checks speak, and they say different things: {found:?}"
    );
    assert!(found
        .iter()
        .any(|f| f.contains("outside its declared range")));
    assert!(found.iter().any(|f| f.contains("no value `step` can hold")));
}
