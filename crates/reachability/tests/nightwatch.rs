//! The third specimen, for one question: does `cutscene` earn its place?
//!
//! `cutscene.ont.yml` seeded the class while doubting it and named the reduction to argue
//! against — "an interaction whose response is long and whose input is locked". This game
//! wants a sequence twice, in two shapes, so that the argument has something to be about.
//!
//! One shape reduces to a binding. The other does not, and what fails is the *trigger*.

use reachability::{analyse, Cause};

const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");

fn load(source: &str) -> game_format::Game {
    let (mut game, problems) = game_format::parse(source, "nightwatch.game");
    let problems: Vec<String> = problems
        .into_iter()
        .chain(game_format::resolve(&mut game))
        .map(|p| p.to_string())
        .collect();
    assert!(
        problems.is_empty(),
        "nightwatch does not resolve:\n{}",
        problems.join("\n")
    );
    game
}

/// The two bindings that carry the sequence: the guarded `walk-to` on each door into `hall`.
fn sequence_copies(game: &game_format::Game) -> Vec<&game_format::Binding> {
    game.bindings
        .iter()
        .filter(|b| {
            b.verb == "walk-to"
                && b.effects
                    .iter()
                    .any(|e| matches!(e, game_format::Effect::Spend(x, _) if x == "ledger"))
        })
        .collect()
}

#[test]
fn the_specimen_parses_and_is_winnable() {
    let game = load(NIGHTWATCH);
    assert_eq!(game.start_room.as_deref(), Some("yard"));
    let r = analyse(&game);
    assert!(
        r.winnable,
        "a specimen that cannot be finished is not a game"
    );
}

/// **The reduction, asserted.** `study.case` is a whole scripted beat — the wire goes in, the
/// lock turns over, the lid comes up on its own weight — and it is one binding with one
/// effect. Everything that makes it a *scene* is in the `say`, and `say` is presentation:
/// `effect-list-semantics` puts it outside the state model, and `Binding` has no field for it.
///
/// So "the response is long" is not a fact about the vocabulary, and the first half of
/// `cutscene`'s reduction goes through.
#[test]
fn a_click_triggered_sequence_is_one_binding() {
    let game = load(NIGHTWATCH);
    let beat = game
        .bindings
        .iter()
        .find(|b| b.verb == "use-with" && b.object.as_deref() == Some("wire"))
        .expect("the case");
    assert_eq!(beat.effects.len(), 1, "a scripted beat, and one write");
    assert!(beat.when.is_some(), "gated, so it plays once");
}

/// **The cost of the workaround, structurally.** An arrival trigger has no site, so the
/// sequence is attached to the click that precedes each arrival — and `hall` has two doors.
/// Both copies exist, they are byte-identical in effect, and nothing relates them.
#[test]
fn the_arrival_triggered_sequence_is_written_once_per_entrance() {
    let game = load(NIGHTWATCH);
    let copies = sequence_copies(&game);
    assert_eq!(copies.len(), 2, "one copy per door into the hall");

    let shapes: Vec<String> = copies.iter().map(|b| format!("{:?}", b.effects)).collect();
    assert_eq!(shapes[0], shapes[1], "the copies are the same sequence");

    let guards: Vec<String> = copies.iter().map(|b| format!("{:?}", b.when)).collect();
    assert_eq!(guards[0], guards[1], "and the same guard");

    // and they are on different sites, which is the whole reason there are two
    let sites: Vec<String> = copies.iter().map(|b| format!("{:?}", b.site)).collect();
    assert_ne!(sites[0], sites[1]);
}

/// **And the duplication is a diagnostic cost, not only an authoring one.** One authored
/// intent, reported once per copy. A room with five doors would report the same defect five
/// times, and nothing in the output says they are one.
#[test]
fn one_defect_is_reported_once_per_entrance() {
    let r = analyse(&load(NIGHTWATCH));
    assert_eq!(
        r.strandings.len(),
        2,
        "two reports for one sequence; got:\n{}",
        r.render()
    );
    assert!(r.strandings.iter().all(|s| s.cause == Cause::Consumption));
    // `object` is the verb's second object and `walk-to` has none — which is itself the
    // shape of the problem: the sequence hangs off a movement verb, not off a thing used.
    assert!(r.strandings.iter().all(|s| s.object.is_none()));
    assert!(r.strandings.iter().all(|s| s.detail.contains("ledger")));
    let targets: Vec<&str> = r.strandings.iter().map(|s| s.target.as_str()).collect();
    assert!(targets.contains(&"yard.hall-door"));
    assert!(targets.contains(&"study.hall-door"));
}

/// **The failure the duplication invites, and what this project does and does not catch.**
/// Drift one copy's guard and the game really changes — a bigger reachable set, more
/// transitions. **The findings do not move at all**: the same two strandings, same lines,
/// same cause, same counts, same examples. Only the summary numbers differ, and nothing says
/// that a number moving means two copies of one sequence have come apart.
///
/// So an author reading the report learns nothing, and an author diffing the totals learns
/// that *something* changed. This is `puzzle/state-gate`'s complaint arriving from a
/// direction that node did not name: "a gate depending on three flags is stated three times,
/// or more, and nothing checks that the statements agree."
#[test]
fn a_copy_may_drift_and_no_check_here_reports_it() {
    let clean = analyse(&load(NIGHTWATCH));

    // change the *second* copy only
    let i = NIGHTWATCH
        .rfind("      when  alarm\n")
        .expect("the second copy");
    let drifted = format!(
        "{}      when  case-open\n{}",
        &NIGHTWATCH[..i],
        &NIGHTWATCH[i + "      when  alarm\n".len()..]
    );
    let r = analyse(&load(&drifted));

    assert_ne!(
        r.states, clean.states,
        "the drift must actually change the game, or this test proves nothing"
    );
    assert_ne!(r.transitions, clean.transitions);

    // and yet every finding is identical
    let findings = |x: &reachability::Report| -> Vec<String> {
        x.strandings
            .iter()
            .map(|s| {
                format!(
                    "{} {} {} {:?} {} {}",
                    s.line, s.verb, s.target, s.cause, s.detail, s.example
                )
            })
            .collect()
    };
    assert_eq!(
        findings(&r),
        findings(&clean),
        "the findings should have moved and did not — that is what this specimen is for"
    );
    assert_eq!(r.stranded, clean.stranded);
}

#[test]
fn the_reachable_set_is_a_fraction_of_the_space() {
    let r = analyse(&load(NIGHTWATCH));
    assert!(r.observed_space > r.states as u128);
}
