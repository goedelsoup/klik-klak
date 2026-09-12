//! realizes: subsystem/response-presenter
//!
//! **Turn a response into something perceived, and do not return until it is over.**
//!
//! Both halves matter and only the first is obvious: a component that delivers a line and returns
//! immediately has presented nothing a player can read.
//!
//! **The second half is what keeps a binding an edge.** `.yidam/decisions/effect-list-semantics.yml`
//! says effects are *instantaneous* and a binding *atomic* — nothing observes a value between two
//! writes, so the reachable set is a graph rather than a search over states-and-time. Give a
//! response duration and let the game go on underneath it and other bindings fire during the
//! interval, which is the dimension that decision refused. **Block, and the interval contains
//! nothing**: no state exists during it that did not exist before it, because nothing can act.
//! The two positions are a claim and its precondition — `.yidam/decisions/held.yml`.
//!
//! AGS ran both arms. Its declarative layer blocked by default because it had nowhere else to put
//! the question; its replacement did not, and the manual gained `BlockingScripts.md` for the
//! consequences — three script threads, a queue of five, and events lost when a thread blocks too
//! long.
//!
//! **This component decides how long there is to perceive and nothing about seconds.** Nothing in
//! the catalog states a reading speed, so none is invented here.

/// What perceiving is, which this component does not decide.
///
/// A terminal writes a line and returns; something with a frame holds the player for a while;
/// something with a voice waits for it to finish. The component's guarantee is that it returns
/// when this does, and it has no opinion about when that is.
pub trait Perceive {
    /// Deliver the line, and return when the player has had it.
    fn deliver(&mut self, line: &str);
}

/// **How much there is to perceive.**
///
/// Characters, because `subsystem/response-presenter` already says so: duration is *"an accident…
/// a line's length is how long the game is held, and that is the only lever over pacing this
/// vocabulary offers"*, and *"an author who wants a beat writes a longer sentence"*. That was an
/// `[inference]` about an authoring facility nobody decided on. It is a number here.
pub fn weight(line: &str) -> usize {
    line.chars().count()
}

/// **Present one line, and do not return until it is over.**
///
/// Returns what it held the player for. What it is given is a line; where the line came from is
/// not its question — which is why this is a component rather than part of the thing that produced
/// it.
pub fn present(to: &mut impl Perceive, line: &str) -> usize {
    to.deliver(line);
    weight(line)
}

/// Present several in order, and return the total.
///
/// Order is the whole of it. A chain of forced bindings says its lines in the sequence they fired
/// — `.yidam/decisions/walked-in.yml` — and a player reading two lines in sequence is the nearest
/// thing to observable ordering this vocabulary has.
pub fn present_all<'a>(to: &mut impl Perceive, lines: impl IntoIterator<Item = &'a str>) -> usize {
    lines.into_iter().map(|l| present(to, l)).sum()
}
