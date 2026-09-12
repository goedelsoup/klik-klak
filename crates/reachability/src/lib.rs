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

//! **The evaluator is not here.** `Ix::in_menu` and `Ix::live` are `engine/verb-menu`; `offered`,
//! `apply`, `spends` and `settle` are `engine/interaction-resolver`; the world and its state are
//! `game_format::World` and `game_format::State`. This crate depends on the engine rather than
//! carrying a second copy of the language's semantics, because a checker with its own evaluator
//! is two systems that can disagree. `.yidam/decisions/two-components.yml`.
//!
//! What is left is the **search**: a breadth-first walk of the state graph the engine defines,
//! and the three functions that serve only it — [`describe`], [`observable`] and [`can_finish`].

pub use game_format::State;
use game_format::{Binding, Effect, Game, Site, World};
use interaction_resolver as resolver;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

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
    /// Flags the game only ever sets, which are not declared `latched`.
    ///
    /// The mirror of [`Report::latch_violations`], and it exists because the default is the
    /// wrong way round. `world-flag/latched-flag` weighed eight flag names from a blog post and
    /// declined to invert the schema on them; a shipped game's source says **32 of 41** boolean
    /// state slots move one way. An author writing the common case out by hand will sometimes
    /// not, and this is the one direction the checker could not previously see: a declaration
    /// that is true and absent costs nothing to make and buys the guarantee.
    pub unclaimed_latches: Vec<String>,
    /// Distinct `(target, verb)` entries a contextual menu would ever show, and how many of
    /// them are already showing before the player has done anything.
    ///
    /// The gap between them is what the interface tells the player over the course of the game
    /// without being asked — `default-binding` names that cost and had no number for it: *"what
    /// the player learns: what the object affords, before trying"*.
    pub menu_entries: (usize, usize),
    /// Every `(reachable state, target, verb)` the menu holds — the size of the interface summed
    /// over the whole game, where [`Report::menu_entries`] counts each entry once however long it
    /// is up.
    ///
    /// This is the one an author's choice between withholding and refusing moves: both forms show
    /// the entry *somewhere*, and what an `else` buys is the states where the guard fails.
    pub menu_slots: usize,
    /// Bindings in no menu in any reachable state: authored, and never shown to anybody.
    pub unshown: Vec<String>,
    /// Bindings the menu shows in a reachable state which decline there and say nothing.
    ///
    /// **`subsystem/interaction-resolver`'s one guarantee, held against a game.** *"Never
    /// nothing — a silent miss is the failure players read as brokenness, and it is entirely
    /// preventable at this boundary."* A `when` with an `else` under it is offered and declines;
    /// if no clause of the `else` list covers the state it was declined in, the player clicks and
    /// is told nothing.
    ///
    /// This could not be looked for until the parse tree kept response text
    /// (`.yidam/decisions/authored-text.yml`) or asked until the resolver was a component that
    /// answered in outcomes (`.yidam/decisions/two-components.yml`). It is narrower than *fired
    /// and said nothing*, which is not a miss at all: walking through a door says nothing and the
    /// movement is the response. `.yidam/decisions/playable.yml`.
    pub silent: Vec<String>,
    /// Flags, counters and items the game declares and no predicate reads.
    ///
    /// `.yidam/decisions/observed-state.yml` asked whether its rule *"wants enforcing rather than
    /// observing"*. It wants both, and which one depends on who wrote the state. Where the
    /// **instrument** invented a component — takeable hotspots, actors, conversation options — the
    /// fix is to stop inventing it, and that has been done three times. Where the **author**
    /// declared one nothing reads, narrowing it silently would hide an authoring mistake in the
    /// same breath as correcting a figure, so it is narrowed *and* named.
    pub unread: Vec<String>,
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
    /// Authored lines no reachable state ever selects — `failure-mode/orphaned-dialogue-branch`,
    /// counted.
    ///
    /// That node says the mode *"belongs to `branching-tree` specifically rather than to
    /// conversations generally. A topic list has no paths, so it cannot orphan anything"*, and
    /// says of shipped games that this is *"why nobody counts it"*. Both halves are addressed
    /// here: the guarded response list orphans without any path to orphan it, and this is the
    /// count.
    ///
    /// It could not be asked until the parse tree kept the lines
    /// (`.yidam/decisions/authored-text.yml`), and not answered correctly until a forced
    /// binding's line was selected in the state it fired from
    /// (`.yidam/decisions/walked-in.yml`) — an `on-enter` line is read between two resting
    /// states and a census taken over the resting states alone would call every one of them
    /// unreachable.
    pub orphaned: Vec<String>,
    /// Rooms whose entry bindings move the player in a loop that never settles.
    ///
    /// An `on-enter` binding is forced, so a state in which one is offered is transient — the
    /// player is never left there. Two rooms that each throw the player into the other are
    /// therefore a game with no resting state at all, and a search that chases it does not
    /// stop. This is the same shape as an item with no ceiling, and it is reported for the
    /// same reason: a checker that hangs is worse than one that says what it found.
    pub entry_cycles: Vec<String>,
}

