//! Whether every state a player can reach can still reach the ending.
//!
//! This is `.yidam/skills/reachability.md`, implemented. It is
//! `failure-mode/unwinnable-state` detected mechanically rather than by walking into it, and
//! it is only possible because the authoring language is bounded: every precondition is a
//! finite predicate over declared names, so the state space is enumerable and the question
//! is decidable. A game with a general-purpose escape hatch is not a slower input here — it
//! is not an input at all.
//!
//! What it computes, in the terms that skill asked for:
//!
//! * the reachable set,
//! * the stranded set — reachable, and unable to reach the ending,
//! * and for each stranded set the binding that stranded it, classified as **consumption**
//!   or **ordering**, which `unwinnable-state` names as its two causes.
//!
//! It also checks a claim the corpus makes about itself: `world-flag/latched-flag` says a
//! latching flag never goes back, and [`Report::latch_violations`] is that declaration held
//! against what the game actually does.

use game_format::{Binding, Effect, Game, Pred, Site};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// an object needed later is spent earlier
    Consumption,
    /// a gate closes with something still needed behind it
    Ordering,
}

#[derive(Debug, Clone)]
pub struct Stranding {
    pub line: usize,
    pub verb: String,
    pub object: Option<String>,
    pub target: String,
    pub cause: Cause,
    pub detail: String,
    /// how many reachable states this binding strands
    pub states: usize,
    /// one stranded state, so the report says what was lost rather than only how much
    pub example: String,
}

#[derive(Debug, Default)]
pub struct Report {
    pub states: usize,
    pub transitions: usize,
    pub winnable: bool,
    pub stranded: usize,
    pub strandings: Vec<Stranding>,
    pub latch_violations: Vec<String>,
    /// per counter, whether it only ever rises across a reachable transition
    pub monotone_counters: Vec<(String, bool)>,
    /// Where a game handed out more of an item than it declared room for.
    ///
    /// Found by a mutation test: clearing a latch that guarded an acquisition source made
    /// that source unbounded, the item count rose forever, and the search did not stop. The
    /// declared ceiling closes the space; this reports where the game pressed against it,
    /// because a clamp that says nothing is a wrong answer given confidently.
    pub overflows: Vec<String>,
    /// How many combinations exist of the values these variables were *observed* to take.
    ///
    /// `world-flag/latched-flag` argues that flags moving one way make the reachable space a
    /// lattice rather than a graph. This is the measurable form of that: the gap between
    /// this number and [`Report::states`] is how much structure the game has. It is an
    /// observed bound rather than a declared one — an item's quantity has no declared
    /// ceiling — so it says how far the reachable set collapses the space actually used, and
    /// not how far it collapses every space the declarations permit.
    pub observed_space: u128,
    /// Whether [`Report::observed_space`] overflowed and is a ceiling rather than a count.
    ///
    /// A generated corridor of eighty rooms reached it, and the report went on printing a
    /// percentage computed from `u128::MAX` as though it had measured something. A number
    /// that cannot be produced must not be presented as one.
    pub space_saturated: bool,
    /// Rooms whose entry bindings move the player in a loop that never settles.
    ///
    /// An `on-enter` binding is forced, so a state in which one is offered is transient — the
    /// player is never left there. Two rooms that each throw the player into the other are
    /// therefore a game with no resting state at all, and a search that chases it does not
    /// stop. This is the same shape as an item with no ceiling, and it is reported for the
    /// same reason: a checker that hangs is worse than one that says what it found.
    pub entry_cycles: Vec<String>,
}

struct Index<'a> {
    game: &'a Game,
    rooms: Vec<String>,
    items: Vec<String>,
    flags: Vec<String>,
    counters: Vec<String>,
    options: Vec<String>,
    convs: Vec<String>,
    caps: Vec<i64>,
    /// hotspots that stand for an item, in declaration order
    takeable: Vec<String>,
    at: BTreeMap<String, usize>,
    /// actors some predicate names, in declaration order. See [`State::actors`].
    actors: Vec<String>,
    /// bindings sited at `on-enter`, indexed by room slot and kept in written order.
    ///
    /// These are not offered to the player, so they are not in `acting` and never appear as a
    /// choice. They fire in [`Index::settle`], which is what "forced" means here.
    entries: Vec<Vec<&'a Binding>>,
}

