//! realizes: subsystem/interaction-resolver
//!
//! **Given a composed binding, produce exactly one outcome: an authored response, an authored
//! refusal, or a default. Never nothing.**
//!
//! *"Never nothing"* is the whole specification — a silent miss is the failure players read as
//! brokenness, and it is entirely preventable at this boundary. There is no default here and no
//! miss path, because what a player can pick is what `verb-menu` showed them and that shows
//! authored bindings only. The corpus argues it the other way round and reaches the same place:
//! *"this component is asked about an unauthored pair never, the hit path is the only path, and
//! the rare branch it might have optimized does not exist."*
//!
//! It also runs the arrival trigger, which has no verb, no target and no second object:
//! [`settle`] fires every `on-enter` binding whose room the player has entered, to a fixpoint.
//!
//! It was `Ix::live`, `Ix::offered`, `Ix::apply` and `Ix::settle`, private methods of the checker,
//! until `.yidam/decisions/two-components.yml`.

use game_format::{slot, Binding, Effect, Response, Site, State, World};
use std::collections::HashSet;

/// What happens when the player picks an entry. Two variants and no third: an outcome is always
/// produced, which is the component's one guarantee.
#[derive(Debug, Clone)]
pub enum Outcome<'a> {
    /// Its precondition held. The line it says, and the state it leaves behind.
    Fired { says: Option<&'a str>, next: State },
    /// Its precondition did not hold and it had an `else`, so the player is told why.
    ///
    /// `says: None` is the hole `.yidam/decisions/authored-text.yml` opened and declined to check:
    /// a refusal list whose clauses do not cover the state the binding was refused in. It is the
    /// silent miss this component exists to prevent, arriving from inside rather than from an
    /// unauthored pair.
    Refused { says: Option<&'a str> },
}

/// A forced binding that fired, and what it said.
///
/// **The line is selected here rather than by the caller, and that is the whole reason this type
/// exists.** A chain of `on-enter` bindings fires each from the state the previous one left, so a
/// guarded response list evaluated afterwards against the settled state selects the wrong clause.
/// Only [`settle_tracking`] sees the intermediate states. `.yidam/decisions/walked-in.yml`.
#[derive(Debug, Clone, Copy)]
pub struct Fired<'a> {
    pub binding: &'a Binding,
    pub says: Option<&'a str>,
}

/// The first line whose predicate holds, or the first with none. A response list is
/// first-match-wins, which is how the author reads it: `otherwise` is a fallback because it is
/// written last.
pub fn select<'a>(w: &World, list: &'a [Response], s: &State) -> Option<&'a str> {
    list.iter()
        .find(|r| r.when.as_ref().map(|q| w.holds(q, s)).unwrap_or(true))
        .map(|r| r.text.as_str())
}

/// **The component, in one function.**
///
/// Only ever asked about a binding `verb_menu::in_menu` admitted, so the two branches below are
/// exhaustive over what a player can do.
pub fn resolve<'a>(w: &World, b: &'a Binding, s: &State) -> Outcome<'a> {
    if offered(w, b, s) {
        Outcome::Fired {
            says: select(w, &b.says, s),
            next: apply(w, b, s),
        }
    } else {
        Outcome::Refused {
            says: select(w, &b.refusals, s),
        }
    }
}

/// Is this binding offered in this state? Site and mode first, then the two structural
/// preconditions `interaction/binary-binding` says the format enforces, then the
/// authored one.
///
/// **"Offered" here means *can fire*, and that is not what an interface means by it.** The
/// difference is the last line: a binding whose `when` fails is not a transition, and a
/// binding whose `when` fails *with a refusal under it* is still shown to the player and
/// declines. `verb_menu::in_menu` is the other sense, and the two were the same function for
/// seven phases because the search was the only reader.
pub fn offered(w: &World, b: &Binding, s: &State) -> bool {
    verb_menu::live(w, b, s) && b.when.as_ref().map(|q| w.holds(q, s)).unwrap_or(true)
}

/// The same transition with its spends left out. Used to ask whether the spending is
/// Everything this binding spends when fired from `s` — **including what the bindings it
/// forces spend.** A chosen binding's own effect list stopped being the whole answer when
/// `on-enter` arrived: walking through a door spends nothing, and the watchman on the
/// other side of it takes the ledger.
pub fn spends(w: &World, b: &Binding, s: &State) -> Vec<String> {
    let after = apply(w, b, s);
    let fired = settle_tracking(w, after)
        .map(|(_, f)| f)
        .unwrap_or_default();
    let mut out: Vec<String> = Vec::new();
    for e in b
        .effects
        .iter()
        .chain(fired.iter().flat_map(|f| &f.binding.effects))
    {
        if let Effect::Spend(x, _) = e {
            if !out.contains(x) {
                out.push(x.clone());
            }
        }
    }
    out
}

