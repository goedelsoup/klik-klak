//! composes: verb-menu, interaction-resolver
//!
//! **The longest line anybody wrote in four specimens is one the engine never says.**
//!
//! `nightwatch.game:89` is 143 characters — *"Three steps in, a lamp comes up. The watchman takes
//! the ledger out of your hands without hurrying, and the door at the far end is already open."*
//! It is the response of an `on-enter` binding, and an `on-enter` binding is **forced**: it fires
//! inside `interaction_resolver::settle` rather than being chosen.
//!
//! `settle` returns a state. Every line the bindings it fired would have said is dropped on the
//! way out, so a player who walks into the hall carrying the ledger is told nothing and finds
//! themselves in a cell.
//!
//! **Nothing could have caught this.** A response writes no state, so the search cannot see it;
//! the component fixtures ask one binding at a time, so neither can they. It took playing.
//! `interaction_resolver::settle_tracking` has returned the list of bindings that fired since it
//! was written — for the cause classifier, which wanted their *effects* — and nobody wanted their
//! words until there was something to say them to.
//!
//! This pins the defect. It is rewritten into the rule in the same phase.

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

/// **It is the longest response in the corpus, and it is the one that is lost.**
#[test]
fn the_longest_line_in_four_specimens_is_never_said() {
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
    assert!(
        !session.transcript().contains("Three steps in"),
        "and said nothing on the way"
    );
    assert_eq!(
        session.log.last().expect("a turn").said,
        None,
        "the turn that walked in reports no line at all"
    );
}

/// **The client asks for a state and the tracking form goes unused.**
///
/// `settle_tracking` already returns which forced bindings fired. `crates/reachability` uses it,
/// for their effects — a chosen binding's effect list stopped being everything that happens when
/// it fires the day `on-enter` arrived. The words were dropped by the same arithmetic and nobody
/// noticed, because nothing said words.
#[test]
fn the_loop_settles_without_asking_what_was_said() {
    let src = include_str!("../src/lib.rs");
    assert!(
        src.contains("settle(&self.world, next)") && !src.contains("settle_tracking"),
        "the loop takes the state and leaves the list"
    );
    let resolver = include_str!("../../interaction-resolver/src/lib.rs");
    assert!(
        resolver.contains("pub fn settle_tracking<'a>("),
        "which the resolver has been offering since it was written"
    );
}
