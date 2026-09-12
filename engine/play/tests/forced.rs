//! composes: verb-menu, interaction-resolver
//!
//! **The longest line anybody wrote in four specimens is one the engine said nothing of.**
//!
//! `nightwatch.game:89` is 143 characters — *"Three steps in, a lamp comes up. The watchman takes
//! the ledger out of your hands without hurrying, and the door at the far end is already open."*
//! It is the response of an `on-enter` binding, and an `on-enter` binding is **forced**: it fires
//! inside `interaction_resolver::settle` rather than being chosen.
//!
//! `settle` returns a state, and every line the bindings it fired would have said was dropped on
//! the way out — a player who walked into the hall carrying the ledger was told nothing and found
//! themselves in a cell. `settle_tracking` returns the words with the bindings now, and **selects
//! each in the state it fired from**, because a chain fires each binding from the state the
//! previous one left and a guarded list read afterwards would pick the wrong clause.
//!
//! **Nothing could have caught this.** A response writes no state, so the search cannot see it;
//! the component fixtures ask one binding at a time, so neither can they. It took playing.
//! `interaction_resolver::settle_tracking` has returned the list of bindings that fired since it
//! was written — for the cause classifier, which wanted their *effects* — and nobody wanted their
//! words until there was something to say them to.
//!
//! `.yidam/decisions/walked-in.yml`.

use play::{load, Session, Stop};

const NIGHTWATCH: &str = include_str!("../../../design/specimen/nightwatch.game");

/// The run that takes the ledger and walks into the hall.
const INTO_THE_HALL: [&str; 5] = [
    "pick-up loose-wire",
    "walk-to window",
    "use-with wire on case",
    "pick-up ledger-in-case",
    "walk-to hall-door",
];

const THE_LINE: &str = "Three steps in, a lamp comes up. The watchman takes the ledger out of \
                        your hands without hurrying, and the door at the far end is already open.";

/// **It is the longest response in the corpus, and it is said.**
#[test]
fn the_longest_line_in_four_specimens_is_said() {
    assert_eq!(THE_LINE.chars().count(), 143);
    assert!(
        NIGHTWATCH.contains(THE_LINE),
        "an author wrote it, on the `on-enter` that springs the trap"
    );

    let game = load(NIGHTWATCH, "nightwatch").expect("the specimen reads clean");
    let mut session = Session::start(&game).expect("it starts");
    assert_eq!(session.play(&INTO_THE_HALL), Stop::OutOfInput);

    assert_eq!(
        session.here(),
        "The Cell",
        "the trigger fired and moved them"
    );
    let last = session.log.last().expect("a turn");
    assert_eq!(
        last.said, None,
        "the binding the player chose says nothing — `walk-to` on a door"
    );
    assert_eq!(
        last.forced,
        [THE_LINE],
        "and what they are told is the arrival trigger's, which is not a response to anything \
         they picked"
    );
    assert!(
        session.transcript().contains("Three steps in"),
        "so it reaches the transcript"
    );
    assert!(!last.silent(), "and the turn is not a silent one");
}

/// **A forced line is selected in the state it fired from.**
///
/// The rule that made this the resolver's job rather than the client's. `nightwatch.game`'s
/// trigger is guarded — `when alarm` — and it fires from the state the player's own binding left,
/// not from the settled one. There is no chain in the corpus long enough to make the two differ
/// yet, so this holds the shape rather than a case: the words come back attached to the bindings,
/// from the one function that sees the states in between.
#[test]
fn the_words_come_back_attached_to_the_bindings_that_said_them() {
    let resolver = include_str!("../../interaction-resolver/src/lib.rs");
    assert!(
        resolver.contains("pub struct Fired<'a> {")
            && resolver.contains("pub says: Option<&'a Response>,"),
        "a fired binding carries what it said"
    );
    assert!(
        resolver.contains("let says = clause(w, &b.says, &cur);"),
        "selected before the effects are folded"
    );
}

/// **The loop asks what was said, and the search still asks only what was done.**
///
/// `settle_tracking` has returned the list of fired bindings since it was written —
/// `crates/reachability` uses it for their *effects*, because a chosen binding's effect list
/// stopped being everything that happens when it fires the day `on-enter` arrived. The words were
/// dropped by the same arithmetic and nobody noticed, because nothing said words.
///
/// The seam holds: the client reads `.says`, the search reads `.binding.effects` and no text.
///
/// **`Fired.says` became the clause rather than the string and the seam held anyway, which is the
/// sharper statement of it.** `.yidam/decisions/orphaned-lines.yml` gave the search a reason to
/// touch `.says` for the first time: a line no reachable state selects is authored text nobody
/// can read, and naming it needs the clause. What the search takes off the clause is `.line`. So
/// the rule is not that the search never mentions a response — it is that **the search counts
/// lines and never reads one**, which is what `two-components` drew the seam for.
#[test]
fn the_loop_asks_what_was_said_and_the_search_does_not() {
    let src = include_str!("../src/lib.rs");
    assert!(
        src.contains("settle_tracking(&self.world, next)") && !src.contains(" settle(&self.world"),
        "the loop takes the state and the list"
    );
    assert!(
        src.contains(".filter_map(|f| f.says.map(|r| r.text.as_str()))"),
        "and reads the words off it"
    );

    let search = include_str!("../../../crates/reachability/src/lib.rs");
    for reading in [".text", "r.text", "says.text"] {
        assert!(
            !search.contains(reading),
            "the search reads a response's text at {reading}"
        );
    }
    assert!(
        search.contains("f.says.map(|r| r.line)"),
        "and takes the line off it instead"
    );
}
