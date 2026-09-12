//! realizes: subsystem/verb-menu
//!
//! **Given a target and a state, the verbs to display and no others.**
//!
//! The negative half is the load-bearing half: a verb this component withholds is one the
//! resolver is never asked about, which is why `interaction-resolver` can promise *never nothing*
//! without a default to fall through to.
//!
//! Nothing here was designed as an interface. The derivation is three decisions taken separately,
//! none of them about a player — `.yidam/decisions/default-binding.yml` (the menu holds authored
//! bindings and nothing stands in for an unauthored pair), `.yidam/decisions/arity-cost.yml` (a
//! binary verb is keyed by its second object, so its entry appears while that object is carried),
//! and `.yidam/decisions/availability.yml` (a `when` with no `else` withholds; one with an `else`
//! shows the entry and declines). `.yidam/decisions/verb-menu.yml` put it: *"`Ix::in_menu` is
//! those three and nothing else."*
//!
//! It was `Ix::in_menu`, a private method of the checker, until
//! `.yidam/decisions/two-components.yml`.

use game_format::{slot, Binding, Site, State, World};

/// One entry a player can point at: a verb, its second object where it has one, and what the
/// target is called.
///
/// The label is the payoff of `.yidam/decisions/authored-text.yml`. A menu built before that could
/// print `quay.crate`; this one prints *"a nailed-shut crate"*, because the parse tree stopped
/// throwing the label away.
#[derive(Debug, Clone, Copy)]
pub struct Entry<'a> {
    pub verb: &'a str,
    /// the second object of a binary verb, as written
    pub object: Option<&'a str>,
    /// the hotspot, actor or option the entry hangs on, by canonical name
    pub target: &'a str,
    /// what that target is called, when the author gave it a label
    pub label: Option<&'a str>,
    pub binding: &'a Binding,
}

/// **The whole of this component: every entry shown at `target` in this state, in written
/// order.**
pub fn at<'a>(w: &'a World<'a>, target: &str, s: &State) -> Vec<Entry<'a>> {
    shown(w, s)
        .into_iter()
        .filter(|e| e.target == target)
        .collect()
}

/// Every entry shown anywhere in this state.
///
/// A menu is per-target and this is not one; it is what the search enumerates, and what a
/// player sees is [`at`]. Kept because the figures the corpus verifies are counted this way —
/// 88 distinct target-and-verb entries and 698 slots over 145 reachable states.
pub fn shown<'a>(w: &'a World<'a>, s: &State) -> Vec<Entry<'a>> {
    w.game
        .bindings
        .iter()
        .filter(|b| in_menu(w, b, s))
        .map(|b| {
            let target = match &b.site {
                Site::InRoom { target, .. } => target.as_str(),
                Site::InConversation { option, .. } => option.as_str(),
                Site::OnEnter { room } => room.as_str(),
            };
            Entry {
                verb: b.verb.as_str(),
                object: b.object.as_deref(),
                target,
                label: w.game.label(target),
                binding: b,
            }
        })
        .collect()
}

/// **What a contextual menu would show at this binding's target in this state.**
///
/// This design has an interface and never declared one. It does not need to: the derivation
/// is three decisions taken separately, for reasons that were not about a player at all.
///
/// - `.yidam/decisions/default-binding.yml` — the menu holds authored bindings and nothing
///   stands in for an unauthored cell, which is why [`live`] enumerates rather than
///   crosses.
/// - `.yidam/decisions/arity-cost.yml` — a binary verb is keyed by its second object, so its
///   entry appears only while that object is carried. That is the `b.object` clause below.
/// - `.yidam/decisions/availability.yml` — a `when` with no `else` **withholds** and a `when`
///   with one **offers and declines**. That is the whole of the last line.
///
/// An `on-enter` binding is never in a menu, because it is forced rather than chosen and
/// there is no target to point at.
pub fn in_menu(w: &World, b: &Binding, s: &State) -> bool {
    !matches!(b.site, Site::OnEnter { .. })
        && live(w, b, s)
        && (b.refuses || b.when.as_ref().map(|q| w.holds(q, s)).unwrap_or(true))
}

/// Everything `offered` asks except the authored guard: is the target there to be pointed
/// at, and does the player hold what a binary verb needs?
pub fn live(w: &World, b: &Binding, s: &State) -> bool {
    match (&b.site, s.mode) {
        (Site::InConversation { conversation, .. }, Some(m)) => {
            if slot(&w.convs, conversation) != m {
                return false;
            }
        }
        (Site::InRoom { room, target }, None) => {
            if w.at[room] != s.room {
                return false;
            }
            // an actor who has walked off is not there to be addressed. `actor <name>`
            // inside a room says where you may talk to them; `holds` and `move ... to`
            // say where they *are*, and the site is only live when the two agree. This is
            // the same shape as the acquisition check below it — a site whose subject has
            // gone is not a site.
            if let Some(i) = w.actors.iter().position(|a| a == target) {
                if s.actors[i] != s.room {
                    return false;
                }
            }
            // an acquisition site whose object has been taken is not there any more
            if let Some(i) = w.takeable.iter().position(|t| t == target) {
                if s.taken[i] {
                    return false;
                }
            }
        }
        // An entry binding is offered by being in its room and nowhere else. The `None`
        // is doing work: a forced binding must not fire while the player is inside a
        // conversation, or the mode would be left by something the player never chose.
        (Site::OnEnter { room }, None) => {
            if w.at[room] != s.room {
                return false;
            }
        }
        _ => return false,
    }
    if let Some(o) = &b.object {
        if s.items[slot(&w.items, o)] < 1 {
            return false;
        }
    }
    true
}