/// The same transition with its spends left out. Used to ask whether the spending is what
/// stranded a state, rather than guessing from which effects the binding carries.
///
/// (Those two sentences were on `spends` until this crate was split out of the checker, where
/// they had run together with that function's own doc comment and described neither.)
pub fn apply_without_spending(w: &World, b: &Binding, s: &State) -> State {
    let after = apply(w, b, s);
    // The forced bindings run, as they would have — and what *they* spend counts as this
    // binding's spending for the purpose of the counterfactual. The player chose to walk
    // through the door; the watchman taking the ledger is a consequence of that choice
    // and is nowhere in the door's effect list.
    //
    // This is the third time a new primitive has broken the cause classifier, and the
    // third time in the same place: it assumed everything a binding does is written in
    // that binding.
    let (mut n, fired) = settle_tracking(w, after.clone()).unwrap_or((after, Vec::new()));
    for e in b
        .effects
        .iter()
        .chain(fired.iter().flat_map(|f| &f.binding.effects))
    {
        if let Effect::Spend(x, _) = e {
            if let Some(i) = w.items.iter().position(|y| y == x) {
                n.items[i] = s.items[i];
            }
        }
    }
    n
}

/// Fold the binding's effect list into the state, in written order.
///
/// Sequential, total, instantaneous and atomic — `.yidam/decisions/effect-list-semantics.yml`.
/// The whole list applies, nothing observes a value between two effects, and the result is one
/// state rather than a sequence of them.
pub fn apply(w: &World, b: &Binding, s: &State) -> State {
    let mut n = s.clone();
    if let Site::InConversation { option, .. } = &b.site {
        // Unobserved: nothing asks whether this option was taken, so recording it would
        // split states no predicate can tell apart.
        if let Some(i) = w.options.iter().position(|o| o == option) {
            n.chosen[i] = true;
        }
    }
    for e in &b.effects {
        match e {
            // A write to something no predicate reads is not a transition: it changes
            // nothing anybody in the game can tell apart. Same shape as the `chosen` fix.
            Effect::SetFlag(x) => {
                if let Some(i) = w.flags.iter().position(|f| f == x) {
                    n.flags[i] = true;
                }
            }
            Effect::ClearFlag(x) => {
                if let Some(i) = w.flags.iter().position(|f| f == x) {
                    n.flags[i] = false;
                }
            }
            Effect::SetCounter(x, v) => {
                if let Some(i) = w.counters.iter().position(|c| c == x) {
                    n.counters[i] = *v;
                }
            }
            Effect::Give(x, q) => {
                let Some(i) = w.items.iter().position(|y| y == x) else {
                    continue;
                };
                n.items[i] = (n.items[i] + q).min(w.caps[i]);
            }
            Effect::Spend(x, q) => {
                let Some(i) = w.items.iter().position(|y| y == x) else {
                    continue;
                };
                n.items[i] = (n.items[i] - q).max(0);
            }
            Effect::Move(None, x) => n.room = w.at[x],
            Effect::Move(Some(a), x) => {
                // Unobserved: nothing reads it, so writing it would split states that are
                // not distinguishable. See `State::actors`.
                if let Some(i) = w.actors.iter().position(|n| n == a) {
                    n.actors[i] = w.at[x];
                }
            }
            Effect::Enter(x) => n.mode = Some(slot(&w.convs, x)),
            Effect::Leave => n.mode = None,
        }
    }
    // taking the object is the room losing it
    if let Site::InRoom { target, .. } = &b.site {
        if let Some(i) = w.takeable.iter().position(|t| t == target) {
            let stands_for = &w.game.carried_form[target];
            if b.effects
                .iter()
                .any(|e| matches!(e, Effect::Give(x, _) if x == stands_for))
            {
                n.taken[i] = true;
            }
        }
    }
    n
}

/// Fire the forced bindings until the player stops arriving anywhere.
///
/// An entry binding fires on *arriving* in a room, not while standing in one — which is
/// how both AGS eras document the event, and is what keeps a description-only entry
/// binding from firing forever. So: fire this room's offered entry bindings in written
/// order; if that moved the player, they have arrived somewhere and the new room's fire
/// too; if it did not, the state has settled.
///
/// **This is the second primitive in this design that costs decidability**, after item
/// quantity. Two rooms whose entry bindings move the player into each other never settle,
/// and the search would not terminate. The guard is exact rather than a step budget: the
/// states visited in one chain are remembered, and a repeat is a cycle. It terminates
/// because a chain that does not repeat a state cannot be longer than the state space.
///
/// `Err` carries the room the chain was in when it closed, for the report.
pub fn settle(w: &World, cur: State) -> Result<State, usize> {
    settle_tracking(w, cur).map(|(s, _)| s)
}

/// [`settle`], also reporting which forced bindings fired **and what each of them said**.
///
/// The list is what [`apply_without_spending`] needs and could not get from the binding it was
/// handed: a chosen binding's effect list is no longer everything that happens when it fires. The
/// words came later and for the same reason — a chosen binding's response is no longer everything
/// the player is told. `nightwatch.game`'s watchman takes the ledger in the longest line anybody
/// wrote in four specimens, and it was dropped here for as long as this function existed.
pub fn settle_tracking<'a>(
    w: &World<'a>,
    mut cur: State,
) -> Result<(State, Vec<Fired<'a>>), usize> {
    let mut seen: HashSet<State> = HashSet::new();
    let mut fired: Vec<Fired<'a>> = Vec::new();
    loop {
        if !seen.insert(cur.clone()) {
            return Err(cur.room);
        }
        let before = cur.room;
        for b in &w.entries[cur.room] {
            if offered(w, b, &cur) {
                // selected before the effects are folded, because the state it fired from is the
                // state its guards were written against
                let says = select(w, &b.says, &cur);
                cur = apply(w, b, &cur);
                fired.push(Fired { binding: b, says });
            }
        }
        if cur.room == before {
            return Ok((cur, fired));
        }
    }
}
