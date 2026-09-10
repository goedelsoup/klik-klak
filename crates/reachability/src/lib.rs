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
}

fn slot(v: &[String], name: &str) -> usize {
    v.iter()
        .position(|x| x == name)
        .expect("resolved before search")
}

impl<'a> Index<'a> {
    fn new(game: &'a Game) -> Self {
        let mut options = Vec::new();
        let mut convs = Vec::new();
        for b in &game.bindings {
            if let Site::InConversation {
                conversation,
                option,
            } = &b.site
            {
                if !convs.contains(conversation) {
                    convs.push(conversation.clone());
                }
                if !options.contains(option) {
                    options.push(option.clone());
                }
            }
        }
        let mut at = BTreeMap::new();
        for (i, r) in game.rooms.iter().enumerate() {
            at.insert(r.clone(), i);
        }
        Index {
            game,
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
        parts.join("; ")
    }

    fn holds(&self, q: &Pred, s: &State) -> bool {
        match q {
            Pred::Flag(x) => s.flags[slot(&self.flags, x)],
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
                // an acquisition site whose object has been taken is not there any more
                if let Some(i) = self.takeable.iter().position(|t| t == target) {
                    if s.taken[i] {
                        return false;
                    }
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
    fn apply_without_spending(&self, b: &Binding, s: &State) -> State {
        let mut n = self.apply(b, s);
        for e in &b.effects {
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
                let next = self.apply(b, &cur);
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
            n.chosen[slot(&self.options, option)] = true;
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
                Effect::Move(x) => n.room = self.at[x],
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
            !b.effects.is_empty() || b.ending || matches!(b.site, Site::InConversation { .. })
        })
        .collect();

    let mut id: HashMap<State, usize> = HashMap::new();
    let mut states: Vec<State> = Vec::new();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new(); // from, to, binding index in `acting`
    let mut goals: HashSet<usize> = HashSet::new();

    let start = ix.initial();
    id.insert(start.clone(), 0);
    states.push(start);
    let mut queue = VecDeque::from([0usize]);

    while let Some(cur) = queue.pop_front() {
        for (bi, b) in acting.iter().enumerate() {
            if !ix.offered(b, &states[cur]) {
                continue;
            }
            let next = ix.apply(b, &states[cur]);
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
            let spent: Vec<&str> = b
                .effects
                .iter()
                .filter_map(|e| match e {
                    Effect::Spend(x, _) => Some(x.as_str()),
                    _ => None,
                })
                .collect();
            // Which cause, asked as a counterfactual rather than read off the effect list.
            // A binding that both spends and moves carries both kinds of effect, and reading
            // the cause off the list picks one by fiat — the ferry specimen's toll spends
            // three coins AND crosses a river, and the coins are not why the player is
            // stranded. So take the spending away: if that rescues the state, the spending
            // was the cause; if it is still stranded, something else was.
            let source = &states[example_from[&bi]];
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

    // the product of the ranges each variable was observed to take
    let mut observed: u128 = 1;
    let mut span = |vals: Vec<i64>| {
        let lo = vals.iter().copied().min().unwrap_or(0);
        let hi = vals.iter().copied().max().unwrap_or(0);
        observed = observed.saturating_mul((hi - lo + 1).max(1) as u128);
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
    for i in 0..ix.takeable.len() {
        span(states.iter().map(|s| s.taken[i] as i64).collect());
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
        if self.observed_space > 0 {
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