fn slot(v: &[String], name: &str) -> usize {
    v.iter()
        .position(|x| x == name)
        .expect("resolved before search")
}

impl<'a> Index<'a> {
    fn new(game: &'a Game) -> Self {
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
            for q in b.when.iter().chain(b.guards.iter()) {
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
            for q in b.when.iter().chain(b.guards.iter()) {
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

        let mut entries: Vec<Vec<&Binding>> = vec![Vec::new(); game.rooms.len()];
        for b in &game.bindings {
            if let Site::OnEnter { room } = &b.site {
                entries[at[room]].push(b);
            }
        }
        Index {
            game,
            actors,
            entries,
            caps: game
                .items
                .iter()
                .map(|i| game.item_cap.get(i).copied().unwrap_or(1))
                .collect(),
            takeable: game.carried_form.keys().cloned().collect(),
            rooms: game.rooms.clone(),
            items: game.items.clone(),
            flags: game.flags.iter().map(|f| f.name.clone()).collect(),
            counters: game.counters.iter().map(|c| c.name.clone()).collect(),
            options,
            convs,
            at,
        }
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
    fn settle(&self, cur: State) -> Result<State, usize> {
        self.settle_tracking(cur).map(|(s, _)| s)
    }

    /// [`Index::settle`], also reporting which forced bindings fired.
    ///
    /// The list is what [`Index::apply_without_spending`] needs and could not get from the
    /// binding it was handed: a chosen binding's effect list is no longer everything that
    /// happens when it fires.
    fn settle_tracking(&self, mut cur: State) -> Result<(State, Vec<&'a Binding>), usize> {
        let mut seen: HashSet<State> = HashSet::new();
        let mut fired: Vec<&'a Binding> = Vec::new();
        loop {
            if !seen.insert(cur.clone()) {
                return Err(cur.room);
            }
            let before = cur.room;
            for b in &self.entries[cur.room] {
                if self.offered(b, &cur) {
                    cur = self.apply(b, &cur);
                    fired.push(b);
                }
            }
            if cur.room == before {
                return Ok((cur, fired));
            }
        }
    }

    fn initial(&self) -> State {
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

    /// A stranded state, in the terms an author wrote the game in.
    fn describe(&self, s: &State) -> String {
        let mut parts = Vec::new();
        parts.push(format!("in {}", self.rooms[s.room]));
        let carried: Vec<String> = self
            .items
            .iter()
            .enumerate()
            .filter(|(i, _)| s.items[*i] > 0)
            .map(|(i, n)| {
                if s.items[i] == 1 {
                    n.clone()
                } else {
                    format!("{n}×{}", s.items[i])
                }
            })
            .collect();
        parts.push(if carried.is_empty() {
            "carrying nothing".to_string()
        } else {
            format!("carrying {}", carried.join(", "))
        });
        for (i, c) in self.counters.iter().enumerate() {
            parts.push(format!("{c}={}", s.counters[i]));
        }
        for (i, a) in self.actors.iter().enumerate() {
            parts.push(format!("{a} in {}", self.rooms[s.actors[i]]));
        }
        parts.join("; ")
    }

    fn holds(&self, q: &Pred, s: &State) -> bool {
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

    /// Is this binding offered in this state? Site and mode first, then the two structural
    /// preconditions `interaction/binary-binding` says the format enforces, then the
    /// authored one.
    fn offered(&self, b: &Binding, s: &State) -> bool {
        match (&b.site, s.mode) {
            (Site::InConversation { conversation, .. }, Some(m)) => {
                if slot(&self.convs, conversation) != m {
                    return false;
                }
            }
            (Site::InRoom { room, target }, None) => {
                if self.at[room] != s.room {
                    return false;
                }
                // an actor who has walked off is not there to be addressed. `actor <name>`
                // inside a room says where you may talk to them; `holds` and `move ... to`
                // say where they *are*, and the site is only live when the two agree. This is
                // the same shape as the acquisition check below it — a site whose subject has
                // gone is not a site.
                if let Some(i) = self.actors.iter().position(|a| a == target) {
                    if s.actors[i] != s.room {
                        return false;
                    }
                }
                // an acquisition site whose object has been taken is not there any more
                if let Some(i) = self.takeable.iter().position(|t| t == target) {
                    if s.taken[i] {
                        return false;
                    }
                }
            }
            // An entry binding is offered by being in its room and nowhere else. The `None`
            // is doing work: a forced binding must not fire while the player is inside a
            // conversation, or the mode would be left by something the player never chose.
            (Site::OnEnter { room }, None) => {
                if self.at[room] != s.room {
                    return false;
                }
            }
            _ => return false,
        }
        if let Some(o) = &b.object {
            if s.items[slot(&self.items, o)] < 1 {
                return false;
            }
        }
        b.when.as_ref().map(|q| self.holds(q, s)).unwrap_or(true)
    }

    /// The same transition with its spends left out. Used to ask whether the spending is
    /// what stranded a state, rather than guessing from which effects the binding carries.
    /// Everything this binding spends when fired from `s` — **including what the bindings it
    /// forces spend.** A chosen binding's own effect list stopped being the whole answer when
    /// `on-enter` arrived: walking through a door spends nothing, and the watchman on the
    /// other side of it takes the ledger.
    fn spends(&self, b: &Binding, s: &State) -> Vec<String> {
        let after = self.apply(b, s);
        let fired = self
            .settle_tracking(after)
            .map(|(_, f)| f)
            .unwrap_or_default();
        let mut out: Vec<String> = Vec::new();
        for e in b
            .effects
            .iter()
            .chain(fired.iter().flat_map(|f| &f.effects))
        {
            if let Effect::Spend(x, _) = e {
                if !out.contains(x) {
                    out.push(x.clone());
                }
            }
        }
        out
    }

    fn apply_without_spending(&self, b: &Binding, s: &State) -> State {
        let after = self.apply(b, s);
        // The forced bindings run, as they would have — and what *they* spend counts as this
        // binding's spending for the purpose of the counterfactual. The player chose to walk
        // through the door; the watchman taking the ledger is a consequence of that choice
        // and is nowhere in the door's effect list.
        //
        // This is the third time a new primitive has broken the cause classifier, and the
        // third time in the same place: it assumed everything a binding does is written in
        // that binding.
        let (mut n, fired) = self
            .settle_tracking(after.clone())
            .unwrap_or((after, Vec::new()));
        for e in b
            .effects
            .iter()
            .chain(fired.iter().flat_map(|f| &f.effects))
        {
            if let Effect::Spend(x, _) = e {
                n.items[slot(&self.items, x)] = s.items[slot(&self.items, x)];
            }
        }
        n
    }

    /// Can an ending still be reached from here? A forward search from an arbitrary state,
    /// which is what makes the counterfactual above answerable: `winning` is defined only
    /// over states a player can actually get to, and a counterfactual state is one they
    /// cannot.
    fn can_finish(&self, acting: &[&Binding], from: &State) -> bool {
        let mut seen: HashSet<State> = HashSet::from([from.clone()]);
        let mut q: VecDeque<State> = VecDeque::from([from.clone()]);
        while let Some(cur) = q.pop_front() {
            for b in acting {
                if !self.offered(b, &cur) {
                    continue;
                }
                if b.ending {
                    return true;
                }
                // settled, like the real search — a counterfactual that rests where the game
                // never would is not a counterfactual about this game
                let Ok(next) = self.settle(self.apply(b, &cur)) else {
                    continue;
                };
                if seen.insert(next.clone()) {
                    q.push_back(next);
                }
            }
        }
        false
    }

    fn apply(&self, b: &Binding, s: &State) -> State {
        let mut n = s.clone();
        if let Site::InConversation { option, .. } = &b.site {
            // Unobserved: nothing asks whether this option was taken, so recording it would
            // split states no predicate can tell apart.
            if let Some(i) = self.options.iter().position(|o| o == option) {
                n.chosen[i] = true;
            }
        }
        for e in &b.effects {
            match e {
                Effect::SetFlag(x) => n.flags[slot(&self.flags, x)] = true,
                Effect::ClearFlag(x) => n.flags[slot(&self.flags, x)] = false,
                Effect::SetCounter(x, v) => n.counters[slot(&self.counters, x)] = *v,
                Effect::Give(x, q) => {
                    let i = slot(&self.items, x);
                    n.items[i] = (n.items[i] + q).min(self.caps[i]);
                }
                Effect::Spend(x, q) => {
                    let i = slot(&self.items, x);
                    n.items[i] = (n.items[i] - q).max(0);
                }
                Effect::Move(None, x) => n.room = self.at[x],
                Effect::Move(Some(a), x) => {
                    // Unobserved: nothing reads it, so writing it would split states that are
                    // not distinguishable. See `State::actors`.
                    if let Some(i) = self.actors.iter().position(|n| n == a) {
                        n.actors[i] = self.at[x];
                    }
                }
                Effect::Enter(x) => n.mode = Some(slot(&self.convs, x)),
                Effect::Leave => n.mode = None,
            }
        }
        // taking the object is the room losing it
        if let Site::InRoom { target, .. } = &b.site {
            if let Some(i) = self.takeable.iter().position(|t| t == target) {
                let stands_for = &self.game.carried_form[target];
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
}

pub fn analyse(game: &Game) -> Report {
    let ix = Index::new(game);
    // Effect-free bindings are responses: they say something and change nothing, so they
    // cannot move the search and are left out of it.
    //
    // `b.ending` is the exception and was missing until a second specimen found it. A
    // binding that ends the game need write nothing — "you open the gate and walk out" is a
    // complete ending — and filtering it left the search with no goal at all, so the whole
    // game reported as unreachable. The first specimen hid this: its ending also set a
    // counter and spent an item, so it survived the filter for reasons that had nothing to
    // do with being an ending.
    let acting: Vec<&Binding> = game
        .bindings
        .iter()
        .filter(|b| {
            // An entry binding is forced, not offered. It fires in `settle` and must never
            // appear as something the player chose to do — including in a stranding report,
            // where naming it as the cause would be true and useless: nobody picked it.
            !matches!(b.site, Site::OnEnter { .. })
                && (!b.effects.is_empty()
                    || b.ending
                    || matches!(b.site, Site::InConversation { .. }))
        })
        .collect();

    let mut id: HashMap<State, usize> = HashMap::new();
    let mut states: Vec<State> = Vec::new();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new(); // from, to, binding index in `acting`
    let mut goals: HashSet<usize> = HashSet::new();

    let mut entry_cycles: Vec<String> = Vec::new();
    let note_cycle = |room: usize, ix: &Index, out: &mut Vec<String>| {
        let msg = format!(
            "entering {} starts a chain of `on-enter` bindings that never settles",
            ix.rooms[room]
        );
        if !out.contains(&msg) {
            out.push(msg);
        }
    };

    // The start room is arrived in too, which is how both AGS eras document it.
    let start = match ix.settle(ix.initial()) {
        Ok(s) => s,
        Err(room) => {
            note_cycle(room, &ix, &mut entry_cycles);
            ix.initial()
        }
    };
    id.insert(start.clone(), 0);
    states.push(start);
    let mut queue = VecDeque::from([0usize]);

    while let Some(cur) = queue.pop_front() {
        for (bi, b) in acting.iter().enumerate() {
            if !ix.offered(b, &states[cur]) {
                continue;
            }
            let next = match ix.settle(ix.apply(b, &states[cur])) {
                Ok(n) => n,
                Err(room) => {
                    note_cycle(room, &ix, &mut entry_cycles);
                    continue;
                }
            };
            let fresh = !id.contains_key(&next);
            let to = match id.get(&next) {
                Some(i) => *i,
                None => {
                    let i = states.len();
                    id.insert(next.clone(), i);
                    states.push(next);
                    i
                }
            };
            // An ending ends the game. Expanding past one asks whether the player can reach
            // the ending a SECOND time, which nothing can, so every post-ending state reads
            // as stranded and every binding offered there reads as a stranding. The first
            // run of this calculator reported exactly that, on `pick-up loose-match`.
            if b.ending {
                goals.insert(to);
            } else if fresh {
                queue.push_back(to);
            }
            edges.push((cur, to, bi));
        }
    }

    // states from which a goal is still reachable, by reverse search from the goals
    let mut back: HashMap<usize, Vec<usize>> = HashMap::new();
    for (from, to, _) in &edges {
        back.entry(*to).or_default().push(*from);
    }
    let mut winning: HashSet<usize> = goals.clone();
    let mut q: VecDeque<usize> = goals.iter().copied().collect();
    while let Some(cur) = q.pop_front() {
        for prev in back.get(&cur).into_iter().flatten() {
            if winning.insert(*prev) {
                q.push_back(*prev);
            }
        }
    }

    // a stranding is an edge stepping out of the winning set
    let mut per_binding: BTreeMap<usize, usize> = BTreeMap::new();
    let mut example_of: BTreeMap<usize, usize> = BTreeMap::new();
    let mut example_from: BTreeMap<usize, usize> = BTreeMap::new();
    for (from, to, bi) in &edges {
        if winning.contains(from) && !winning.contains(to) {
            *per_binding.entry(*bi).or_insert(0) += 1;
            example_of.entry(*bi).or_insert(*to);
            example_from.entry(*bi).or_insert(*from);
        }
    }
    let mut strandings: Vec<Stranding> = per_binding
        .into_iter()
        .map(|(bi, n)| {
            let b = acting[bi];
            let source = &states[example_from[&bi]];
            let spent = ix.spends(b, source);
            // Which cause, asked as a counterfactual rather than read off the effect list.
            // A binding that both spends and moves carries both kinds of effect, and reading
            // the cause off the list picks one by fiat — the ferry specimen's toll spends
            // three coins AND crosses a river, and the coins are not why the player is
            // stranded. So take the spending away: if that rescues the state, the spending
            // was the cause; if it is still stranded, something else was.
            let rescued =
                !spent.is_empty() && ix.can_finish(&acting, &ix.apply_without_spending(b, source));
            let (cause, detail) = if rescued {
                (
                    Cause::Consumption,
                    format!("spends {}, which is needed later", spent.join(", ")),
                )
            } else {
                (
                    Cause::Ordering,
                    "a gate closed with something still needed behind it".to_string(),
                )
            };
            let target = match &b.site {
                Site::InRoom { target, .. } => target.clone(),
                Site::InConversation { option, .. } => option.clone(),
                Site::OnEnter { room } => room.clone(),
            };
            let example = example_of
                .get(&bi)
                .map(|i| ix.describe(&states[*i]))
                .unwrap_or_default();
            Stranding {
                line: b.line,
                verb: b.verb.clone(),
                object: b.object.clone(),
                target,
                cause,
                detail,
                states: n,
                example,
            }
        })
        .collect();
    strandings.sort_by_key(|s| s.line);

    // the corpus claims a latching flag never goes back; hold it against the game
    let mut latch_violations = Vec::new();
    for f in &game.flags {
        if !f.latched {
            continue;
        }
        for (_, _, bi) in &edges {
            let b = acting[*bi];
            if b.effects
                .iter()
                .any(|e| matches!(e, Effect::ClearFlag(x) if *x == f.name))
            {
                latch_violations.push(format!(
                    "`{}` is declared latched and line {} clears it",
                    f.name, b.line
                ));
                break;
            }
        }
    }
    latch_violations.sort();
    latch_violations.dedup();

    // The product of the ranges each variable was observed to take — over the variables an
    // author *declared*, which is not the same as the fields of `State`.
    //
    // `taken` is excluded and its exclusion is the whole reason this comment exists. It is one
    // bit per acquisition site, recording that the site has fired, and it is determined by the
    // item count: it is bookkeeping this checker keeps, not world state anybody wrote down.
    // Counting it doubled the space per takeable object and doubled nothing in the reachable
    // set, so it made every game look more collapsed than it is. A generated family of
    // n-token games made this unmissable: 2^n reachable states against a reported 4^n space,
    // a "collapse" of 2^-n that was entirely this redundancy.
    let mut observed: u128 = 1;
    let mut saturated = false;
    let mut span = |vals: Vec<i64>| {
        let lo = vals.iter().copied().min().unwrap_or(0);
        let hi = vals.iter().copied().max().unwrap_or(0);
        let factor = (hi - lo + 1).max(1) as u128;
        match observed.checked_mul(factor) {
            Some(v) => observed = v,
            None => {
                saturated = true;
                observed = u128::MAX;
            }
        }
    };
    span(states.iter().map(|s| s.room as i64).collect());
    span(
        states
            .iter()
            .map(|s| s.mode.map(|m| m as i64 + 1).unwrap_or(0))
            .collect(),
    );
    for i in 0..ix.items.len() {
        span(states.iter().map(|s| s.items[i]).collect());
    }
    for i in 0..ix.flags.len() {
        span(states.iter().map(|s| s.flags[i] as i64).collect());
    }
    for i in 0..ix.counters.len() {
        span(states.iter().map(|s| s.counters[i]).collect());
    }
    for i in 0..ix.options.len() {
        span(states.iter().map(|s| s.chosen[i] as i64).collect());
    }
    // An observed actor's room is a variable the author declared — `holds` gave it a value and
    // a predicate reads it — so it belongs in the product for the same reason the player's room
    // does, and an unobserved one is absent from `State` and so cannot be counted by mistake.
    for i in 0..ix.actors.len() {
        span(states.iter().map(|s| s.actors[i] as i64).collect());
    }

    // where the game handed out more than it declared room for
    let mut overflows: Vec<String> = Vec::new();
    for (from, _, bi) in &edges {
        let b = acting[*bi];
        for e in &b.effects {
            if let Effect::Give(x, q) = e {
                let i = slot(&ix.items, x);
                if states[*from].items[i] + q > ix.caps[i] {
                    overflows.push(format!(
                        "line {}: gives {q} {x} where {} is already carried and {} is the declared ceiling",
                        b.line, states[*from].items[i], ix.caps[i]
                    ));
                }
            }
        }
    }
    overflows.sort();
    overflows.dedup();

    let mut monotone_counters = Vec::new();
    for (ci, c) in game.counters.iter().enumerate() {
        let rises_only = edges
            .iter()
            .all(|(from, to, _)| states[*to].counters[ci] >= states[*from].counters[ci]);
        monotone_counters.push((c.name.clone(), rises_only));
    }

    Report {
        entry_cycles,
        space_saturated: saturated,
        states: states.len(),
        transitions: edges.len(),
        winnable: !goals.is_empty(),
        stranded: states.len() - winning.len(),
        strandings,
        latch_violations,
        monotone_counters,
        overflows,
        observed_space: observed,
    }
}

impl Report {
    pub fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "{} reachable state(s), {} transition(s).\n",
            self.states, self.transitions
        ));
        if self.space_saturated {
            s.push_str(
                "The declared variables have more combinations than can be counted here, so no \
                 fraction is reported.\n",
            );
        } else if self.observed_space > 0 {
            let pct = self.states as f64 * 100.0 / self.observed_space as f64;
            s.push_str(&format!(
                "{} of {} combinations of the values these variables take ({:.3}%).\n",
                self.states, self.observed_space, pct
            ));
        }
        s.push_str(if self.winnable {
            "The ending is reachable.\n"
        } else {
            "THE ENDING IS NOT REACHABLE FROM THE START.\n"
        });
        if self.stranded == 0 {
            s.push_str("No reachable state is stranded.\n");
        } else {
            s.push_str(&format!(
                "\n{} reachable state(s) can no longer reach the ending.\nStranded by:\n",
                self.stranded
            ));
            for st in &self.strandings {
                let cause = match st.cause {
                    Cause::Consumption => "consumption",
                    Cause::Ordering => "ordering",
                };
                let obj = st
                    .object
                    .as_deref()
                    .map(|o| format!(" {o}"))
                    .unwrap_or_default();
                s.push_str(&format!(
                    "  line {:>4}  {}{} on {}  [{}]\n            {} — {} state(s)\n            e.g. {}\n",
                    st.line, st.verb, obj, st.target, cause, st.detail, st.states, st.example
                ));
            }
        }
        if !self.entry_cycles.is_empty() {
            s.push_str("\nForced entry bindings that never settle:\n");
            for v in &self.entry_cycles {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.overflows.is_empty() {
            s.push_str("\nMore of an item given than the declaration has room for:\n");
            for v in &self.overflows {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.latch_violations.is_empty() {
            s.push_str("\nLatch declarations contradicted by the game:\n");
            for v in &self.latch_violations {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.monotone_counters.is_empty() {
            s.push_str("\nCounters, measured over the reachable graph:\n");
            for (name, rises) in &self.monotone_counters {
                s.push_str(&format!(
                    "  {name}: {}\n",
                    if *rises {
                        "only ever rises — a partial order, not a search"
                    } else {
                        "falls somewhere"
                    }
                ));
            }
        }
        s
    }
}