/// A stranded state, in the terms an author wrote the game in.
fn describe(w: &World, s: &State) -> String {
    let mut parts = Vec::new();
    parts.push(format!("in {}", w.rooms[s.room]));
    let carried: Vec<String> = w
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
    for (i, c) in w.counters.iter().enumerate() {
        parts.push(format!("{c}={}", s.counters[i]));
    }
    for (i, a) in w.actors.iter().enumerate() {
        parts.push(format!("{a} in {}", w.rooms[s.actors[i]]));
    }
    parts.join("; ")
}

/// **Does firing this binding change anything the game can tell apart?**
///
/// The same rule as [`Ix::in_menu`]'s neighbours one layer down. A binding whose every effect
/// writes a slot no predicate reads moves the search nowhere, and counting it as a transition
/// is the defect `.yidam/decisions/observed-state.yml` names arriving in a fourth place — this
/// time in `transitions`, which is a published figure.
///
/// A `give` at an acquisition site is observable however unread the item is, because taking the
/// object is the room losing it and `live` reads that.
fn observable(w: &World, b: &Binding) -> bool {
    b.effects.iter().any(|e| match e {
        Effect::Move(..) | Effect::Enter(_) | Effect::Leave => true,
        Effect::SetFlag(x) | Effect::ClearFlag(x) => w.flags.contains(x),
        Effect::SetCounter(x, _) => w.counters.contains(x),
        Effect::Spend(x, _) => w.items.contains(x),
        Effect::Give(x, _) => {
            w.items.contains(x)
                || matches!(&b.site, Site::InRoom { target, .. }
                    if w.game.carried_form.get(target) == Some(x))
        }
    })
}

/// Can an ending still be reached from here? A forward search from an arbitrary state,
/// which is what makes the counterfactual above answerable: `winning` is defined only
/// over states a player can actually get to, and a counterfactual state is one they
/// cannot.
fn can_finish(w: &World, acting: &[&Binding], from: &State) -> bool {
    let mut seen: HashSet<State> = HashSet::from([from.clone()]);
    let mut q: VecDeque<State> = VecDeque::from([from.clone()]);
    while let Some(cur) = q.pop_front() {
        for b in acting {
            if !resolver::offered(w, b, &cur) {
                continue;
            }
            if b.ending {
                return true;
            }
            // settled, like the real search — a counterfactual that rests where the game
            // never would is not a counterfactual about this game
            let Ok(next) = resolver::settle(w, resolver::apply(w, b, &cur)) else {
                continue;
            };
            if seen.insert(next.clone()) {
                q.push_back(next);
            }
        }
    }
    false
}

