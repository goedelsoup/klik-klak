//! The world a `.game` declares, and a state of it.
//!
//! **This is not a subsystem and the corpus is why.** All four components in the
//! `subsystem` tier declare no state across frames — the resolver *"None across frames"*, the
//! menu *"None"*, the solver *"None across frames, given the walkbox set"*, and the presenter's
//! is transient and deliberately unsaved. So nothing in that tier owns a world, and a module
//! under `engine/` that held one would be inventing a fifth component, which the gate forbids.
//!
//! It lands here instead because a [`World`] is the game's declared names arranged for lookup and
//! [`World::holds`] is the runtime twin of the admission rule [`crate::resolve`] already is:
//! `resolve` asks whether an operand names something declared, `holds` asks what it is now.
//! `.yidam/decisions/two-components.yml`.

use crate::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct State {
    pub room: usize,
    pub mode: Option<usize>,
    /// how many of each item is carried — items bear a quantity, which is what makes
    /// `count` a question worth asking and `carrying` sugar for `count >= 1`
    pub items: Vec<i64>,
    /// per acquisition site, whether its object has been taken.
    ///
    /// `inventory-item -carried-form-of-> hotspot` says the hotspot is what the object *is*
    /// when it is not carried, so taking it is the room losing it — the corpus settled that
    /// in `effect-vocabulary` and this is where it stops being an argument. Without it an
    /// acquisition site would hand out an unbounded supply and the state space would not
    /// close.
    pub taken: Vec<bool>,
    pub flags: Vec<bool>,
    pub counters: Vec<i64>,
    pub chosen: Vec<bool>,
    /// where each **observed** actor is, by room slot.
    ///
    /// Observed means *some predicate asks*. An actor nobody asks about is not in this vector
    /// and `move <actor> to <room>` on them writes nothing, because an effect nothing can read
    /// is not state and two states differing only there are the same state. That is what keeps
    /// the cost proportional to the question: a game that never reads a position pays nothing,
    /// and one that reads *n* of them multiplies its space by rooms^*n*.
    pub actors: Vec<usize>,
}

/// The game's declared names, arranged for lookup.
///
/// Every field is `pub` because three crates read them: the menu, the resolver, and the search
/// that drives both. It was `struct Index` — private, and most of `crates/reachability` — until
/// `.yidam/decisions/two-components.yml`.
pub struct World<'a> {
    pub game: &'a Game,
    pub rooms: Vec<String>,
    pub items: Vec<String>,
    pub flags: Vec<String>,
    pub counters: Vec<String>,
    pub options: Vec<String>,
    pub convs: Vec<String>,
    pub caps: Vec<i64>,
    /// hotspots that stand for an item, in declaration order
    pub takeable: Vec<String>,
    pub at: BTreeMap<String, usize>,
    /// actors some predicate names, in declaration order. See [`State::actors`].
    pub actors: Vec<String>,
    /// bindings sited at `on-enter`, indexed by room slot and kept in written order.
    ///
    /// These are not offered to the player, so they are not in `acting` and never appear as a
    /// choice. They fire in `interaction_resolver::settle`, which is what "forced" means here.
    pub entries: Vec<Vec<&'a Binding>>,
}

pub fn slot(v: &[String], name: &str) -> usize {
    v.iter()
        .position(|x| x == name)
        .expect("resolved before search")
}

