//! realizes: subsystem/response-presenter
//!
//! **The guarantee is that it returns when the sink returns, and that is all of it.**
//!
//! A component that delivers a line and returns immediately has presented nothing a player can
//! read — so the test that matters is not what it prints but *when it comes back*. A sink that
//! does not finish must keep the caller, because that is what keeps a binding one edge:
//! `.yidam/decisions/held.yml`.

use response_presenter::{present, present_all, weight, Perceive};

/// A sink that records what it was given, and in what order.
#[derive(Default)]
struct Heard {
    lines: Vec<String>,
    /// set when `deliver` returns, so a caller that came back early is visible
    done: bool,
}

impl Perceive for Heard {
    fn deliver(&mut self, line: &str) {
        self.done = false;
        self.lines.push(line.to_string());
        self.done = true;
    }
}

/// **It does not return until the sink has.**
#[test]
fn presenting_returns_only_when_the_sink_does() {
    let mut heard = Heard::default();
    let held = present(&mut heard, "The lid comes away with a squeal of nails.");
    assert!(heard.done, "the sink finished before the caller came back");
    assert_eq!(heard.lines, ["The lid comes away with a squeal of nails."]);
    assert_eq!(held, 42, "and it says how much it held them for");
}

/// **Order is the whole of presenting several.**
///
/// A chain of forced bindings says its lines in the sequence they fired, and a player reading two
/// lines in order is the nearest thing to observable ordering this vocabulary has — which
/// `.yidam/decisions/effect-list-semantics.yml` calls atomic and means about *state*.
#[test]
fn several_are_presented_in_the_order_they_were_said() {
    let mut heard = Heard::default();
    let total = present_all(&mut heard, ["first", "second", "third"]);
    assert_eq!(heard.lines, ["first", "second", "third"]);
    assert_eq!(total, 5 + 6 + 5, "and the total is what it held them for");
}

/// **How long is the line's length, and the component decides nothing about seconds.**
///
/// `subsystem/response-presenter` says duration is an accident: *"a line's length is how long the
/// game is held, and that is the only lever over pacing this vocabulary offers… an author who
/// wants a beat writes a longer sentence."* Nothing in the catalog states a reading speed, so this
/// component invents none — the sink decides what a character is worth.
#[test]
fn how_long_is_how_much_and_not_how_many_seconds() {
    assert_eq!(weight("Mm."), 3);
    assert_eq!(weight(""), 0);
    // characters, not bytes: this format carries em dashes in its authored prose
    assert_eq!(weight("A crate — nailed shut."), 22);

    let src = include_str!("../src/lib.rs");
    for unit in ["Duration", "millis", "secs", "sleep"] {
        assert!(!src.contains(unit), "the component reached for {unit}");
    }
}