pub fn analyse(game: &Game) -> Report {
    let ix = World::new(game);
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
                && (observable(&ix, b) || b.ending || matches!(b.site, Site::InConversation { .. }))
        })
        .collect();

    let mut id: HashMap<State, usize> = HashMap::new();
    let mut states: Vec<State> = Vec::new();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new(); // from, to, binding index in `acting`
    let mut goals: HashSet<usize> = HashSet::new();

    let mut entry_cycles: Vec<String> = Vec::new();
    let note_cycle = |room: usize, ix: &World, out: &mut Vec<String>| {
        let msg = format!(
            "entering {} starts a chain of `on-enter` bindings that never settles",
            ix.rooms[room]
        );
        if !out.contains(&msg) {
            out.push(msg);
        }
    };

    // Lines selected in some reachable state. A forced binding's line is selected in a state
    // the search never stores — the chain runs between two resting states — so it is collected
    // here, where `settle_tracking` can still see it, rather than in the loop below.
    let mut read: BTreeSet<usize> = BTreeSet::new();

    // The start room is arrived in too, which is how both AGS eras document it.
    let start = match resolver::settle_tracking(&ix, ix.initial()) {
        Ok((s, fired)) => {
            read.extend(fired.iter().filter_map(|f| f.says.map(|r| r.line)));
            s
        }
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
            if !resolver::offered(&ix, b, &states[cur]) {
                continue;
            }
            let next = match resolver::settle_tracking(&ix, resolver::apply(&ix, b, &states[cur])) {
                Ok((n, fired)) => {
                    read.extend(fired.iter().filter_map(|f| f.says.map(|r| r.line)));
                    n
                }
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
            let spent = resolver::spends(&ix, b, source);
            // Which cause, asked as a counterfactual rather than read off the effect list.
            // A binding that both spends and moves carries both kinds of effect, and reading
            // the cause off the list picks one by fiat — the ferry specimen's toll spends
            // three coins AND crosses a river, and the coins are not why the player is
            // stranded. So take the spending away: if that rescues the state, the spending
            // was the cause; if it is still stranded, something else was.
            let rescued = !spent.is_empty()
                && can_finish(
                    &ix,
                    &acting,
                    &resolver::apply_without_spending(&ix, b, source),
                );
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
                .map(|i| describe(&ix, &states[*i]))
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

    // **The interface this design never declared, held against the game.**
    //
    // `.yidam/decisions/tier-split.yml` recorded that four questions have resolved at a third
    // answer site the ontology has no tier for, and that this design has never specified one.
    // It turns out to have specified one all along, in pieces, none of which was about a
    // player: see `Ix::in_menu`. This is what falls out of reading them together.
    //
    // The sweep is over **every** binding and not over `acting`, which is the first thing the
    // menu shows that the search cannot: 42 of the four specimens' 91 bindings are `look-at`
    // with no effects, so they are filtered out of the transition graph before it is built and
    // are the largest single thing a player is shown.
    // What the author declared and nothing can read. The vectors are already narrowed to the
    // read ones in `Index::new`, so this is the difference between the declaration and the state.
    let mut unread: Vec<String> = Vec::new();
    for (kind, declared, kept) in [
        (
            "flag",
            game.flags
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<_>>(),
            &ix.flags,
        ),
        (
            "counter",
            game.counters.iter().map(|c| c.name.clone()).collect(),
            &ix.counters,
        ),
        ("item", game.items.clone(), &ix.items),
    ] {
        for d in declared.iter().filter(|d| !kept.contains(d)) {
            unread.push(format!(
                "`{kind} {d}` is declared and no predicate reads it, so writing it changes \
                 nothing the game can tell apart"
            ));
        }
    }
    unread.sort();

    let mut shown_ever: BTreeSet<(String, String)> = BTreeSet::new();
    let mut shown_at_start: BTreeSet<(String, String)> = BTreeSet::new();
    let mut unshown: Vec<String> = Vec::new();
    let mut silent: Vec<String> = Vec::new();
    let mut menu_slots = 0usize;
    for b in &game.bindings {
        if matches!(b.site, Site::OnEnter { .. }) {
            continue;
        }
        let key = match &b.site {
            Site::InRoom { room, target } => (format!("{room}/{target}"), b.verb.clone()),
            Site::InConversation {
                conversation,
                option,
            } => (format!("{conversation}/{option}"), b.verb.clone()),
            Site::OnEnter { .. } => unreachable!(),
        };
        let mut ever = false;
        for (i, st) in states.iter().enumerate() {
            if verb_menu::in_menu(&ix, b, st) {
                ever = true;
                menu_slots += 1;
                shown_ever.insert(key.clone());
                if i == 0 {
                    shown_at_start.insert(key.clone());
                }
                // **The silent miss, which could not be looked for until the lines were kept.**
                // A binding shown in this state whose `when` does not hold declines, and if no
                // clause of its refusal list covers the state it declines without a word.
                // `subsystem/interaction-resolver`: *"never nothing — a silent miss is the failure
                // players read as brokenness"*.
                // Asked as `offered` + `select` rather than through `resolve`, because `resolve`
                // applies the effect list to build the state it would leave and this loop runs
                // once per binding per reachable state. The two are the same question: a binding
                // in the menu that is not offered is one that declines.
                // Which line this binding says here — the `says` list when it fires, the
                // `else` list when it declines, because those are the two things a player can
                // be told and a list is only read in its own case.
                let declines = !resolver::offered(&ix, b, st);
                let list = if declines { &b.refusals } else { &b.says };
                if let Some(r) = resolver::clause(&ix, list, st) {
                    read.insert(r.line);
                }
                if declines && resolver::select(&ix, &b.refusals, st).is_none() {
                    silent.push(format!(
                        "`{}{}` at line {} is shown and declines in a reachable state, and its \
                         `else` says nothing there",
                        b.verb,
                        b.object
                            .as_ref()
                            .map(|o| format!(" {o}"))
                            .unwrap_or_default(),
                        b.line
                    ));
                }
            }
        }
        if !ever {
            unshown.push(format!(
                "`{}{}` at line {} is in no menu in any reachable state, so it is written and \
                 never shown",
                b.verb,
                b.object
                    .as_ref()
                    .map(|o| format!(" {o}"))
                    .unwrap_or_default(),
                b.line
            ));
        }
    }
    unshown.sort();
    unshown.dedup();
    silent.sort();
    silent.dedup();

    // **Authored lines no reachable state selects.** A response list is first-match-wins, so a
    // clause is reached only where every clause above it fails; the search knows which states
    // exist and the resolver knows which clause each one picks, and the two together are the
    // whole question. A binding in no menu orphans every line under it, which is `unshown`
    // counted a second way and is left in: the line is unreachable either way, and an author
    // fixing the binding and an author fixing the guard are doing different work.
    let mut orphaned: Vec<String> = Vec::new();
    for b in &game.bindings {
        for (list, kind) in [(&b.says, "line"), (&b.refusals, "refusal")] {
            for r in list {
                if !read.contains(&r.line) {
                    orphaned.push(format!(
                        "line {}: the {kind} under `{}{}` at line {} — written, and reached in no state a \
                         player can get to",
                        r.line,
                        b.verb,
                        b.object
                            .as_ref()
                            .map(|o| format!(" {o}"))
                            .unwrap_or_default(),
                        b.line
                    ));
                }
            }
        }
    }
    orphaned.sort();
    orphaned.dedup();

    // the corpus claims a latching flag never goes back; hold it against the game, both ways
    let mut latch_violations = Vec::new();
    let mut unclaimed_latches = Vec::new();
    for f in &game.flags {
        let written = |want_clear: bool| {
            edges.iter().find_map(|(_, _, bi)| {
                let b = acting[*bi];
                b.effects
                    .iter()
                    .any(|e| match e {
                        Effect::ClearFlag(x) => want_clear && *x == f.name,
                        Effect::SetFlag(x) => !want_clear && *x == f.name,
                        _ => false,
                    })
                    .then_some(b.line)
            })
        };
        match (f.latched, written(true), written(false)) {
            (true, Some(line), _) => latch_violations.push(format!(
                "`{}` is declared latched and line {} clears it",
                f.name, line
            )),
            // A flag nothing sets is a constant rather than a latch, and is somebody else's
            // finding; a flag the game only ever sets has the guarantee and is not claiming it.
            // A flag no predicate reads is neither: `unread` has already said to delete it, and
            // telling an author to declare a property of something that should not exist is the
            // checker preferring its own tidiness to theirs.
            (false, None, Some(line)) if ix.flags.contains(&f.name) => {
                unclaimed_latches.push(format!(
                    "`{}` is set at line {} and nothing anywhere clears it, so it is latched \
                 in fact and does not say so",
                    f.name, line
                ))
            }
            _ => {}
        }
    }
    latch_violations.sort();
    latch_violations.dedup();
    unclaimed_latches.sort();
    unclaimed_latches.dedup();

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
                // an item nothing reads has no count to overflow; `unread` is its finding
                let Some(i) = ix.items.iter().position(|y| y == x) else {
                    continue;
                };
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
        unclaimed_latches,
        unread,
        menu_entries: (shown_ever.len(), shown_at_start.len()),
        menu_slots,
        unshown,
        silent,
        monotone_counters,
        overflows,
        observed_space: observed,
        orphaned,
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
        if !self.unread.is_empty() {
            s.push_str("\nDeclared, written, and readable by nothing:\n");
            for v in &self.unread {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.unshown.is_empty() {
            s.push_str("\nAuthored and shown to nobody:\n");
            for v in &self.unshown {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.silent.is_empty() {
            s.push_str("\nShown, declining, and saying nothing:\n");
            for v in &self.silent {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.orphaned.is_empty() {
            s.push_str("\nWritten, and selected in no reachable state:\n");
            for v in &self.orphaned {
                s.push_str(&format!("  {v}\n"));
            }
        }
        if !self.unclaimed_latches.is_empty() {
            s.push_str("\nFlags that latch and do not say so:\n");
            for v in &self.unclaimed_latches {
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