impl<'a> World<'a> {
    pub fn new(game: &'a Game) -> Self {
        let mut convs = Vec::new();
        for b in &game.bindings {
            if let Site::InConversation { conversation, .. } = &b.site {
                if !convs.contains(conversation) {
                    convs.push(conversation.clone());
                }
            }
        }

        // Which options are **tracked**: those some `chosen` predicate asks about, and no
        // others. The same rule the actor vector below is built on, applied a third time.
        //
        // This vector used to hold every option in the game, so taking one split the state
        // space whether or not anything could tell the difference. That is bookkeeping counted
        // as world state — the defect `scaling.rs` names for takeable hotspots and the rule
        // above names for actors — and it is worse here than either, because an unread
        // `chosen` bit is written by an option the player can take repeatedly, so it inflated
        // the **reachable set** and not only the declared space. `lighthouse.game` reported
        // 103 states and 193 transitions with three option bits of which one was ever read;
        // it is 91 and 173. `.yidam/decisions/conversation-reduction.yml`.
        let mut options: Vec<String> = Vec::new();
        fn asked_chosen(q: &Pred, into: &mut Vec<String>) {
            match q {
                Pred::Chosen(o) => {
                    if !into.contains(o) {
                        into.push(o.clone());
                    }
                }
                Pred::Not(x) => asked_chosen(x, into),
                Pred::And(x, y) | Pred::Or(x, y) => {
                    asked_chosen(x, into);
                    asked_chosen(y, into);
                }
                _ => {}
            }
        }
        for b in &game.bindings {
            for q in b.when.iter().chain(b.guards()) {
                asked_chosen(q, &mut options);
            }
        }
        let mut at = BTreeMap::new();
        for (i, r) in game.rooms.iter().enumerate() {
            at.insert(r.clone(), i);
        }
        // Which actors are **tracked**: those some predicate asks about, and those some effect
        // moves. Collected before anything else, because it decides the width of the state and
        // therefore the size of the search.
        //
        // Being *moved* has to be enough on its own, and that was not obvious. `offered` gates
        // an actor's site on the actor being in the room, so the site is itself a read — an
        // actor who is moved and never named in a predicate still has to be followed, or their
        // bindings would stay live in a room they have left. An actor nobody moves and nobody
        // asks about is absent from `State` entirely, which is every actor in the three
        // specimens written before this primitive existed.
        let mut actors: Vec<String> = Vec::new();
        fn asked(q: &Pred, into: &mut Vec<String>) {
            match q {
                Pred::At(a, _) => {
                    if !into.contains(a) {
                        into.push(a.clone());
                    }
                }
                Pred::Not(x) => asked(x, into),
                Pred::And(x, y) | Pred::Or(x, y) => {
                    asked(x, into);
                    asked(y, into);
                }
                _ => {}
            }
        }
        for b in &game.bindings {
            for q in b.when.iter().chain(b.guards()) {
                asked(q, &mut actors);
            }
            for e in &b.effects {
                if let Effect::Move(Some(a), _) = e {
                    if !actors.contains(a) {
                        actors.push(a.clone());
                    }
                }
            }
        }

        // Which flags, counters and items are **read**, and therefore which of them are state.
        //
        // `.yidam/decisions/observed-state.yml` states the rule — *"a component of `State` exists
        // only if some predicate in the game can read it"* — and narrowed three vectors to obey it,
        // one at a time, each after a figure came out wrong in the flattering direction. These
        // three were still built by enumerating declarations, which is the same defect in the one
        // place nobody had looked.
        //
        // An item is read where a predicate names it and also where a binary verb takes it as a
        // second object, because `offered` requires it carried. `Spend` is not a read: reducing a
        // count nothing consults changes nothing anybody can observe.
        let (mut rflags, mut rcounters, mut ritems) = (
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
        );
        fn reads(q: &Pred, f: &mut Vec<String>, c: &mut Vec<String>, i: &mut Vec<String>) {
            let note = |v: &mut Vec<String>, x: &String| {
                if !v.contains(x) {
                    v.push(x.clone());
                }
            };
            match q {
                Pred::Flag(x) => note(f, x),
                Pred::Cmp(x, _, _) => note(c, x),
                Pred::Carrying(x) | Pred::Count(x, _, _) => note(i, x),
                Pred::Not(a) => reads(a, f, c, i),
                Pred::And(a, b) | Pred::Or(a, b) => {
                    reads(a, f, c, i);
                    reads(b, f, c, i);
                }
                _ => {}
            }
        }
        for b in &game.bindings {
            for q in b.when.iter().chain(b.guards()) {
                reads(q, &mut rflags, &mut rcounters, &mut ritems);
            }
            if let Some(o) = &b.object {
                if !ritems.contains(o) {
                    ritems.push(o.clone());
                }
            }
        }

        let mut entries: Vec<Vec<&Binding>> = vec![Vec::new(); game.rooms.len()];
        for b in &game.bindings {
            if let Site::OnEnter { room } = &b.site {
                entries[at[room]].push(b);
            }
        }
        World {
            game,
            actors,
            entries,
            caps: game
                .items
                .iter()
                .filter(|i| ritems.contains(i))
                .map(|i| game.item_cap.get(i).copied().unwrap_or(1))
                .collect(),
            takeable: game.carried_form.keys().cloned().collect(),
            rooms: game.rooms.clone(),
            items: game
                .items
                .iter()
                .filter(|i| ritems.contains(i))
                .cloned()
                .collect(),
            flags: game
                .flags
                .iter()
                .map(|f| f.name.clone())
                .filter(|n| rflags.contains(n))
                .collect(),
            counters: game
                .counters
                .iter()
                .map(|c| c.name.clone())
                .filter(|n| rcounters.contains(n))
                .collect(),
            options,
            convs,
            at,
        }
    }

    pub fn initial(&self) -> State {
        State {
            room: self
                .game
                .start_room
                .as_ref()
                .map(|r| self.at[r])
                .unwrap_or(0),
            mode: None,
            items: vec![0; self.items.len()],
            taken: vec![false; self.takeable.len()],
            flags: vec![false; self.flags.len()],
            counters: self.game.counters.iter().map(|c| c.lo).collect(),
            chosen: vec![false; self.options.len()],
            actors: self
                .actors
                .iter()
                .map(|a| {
                    self.game
                        .actor_start
                        .get(a)
                        .map(|r| self.at[r])
                        .unwrap_or(0)
                })
                .collect(),
        }
    }

    pub fn holds(&self, q: &Pred, s: &State) -> bool {
        match q {
            Pred::Flag(x) => s.flags[slot(&self.flags, x)],
            Pred::At(a, r) => s.actors[slot(&self.actors, a)] == self.at[r],
            Pred::Carrying(x) => s.items[slot(&self.items, x)] >= 1,
            Pred::Count(x, c, v) => c.holds(s.items[slot(&self.items, x)], *v),
            Pred::Chosen(x) => s.chosen[slot(&self.options, x)],
            Pred::Cmp(x, c, v) => c.holds(s.counters[slot(&self.counters, x)], *v),
            Pred::Not(a) => !self.holds(a, s),
            Pred::And(a, b) => self.holds(a, s) && self.holds(b, s),
            Pred::Or(a, b) => self.holds(a, s) || self.holds(b, s),
        }
    }
}
